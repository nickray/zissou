#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub tr: TR,
    #[doc = "0x04 - date register"]
    pub dr: DR,
    #[doc = "0x08 - control register"]
    pub cr: CR,
    #[doc = "0x0c - initialization and status register"]
    pub isr: ISR,
    #[doc = "0x10 - prescaler register"]
    pub prer: PRER,
    #[doc = "0x14 - wakeup timer register"]
    pub wutr: WUTR,
    _reserved0: [u8; 4usize],
    #[doc = "0x1c - alarm A register"]
    pub alrmar: ALRMAR,
    #[doc = "0x20 - alarm B register"]
    pub alrmbr: ALRMBR,
    #[doc = "0x24 - write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - sub second register"]
    pub ssr: SSR,
    #[doc = "0x2c - shift control register"]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - time stamp time register"]
    pub tstr: TSTR,
    #[doc = "0x34 - time stamp date register"]
    pub tsdr: TSDR,
    #[doc = "0x38 - timestamp sub second register"]
    pub tsssr: TSSSR,
    #[doc = "0x3c - calibration register"]
    pub calr: CALR,
    #[doc = "0x40 - tamper configuration register"]
    pub tampcr: TAMPCR,
    #[doc = "0x44 - alarm A sub second register"]
    pub alrmassr: ALRMASSR,
    #[doc = "0x48 - alarm B sub second register"]
    pub alrmbssr: ALRMBSSR,
    #[doc = "0x4c - option register"]
    pub or: OR,
    #[doc = "0x50 - backup register"]
    pub bkpr: [BKPR; 32],
}
#[doc = "time register"]
pub struct TR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "time register"]
pub mod tr;
#[doc = "date register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "date register"]
pub mod dr;
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "initialization and status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization and status register"]
pub mod isr;
#[doc = "prescaler register"]
pub struct PRER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "prescaler register"]
pub mod prer;
#[doc = "wakeup timer register"]
pub struct WUTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "wakeup timer register"]
pub mod wutr;
#[doc = "alarm A register"]
pub struct ALRMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "alarm B register"]
pub struct ALRMBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "alarm B register"]
pub mod alrmbr;
#[doc = "write protection register"]
pub struct WPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "write protection register"]
pub mod wpr;
#[doc = "sub second register"]
pub struct SSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "sub second register"]
pub mod ssr;
#[doc = "shift control register"]
pub struct SHIFTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "shift control register"]
pub mod shiftr;
#[doc = "time stamp time register"]
pub struct TSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "time stamp time register"]
pub mod tstr;
#[doc = "time stamp date register"]
pub struct TSDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "time stamp date register"]
pub mod tsdr;
#[doc = "timestamp sub second register"]
pub struct TSSSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "timestamp sub second register"]
pub mod tsssr;
#[doc = "calibration register"]
pub struct CALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "calibration register"]
pub mod calr;
#[doc = "tamper configuration register"]
pub struct TAMPCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "tamper configuration register"]
pub mod tampcr;
#[doc = "alarm A sub second register"]
pub struct ALRMASSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "alarm B sub second register"]
pub struct ALRMBSSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "alarm B sub second register"]
pub mod alrmbssr;
#[doc = "option register"]
pub struct OR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "option register"]
pub mod or;
#[doc = "backup register"]
pub struct BKPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkpr;
