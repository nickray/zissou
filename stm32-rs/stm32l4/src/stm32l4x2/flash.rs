#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access control register"]
    pub acr: ACR,
    #[doc = "0x04 - Power down key register"]
    pub pdkeyr: PDKEYR,
    #[doc = "0x08 - Flash key register"]
    pub keyr: KEYR,
    #[doc = "0x0c - Option byte key register"]
    pub optkeyr: OPTKEYR,
    #[doc = "0x10 - Status register"]
    pub sr: SR,
    #[doc = "0x14 - Flash control register"]
    pub cr: CR,
    #[doc = "0x18 - Flash ECC register"]
    pub eccr: ECCR,
    _reserved0: [u8; 4usize],
    #[doc = "0x20 - Flash option register"]
    pub optr: OPTR,
    #[doc = "0x24 - Flash Bank 1 PCROP Start address register"]
    pub pcrop1sr: PCROP1SR,
    #[doc = "0x28 - Flash Bank 1 PCROP End address register"]
    pub pcrop1er: PCROP1ER,
    #[doc = "0x2c - Flash Bank 1 WRP area A address register"]
    pub wrp1ar: WRP1AR,
    #[doc = "0x30 - Flash Bank 1 WRP area B address register"]
    pub wrp1br: WRP1BR,
    _reserved1: [u8; 16usize],
    #[doc = "0x44 - Flash Bank 2 PCROP Start address register"]
    pub pcrop2sr: PCROP2SR,
    #[doc = "0x48 - Flash Bank 2 PCROP End address register"]
    pub pcrop2er: PCROP2ER,
    #[doc = "0x4c - Flash Bank 2 WRP area A address register"]
    pub wrp2ar: WRP2AR,
    #[doc = "0x50 - Flash Bank 2 WRP area B address register"]
    pub wrp2br: WRP2BR,
}
#[doc = "Access control register"]
pub struct ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access control register"]
pub mod acr;
#[doc = "Power down key register"]
pub struct PDKEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power down key register"]
pub mod pdkeyr;
#[doc = "Flash key register"]
pub struct KEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash key register"]
pub mod keyr;
#[doc = "Option byte key register"]
pub struct OPTKEYR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Option byte key register"]
pub mod optkeyr;
#[doc = "Status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod sr;
#[doc = "Flash control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash control register"]
pub mod cr;
#[doc = "Flash ECC register"]
pub struct ECCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash ECC register"]
pub mod eccr;
#[doc = "Flash option register"]
pub struct OPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash option register"]
pub mod optr;
#[doc = "Flash Bank 1 PCROP Start address register"]
pub struct PCROP1SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 1 PCROP Start address register"]
pub mod pcrop1sr;
#[doc = "Flash Bank 1 PCROP End address register"]
pub struct PCROP1ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 1 PCROP End address register"]
pub mod pcrop1er;
#[doc = "Flash Bank 1 WRP area A address register"]
pub struct WRP1AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 1 WRP area A address register"]
pub mod wrp1ar;
#[doc = "Flash Bank 1 WRP area B address register"]
pub struct WRP1BR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 1 WRP area B address register"]
pub mod wrp1br;
#[doc = "Flash Bank 2 PCROP Start address register"]
pub struct PCROP2SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 2 PCROP Start address register"]
pub mod pcrop2sr;
#[doc = "Flash Bank 2 PCROP End address register"]
pub struct PCROP2ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 2 PCROP End address register"]
pub mod pcrop2er;
#[doc = "Flash Bank 2 WRP area A address register"]
pub struct WRP2AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 2 WRP area A address register"]
pub mod wrp2ar;
#[doc = "Flash Bank 2 WRP area B address register"]
pub struct WRP2BR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Bank 2 WRP area B address register"]
pub mod wrp2br;
