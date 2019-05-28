#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"]
    pub fs_gotgctl: FS_GOTGCTL,
    #[doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"]
    pub fs_gotgint: FS_GOTGINT,
    #[doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
    pub fs_gahbcfg: FS_GAHBCFG,
    #[doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
    pub fs_gusbcfg: FS_GUSBCFG,
    #[doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"]
    pub fs_grstctl: FS_GRSTCTL,
    #[doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
    pub fs_gintsts: FS_GINTSTS,
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    pub fs_gintmsk: FS_GINTMSK,
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    pub fs_grxstsr_device: FS_GRXSTSR_DEVICE,
    _reserved0: [u8; 4usize],
    #[doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
    pub fs_grxfsiz: FS_GRXFSIZ,
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)"]
    pub fs_gnptxfsiz_device: FS_GNPTXFSIZ_DEVICE,
    #[doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
    pub fs_gnptxsts: FS_GNPTXSTS,
    _reserved1: [u8; 8usize],
    #[doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"]
    pub fs_gccfg: FS_GCCFG,
    #[doc = "0x3c - core ID register"]
    pub fs_cid: FS_CID,
    _reserved2: [u8; 192usize],
    #[doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
    pub fs_hptxfsiz: FS_HPTXFSIZ,
    #[doc = "0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
    pub fs_dieptxf1: FS_DIEPTXF1,
    #[doc = "0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
    pub fs_dieptxf2: FS_DIEPTXF2,
    #[doc = "0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
    pub fs_dieptxf3: FS_DIEPTXF3,
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub struct FS_GOTGCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub mod fs_gotgctl;
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub struct FS_GOTGINT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub mod fs_gotgint;
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub struct FS_GAHBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub mod fs_gahbcfg;
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub struct FS_GUSBCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub mod fs_gusbcfg;
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub struct FS_GRSTCTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub mod fs_grstctl;
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub struct FS_GINTSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub mod fs_gintsts;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub struct FS_GINTMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod fs_gintmsk;
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub struct FS_GRXSTSR_DEVICE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub mod fs_grxstsr_device;
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub struct FS_GRXSTSR_HOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub mod fs_grxstsr_host;
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub struct FS_GRXFSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub mod fs_grxfsiz;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)"]
pub struct FS_GNPTXFSIZ_DEVICE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)"]
pub mod fs_gnptxfsiz_device;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)"]
pub struct FS_GNPTXFSIZ_HOST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)"]
pub mod fs_gnptxfsiz_host;
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub struct FS_GNPTXSTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub mod fs_gnptxsts;
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub struct FS_GCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub mod fs_gccfg;
#[doc = "core ID register"]
pub struct FS_CID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "core ID register"]
pub mod fs_cid;
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub struct FS_HPTXFSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub mod fs_hptxfsiz;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub struct FS_DIEPTXF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub mod fs_dieptxf1;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub struct FS_DIEPTXF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub mod fs_dieptxf2;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub struct FS_DIEPTXF3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub mod fs_dieptxf3;
