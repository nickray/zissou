#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS device configuration register (OTG_FS_DCFG)"]
    pub fs_dcfg: FS_DCFG,
    #[doc = "0x04 - OTG_FS device control register (OTG_FS_DCTL)"]
    pub fs_dctl: FS_DCTL,
    #[doc = "0x08 - OTG_FS device status register (OTG_FS_DSTS)"]
    pub fs_dsts: FS_DSTS,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
    pub fs_diepmsk: FS_DIEPMSK,
    #[doc = "0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
    pub fs_doepmsk: FS_DOEPMSK,
    #[doc = "0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
    pub fs_daint: FS_DAINT,
    #[doc = "0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
    pub fs_daintmsk: FS_DAINTMSK,
    _reserved1: [u8; 8usize],
    #[doc = "0x28 - OTG_FS device VBUS discharge time register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0x2c - OTG_FS device VBUS pulsing time register"]
    pub dvbuspulse: DVBUSPULSE,
    _reserved2: [u8; 4usize],
    #[doc = "0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved3: [u8; 200usize],
    #[doc = "0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
    pub fs_diepctl0: FS_DIEPCTL0,
    _reserved4: [u8; 4usize],
    #[doc = "0x108 - device endpoint-x interrupt register"]
    pub diepint0: DIEPINT0,
    _reserved5: [u8; 4usize],
    #[doc = "0x110 - device endpoint-0 transfer size register"]
    pub dieptsiz0: DIEPTSIZ0,
    _reserved6: [u8; 4usize],
    #[doc = "0x118 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts0: DTXFSTS0,
    _reserved7: [u8; 4usize],
    #[doc = "0x120 - OTG device endpoint-1 control register"]
    pub diepctl1: DIEPCTL1,
    _reserved8: [u8; 4usize],
    #[doc = "0x128 - device endpoint-1 interrupt register"]
    pub diepint1: DIEPINT1,
    _reserved9: [u8; 4usize],
    #[doc = "0x130 - device endpoint-1 transfer size register"]
    pub dieptsiz1: DIEPTSIZ1,
    _reserved10: [u8; 4usize],
    #[doc = "0x138 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts1: DTXFSTS1,
    _reserved11: [u8; 4usize],
    #[doc = "0x140 - OTG device endpoint-2 control register"]
    pub diepctl2: DIEPCTL2,
    _reserved12: [u8; 4usize],
    #[doc = "0x148 - device endpoint-2 interrupt register"]
    pub diepint2: DIEPINT2,
    _reserved13: [u8; 4usize],
    #[doc = "0x150 - device endpoint-2 transfer size register"]
    pub dieptsiz2: DIEPTSIZ2,
    _reserved14: [u8; 4usize],
    #[doc = "0x158 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts2: DTXFSTS2,
    _reserved15: [u8; 4usize],
    #[doc = "0x160 - OTG device endpoint-3 control register"]
    pub diepctl3: DIEPCTL3,
    _reserved16: [u8; 4usize],
    #[doc = "0x168 - device endpoint-3 interrupt register"]
    pub diepint3: DIEPINT3,
    _reserved17: [u8; 4usize],
    #[doc = "0x170 - device endpoint-3 transfer size register"]
    pub dieptsiz3: DIEPTSIZ3,
    _reserved18: [u8; 4usize],
    #[doc = "0x178 - OTG_FS device IN endpoint transmit FIFO status register"]
    pub dtxfsts3: DTXFSTS3,
    _reserved19: [u8; 388usize],
    #[doc = "0x300 - device endpoint-0 control register"]
    pub doepctl0: DOEPCTL0,
    _reserved20: [u8; 4usize],
    #[doc = "0x308 - device endpoint-0 interrupt register"]
    pub doepint0: DOEPINT0,
    _reserved21: [u8; 4usize],
    #[doc = "0x310 - device OUT endpoint-0 transfer size register"]
    pub doeptsiz0: DOEPTSIZ0,
    _reserved22: [u8; 12usize],
    #[doc = "0x320 - device endpoint-1 control register"]
    pub doepctl1: DOEPCTL1,
    _reserved23: [u8; 4usize],
    #[doc = "0x328 - device endpoint-1 interrupt register"]
    pub doepint1: DOEPINT1,
    _reserved24: [u8; 4usize],
    #[doc = "0x330 - device OUT endpoint-1 transfer size register"]
    pub doeptsiz1: DOEPTSIZ1,
    _reserved25: [u8; 12usize],
    #[doc = "0x340 - device endpoint-2 control register"]
    pub doepctl2: DOEPCTL2,
    _reserved26: [u8; 4usize],
    #[doc = "0x348 - device endpoint-2 interrupt register"]
    pub doepint2: DOEPINT2,
    _reserved27: [u8; 4usize],
    #[doc = "0x350 - device OUT endpoint-2 transfer size register"]
    pub doeptsiz2: DOEPTSIZ2,
    _reserved28: [u8; 12usize],
    #[doc = "0x360 - device endpoint-3 control register"]
    pub doepctl3: DOEPCTL3,
    _reserved29: [u8; 4usize],
    #[doc = "0x368 - device endpoint-3 interrupt register"]
    pub doepint3: DOEPINT3,
    _reserved30: [u8; 4usize],
    #[doc = "0x370 - device OUT endpoint-3 transfer size register"]
    pub doeptsiz3: DOEPTSIZ3,
}
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub struct FS_DCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)"]
pub mod fs_dcfg;
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub struct FS_DCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device control register (OTG_FS_DCTL)"]
pub mod fs_dctl;
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub struct FS_DSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device status register (OTG_FS_DSTS)"]
pub mod fs_dsts;
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub struct FS_DIEPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"]
pub mod fs_diepmsk;
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub struct FS_DOEPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"]
pub mod fs_doepmsk;
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub struct FS_DAINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"]
pub mod fs_daint;
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub struct FS_DAINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"]
pub mod fs_daintmsk;
#[doc = "OTG_FS device VBUS discharge time register"]
pub struct DVBUSDIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device VBUS discharge time register"]
pub mod dvbusdis;
#[doc = "OTG_FS device VBUS pulsing time register"]
pub struct DVBUSPULSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device VBUS pulsing time register"]
pub mod dvbuspulse;
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub struct DIEPEMPMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"]
pub mod diepempmsk;
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub struct FS_DIEPCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)"]
pub mod fs_diepctl0;
#[doc = "OTG device endpoint-1 control register"]
pub struct DIEPCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-1 control register"]
pub mod diepctl1;
#[doc = "OTG device endpoint-2 control register"]
pub struct DIEPCTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-2 control register"]
pub mod diepctl2;
#[doc = "OTG device endpoint-3 control register"]
pub struct DIEPCTL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG device endpoint-3 control register"]
pub mod diepctl3;
#[doc = "device endpoint-0 control register"]
pub struct DOEPCTL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-0 control register"]
pub mod doepctl0;
#[doc = "device endpoint-1 control register"]
pub struct DOEPCTL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-1 control register"]
pub mod doepctl1;
#[doc = "device endpoint-2 control register"]
pub struct DOEPCTL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-2 control register"]
pub mod doepctl2;
#[doc = "device endpoint-3 control register"]
pub struct DOEPCTL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-3 control register"]
pub mod doepctl3;
#[doc = "device endpoint-x interrupt register"]
pub struct DIEPINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-x interrupt register"]
pub mod diepint0;
#[doc = "device endpoint-1 interrupt register"]
pub struct DIEPINT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-1 interrupt register"]
pub mod diepint1;
#[doc = "device endpoint-2 interrupt register"]
pub struct DIEPINT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-2 interrupt register"]
pub mod diepint2;
#[doc = "device endpoint-3 interrupt register"]
pub struct DIEPINT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-3 interrupt register"]
pub mod diepint3;
#[doc = "device endpoint-0 interrupt register"]
pub struct DOEPINT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-0 interrupt register"]
pub mod doepint0;
#[doc = "device endpoint-1 interrupt register"]
pub struct DOEPINT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-1 interrupt register"]
pub mod doepint1;
#[doc = "device endpoint-2 interrupt register"]
pub struct DOEPINT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-2 interrupt register"]
pub mod doepint2;
#[doc = "device endpoint-3 interrupt register"]
pub struct DOEPINT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-3 interrupt register"]
pub mod doepint3;
#[doc = "device endpoint-0 transfer size register"]
pub struct DIEPTSIZ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-0 transfer size register"]
pub mod dieptsiz0;
#[doc = "device OUT endpoint-0 transfer size register"]
pub struct DOEPTSIZ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-0 transfer size register"]
pub mod doeptsiz0;
#[doc = "device endpoint-1 transfer size register"]
pub struct DIEPTSIZ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-1 transfer size register"]
pub mod dieptsiz1;
#[doc = "device endpoint-2 transfer size register"]
pub struct DIEPTSIZ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-2 transfer size register"]
pub mod dieptsiz2;
#[doc = "device endpoint-3 transfer size register"]
pub struct DIEPTSIZ3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device endpoint-3 transfer size register"]
pub mod dieptsiz3;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct DTXFSTS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts0;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct DTXFSTS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts1;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct DTXFSTS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts2;
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub struct DTXFSTS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register"]
pub mod dtxfsts3;
#[doc = "device OUT endpoint-1 transfer size register"]
pub struct DOEPTSIZ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-1 transfer size register"]
pub mod doeptsiz1;
#[doc = "device OUT endpoint-2 transfer size register"]
pub struct DOEPTSIZ2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-2 transfer size register"]
pub mod doeptsiz2;
#[doc = "device OUT endpoint-3 transfer size register"]
pub struct DOEPTSIZ3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device OUT endpoint-3 transfer size register"]
pub mod doeptsiz3;
