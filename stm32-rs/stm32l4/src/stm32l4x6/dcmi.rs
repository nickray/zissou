#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - raw interrupt status register"]
    pub ris: RIS,
    #[doc = "0x0c - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x10 - masked interrupt status register"]
    pub mis: MIS,
    #[doc = "0x14 - interrupt clear register"]
    pub icr: ICR,
    #[doc = "0x18 - embedded synchronization code register"]
    pub escr: ESCR,
    #[doc = "0x1c - embedded synchronization unmask register"]
    pub esur: ESUR,
    #[doc = "0x20 - crop window start"]
    pub cwstrt: CWSTRT,
    #[doc = "0x24 - crop window size"]
    pub cwsize: CWSIZE,
    #[doc = "0x28 - data register"]
    pub dr: DR,
}
#[doc = "control register 1"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod cr;
#[doc = "status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "raw interrupt status register"]
pub struct RIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "raw interrupt status register"]
pub mod ris;
#[doc = "interrupt enable register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "masked interrupt status register"]
pub struct MIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "masked interrupt status register"]
pub mod mis;
#[doc = "interrupt clear register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt clear register"]
pub mod icr;
#[doc = "embedded synchronization code register"]
pub struct ESCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "embedded synchronization code register"]
pub mod escr;
#[doc = "embedded synchronization unmask register"]
pub struct ESUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "embedded synchronization unmask register"]
pub mod esur;
#[doc = "crop window start"]
pub struct CWSTRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "crop window start"]
pub mod cwstrt;
#[doc = "crop window size"]
pub struct CWSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "crop window size"]
pub mod cwsize;
#[doc = "data register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register"]
pub mod dr;
