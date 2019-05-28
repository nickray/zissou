#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Debug MCU Configuration Register"]
    pub cr: CR,
    #[doc = "0x08 - APB Low Freeze Register 1"]
    pub apb1_fzr1: APB1_FZR1,
    #[doc = "0x0c - APB Low Freeze Register 2"]
    pub apb1_fzr2: APB1_FZR2,
    #[doc = "0x10 - APB High Freeze Register"]
    pub apb2_fzr: APB2_FZR,
}
#[doc = "MCU Device ID Code Register"]
pub struct IDCODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "Debug MCU Configuration Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB Low Freeze Register 1"]
pub struct APB1_FZR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB Low Freeze Register 1"]
pub mod apb1_fzr1;
#[doc = "APB Low Freeze Register 2"]
pub struct APB1_FZR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB Low Freeze Register 2"]
pub mod apb1_fzr2;
#[doc = "APB High Freeze Register"]
pub struct APB2_FZR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "APB High Freeze Register"]
pub mod apb2_fzr;
