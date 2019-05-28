#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OPAMP1 control/status register"]
    pub opamp1_csr: OPAMP1_CSR,
    #[doc = "0x04 - OPAMP1 offset trimming register in normal mode"]
    pub opamp1_otr: OPAMP1_OTR,
    #[doc = "0x08 - OPAMP1 offset trimming register in low-power mode"]
    pub opamp1_lpotr: OPAMP1_LPOTR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - OPAMP2 control/status register"]
    pub opamp2_csr: OPAMP2_CSR,
    #[doc = "0x14 - OPAMP2 offset trimming register in normal mode"]
    pub opamp2_otr: OPAMP2_OTR,
    #[doc = "0x18 - OPAMP2 offset trimming register in low-power mode"]
    pub opamp2_lpotr: OPAMP2_LPOTR,
}
#[doc = "OPAMP1 control/status register"]
pub struct OPAMP1_CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_csr;
#[doc = "OPAMP1 offset trimming register in normal mode"]
pub struct OPAMP1_OTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OPAMP1 offset trimming register in normal mode"]
pub mod opamp1_otr;
#[doc = "OPAMP1 offset trimming register in low-power mode"]
pub struct OPAMP1_LPOTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OPAMP1 offset trimming register in low-power mode"]
pub mod opamp1_lpotr;
#[doc = "OPAMP2 control/status register"]
pub struct OPAMP2_CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_csr;
#[doc = "OPAMP2 offset trimming register in normal mode"]
pub struct OPAMP2_OTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OPAMP2 offset trimming register in normal mode"]
pub mod opamp2_otr;
#[doc = "OPAMP2 offset trimming register in low-power mode"]
pub struct OPAMP2_LPOTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OPAMP2 offset trimming register in low-power mode"]
pub mod opamp2_lpotr;
