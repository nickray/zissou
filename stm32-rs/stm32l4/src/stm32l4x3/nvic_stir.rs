#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software trigger interrupt register"]
    pub stir: STIR,
}
#[doc = "Software trigger interrupt register"]
pub struct STIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software trigger interrupt register"]
pub mod stir;
