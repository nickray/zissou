#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Code segment start address"]
    pub cssa: CSSA,
    #[doc = "0x04 - Code segment length"]
    pub csl: CSL,
    #[doc = "0x08 - Non-volatile data segment start address"]
    pub nvdssa: NVDSSA,
    #[doc = "0x0c - Non-volatile data segment length"]
    pub nvdsl: NVDSL,
    #[doc = "0x10 - Volatile data segment start address"]
    pub vdssa: VDSSA,
    #[doc = "0x14 - Volatile data segment length"]
    pub vdsl: VDSL,
    _reserved0: [u8; 8usize],
    #[doc = "0x20 - Configuration register"]
    pub cr: CR,
}
#[doc = "Code segment start address"]
pub struct CSSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Code segment start address"]
pub mod cssa;
#[doc = "Code segment length"]
pub struct CSL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Code segment length"]
pub mod csl;
#[doc = "Non-volatile data segment start address"]
pub struct NVDSSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-volatile data segment start address"]
pub mod nvdssa;
#[doc = "Non-volatile data segment length"]
pub struct NVDSL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non-volatile data segment length"]
pub mod nvdsl;
#[doc = "Volatile data segment start address"]
pub struct VDSSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Volatile data segment start address"]
pub mod vdssa;
#[doc = "Volatile data segment length"]
pub struct VDSL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Volatile data segment length"]
pub mod vdsl;
#[doc = "Configuration register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod cr;
