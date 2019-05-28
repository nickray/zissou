#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x08 - interrupt flag clear register"]
    pub ifcr: IFCR,
    #[doc = "0x0c - foreground memory address register"]
    pub fgmar: FGMAR,
    #[doc = "0x10 - foreground offset register"]
    pub fgor: FGOR,
    #[doc = "0x14 - background memory address register"]
    pub bgmar: BGMAR,
    #[doc = "0x18 - background offset register"]
    pub bgor: BGOR,
    #[doc = "0x1c - foreground PFC control register"]
    pub fgpfccr: FGPFCCR,
    #[doc = "0x20 - foreground color register"]
    pub fgcolr: FGCOLR,
    #[doc = "0x24 - background PFC control register"]
    pub bgpfccr: BGPFCCR,
    #[doc = "0x28 - background color register"]
    pub bgcolr: BGCOLR,
    #[doc = "0x2c - foreground CLUT memory address register"]
    pub fgcmar: FGCMAR,
    #[doc = "0x30 - background CLUT memory address register"]
    pub bgcmar: BGCMAR,
    #[doc = "0x34 - output PFC control register"]
    pub opfccr: OPFCCR,
    #[doc = "0x38 - output color register"]
    pub ocolr: OCOLR,
    #[doc = "0x3c - output memory address register"]
    pub omar: OMAR,
    #[doc = "0x40 - output offset register"]
    pub oor: OOR,
    #[doc = "0x44 - number of line register"]
    pub nlr: NLR,
    #[doc = "0x48 - line watermark register"]
    pub lwr: LWR,
    #[doc = "0x4c - AHB master timer configuration register"]
    pub amtcr: AMTCR,
    _reserved0: [u8; 944usize],
    #[doc = "0x400 - FGCLUT"]
    pub fgclut: FGCLUT,
    _reserved1: [u8; 1020usize],
    #[doc = "0x800 - BGCLUT"]
    pub bgclut: BGCLUT,
}
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "interrupt flag clear register"]
pub struct IFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt flag clear register"]
pub mod ifcr;
#[doc = "foreground memory address register"]
pub struct FGMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "foreground memory address register"]
pub mod fgmar;
#[doc = "foreground offset register"]
pub struct FGOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "foreground offset register"]
pub mod fgor;
#[doc = "background memory address register"]
pub struct BGMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "background memory address register"]
pub mod bgmar;
#[doc = "background offset register"]
pub struct BGOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "background offset register"]
pub mod bgor;
#[doc = "foreground PFC control register"]
pub struct FGPFCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "foreground PFC control register"]
pub mod fgpfccr;
#[doc = "foreground color register"]
pub struct FGCOLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "foreground color register"]
pub mod fgcolr;
#[doc = "background PFC control register"]
pub struct BGPFCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "background PFC control register"]
pub mod bgpfccr;
#[doc = "background color register"]
pub struct BGCOLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "background color register"]
pub mod bgcolr;
#[doc = "foreground CLUT memory address register"]
pub struct FGCMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "foreground CLUT memory address register"]
pub mod fgcmar;
#[doc = "background CLUT memory address register"]
pub struct BGCMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "background CLUT memory address register"]
pub mod bgcmar;
#[doc = "output PFC control register"]
pub struct OPFCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "output PFC control register"]
pub mod opfccr;
#[doc = "output color register"]
pub struct OCOLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "output color register"]
pub mod ocolr;
#[doc = "output memory address register"]
pub struct OMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "output memory address register"]
pub mod omar;
#[doc = "output offset register"]
pub struct OOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "output offset register"]
pub mod oor;
#[doc = "number of line register"]
pub struct NLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "number of line register"]
pub mod nlr;
#[doc = "line watermark register"]
pub struct LWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "line watermark register"]
pub mod lwr;
#[doc = "AHB master timer configuration register"]
pub struct AMTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AHB master timer configuration register"]
pub mod amtcr;
#[doc = "FGCLUT"]
pub struct FGCLUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FGCLUT"]
pub mod fgclut;
#[doc = "BGCLUT"]
pub struct BGCLUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BGCLUT"]
pub mod bgclut;
