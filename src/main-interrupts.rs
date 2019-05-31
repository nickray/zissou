#![no_std]
#![no_main]
#![feature(proc_macro_hygiene)]

// use core::fmt;
use cortex_m::asm::wfi;
use ufmt::uwrite;
extern crate panic_halt;
use cortex_m_rt::entry;

use hal::prelude::*;
use hal::stm32::{interrupt, Interrupt};
use stm32l4xx_hal as hal;

use stm32l43x_usbd::UsbBus;
use usb_device::{prelude::*, bus::UsbBusAllocator};

mod usb;

/// This is the version number generated by git.
// static GIT_DESCRIBE: &'static str = env!("GIT_DESCRIBE");
static CCID_PRODUCT: &'static str = concat!("Zissou v", env!("GIT_DESCRIBE"));

// TODO: use RTFM instead
static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_CCID: Option<usb::ccid::SmartCard<UsbBus>> = None;
static mut USB_SERIAL: Option<usb::cdc_acm::SerialPort<UsbBus>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBus>> = None;

#[entry]
fn main() -> ! {
    let cp = hal::cortex_m::Peripherals::take().unwrap();
    let dp = hal::device::Peripherals::take().unwrap();

    // let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .hsi48(true)
        .freeze();

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    let usb_dp = gpioa.pa12.into_af10(&mut gpioa.moder, &mut gpioa.afrh);

    // disable Vddusb power isolation
    let pwr = dp.PWR.constrain(&mut rcc.apb1r1); // turns it on
    pwr.enable_usb();

    unsafe {
        let usb_bus = UsbBus::usb_with_reset(
            dp.USB,
            &mut rcc.apb1r1,
            &clocks,
            &mut gpioa.moder,
            &mut gpioa.otyper,
            usb_dp,
        );
        USB_BUS = Some(usb_bus);

        // let mut smartcard = usb::ccid::SmartCard::new(&usb_bus);
        USB_CCID = Some(usb::ccid::SmartCard::new(USB_BUS.as_ref().unwrap()));
        // let mut serial = usb::cdc_acm::SerialPort::new(&usb_bus);
        USB_SERIAL = Some(usb::cdc_acm::SerialPort::new(USB_BUS.as_ref().unwrap()));

        // let mut serial2 = usbd_serial::cdc_acm::CdcAcmClass::new(&usb_bus, 64);

        // vid/pid: http://pid.codes/1209/CC1D/
        let mut usb_dev = UsbDeviceBuilder::new(
                USB_BUS.as_ref().unwrap(),
                UsbVidPid(0x1209, 0xcc1d),
            )
            .manufacturer("HardcoreBits")
            .product(CCID_PRODUCT)
            .serial_number("∴ wfi ∴")
            .build();

        // "ensures that the host re-enumerates your device after a new program has been flashed."
        // usb_dev.force_reset().expect("reset failed");

        USB_DEVICE = Some(usb_dev);
    }

    let mut nvic = cp.NVIC;
    nvic.enable(Interrupt::USB);

    let mut buf = [0u8; 64];
    let mut i: u32 = 0;

    loop {
        wfi();
    }
}

#[interrupt]
fn USB() {
    usb_interrupt();
}

fn usb_interrupt() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    let smartcard = unsafe { USB_CCID.as_mut().unwrap() };
    let serial = unsafe { USB_SERIAL.as_mut().unwrap() };

    // if !usb_dev.poll(&mut [serial]) {
    if !usb_dev.poll(&mut [smartcard, serial]) {
        return;
    }

    let mut buf = [0u8; 8];

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