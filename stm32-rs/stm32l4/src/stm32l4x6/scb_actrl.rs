#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Auxiliary control register"]
    pub actrl: ACTRL,
}
#[doc = "Auxiliary control register"]
pub struct ACTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary control register"]
pub mod actrl;
