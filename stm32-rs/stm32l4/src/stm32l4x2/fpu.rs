#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Floating-point context control register"]
    pub fpccr: FPCCR,
    #[doc = "0x04 - Floating-point context address register"]
    pub fpcar: FPCAR,
    #[doc = "0x08 - Floating-point status control register"]
    pub fpscr: FPSCR,
}
#[doc = "Floating-point context control register"]
pub struct FPCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Floating-point context control register"]
pub mod fpccr;
#[doc = "Floating-point context address register"]
pub struct FPCAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Floating-point context address register"]
pub mod fpcar;
#[doc = "Floating-point status control register"]
pub struct FPSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Floating-point status control register"]
pub mod fpscr;
