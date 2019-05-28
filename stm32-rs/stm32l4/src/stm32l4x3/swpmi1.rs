#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SWPMI Configuration/Control register"]
    pub cr: CR,
    #[doc = "0x04 - SWPMI Bitrate register"]
    pub brr: BRR,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - SWPMI Interrupt and Status register"]
    pub isr: ISR,
    #[doc = "0x10 - SWPMI Interrupt Flag Clear register"]
    pub icr: ICR,
    #[doc = "0x14 - SWPMI Interrupt Enable register"]
    pub ier: IER,
    #[doc = "0x18 - SWPMI Receive Frame Length register"]
    pub rfl: RFL,
    #[doc = "0x1c - SWPMI Transmit data register"]
    pub tdr: TDR,
    #[doc = "0x20 - SWPMI Receive data register"]
    pub rdr: RDR,
}
#[doc = "SWPMI Configuration/Control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SWPMI Configuration/Control register"]
pub mod cr;
#[doc = "SWPMI Bitrate register"]
pub struct BRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SWPMI Bitrate register"]
pub mod brr;
#[doc = "SWPMI Interrupt and Status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SWPMI Interrupt and Status register"]
pub mod isr;
#[doc = "SWPMI Interrupt Flag Clear register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SWPMI Interrupt Flag Clear register"]
pub mod icr;
#[doc = "SWPMI Interrupt Enable register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SWPMI Interrupt Enable register"]
pub mod ier;
#[doc = "SWPMI Receive Frame Length register"]
pub struct RFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SWPMI Receive Frame Length register"]
pub mod rfl;
#[doc = "SWPMI Transmit data register"]
pub struct TDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SWPMI Transmit data register"]
pub mod tdr;
#[doc = "SWPMI Receive data register"]
pub struct RDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SWPMI Receive data register"]
pub mod rdr;
