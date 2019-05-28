#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - interrupt status register"]
    pub isr: ISR,
    #[doc = "0x04 - interrupt flag clear register"]
    pub ifcr: IFCR,
    #[doc = "0x08 - channel x configuration register"]
    pub ccr1: CCR1,
    #[doc = "0x0c - channel x number of data register"]
    pub cndtr1: CNDTR1,
    #[doc = "0x10 - channel x peripheral address register"]
    pub cpar1: CPAR1,
    #[doc = "0x14 - channel x memory address register"]
    pub cmar1: CMAR1,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - channel x configuration register"]
    pub ccr2: CCR2,
    #[doc = "0x20 - channel x number of data register"]
    pub cndtr2: CNDTR2,
    #[doc = "0x24 - channel x peripheral address register"]
    pub cpar2: CPAR2,
    #[doc = "0x28 - channel x memory address register"]
    pub cmar2: CMAR2,
    _reserved1: [u8; 4usize],
    #[doc = "0x30 - channel x configuration register"]
    pub ccr3: CCR3,
    #[doc = "0x34 - channel x number of data register"]
    pub cndtr3: CNDTR3,
    #[doc = "0x38 - channel x peripheral address register"]
    pub cpar3: CPAR3,
    #[doc = "0x3c - channel x memory address register"]
    pub cmar3: CMAR3,
    _reserved2: [u8; 4usize],
    #[doc = "0x44 - channel x configuration register"]
    pub ccr4: CCR4,
    #[doc = "0x48 - channel x number of data register"]
    pub cndtr4: CNDTR4,
    #[doc = "0x4c - channel x peripheral address register"]
    pub cpar4: CPAR4,
    #[doc = "0x50 - channel x memory address register"]
    pub cmar4: CMAR4,
    _reserved3: [u8; 4usize],
    #[doc = "0x58 - channel x configuration register"]
    pub ccr5: CCR5,
    #[doc = "0x5c - channel x number of data register"]
    pub cndtr5: CNDTR5,
    #[doc = "0x60 - channel x peripheral address register"]
    pub cpar5: CPAR5,
    #[doc = "0x64 - channel x memory address register"]
    pub cmar5: CMAR5,
    _reserved4: [u8; 4usize],
    #[doc = "0x6c - channel x configuration register"]
    pub ccr6: CCR6,
    #[doc = "0x70 - channel x number of data register"]
    pub cndtr6: CNDTR6,
    #[doc = "0x74 - channel x peripheral address register"]
    pub cpar6: CPAR6,
    #[doc = "0x78 - channel x memory address register"]
    pub cmar6: CMAR6,
    _reserved5: [u8; 4usize],
    #[doc = "0x80 - channel x configuration register"]
    pub ccr7: CCR7,
    #[doc = "0x84 - channel x number of data register"]
    pub cndtr7: CNDTR7,
    #[doc = "0x88 - channel x peripheral address register"]
    pub cpar7: CPAR7,
    #[doc = "0x8c - channel x memory address register"]
    pub cmar7: CMAR7,
    _reserved6: [u8; 24usize],
    #[doc = "0xa8 - channel selection register"]
    pub cselr: CSELR,
}
#[doc = "interrupt status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt status register"]
pub mod isr;
#[doc = "interrupt flag clear register"]
pub struct IFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt flag clear register"]
pub mod ifcr;
#[doc = "channel x configuration register"]
pub struct CCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x configuration register"]
pub mod ccr1;
#[doc = "channel x number of data register"]
pub struct CNDTR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x number of data register"]
pub mod cndtr1;
#[doc = "channel x peripheral address register"]
pub struct CPAR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x peripheral address register"]
pub mod cpar1;
#[doc = "channel x memory address register"]
pub struct CMAR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x memory address register"]
pub mod cmar1;
#[doc = "channel x configuration register"]
pub struct CCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x configuration register"]
pub mod ccr2;
#[doc = "channel x number of data register"]
pub struct CNDTR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x number of data register"]
pub mod cndtr2;
#[doc = "channel x peripheral address register"]
pub struct CPAR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x peripheral address register"]
pub mod cpar2;
#[doc = "channel x memory address register"]
pub struct CMAR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x memory address register"]
pub mod cmar2;
#[doc = "channel x configuration register"]
pub struct CCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x configuration register"]
pub mod ccr3;
#[doc = "channel x number of data register"]
pub struct CNDTR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x number of data register"]
pub mod cndtr3;
#[doc = "channel x peripheral address register"]
pub struct CPAR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x peripheral address register"]
pub mod cpar3;
#[doc = "channel x memory address register"]
pub struct CMAR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x memory address register"]
pub mod cmar3;
#[doc = "channel x configuration register"]
pub struct CCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x configuration register"]
pub mod ccr4;
#[doc = "channel x number of data register"]
pub struct CNDTR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x number of data register"]
pub mod cndtr4;
#[doc = "channel x peripheral address register"]
pub struct CPAR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x peripheral address register"]
pub mod cpar4;
#[doc = "channel x memory address register"]
pub struct CMAR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x memory address register"]
pub mod cmar4;
#[doc = "channel x configuration register"]
pub struct CCR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x configuration register"]
pub mod ccr5;
#[doc = "channel x number of data register"]
pub struct CNDTR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x number of data register"]
pub mod cndtr5;
#[doc = "channel x peripheral address register"]
pub struct CPAR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x peripheral address register"]
pub mod cpar5;
#[doc = "channel x memory address register"]
pub struct CMAR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x memory address register"]
pub mod cmar5;
#[doc = "channel x configuration register"]
pub struct CCR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x configuration register"]
pub mod ccr6;
#[doc = "channel x number of data register"]
pub struct CNDTR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x number of data register"]
pub mod cndtr6;
#[doc = "channel x peripheral address register"]
pub struct CPAR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x peripheral address register"]
pub mod cpar6;
#[doc = "channel x memory address register"]
pub struct CMAR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x memory address register"]
pub mod cmar6;
#[doc = "channel x configuration register"]
pub struct CCR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x configuration register"]
pub mod ccr7;
#[doc = "channel x number of data register"]
pub struct CNDTR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x number of data register"]
pub mod cndtr7;
#[doc = "channel x peripheral address register"]
pub struct CPAR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x peripheral address register"]
pub mod cpar7;
#[doc = "channel x memory address register"]
pub struct CMAR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel x memory address register"]
pub mod cmar7;
#[doc = "channel selection register"]
pub struct CSELR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel selection register"]
pub mod cselr;
