#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - frame control register"]
    pub fcr: FCR,
    #[doc = "0x08 - status register"]
    pub sr: SR,
    #[doc = "0x0c - clear register"]
    pub clr: CLR,
    _reserved0: [u8; 4usize],
    #[doc = "0x14 - display memory"]
    pub ram_com0: RAM_COM0,
    _reserved1: [u8; 4usize],
    #[doc = "0x1c - display memory"]
    pub ram_com1: RAM_COM1,
    _reserved2: [u8; 4usize],
    #[doc = "0x24 - display memory"]
    pub ram_com2: RAM_COM2,
    _reserved3: [u8; 4usize],
    #[doc = "0x2c - display memory"]
    pub ram_com3: RAM_COM3,
    _reserved4: [u8; 4usize],
    #[doc = "0x34 - display memory"]
    pub ram_com4: RAM_COM4,
    _reserved5: [u8; 4usize],
    #[doc = "0x3c - display memory"]
    pub ram_com5: RAM_COM5,
    _reserved6: [u8; 4usize],
    #[doc = "0x44 - display memory"]
    pub ram_com6: RAM_COM6,
    _reserved7: [u8; 4usize],
    #[doc = "0x4c - display memory"]
    pub ram_com7: RAM_COM7,
}
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "frame control register"]
pub struct FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "frame control register"]
pub mod fcr;
#[doc = "status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "clear register"]
pub struct CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "clear register"]
pub mod clr;
#[doc = "display memory"]
pub struct RAM_COM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "display memory"]
pub mod ram_com0;
#[doc = "display memory"]
pub struct RAM_COM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "display memory"]
pub mod ram_com1;
#[doc = "display memory"]
pub struct RAM_COM2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "display memory"]
pub mod ram_com2;
#[doc = "display memory"]
pub struct RAM_COM3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "display memory"]
pub mod ram_com3;
#[doc = "display memory"]
pub struct RAM_COM4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "display memory"]
pub mod ram_com4;
#[doc = "display memory"]
pub struct RAM_COM5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "display memory"]
pub mod ram_com5;
#[doc = "display memory"]
pub struct RAM_COM6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "display memory"]
pub mod ram_com6;
#[doc = "display memory"]
pub struct RAM_COM7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "display memory"]
pub mod ram_com7;
