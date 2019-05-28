#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - status register"]
    pub sr: SR,
    #[doc = "0x08 - data input register"]
    pub dinr: DINR,
    #[doc = "0x0c - data output register"]
    pub doutr: DOUTR,
    #[doc = "0x10 - key register 0"]
    pub keyr0: KEYR0,
    #[doc = "0x14 - key register 1"]
    pub keyr1: KEYR1,
    #[doc = "0x18 - key register 2"]
    pub keyr2: KEYR2,
    #[doc = "0x1c - key register 3"]
    pub keyr3: KEYR3,
    #[doc = "0x20 - initialization vector register 0"]
    pub ivr0: IVR0,
    #[doc = "0x24 - initialization vector register 1"]
    pub ivr1: IVR1,
    #[doc = "0x28 - initialization vector register 2"]
    pub ivr2: IVR2,
    #[doc = "0x2c - initialization vector register 3"]
    pub ivr3: IVR3,
}
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "data input register"]
pub struct DINR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data input register"]
pub mod dinr;
#[doc = "data output register"]
pub struct DOUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data output register"]
pub mod doutr;
#[doc = "key register 0"]
pub struct KEYR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 0"]
pub mod keyr0;
#[doc = "key register 1"]
pub struct KEYR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 1"]
pub mod keyr1;
#[doc = "key register 2"]
pub struct KEYR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 2"]
pub mod keyr2;
#[doc = "key register 3"]
pub struct KEYR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "key register 3"]
pub mod keyr3;
#[doc = "initialization vector register 0"]
pub struct IVR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register 0"]
pub mod ivr0;
#[doc = "initialization vector register 1"]
pub struct IVR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register 1"]
pub mod ivr1;
#[doc = "initialization vector register 2"]
pub struct IVR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register 2"]
pub mod ivr2;
#[doc = "initialization vector register 3"]
pub struct IVR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization vector register 3"]
pub mod ivr3;
