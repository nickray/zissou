extern crate core;
#[cfg(feature = "unproven")]
use core::cmp;

use crate::rcc::Clocks;
use crate::device::{RCC, RNG};

/// Constrained RNG peripheral
pub struct Rng {
    rng: RNG,
}

impl Rng {

    // the new constructor approach
    pub fn new(rng: RNG, clocks: Clocks) -> Rng {
        // clocks is only passed to play nanny state, does this make sense?
        // can we catch this at compile time instead of panicking?

        // crrcr.crrcr().modify(|_, w| w.hsi48on().set_bit()); // p. 180 in ref-manual
        // ...this is now supposed to be done in RCC configuration before freezing

        // hsi48 should be turned on previously or msi at 48mhz
        let msi = match clocks.msi() {
            Some(msi) => msi == crate::rcc::MsiFreq::RANGE48M,
            None => false,
        };
        let hsi = clocks.hsi48();
        assert!(msi || hsi);

        let _self = Self {
            rng: rng
        };

        _self.enable();

        _self
    }

    fn enable(&self) {
        // need to steal access to RCC, more precisely,
        // RCC.AHB2ENR.RNGEN, which is morally ours

        let rcc = unsafe { (&*RCC::ptr()) };
        // RM0394, 6.2.18 mentions: "After the enable bit is set,
        // there is a 2 clocks delay before the clock be (sic!) active."
        // This is to prevent glitches in the peripheral clocks.
        // TODO: does this need a critical section?
        rcc.ahb2enr.modify(|_, w| w.rngen().set_bit());
        while rcc.ahb2enr.read().rngen().bit_is_clear() {};

        self.rng.cr.modify(|_, w| w.rngen().set_bit());
    }

    #[allow(dead_code)]
    fn disable(&self) {
        self.rng.cr.modify(|_, w| w.rngen().clear_bit());

        let rcc = unsafe { (&*RCC::ptr()) };
        rcc.ahb2enr.modify(|_, w| w.rngen().clear_bit());
        // Probably this is not needed (the peripheral is off,
        // so no need to prevent "clock glitches", see `enable`)
        // while rcc.ahb2enr.read().rngen().bit_is_set() {};
    }

    // cf. https://github.com/nrf-rs/nrf51-hal/blob/master/src/rng.rs#L31
    pub fn free(self) -> RNG {
        // maybe disable the RNG?
        self.rng
    }

    // various methods that are not in the blessed embedded_hal
    // trait list, but may be helpful nonetheless
    // Q: should these be prefixed by underscores?

    pub fn get_random_data(&self)-> u32 {
        while !self.is_data_ready() {}
        let word = self.possibly_invalid_random_data();
        // NB: no need to clear bit here
        word
    }

    // RNG_CR
    /* missing in stm32l4...
    pub fn is_clock_error_detection_enabled(&self) -> bool {
        self.rng.cr.read().ced().bit()
    }
    */

    pub fn is_interrupt_enabled(&self) -> bool {
        self.rng.cr.read().ie().bit()
    }

    pub fn is_enabled(&self) -> bool {
        self.rng.cr.read().rngen().bit()
    }

    // RNG_SR
    pub fn is_clock_error(&self) -> bool {
        self.rng.sr.read().cecs().bit()
    }

    pub fn is_seed_error(&self) -> bool {
        self.rng.sr.read().secs().bit()
    }

    pub fn is_data_ready(&self) -> bool {
        self.rng.sr.read().drdy().bit()
    }

    // RNG_DR
    pub fn possibly_invalid_random_data(&self) -> u32 {
        self.rng.dr.read().rndata().bits()
    }

}

#[derive(Debug)]
pub enum Error {}

#[cfg(feature = "unproven")]
impl crate::hal::blocking::rng::Read for Rng {

    // TODO: this error seems pretty useless if it
    // doesn't flag non-enabled RNG or non-started HSI48,
    // but that would be a runtime cost :/
    type Error = Error;

    fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {

        let mut i = 0usize;
        while i < buffer.len() {
            let random_word: u32 = self.get_random_data();
            let bytes: [u8; 4] = random_word.to_ne_bytes();
            let n = cmp::min(4, buffer.len() - i);
            buffer[i..i + n].copy_from_slice(&bytes[..n]);
            i += n;
        }

        Ok(())

    }
}
