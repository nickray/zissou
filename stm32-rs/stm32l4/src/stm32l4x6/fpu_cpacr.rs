#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Coprocessor access control register"]
    pub cpacr: CPACR,
}
#[doc = "Coprocessor access control register"]
pub struct CPACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Coprocessor access control register"]
pub mod cpacr;
