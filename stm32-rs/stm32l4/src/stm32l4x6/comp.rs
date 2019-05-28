#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator 1 control and status register"]
    pub comp1_csr: COMP1_CSR,
    #[doc = "0x04 - Comparator 2 control and status register"]
    pub comp2_csr: COMP2_CSR,
}
#[doc = "Comparator 1 control and status register"]
pub struct COMP1_CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 1 control and status register"]
pub mod comp1_csr;
#[doc = "Comparator 2 control and status register"]
pub struct COMP2_CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Comparator 2 control and status register"]
pub mod comp2_csr;
