#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - VREF control and status register"]
    pub csr: CSR,
    #[doc = "0x04 - calibration control register"]
    pub ccr: CCR,
}
#[doc = "VREF control and status register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VREF control and status register"]
pub mod csr;
#[doc = "calibration control register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "calibration control register"]
pub mod ccr;
