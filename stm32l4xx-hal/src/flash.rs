//! Flash memory

use crate::stm32::FLASH;

#[cfg(feature = "extra-traits")]
use byteorder::ByteOrder;

#[cfg(feature = "extra-traits")]
use crate::hal::flash::{FlashError, FlashResult, Read, WriteErase, LockingImpl, Locking};
// #[cfg(feature = "extra-traits")]
// use crate::hal::flash::{UnlockGuard, UnlockResult};

#[cfg(feature = "extra-traits")]
// use generic_array::{ArrayLength, GenericArray};
use generic_array::GenericArray;

#[allow(dead_code)]
pub struct Flash {
    flash: FLASH
}

impl Flash {
    // the new constructor approach
    pub fn new(flash: FLASH) -> Flash {
        Self {
            flash: flash
        }
    }
}


// inspired by https://github.com/idubrov/stm32-hal/blob/master/src/flash.rs

pub const FLASH_ORIGIN: usize = 0x08000000;

#[cfg(feature = "extra-traits")]
const FLASH_KEY1: u32 = 0x4567_0123;
#[cfg(feature = "extra-traits")]
const FLASH_KEY2: u32 = 0xCDEF_89AB;
#[cfg(feature = "extra-traits")]
const OPTION_BYTES_FLASH_KEY1: u32 = 0x0819_2A3B;
#[cfg(feature = "extra-traits")]
const OPTION_BYTES_FLASH_KEY2: u32 = 0x4C5D_6E7F;

#[cfg(all(feature = "stm32l4x2", feature="extra-traits"))]
impl LockingImpl for Flash {
    fn is_locked(&self) -> bool {
        self.flash.cr.read().lock().bit_is_set()
    }

    /// unlocks the Flash.
    fn unlock(&mut self) {
        // ehh.. should check BSY here
        // peripheral stalls if it's already locked, so check
        if self.is_locked() {
            unsafe {
                self.flash.keyr.write(|w| w.keyr().bits(FLASH_KEY1));
                self.flash.keyr.write(|w| w.keyr().bits(FLASH_KEY2));
            }
        }
    }

    /// locks the flash
    fn lock(&mut self) {
        self.flash.cr.modify(|_, w| w.lock().set_bit());
    }
}

#[cfg(all(feature = "stm32l4x2", feature="extra-traits"))]
impl Locking for Flash {}

pub trait OptionBytesLocking {
    fn option_bytes_are_locked(&self) -> bool;
    fn option_bytes_unlock(&self);
    fn option_bytes_lock(&self);
}

#[cfg(all(feature = "stm32l4x2", feature="extra-traits"))]
impl OptionBytesLocking for Flash {
    fn option_bytes_are_locked(&self) -> bool {
        self.flash.cr.read().optlock().bit_is_set()
    }

    /// unlocks the Flash.
    fn option_bytes_unlock(&self) {
        // need to make OptionBytesLocking aware of Locking trait
        // if self.locked() {
        //     self.unlock();
        // }
        if self.option_bytes_are_locked() {
            unsafe {
                self.flash.optkeyr.write(|w| w.optkeyr().bits(OPTION_BYTES_FLASH_KEY1));
                self.flash.optkeyr.write(|w| w.optkeyr().bits(OPTION_BYTES_FLASH_KEY2));
            }
        }
    }

    /// locks the flash
    fn option_bytes_lock(&self) {
        self.flash.cr.modify(|_, w| w.optlock().set_bit());
    }
}

impl Flash {
    pub fn get_rdp(&self) -> u8 {
        let rdp_bits = self.flash.optr.read().rdp().bits();
        match rdp_bits {
            0xAA => 0,
            0xCC => 2,
            _ => 1,
        }
    }

    // NB: only effects system after power reset
    // (alternatively, could set FLASH.CR.OBL_LAUNCH)
    #[cfg(all(feature = "stm32l4x2", feature="extra-traits"))]
    pub fn set_rdp(&self, level: u8) {
        assert!(level <= 2);

        let rdp_bits = match level {
            0 => 0xAA,
            2 => 0xCC,
            _ => 0  // anything other than AA and CC
        };

        self.option_bytes_unlock();
        unsafe { self.flash.optr.modify(|_, w| w.rdp().bits(rdp_bits)); }

        // initiate writing
        self.flash.cr.modify(|_, w| w.optstrt().set_bit());

        // wait until done
        while self.flash.sr.read().bsy().bit_is_set() {}

        // optionally, reload options block (from register to internal registers)
        self.flash.cr.modify(|_, w| w.obl_launch().set_bit());
    }

