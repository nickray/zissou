#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - device configuration register"]
    pub dcr: DCR,
    #[doc = "0x08 - status register"]
    pub sr: SR,
    #[doc = "0x0c - flag clear register"]
    pub fcr: FCR,
    #[doc = "0x10 - data length register"]
    pub dlr: DLR,
    #[doc = "0x14 - communication configuration register"]
    pub ccr: CCR,
    #[doc = "0x18 - address register"]
    pub ar: AR,
    #[doc = "0x1c - ABR"]
    pub abr: ABR,
    #[doc = "0x20 - data register"]
    pub dr: DR,
    #[doc = "0x24 - polling status mask register"]
    pub psmkr: PSMKR,
    #[doc = "0x28 - polling status match register"]
    pub psmar: PSMAR,
    #[doc = "0x2c - polling interval register"]
    pub pir: PIR,
    #[doc = "0x30 - low-power timeout register"]
    pub lptr: LPTR,
}
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "device configuration register"]
pub struct DCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device configuration register"]
pub mod dcr;
#[doc = "status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "flag clear register"]
pub struct FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "flag clear register"]
pub mod fcr;
#[doc = "data length register"]
pub struct DLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data length register"]
pub mod dlr;
#[doc = "communication configuration register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "communication configuration register"]
pub mod ccr;
#[doc = "address register"]
pub struct AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "address register"]
pub mod ar;
#[doc = "ABR"]
pub struct ABR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ABR"]
pub mod abr;
#[doc = "data register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register"]
pub mod dr;
#[doc = "polling status mask register"]
pub struct PSMKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "polling status mask register"]
pub mod psmkr;
#[doc = "polling status match register"]
pub struct PSMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "polling status match register"]
pub mod psmar;
#[doc = "polling interval register"]
pub struct PIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "polling interval register"]
pub mod pir;
#[doc = "low-power timeout register"]
pub struct LPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "low-power timeout register"]
pub mod lptr;
