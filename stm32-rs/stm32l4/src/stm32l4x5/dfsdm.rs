#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch0: CH,
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch1: CH,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch2: CH,
    _reserved2: [u8; 12usize],
    #[doc = "0x60 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch3: CH,
    _reserved3: [u8; 12usize],
    #[doc = "0x80 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch4: CH,
    _reserved4: [u8; 12usize],
    #[doc = "0xa0 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch5: CH,
    _reserved5: [u8; 12usize],
    #[doc = "0xc0 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch6: CH,
    _reserved6: [u8; 12usize],
    #[doc = "0xe0 - DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
    pub ch7: CH,
    _reserved7: [u8; 12usize],
    #[doc = "0x100 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    pub flt0: FLT,
    _reserved8: [u8; 196usize],
    #[doc = "0x200 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    pub flt1: FLT,
    _reserved9: [u8; 196usize],
    #[doc = "0x300 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    pub flt2: FLT,
    _reserved10: [u8; 196usize],
    #[doc = "0x400 - Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
    pub flt3: FLT,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - channel configuration y register"]
    pub cfgr1: self::ch::CFGR1,
    #[doc = "0x04 - channel configuration y register"]
    pub cfgr2: self::ch::CFGR2,
    #[doc = "0x08 - analog watchdog and short-circuit detector register"]
    pub awscdr: self::ch::AWSCDR,
    #[doc = "0x0c - channel watchdog filter data register"]
    pub wdatr: self::ch::WDATR,
    #[doc = "0x10 - channel data input register"]
    pub datinr: self::ch::DATINR,
}
#[doc = r" Register block"]
#[doc = "DFSDM Channel cluster: contains CHCFG?R1, CHCFG?R2, CHAWSCD?R, CHWDAT?R and CHDATIN?R registers"]
pub mod ch;
#[doc = r" Register block"]
#[repr(C)]
pub struct FLT {
    #[doc = "0x00 - control register 1"]
    pub cr2: self::flt::CR2,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - interrupt and status register"]
    pub isr: self::flt::ISR,
    #[doc = "0x0c - interrupt flag clear register"]
    pub icr: self::flt::ICR,
    #[doc = "0x10 - injected channel group selection register"]
    pub jchgr: self::flt::JCHGR,
    #[doc = "0x14 - filter control register"]
    pub fcr: self::flt::FCR,
    #[doc = "0x18 - data register for injected group"]
    pub jdatar: self::flt::JDATAR,
    #[doc = "0x1c - data register for the regular channel"]
    pub rdatar: self::flt::RDATAR,
    #[doc = "0x20 - analog watchdog high threshold register"]
    pub awhtr: self::flt::AWHTR,
    #[doc = "0x24 - analog watchdog low threshold register"]
    pub awltr: self::flt::AWLTR,
    #[doc = "0x28 - analog watchdog status register"]
    pub awsr: self::flt::AWSR,
    #[doc = "0x2c - analog watchdog clear flag register"]
    pub awcfr: self::flt::AWCFR,
    #[doc = "0x30 - Extremes detector maximum register"]
    pub exmax: self::flt::EXMAX,
    #[doc = "0x34 - Extremes detector minimum register"]
    pub exmin: self::flt::EXMIN,
    #[doc = "0x38 - conversion timer register"]
    pub cnvtimr: self::flt::CNVTIMR,
}
#[doc = r" Register block"]
#[doc = "Cluster FLT%s, containing DFSDM?_CR1, DFSDM?_ISR, DFSDM?_ICR, DFSDM?_JCHGR, DFSDM?_FCR, DFSDM?_JDATAR, DFSDM?_RDATAR, DFSDM?_AWHTR, DFSDM?_AWLTR, DFSDM?_AWSR, DFSDM?_AWCFR, DFSDM?_EXMAX, DFSDM?_EXMIN, DFSDM?_CNVTIMR"]
pub mod flt;
#[doc = "control register 2"]
pub struct DFSDM0_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 2"]
pub mod dfsdm0_cr2;
#[doc = "control register 2"]
pub struct DFSDM1_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 2"]
pub mod dfsdm1_cr2;
#[doc = "control register 2"]
pub struct DFSDM2_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 2"]
pub mod dfsdm2_cr2;
#[doc = "control register 2"]
pub struct DFSDM3_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 2"]
pub mod dfsdm3_cr2;
