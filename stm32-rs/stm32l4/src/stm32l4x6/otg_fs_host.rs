#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS host configuration register (OTG_FS_HCFG)"]
    pub fs_hcfg: FS_HCFG,
    #[doc = "0x04 - OTG_FS Host frame interval register"]
    pub hfir: HFIR,
    #[doc = "0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
    pub fs_hfnum: FS_HFNUM,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
    pub fs_hptxsts: FS_HPTXSTS,
    #[doc = "0x14 - OTG_FS Host all channels interrupt register"]
    pub haint: HAINT,
    #[doc = "0x18 - OTG_FS host all channels interrupt mask register"]
    pub haintmsk: HAINTMSK,
    _reserved1: [u8; 36usize],
    #[doc = "0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)"]
    pub fs_hprt: FS_HPRT,
    _reserved2: [u8; 188usize],
    #[doc = "0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
    pub fs_hcchar0: FS_HCCHAR0,
    _reserved3: [u8; 4usize],
    #[doc = "0x108 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
    pub fs_hcint0: FS_HCINT0,
    #[doc = "0x10c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
    pub fs_hcintmsk0: FS_HCINTMSK0,
    #[doc = "0x110 - OTG_FS host channel-0 transfer size register"]
    pub fs_hctsiz0: FS_HCTSIZ0,
    _reserved4: [u8; 12usize],
    #[doc = "0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
    pub fs_hcchar1: FS_HCCHAR1,
    _reserved5: [u8; 4usize],
    #[doc = "0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
    pub fs_hcint1: FS_HCINT1,
    #[doc = "0x12c - OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
    pub fs_hcintmsk1: FS_HCINTMSK1,
    #[doc = "0x130 - OTG_FS host channel-1 transfer size register"]
    pub fs_hctsiz1: FS_HCTSIZ1,
    _reserved6: [u8; 12usize],
    #[doc = "0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
    pub fs_hcchar2: FS_HCCHAR2,
    _reserved7: [u8; 4usize],
    #[doc = "0x148 - OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
    pub fs_hcint2: FS_HCINT2,
    #[doc = "0x14c - OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
    pub fs_hcintmsk2: FS_HCINTMSK2,
    #[doc = "0x150 - OTG_FS host channel-2 transfer size register"]
    pub fs_hctsiz2: FS_HCTSIZ2,
    _reserved8: [u8; 12usize],
    #[doc = "0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
    pub fs_hcchar3: FS_HCCHAR3,
    _reserved9: [u8; 4usize],
    #[doc = "0x168 - OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
    pub fs_hcint3: FS_HCINT3,
    #[doc = "0x16c - OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
    pub fs_hcintmsk3: FS_HCINTMSK3,
    #[doc = "0x170 - OTG_FS host channel-3 transfer size register"]
    pub fs_hctsiz3: FS_HCTSIZ3,
    _reserved10: [u8; 12usize],
    #[doc = "0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
    pub fs_hcchar4: FS_HCCHAR4,
    _reserved11: [u8; 4usize],
    #[doc = "0x188 - OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
    pub fs_hcint4: FS_HCINT4,
    #[doc = "0x18c - OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
    pub fs_hcintmsk4: FS_HCINTMSK4,
    #[doc = "0x190 - OTG_FS host channel-x transfer size register"]
    pub fs_hctsiz4: FS_HCTSIZ4,
    _reserved12: [u8; 12usize],
    #[doc = "0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
    pub fs_hcchar5: FS_HCCHAR5,
    _reserved13: [u8; 4usize],
    #[doc = "0x1a8 - OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
    pub fs_hcint5: FS_HCINT5,
    #[doc = "0x1ac - OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
    pub fs_hcintmsk5: FS_HCINTMSK5,
    #[doc = "0x1b0 - OTG_FS host channel-5 transfer size register"]
    pub fs_hctsiz5: FS_HCTSIZ5,
    _reserved14: [u8; 12usize],
    #[doc = "0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
    pub fs_hcchar6: FS_HCCHAR6,
    _reserved15: [u8; 4usize],
    #[doc = "0x1c8 - OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
    pub fs_hcint6: FS_HCINT6,
    #[doc = "0x1cc - OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
    pub fs_hcintmsk6: FS_HCINTMSK6,
    #[doc = "0x1d0 - OTG_FS host channel-6 transfer size register"]
    pub fs_hctsiz6: FS_HCTSIZ6,
    _reserved16: [u8; 12usize],
    #[doc = "0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
    pub fs_hcchar7: FS_HCCHAR7,
    _reserved17: [u8; 4usize],
    #[doc = "0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
    pub fs_hcint7: FS_HCINT7,
    #[doc = "0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
    pub fs_hcintmsk7: FS_HCINTMSK7,
    #[doc = "0x1f0 - OTG_FS host channel-7 transfer size register"]
    pub fs_hctsiz7: FS_HCTSIZ7,
}
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)"]
pub struct FS_HCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)"]
pub mod fs_hcfg;
#[doc = "OTG_FS Host frame interval register"]
pub struct HFIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Host frame interval register"]
pub mod hfir;
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
pub struct FS_HFNUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
pub mod fs_hfnum;
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
pub struct FS_HPTXSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
pub mod fs_hptxsts;
#[doc = "OTG_FS Host all channels interrupt register"]
pub struct HAINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Host all channels interrupt register"]
pub mod haint;
#[doc = "OTG_FS host all channels interrupt mask register"]
pub struct HAINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "OTG_FS host port control and status register (OTG_FS_HPRT)"]
pub struct FS_HPRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host port control and status register (OTG_FS_HPRT)"]
pub mod fs_hprt;
#[doc = "OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
pub struct FS_HCCHAR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
pub mod fs_hcchar0;
#[doc = "OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
pub struct FS_HCCHAR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
pub mod fs_hcchar1;
#[doc = "OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
pub struct FS_HCCHAR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
pub mod fs_hcchar2;
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
pub struct FS_HCCHAR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
pub mod fs_hcchar3;
#[doc = "OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
pub struct FS_HCCHAR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
pub mod fs_hcchar4;
#[doc = "OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
pub struct FS_HCCHAR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
pub mod fs_hcchar5;
#[doc = "OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
pub struct FS_HCCHAR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
pub mod fs_hcchar6;
#[doc = "OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
pub struct FS_HCCHAR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
pub mod fs_hcchar7;
#[doc = "OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
pub struct FS_HCINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
pub mod fs_hcint0;
#[doc = "OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
pub struct FS_HCINT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
pub mod fs_hcint1;
#[doc = "OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
pub struct FS_HCINT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
pub mod fs_hcint2;
#[doc = "OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
pub struct FS_HCINT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
pub mod fs_hcint3;
#[doc = "OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
pub struct FS_HCINT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
pub mod fs_hcint4;
#[doc = "OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
pub struct FS_HCINT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
pub mod fs_hcint5;
#[doc = "OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
pub struct FS_HCINT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
pub mod fs_hcint6;
#[doc = "OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
pub struct FS_HCINT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
pub mod fs_hcint7;
#[doc = "OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
pub struct FS_HCINTMSK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
pub mod fs_hcintmsk0;
#[doc = "OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
pub struct FS_HCINTMSK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
pub mod fs_hcintmsk1;
#[doc = "OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
pub struct FS_HCINTMSK2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
pub mod fs_hcintmsk2;
#[doc = "OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
pub struct FS_HCINTMSK3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
pub mod fs_hcintmsk3;
#[doc = "OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
pub struct FS_HCINTMSK4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
pub mod fs_hcintmsk4;
#[doc = "OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
pub struct FS_HCINTMSK5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
pub mod fs_hcintmsk5;
#[doc = "OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
pub struct FS_HCINTMSK6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
pub mod fs_hcintmsk6;
#[doc = "OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
pub struct FS_HCINTMSK7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
pub mod fs_hcintmsk7;
#[doc = "OTG_FS host channel-0 transfer size register"]
pub struct FS_HCTSIZ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-0 transfer size register"]
pub mod fs_hctsiz0;
#[doc = "OTG_FS host channel-1 transfer size register"]
pub struct FS_HCTSIZ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-1 transfer size register"]
pub mod fs_hctsiz1;
#[doc = "OTG_FS host channel-2 transfer size register"]
pub struct FS_HCTSIZ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-2 transfer size register"]
pub mod fs_hctsiz2;
#[doc = "OTG_FS host channel-3 transfer size register"]
pub struct FS_HCTSIZ3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-3 transfer size register"]
pub mod fs_hctsiz3;
#[doc = "OTG_FS host channel-x transfer size register"]
pub struct FS_HCTSIZ4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-x transfer size register"]
pub mod fs_hctsiz4;
#[doc = "OTG_FS host channel-5 transfer size register"]
pub struct FS_HCTSIZ5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-5 transfer size register"]
pub mod fs_hctsiz5;
#[doc = "OTG_FS host channel-6 transfer size register"]
pub struct FS_HCTSIZ6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-6 transfer size register"]
pub mod fs_hctsiz6;
#[doc = "OTG_FS host channel-7 transfer size register"]
pub struct FS_HCTSIZ7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS host channel-7 transfer size register"]
pub mod fs_hctsiz7;
