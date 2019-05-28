#![no_std]
#![no_main]

/// CDC-ACM serial port example using interrupts.

extern crate panic_semihosting;

use cortex_m::asm::wfi;
use cortex_m_rt::entry;
use stm32l4xx_hal::{prelude::*, stm32};
use stm32l4xx_hal::stm32::{interrupt, Interrupt};

use usb_device::{prelude::*, bus::UsbBusAllocator};
use stm32l43x_usbd::UsbBus;

mod cdc_acm;

static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_SERIAL: Option<cdc_acm::SerialPort<UsbBus>> = None;
static mut USB_DEVICE: Option<UsbDevice<UsbBus>> = None;

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        // .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        // .pclk2(24.mhz())
        .hsi48(true)
        .freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    let usb_dp = gpioa.pa12.into_af10(&mut gpioa.moder, &mut gpioa.afrh);

    // disable Vddusb power isolation
    let pwr = dp.PWR.constrain(&mut rcc.apb1r1); // turns it on
    pwr.enable_usb();

    // Unsafe to allow access to static variables
    unsafe {
        let usb_bus = UsbBus::usb_with_reset(dp.USB,
            &mut rcc.apb1r1, &clocks, &mut gpioa.moder, &mut gpioa.otyper, usb_dp);

        USB_BUS = Some(usb_bus);

        USB_SERIAL = Some(cdc_acm::SerialPort::new(USB_BUS.as_ref().unwrap()));

        let mut usb_dev = UsbDeviceBuilder::new(
                &usb_bus,
                UsbVidPid(0x1209, 0x5070),
            )
            .manufacturer("Hardcore Bits")
            .product("USB in Rust on NUCLEO-L432KC")
            .serial_number("12.05.2019")
            .device_class(cdc_acm::USB_CLASS_CDC)
            .build();

        // usb_dev.force_reset().expect("reset failed");

        USB_DEVICE = Some(usb_dev);
    }

    let mut nvic = p.NVIC;

    nvic.enable(Interrupt::USB);

    loop { wfi(); }
}

#[interrupt]
fn USB() {
    usb_interrupt();
}

fn usb_interrupt() {
    let usb_dev = unsafe { USB_DEVICE.as_mut().unwrap() };
    let serial = unsafe { USB_SERIAL.as_mut().unwrap() };

    if !usb_dev.poll(&mut [serial]) {
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
