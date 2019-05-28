#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBGMCU_IDCODE"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Debug MCU configuration register"]
    pub cr: CR,
    #[doc = "0x08 - Debug MCU APB1 freeze register1"]
    pub apb1fzr1: APB1FZR1,
    #[doc = "0x0c - Debug MCU APB1 freeze register 2"]
    pub apb1fzr2: APB1FZR2,
    #[doc = "0x10 - Debug MCU APB2 freeze register"]
    pub apb2fzr: APB2FZR,
}
#[doc = "DBGMCU_IDCODE"]
pub struct IDCODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DBGMCU_IDCODE"]
pub mod idcode;
#[doc = "Debug MCU configuration register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug MCU configuration register"]
pub mod cr;
#[doc = "Debug MCU APB1 freeze register1"]
pub struct APB1FZR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug MCU APB1 freeze register1"]
pub mod apb1fzr1;
#[doc = "Debug MCU APB1 freeze register 2"]
pub struct APB1FZR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug MCU APB1 freeze register 2"]
pub mod apb1fzr2;
#[doc = "Debug MCU APB2 freeze register"]
pub struct APB2FZR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug MCU APB2 freeze register"]
pub mod apb2fzr;
