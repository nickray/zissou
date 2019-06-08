#![no_main]
#![no_std]
#![allow(non_snake_case)]
#![feature(proc_macro_hygiene)]

// extern crate panic_halt;
extern crate panic_semihosting;

mod gordon;
mod usb;

use rtfm::{app, Instant};
use stm32l4xx_hal::prelude::*;
use stm32l4xx_hal as hal;
use stm32l43x_usbd::UsbBus;
use ufmt::uwrite;
use usb_device::bus;
use usb_device::prelude::*;
use littlefs;
use littlefs::FileOpenFlags;
use byteorder::ByteOrder;

use cortex_m_semihosting::hprintln;

static CCID_PRODUCT: &'static str = concat!("Zissou v", env!("GIT_DESCRIBE"));

const PERIOD: u32 = 8_000_000;

fn experiment_with_lfs(lfs: &mut littlefs::LittleFs<gordon::Gordon>) {
    hprintln!("Attempting to mount LFS...").unwrap();
    if let Err(_) = lfs.mount() {
        hprintln!("...failed, attempting to format").unwrap();
        lfs.format().unwrap();
        hprintln!("...success formatting, now mounting again").unwrap();
        lfs.mount().unwrap();
        hprintln!("...success mounting").unwrap();
    } else {
        hprintln!("...done mounting").unwrap();
    }

    let mut random_file = Default::default();
    lfs.file_open(
        &mut random_file,
        "random_file",
        FileOpenFlags::RDWR | FileOpenFlags::CREAT,
    ).unwrap();
    lfs.file_write(&mut random_file, &[0u8, 1, 2, 3, 4]).unwrap();
    lfs.file_sync(random_file).unwrap();

    let mut random_file = Default::default();
    lfs.file_open(
        &mut random_file,
        "random_file",
        FileOpenFlags::RDWR | FileOpenFlags::CREAT,
    ).unwrap();
    let mut buf = [0u8; 12];
    let n = lfs.file_read(&mut random_file, &mut buf).unwrap();
    hprintln!("buf = {:?}", &buf[..n]).unwrap();

    let fs_size = lfs.fs_size().unwrap();
    hprintln!("Size of FS: {} blocks", fs_size).unwrap();
    let mut boot_count: u32 = 0;
    let mut file = Default::default();
    hprintln!("Opening `boot_count` file...").unwrap();
    lfs.file_open(
        &mut file,
        "boot_count",
        FileOpenFlags::RDWR | FileOpenFlags::CREAT,
    ).unwrap();
    hprintln!("...yay!").unwrap();

    // pub fn file_read(&mut self, file: &mut File, buf: &mut [u8]) -> Result<usize, FsError> {
    let mut read_buf = [0u8; 4];
    match lfs.file_read(&mut file, &mut read_buf) {
        Ok(4) => {
            hprintln!("Read 4 bytes").unwrap();
            boot_count = byteorder::NativeEndian::read_u32(&read_buf);
        }
        Ok(n) => {
            hprintln!("Read {} bytes", n).unwrap();
        },
        _ => {
            // panic?
            hprintln!("Failed to read!").unwrap();
        },
    }

    hprintln!("boot_count = {}", boot_count).unwrap();

    boot_count += 1;
    lfs.file_rewind(&mut file).unwrap();
    let mut write_buf = [0u8; 4];
    byteorder::NativeEndian::write_u32(
        &mut write_buf,
        boot_count,
    );
    lfs.file_write(&mut file, &write_buf).unwrap();
    lfs.file_close(file).unwrap();
}

#[app(device = stm32l4xx_hal::stm32)]
const APP: () = {
    static mut USB_DEV: UsbDevice<'static, UsbBus> = ();
    static mut CCID: usb::ccid::SmartCard<'static, UsbBus> = ();
    static mut SERIAL: usb::cdc_acm::SerialPort<'static, UsbBus> = ();
    // static mut GORDON: gordon::Gordon = ();
    static mut LFS: littlefs::LittleFs<gordon::Gordon> = ();

    // #[init(schedule = [heartbeat])]
    #[init]
    fn init() {
        static mut USB_BUS: Option<bus::UsbBusAllocator<UsbBus>> = None;

        let mut rcc = device.RCC.constrain();

        let clocks = rcc
            .cfgr
            // .sysclk(8.mhz())
            // .sysclk(80.mhz())
            .sysclk(48.mhz())
            // .pclk1(24.mhz())
            .hsi48(true)
            .freeze();

        let flash = hal::flash::Flash::new(device.FLASH);
        let storage = gordon::Gordon::new(flash);
        let mut lfs = littlefs::LittleFs::new(storage);
        experiment_with_lfs(&mut lfs);



        let mut gpioa = device.GPIOA.split(&mut rcc.ahb2);

        let usb_dp = gpioa.pa12.into_af10(&mut gpioa.moder, &mut gpioa.afrh);

        // disable Vddusb power isolation
        let pwr = device.PWR.constrain(&mut rcc.apb1r1); // turns it on
        pwr.enable_usb();

        *USB_BUS = Some(UsbBus::usb_with_reset(
            device.USB,
            &mut rcc.apb1r1,
            &clocks,
            &mut gpioa.moder,
            &mut gpioa.otyper,
            usb_dp,
        ));

        let ccid = usb::ccid::SmartCard::new(USB_BUS.as_ref().unwrap());
        let serial = usb::cdc_acm::SerialPort::new(USB_BUS.as_ref().unwrap());

        let mut usb_dev =
            UsbDeviceBuilder::new(USB_BUS.as_ref().unwrap(), UsbVidPid(0x1209, 0xcc1d))
                .manufacturer("Hardcore Bits")
                .product(CCID_PRODUCT)
                .serial_number("∴ RTFM ∴ AEAD ∴")
                .build();

        // usb_dev.force_reset().expect("reset failed");

        // schedule.heartbeat(Instant::now() + PERIOD.cycles()).unwrap();

        USB_DEV = usb_dev;
        CCID = ccid;
        SERIAL = serial;
        LFS = lfs;

    }

    #[interrupt(resources = [USB_DEV, CCID, SERIAL])]
    fn USB() {
        usb_poll(
            &mut resources.USB_DEV,
            &mut resources.CCID,
            &mut resources.SERIAL,
        );
    }

    #[task(schedule = [heartbeat], resources = [SERIAL])]
    // #[task(schedule = [heartbeat], resources = [SERIAL], priority = 4)]
    // #[task(schedule = [heartbeat], priority = 4)]
    fn heartbeat() {
        let now = Instant::now();
        // let serial: &mut usb::cdc_acm::SerialPort<'static, B> = &mut resources.SERIAL;
        // resources.SERIAL.lock(|serial| {
        //     // uwrite!(&mut serial, "heartbeat(scheduled = {:?}, now = {:?})", scheduled, now).unwrap();
        // });
        // uwrite!(&mut serial, "heartbeat(scheduled = {:?}, now = {:?})", scheduled, now).unwrap();
        // uwrite!(&mut serial, "hello {} hello\r\n", i).unwrap();

        schedule.heartbeat(scheduled + PERIOD.cycles()).unwrap();
    }

    extern "C" {
        fn UART4();
    }
};

fn usb_poll<B: bus::UsbBus>(
    usb_dev: &mut UsbDevice<'static, B>,
    ccid: &mut usb::ccid::SmartCard<'static, B>,
    serial: &mut usb::cdc_acm::SerialPort<'static, B>,
) {
    if !usb_dev.poll(&mut [ccid, serial]) {
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
        }
        _ => {}
    }
}