    // cf. set_rdp
    /// DANGER ZONE!
    // If you run this on a NUCLEO-L432KC, you will have issues
    // using OpenOCD/GDB afterwards. Not bricked beyond repair, but annoying.
    //
    // The fix is to keep Nucleo in reset, run something like
    // $ openocd.cfg -c "program something.elf verify reset exit
    // while releasing reset button, where something.elf is a binary
    // that fixes the boot flags
    pub fn set_boot_from_rom(&self) {
        self.flash.optr.modify(|_, w| w.n_swboot0().clear_bit());
        self.flash.optr.modify(|_, w| w.n_boot0().clear_bit());
        // would reset immediately
        // self.flash.cr.modify(|_, w| w.obl_launch().set_bit());
    }

    // cf. set_rdp
    pub fn set_boot_from_flash(&self) {
        self.flash.optr.modify(|_, w| w.n_swboot0().clear_bit());
        self.flash.optr.modify(|_, w| w.n_boot0().set_bit());
        // would reset immediately
        // self.flash.cr.modify(|_, w| w.obl_launch().set_bit());
    }

    pub fn get_boot_bits(&self) -> (bool, bool, bool) {
        (
            self.flash.optr.read().n_boot0().bit(),
            self.flash.optr.read().n_swboot0().bit(),
            self.flash.optr.read().n_boot1().bit(),
        )
    }
    pub fn get_optr(&self) -> u32 {
        self.flash.optr.read().bits()
    }
}

#[cfg(feature = "stm32l4x2")]
pub const READ_SIZE: usize = 8;
#[cfg(feature = "stm32l4x2")]
pub const WRITE_SIZE: usize = 8;
#[cfg(feature = "stm32l4x2")]
pub const PAGE_SIZE: usize = 2048;

#[cfg(all(feature = "stm32l4x2", feature="extra-traits"))]
// impl Read for Flash {
impl Read<generic_array::typenum::U8> for Flash {
    // flash read is two consecutive u32 words, aligned
    fn read_native(&self, address: usize, array: &mut GenericArray<u8, generic_array::typenum::U8>) {
        unsafe {
            byteorder::NativeEndian::write_u32(
                &mut array.as_mut_slice()[..4],
                core::ptr::read_volatile(address as *mut u32),
            );
            byteorder::NativeEndian::write_u32(
                &mut array.as_mut_slice()[4..],
                core::ptr::read_volatile((address + 4) as *mut u32),
            );
        }
    }
}

#[cfg(all(feature = "stm32l4x2", feature="extra-traits"))]
impl WriteErase<generic_array::typenum::U2048, generic_array::typenum::U8> for Flash {
    fn status(&self) -> FlashResult {
        let sr = self.flash.sr.read();
        if sr.bsy().bit_is_set() {
            Err(FlashError::Busy)
        } else if sr.progerr().bit_is_set() {
            Err(FlashError::ProgrammingError)
        } else if sr.wrperr().bit_is_set() {
            Err(FlashError::WriteProtectionError)
        } else {
            Ok(())
        }
    }

    fn write_native(&mut self, address: usize,
                    array: &GenericArray<u8, generic_array::typenum::U8>) -> FlashResult {
        self.status()?;

        // enable programming
        self.flash.cr.modify(|_, w| w.pg().set_bit());

        // write words consecutively
        unsafe {
            // Program the first word
            core::ptr::write_volatile(
                address as *mut u32,
                byteorder::NativeEndian::read_u32(&array.as_slice()[..4])
            );
            // Program the second word
            core::ptr::write_volatile(
                (address + 4) as *mut u32,
                byteorder::NativeEndian::read_u32(&array.as_slice()[4..])
            );
        }

        // wait until done
        while self.flash.sr.read().bsy().bit_is_set() {}

        // disable programming
        self.flash.cr.modify(|_, w| w.pg().clear_bit());

        match self.flash.sr.read().bits() {
            0 => Ok(()),
            _ => Err(FlashError::ProgrammingError),
        }
    }

    // TODO: use critical section?
    fn erase_page(&mut self, page: u8) -> FlashResult {
        self.status()?;

        // enable page erase
        self.flash.cr.modify(|_, w| w.per().set_bit());
        // set page number
        unsafe { self.flash.cr.modify(|_, w| w.pnb().bits(page)); }
        // start erase page
        self.flash.cr.modify(|_, w| w.start().set_bit());
        // wait until done
        while !self.flash.sr.read().bsy().bit_is_clear() {}
        // disable page erase
        self.flash.cr.modify(|_, w| w.per().clear_bit());

        Ok(())
    }

    fn erase_all_pages(&mut self) -> FlashResult {
        self.status()?;

        // enable mass erase
        self.flash.cr.modify(|_, w| w.mer1().set_bit());
        // start mass erase
        self.flash.cr.modify(|_, w| w.start().set_bit());
        // wait until done
        while !self.flash.sr.read().bsy().bit_is_clear() {}
        // disable mass erase
        self.flash.cr.modify(|_, w| w.mer1().clear_bit());

        Ok(())
    }

}
