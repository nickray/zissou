#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SysTick control and status register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - SysTick reload value register"]
    pub load: LOAD,
    #[doc = "0x08 - SysTick current value register"]
    pub val: VAL,
    #[doc = "0x0c - SysTick calibration value register"]
    pub calib: CALIB,
}
#[doc = "SysTick control and status register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick control and status register"]
pub mod ctrl;
#[doc = "SysTick reload value register"]
pub struct LOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick reload value register"]
pub mod load;
#[doc = "SysTick current value register"]
pub struct VAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick current value register"]
pub mod val;
#[doc = "SysTick calibration value register"]
pub struct CALIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick calibration value register"]
pub mod calib;
