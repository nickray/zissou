use core::slice;
use core::mem;
use bare_metal::CriticalSection;
use vcell::VolatileCell;
use cortex_m::interrupt;
use stm32l4xx_hal::stm32::{USB, usb};
use usb_device::{Result, UsbError};
use usb_device::endpoint::EndpointType;
use crate::atomic_mutex::AtomicMutex;

// use cortex_m_semihosting::hprintln;

// this could/should probably be a VolatileCell<u8>, as L432
// does not have the limitations of F103
type EndpointBuffer = &'static mut [VolatileCell<u16>];

pub const NUM_ENDPOINTS: usize = 8;

// NB: in contrast to the F103, the "USB SRAM" (starting at 0x4000_6000)
// is half-word (16 bit) wide.

#[repr(C)]
// #[derive(Debug)]
struct BufferDescriptor {
    pub addr_tx: VolatileCell<u16>,
    pub count_tx: VolatileCell<u16>,
    pub addr_rx: VolatileCell<u16>,
    pub count_rx: VolatileCell<u16>,
}

/// Arbitrates access to the endpoint-specific registers and packet buffer memory.
#[derive(Default)]
pub struct Endpoint {
    out_buf: Option<AtomicMutex<EndpointBuffer>>,
    in_buf: Option<AtomicMutex<EndpointBuffer>>,
    ep_type: Option<EndpointType>,
    index: u8,
}

pub fn calculate_count_rx(mut size: usize) -> Result<(usize, u16)> {
    // USB_COUNTn_RX entries are (BLSIZE, NUM_BLOCK[4:0], COUNTn_RX[9:0]).
    // If BLSIZE = 0, then the buffer size is 2*NUM_BLOCK byte
    // If BLSIZE = 1, then the buffer size is 32*(NUM_BLOCK - 1) bytes
    if size <= 62 {
        // Buffer size is in units of 2 bytes, 0 = 0 bytes
        size = (size + 1) & !0x01; // round up to next multiple of two

        let size_bits = size >> 1; // to be placed in NUM_BLOCK

        Ok((
            size,   // actual size of buffer
            (size_bits << 10) as u16  // the NUM_BLOCK bits (start at bit 10)
        ))
    } else if size <= 1024 {
        // Buffer size is in units of 32 bytes, 0 = 32 bytes
        size = (size + 31) & !0x1f;  // round up to next multiple of thirty-two

        let size_bits = (size >> 5) - 1;  // to be placed in NUM_BLOCK

        Ok((size, (0x8000 | (size_bits << 10)) as u16))  // as above
    } else {
        Err(UsbError::EndpointMemoryOverflow)
    }
}

impl Endpoint {
    // start of USB SRAM
    // const MEM_ADDR: *mut VolatileCell<u16> = 0x4000_6000 as *mut VolatileCell<u16>;
    const MEM_ADDR: *mut VolatileCell<u16> = 0x4000_6C00 as *mut VolatileCell<u16>;
    // total size of USB SRAM in bytes
    pub const MEM_SIZE: usize = 1024;
    // offset in USB SRAM where endpoint buffers start (i.e., after buffer table descriptors)
    pub const MEM_START: u16 = (mem::size_of::<BufferDescriptor>() * NUM_ENDPOINTS) as u16;

    pub fn new(index: u8) -> Endpoint {
        // nothing allocated in constructor
        Endpoint {
            out_buf: None,
            in_buf: None,
            ep_type: None,
            index,
        }
    }

    fn make_buf(addr: u16, size: usize)
        -> Option<AtomicMutex<&'static mut [VolatileCell<u16>]>>
    {
        // divide `addr` and `size` by two since they refer to
        // bytes, but EndpointBuffer contains halfwords
        Some(AtomicMutex::new(
            unsafe {
                slice::from_raw_parts_mut(
                    Self::MEM_ADDR.offset((addr >> 1) as isize),
                    size >> 1)
            }
        ))
    }

    pub fn ep_type(&self) -> Option<EndpointType> {
        self.ep_type
    }

    pub fn set_ep_type(&mut self, ep_type: EndpointType) {
        self.ep_type = Some(ep_type);
    }

    pub fn is_out_buf_set(&self) -> bool {
        self.out_buf.is_some()
    }

