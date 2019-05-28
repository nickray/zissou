#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x08 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x0c - interrupt status register"]
    pub isr: ISR,
    #[doc = "0x10 - I/O hysteresis control register"]
    pub iohcr: IOHCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x18 - I/O analog switch control register"]
    pub ioascr: IOASCR,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - I/O sampling control register"]
    pub ioscr: IOSCR,
    _reserved2: [u8; 4usize],
    #[doc = "0x28 - I/O channel control register"]
    pub ioccr: IOCCR,
    _reserved3: [u8; 4usize],
    #[doc = "0x30 - I/O group control status register"]
    pub iogcsr: IOGCSR,
    #[doc = "0x34 - I/O group x counter register"]
    pub iog1cr: IOGCR,
    #[doc = "0x38 - I/O group x counter register"]
    pub iog2cr: IOGCR,
    #[doc = "0x3c - I/O group x counter register"]
    pub iog3cr: IOGCR,
    #[doc = "0x40 - I/O group x counter register"]
    pub iog4cr: IOGCR,
    #[doc = "0x44 - I/O group x counter register"]
    pub iog5cr: IOGCR,
    #[doc = "0x48 - I/O group x counter register"]
    pub iog6cr: IOGCR,
    #[doc = "0x4c - I/O group x counter register"]
    pub iog7cr: IOGCR,
    #[doc = "0x50 - I/O group x counter register"]
    pub iog8cr: IOGCR,
}
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "interrupt enable register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "interrupt clear register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "interrupt status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "I/O hysteresis control register"]
pub struct IOHCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O hysteresis control register"]
pub mod iohcr;
#[doc = "I/O analog switch control register"]
pub struct IOASCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O analog switch control register"]
pub mod ioascr;
#[doc = "I/O sampling control register"]
pub struct IOSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O sampling control register"]
pub mod ioscr;
#[doc = "I/O channel control register"]
pub struct IOCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O channel control register"]
pub mod ioccr;
#[doc = "I/O group control status register"]
pub struct IOGCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O group control status register"]
pub mod iogcsr;
#[doc = "I/O group x counter register"]
pub struct IOGCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O group x counter register"]
pub mod iogcr;
