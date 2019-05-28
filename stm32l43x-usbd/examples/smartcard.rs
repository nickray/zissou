#![no_std]
#![no_main]

// extern crate panic_halt;
extern crate panic_semihosting;
use cortex_m_rt::entry;

use stm32l4xx_hal as hal;
use hal::prelude::*;

use usb_device::prelude::*;
use stm32l43x_usbd::UsbBus;
use cortex_m_semihosting::hprintln;

mod ccid;
mod cdc_acm;

#[entry]
fn main() -> ! {
    let dp = hal::device::Peripherals::take().unwrap();

    // let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .hsi48(true)
        .freeze();

    // let flash = hal::flash::Flash::new(dp.FLASH);
    // flash.unlock();
    // flash.option_bytes_unlock();
    // flash.set_boot_from_flash();
    // flash.option_bytes_lock();
    // flash.lock();

    // hprintln!("").unwrap();
    // hprintln!("").unwrap();

    let flash = hal::flash::Flash::new(dp.FLASH);
    hprintln!("option bytes {:b}", flash.get_optr()).unwrap();

    /*
    hprintln!("flash RDP = {}", flash.get_rdp()).unwrap();

    flash.unlock();
    flash.option_bytes_unlock();

    flash.set_boot_from_flash();
    */

    // hprintln!("boot bits: {:?}", flash.get_boot_bits()).unwrap();

    /*
    let page = 100;

    hprintln!("LET'S GO").unwrap();
    flash.erase_page(page).expect("could not erase page");

    let some_bytes: [u8; 8] = [1, 2, 3, 4, 0xA, 0xB, 0xC, 0xD];
    flash.write(0x8032000, &some_bytes).expect("could not write dword");

    // let mut hmm = [0u8; 4];
    // let hmm = flash.read_word(08032000);
    // hprintln!("{:?}", hmm).unwrap();

    let mut check_bytes = [0u8; 8];
    flash.read(0x8032000, &mut check_bytes);
    hprintln!("{:?}", check_bytes).unwrap();
    assert!(some_bytes == check_bytes);

    hprintln!("AGAIN").unwrap();
    // flash.erase_page(page).expect("could not erase page");

    let some_other_bytes = [5u8, 6, 7, 8, 0xA1, 0xB1, 0xC1, 0xD1];
    flash.write(0x8032008, &some_other_bytes).expect("could not write dword");
    flash.read(0x8032008, &mut check_bytes);
    hprintln!("{:?}", check_bytes).unwrap();
    assert!(some_other_bytes == check_bytes);

    let mut moar_bytes = [0u8; 16];
    flash.read(0x8032000, &mut moar_bytes);
    assert!(some_bytes == moar_bytes[..8]);
    assert!(some_other_bytes == moar_bytes[8..]);


    hprintln!("GREAT SUCCESS, ALL DONE!").unwrap();
    */

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    let usb_dp = gpioa.pa12.into_af10(&mut gpioa.moder, &mut gpioa.afrh);

    // disable Vddusb power isolation
    let pwr = dp.PWR.constrain(&mut rcc.apb1r1); // turns it on
    pwr.enable_usb();

    let usb_bus = UsbBus::usb_with_reset(dp.USB,
        &mut rcc.apb1r1, &clocks, &mut gpioa.moder, &mut gpioa.otyper, usb_dp);

    let mut smartcard = ccid::SmartCard::new(&usb_bus);
    let mut serial = cdc_acm::SerialPort::new(&usb_bus);

    // vid/pid: http://pid.codes/1209/CC1D/
    let mut usb_dev = UsbDeviceBuilder::new(
            &usb_bus,
            UsbVidPid(0x1209, 0xcc1d),
        )
        .manufacturer("HardcoreBits")
        .product("Zissou")
        .serial_number("N/a")
        .build();

    // usb_dev.force_reset().expect("reset failed");

    let mut buf = [0u8; 64];
    loop {
        if !usb_dev.poll(&mut [&mut smartcard, &mut serial]) {
            continue;
        }

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                serial.write(&buf[0..count]).ok();
            },
            _ => { },
        }
    }
}