    pub fn set_out_buf(&mut self, addr: u16, size_and_bits: (usize, u16)) {
        self.out_buf = Self::make_buf(addr, size_and_bits.0);
        // hprintln!("made out_buf for ep{} at addr 0x{:x} of size {} with bits {}",
        //     self.index, addr, size_and_bits.0, size_and_bits.1).unwrap();

        // hprintln!("BEFORE...");
        // hprintln!(" addr_tx (hex) = 0x{:x}", self.descr().addr_tx.get()).unwrap();
        // hprintln!("count_tx (hex) = 0x{:x}", self.descr().count_tx.get()).unwrap();
        // hprintln!(" addr_rx (hex) = 0x{:x}", self.descr().addr_rx.get()).unwrap();
        // hprintln!("count_rx (hex) = 0x{:x}", self.descr().count_rx.get()).unwrap();
        let descr = self.descr();
        descr.addr_rx.set(addr);
        descr.count_rx.set(size_and_bits.1); // this sets NUM_BLOCK
        // hprintln!("DOUBLE CHECKING!");
        // hprintln!(" addr_tx (hex) = 0x{:x}", self.descr().addr_tx.get()).unwrap();
        // hprintln!("count_tx (hex) = 0x{:x}", self.descr().count_tx.get()).unwrap();
        // hprintln!(" addr_rx (hex) = 0x{:x}", self.descr().addr_rx.get()).unwrap();
        // hprintln!("count_rx (hex) = 0x{:x}", self.descr().count_rx.get()).unwrap();
    }

    pub fn is_in_buf_set(&self) -> bool {
        self.in_buf.is_some()
    }

    pub fn set_in_buf(&mut self, addr: u16, max_packet_size: usize) {
        self.in_buf = Self::make_buf(addr, max_packet_size);
        // hprintln!("made  in_buf for ep{} at addr 0x{:x} of size {}",
        //     self.index, addr, max_packet_size).unwrap();

        let descr = self.descr();
        descr.addr_tx.set(addr);
        // RM: "packet size IN is limited by USB spec to 1023 bytes,
        // and an explicit transmission specifies the count. So there
        // is no analogue of OUT's (BL_SIZE, NUM_BLOCK[4:0]) for IN
        descr.count_tx.set(0);
    }

