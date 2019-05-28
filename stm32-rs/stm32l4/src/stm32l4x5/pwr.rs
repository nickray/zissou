#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - Power control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - Power control register 3"]
    pub cr3: CR3,
    #[doc = "0x0c - Power control register 4"]
    pub cr4: CR4,
    #[doc = "0x10 - Power status register 1"]
    pub sr1: SR1,
    #[doc = "0x14 - Power status register 2"]
    pub sr2: SR2,
    #[doc = "0x18 - Power status clear register"]
    pub scr: SCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Power Port A pull-up control register"]
    pub pucra: PUCRA,
    #[doc = "0x24 - Power Port A pull-down control register"]
    pub pdcra: PDCRA,
    #[doc = "0x28 - Power Port B pull-up control register"]
    pub pucrb: PUCRB,
    #[doc = "0x2c - Power Port B pull-down control register"]
    pub pdcrb: PDCRB,
    #[doc = "0x30 - Power Port C pull-up control register"]
    pub pucrc: PUCRC,
    #[doc = "0x34 - Power Port C pull-down control register"]
    pub pdcrc: PDCRC,
    #[doc = "0x38 - Power Port D pull-up control register"]
    pub pucrd: PUCRD,
    #[doc = "0x3c - Power Port D pull-down control register"]
    pub pdcrd: PDCRD,
    #[doc = "0x40 - Power Port E pull-up control register"]
    pub pucre: PUCRE,
    #[doc = "0x44 - Power Port E pull-down control register"]
    pub pdcre: PDCRE,
    #[doc = "0x48 - Power Port F pull-up control register"]
    pub pucrf: PUCRF,
    #[doc = "0x4c - Power Port F pull-down control register"]
    pub pdcrf: PDCRF,
    #[doc = "0x50 - Power Port G pull-up control register"]
    pub pucrg: PUCRG,
    #[doc = "0x54 - Power Port G pull-down control register"]
    pub pdcrg: PDCRG,
    #[doc = "0x58 - Power Port H pull-up control register"]
    pub pucrh: PUCRH,
    #[doc = "0x5c - Power Port H pull-down control register"]
    pub pdcrh: PDCRH,
}
#[doc = "Power control register 1"]
pub struct CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power control register 1"]
pub mod cr1;
#[doc = "Power control register 2"]
pub struct CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power control register 2"]
pub mod cr2;
#[doc = "Power control register 3"]
pub struct CR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power control register 3"]
pub mod cr3;
#[doc = "Power control register 4"]
pub struct CR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power control register 4"]
pub mod cr4;
#[doc = "Power status register 1"]
pub struct SR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power status register 1"]
pub mod sr1;
#[doc = "Power status register 2"]
pub struct SR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power status register 2"]
pub mod sr2;
#[doc = "Power status clear register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power status clear register"]
pub mod scr;
#[doc = "Power Port A pull-up control register"]
pub struct PUCRA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port A pull-up control register"]
pub mod pucra;
#[doc = "Power Port A pull-down control register"]
pub struct PDCRA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port A pull-down control register"]
pub mod pdcra;
#[doc = "Power Port B pull-up control register"]
pub struct PUCRB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port B pull-up control register"]
pub mod pucrb;
#[doc = "Power Port B pull-down control register"]
pub struct PDCRB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port B pull-down control register"]
pub mod pdcrb;
#[doc = "Power Port C pull-up control register"]
pub struct PUCRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port C pull-up control register"]
pub mod pucrc;
#[doc = "Power Port C pull-down control register"]
pub struct PDCRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port C pull-down control register"]
pub mod pdcrc;
#[doc = "Power Port D pull-up control register"]
pub struct PUCRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port D pull-up control register"]
pub mod pucrd;
#[doc = "Power Port D pull-down control register"]
pub struct PDCRD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port D pull-down control register"]
pub mod pdcrd;
#[doc = "Power Port E pull-up control register"]
pub struct PUCRE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port E pull-up control register"]
pub mod pucre;
#[doc = "Power Port E pull-down control register"]
pub struct PDCRE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port E pull-down control register"]
pub mod pdcre;
#[doc = "Power Port F pull-up control register"]
pub struct PUCRF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port F pull-up control register"]
pub mod pucrf;
#[doc = "Power Port F pull-down control register"]
pub struct PDCRF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port F pull-down control register"]
pub mod pdcrf;
#[doc = "Power Port G pull-up control register"]
pub struct PUCRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port G pull-up control register"]
pub mod pucrg;
#[doc = "Power Port G pull-down control register"]
pub struct PDCRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port G pull-down control register"]
pub mod pdcrg;
#[doc = "Power Port H pull-up control register"]
pub struct PUCRH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port H pull-up control register"]
pub mod pucrh;
#[doc = "Power Port H pull-down control register"]
pub struct PDCRH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Port H pull-down control register"]
pub mod pdcrh;
