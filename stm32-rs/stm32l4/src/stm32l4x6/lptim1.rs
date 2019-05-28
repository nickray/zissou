#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt and Status Register"]
    pub isr: ISR,
    #[doc = "0x04 - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x0c - Configuration Register"]
    pub cfgr: CFGR,
    #[doc = "0x10 - Control Register"]
    pub cr: CR,
    #[doc = "0x14 - Compare Register"]
    pub cmp: CMP,
    #[doc = "0x18 - Autoreload Register"]
    pub arr: ARR,
    #[doc = "0x1c - Counter Register"]
    pub cnt: CNT,
}
#[doc = "Interrupt and Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt and Status Register"]
pub mod isr;
#[doc = "Interrupt Clear Register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Configuration Register"]
pub struct CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod cfgr;
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Compare Register"]
pub struct CMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Register"]
pub mod cmp;
#[doc = "Autoreload Register"]
pub struct ARR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Autoreload Register"]
pub mod arr;
#[doc = "Counter Register"]
pub struct CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Register"]
pub mod cnt;