    fn descr(&self) -> &'static BufferDescriptor {
        // implicitly, we place buffer table descriptors at start of USB SRAM area
        // (in principle, they could be elsewhere, but this is the natural choice
        // FYI: BufferDescriptor is 8 bytes sized
        unsafe { &*(Self::MEM_ADDR as *const BufferDescriptor).offset(self.index as isize) }
    }

    fn reg(&self) -> &'static usb::EPR {
        unsafe { &(*USB::ptr()).epr[self.index as usize] }
    }

    pub fn configure(&self, cs: &CriticalSection) {
        let ep_type = match self.ep_type {
            Some(t) => t,
            None => { return },
        };

        self.reg().modify(|_, w|
            Self::clear_toggle_bits(w)
                .ctr_rx().clear_bit()
                // dtog_rx
                // stat_rx
                .ep_type().bits(ep_type.bits())
                .ep_kind().clear_bit()
                .ctr_tx().clear_bit()
                // dtog_rx
                // stat_tx
                .ea().bits(self.index));

        if self.out_buf.is_some() {
            self.set_stat_rx(cs, EndpointStatus::Valid);
        }

        if self.in_buf.is_some() {
            self.set_stat_tx(cs, EndpointStatus::Nak);
        }
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        let guard = self.in_buf.as_ref().unwrap().try_lock();

        let in_buf = match guard {
            Some(ref b) => b,
            None => { return Err(UsbError::WouldBlock); }
        };

        if buf.len() > in_buf.len() << 1 {
            return Err(UsbError::BufferOverflow);
        }

        let reg = self.reg();

        match reg.read().stat_tx().bits().into() {
            EndpointStatus::Valid | EndpointStatus::Disabled => return Err(UsbError::WouldBlock),
            _ => {},
        };

        self.write_mem(in_buf, buf);
        self.descr().count_tx.set(buf.len() as u16);

        interrupt::free(|cs| {
            self.set_stat_tx(cs, EndpointStatus::Valid);
        });

        Ok(buf.len())
    }

    fn write_mem(&self, mem: &[VolatileCell<u16>], mut buf: &[u8]) {
        // rewrite when `mem` becomes a VolatileCell<u8>
        let mut addr = 0;

        while buf.len() >= 2 {
            mem[addr].set(buf[0] as u16 | ((buf[1] as u16) << 8));
            addr += 1;

            buf = &buf[2..];
        }

        if buf.len() > 0 {
            mem[addr].set(buf[0] as u16);
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let guard = self.out_buf.as_ref().unwrap().try_lock();

        let out_buf = match guard {
            Some(ref b) => b,
            None => { return Err(UsbError::WouldBlock); }
        };
        // hprintln!("ep{}", self.index).unwrap();
        // hprintln!("len(self.out_buf) = {}", out_buf.len()).unwrap();

        let reg = self.reg();
        let reg_v = reg.read();

        let status: EndpointStatus = reg_v.stat_rx().bits().into();

        if status == EndpointStatus::Disabled || !reg_v.ctr_rx().bit_is_set() {
            return Err(UsbError::WouldBlock);
        }

        // hprintln!(" addr_tx (hex) = 0x{:x}", self.descr().addr_tx.get()).unwrap();
        // hprintln!("count_tx (hex) = 0x{:x}", self.descr().count_tx.get()).unwrap();
        // hprintln!(" addr_rx (hex) = 0x{:x}", self.descr().addr_rx.get()).unwrap();
        // hprintln!("count_rx (hex) = 0x{:x}", self.descr().count_rx.get()).unwrap();
        let count = (self.descr().count_rx.get() & 0x3ff) as usize;
        // hprintln!("attempting read of count = {}", count).unwrap();
        if count > buf.len() {
            return Err(UsbError::BufferOverflow);
        }

        self.read_mem(out_buf, &mut buf[0..count]);

        interrupt::free(|cs| {
            self.clear_ctr_rx(cs);
            self.set_stat_rx(cs, EndpointStatus::Valid);
        });

        Ok(count)
    }

    fn read_mem(&self, mem: &[VolatileCell<u16>], mut buf: &mut [u8]) {
        let mut addr = 0;

        // hprintln!("len(mem) = {}, len(buf) = {}", mem.len(), buf.len()).unwrap();
        while buf.len() >= 2 {
            let word = mem[addr].get();

            buf[0] = word as u8;
            buf[1] = (word >> 8) as u8;

            addr += 1;

            buf = &mut {buf}[2..];
        }

        if buf.len() > 0 {
            buf[0] = mem[addr].get() as u8;
        }
    }

    pub fn read_reg(&self) -> usb::epr::R {
        self.reg().read()
    }

    /*pub fn modify<F>(&self, _cs: CriticalSection, f: F)
        where for<'w> F: FnOnce(&usb::epr::R, &'w mut usb::epr::W) -> &'w mut usb::epr::W
    {
        self.reg().modify(f)
    }*/

    fn clear_toggle_bits(w: &mut usb::epr::W) -> &mut usb::epr::W {
        w
            .dtog_rx().clear_bit()
            .dtog_tx().clear_bit()
            .stat_rx().bits(0)
            .stat_tx().bits(0)
    }

    pub fn clear_ctr_rx(&self, _cs: &CriticalSection) {
        self.reg().modify(|_, w| Self::clear_toggle_bits(w).ctr_rx().clear_bit());
    }

    pub fn clear_ctr_tx(&self, _cs: &CriticalSection) {
        self.reg().modify(|_, w| Self::clear_toggle_bits(w).ctr_tx().clear_bit());
    }

    pub fn set_stat_rx(&self, _cs: &CriticalSection, status: EndpointStatus) {
        self.reg().modify(|r, w| Self::clear_toggle_bits(w)
                .stat_rx().bits(r.stat_rx().bits() ^ (status as u8))
        );
    }

    pub fn set_stat_tx(&self, _cs: &CriticalSection, status: EndpointStatus) {
        self.reg().modify(|r, w| Self::clear_toggle_bits(w)
                .stat_tx().bits(r.stat_tx().bits() ^ (status as u8))
        );
    }
}

/*#[repr(transparent)]
struct EndpointReg(usb::EPR);

impl EndpointReg {
}*/

trait EndpointTypeExt {
    fn bits(self) -> u8;
}

impl EndpointTypeExt for EndpointType {
    fn bits(self) -> u8 {
        const BITS: [u8; 4] = [0b01, 0b10, 0b00, 0b11];
        return BITS[self as usize];
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Debug)]
#[allow(unused)]
pub enum EndpointStatus {
    Disabled = 0b00,
    Stall = 0b01,
    Nak = 0b10,
    Valid = 0b11,
}

impl From<u8> for EndpointStatus {
    fn from(v: u8) -> EndpointStatus {
        if v <= 0b11 {
            unsafe { mem::transmute(v) }
        } else {
            EndpointStatus::Disabled
        }
    }
}
