#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - channel configuration y register"]
    pub chcfg0r1: CHCFG0R1,
    #[doc = "0x04 - channel configuration y register"]
    pub chcfg0r2: CHCFG0R2,
    #[doc = "0x08 - analog watchdog and short-circuit detector register"]
    pub awscd0r: AWSCD0R,
    #[doc = "0x0c - channel watchdog filter data register"]
    pub chwdat0r: CHWDAT0R,
    #[doc = "0x10 - channel data input register"]
    pub chdatin0r: CHDATIN0R,
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - CHCFG1R1"]
    pub chcfg1r1: CHCFG1R1,
    #[doc = "0x24 - CHCFG1R2"]
    pub chcfg1r2: CHCFG1R2,
    #[doc = "0x28 - AWSCD1R"]
    pub awscd1r: AWSCD1R,
    #[doc = "0x2c - CHWDAT1R"]
    pub chwdat1r: CHWDAT1R,
    #[doc = "0x30 - CHDATIN1R"]
    pub chdatin1r: CHDATIN1R,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - CHCFG2R1"]
    pub chcfg2r1: CHCFG2R1,
    #[doc = "0x44 - CHCFG2R2"]
    pub chcfg2r2: CHCFG2R2,
    #[doc = "0x48 - AWSCD2R"]
    pub awscd2r: AWSCD2R,
    #[doc = "0x4c - CHWDAT2R"]
    pub chwdat2r: CHWDAT2R,
    #[doc = "0x50 - CHDATIN2R"]
    pub chdatin2r: CHDATIN2R,
    _reserved2: [u8; 12usize],
    #[doc = "0x60 - CHCFG3R1"]
    pub chcfg3r1: CHCFG3R1,
    #[doc = "0x64 - CHCFG3R2"]
    pub chcfg3r2: CHCFG3R2,
    #[doc = "0x68 - AWSCD3R"]
    pub awscd3r: AWSCD3R,
    #[doc = "0x6c - CHWDAT3R"]
    pub chwdat3r: CHWDAT3R,
    #[doc = "0x70 - CHDATIN3R"]
    pub chdatin3r: CHDATIN3R,
    _reserved3: [u8; 12usize],
    #[doc = "0x80 - CHCFG4R1"]
    pub chcfg4r1: CHCFG4R1,
    #[doc = "0x84 - CHCFG4R2"]
    pub chcfg4r2: CHCFG4R2,
    #[doc = "0x88 - AWSCD4R"]
    pub awscd4r: AWSCD4R,
    #[doc = "0x8c - CHWDAT4R"]
    pub chwdat4r: CHWDAT4R,
    #[doc = "0x90 - CHDATIN4R"]
    pub chdatin4r: CHDATIN4R,
    _reserved4: [u8; 12usize],
    #[doc = "0xa0 - CHCFG5R1"]
    pub chcfg5r1: CHCFG5R1,
    #[doc = "0xa4 - CHCFG5R2"]
    pub chcfg5r2: CHCFG5R2,
    #[doc = "0xa8 - AWSCD5R"]
    pub awscd5r: AWSCD5R,
    #[doc = "0xac - CHWDAT5R"]
    pub chwdat5r: CHWDAT5R,
    #[doc = "0xb0 - CHDATIN5R"]
    pub chdatin5r: CHDATIN5R,
    _reserved5: [u8; 12usize],
    #[doc = "0xc0 - CHCFG6R1"]
    pub chcfg6r1: CHCFG6R1,
    #[doc = "0xc4 - CHCFG6R2"]
    pub chcfg6r2: CHCFG6R2,
    #[doc = "0xc8 - AWSCD6R"]
    pub awscd6r: AWSCD6R,
    #[doc = "0xcc - CHWDAT6R"]
    pub chwdat6r: CHWDAT6R,
    #[doc = "0xd0 - CHDATIN6R"]
    pub chdatin6r: CHDATIN6R,
    _reserved6: [u8; 12usize],
    #[doc = "0xe0 - CHCFG7R1"]
    pub chcfg7r1: CHCFG7R1,
    #[doc = "0xe4 - CHCFG7R2"]
    pub chcfg7r2: CHCFG7R2,
    #[doc = "0xe8 - AWSCD7R"]
    pub awscd7r: AWSCD7R,
    #[doc = "0xec - CHWDAT7R"]
    pub chwdat7r: CHWDAT7R,
    #[doc = "0xf0 - CHDATIN7R"]
    pub chdatin7r: CHDATIN7R,
    _reserved7: [u8; 12usize],
    #[doc = "0x100 - control register 1"]
    pub dfsdm0_cr1: DFSDM0_CR1,
    #[doc = "0x104 - control register 2"]
    pub dfsdm0_cr2: DFSDM0_CR2,
    #[doc = "0x108 - interrupt and status register"]
    pub dfsdm0_isr: DFSDM0_ISR,
    #[doc = "0x10c - interrupt flag clear register"]
    pub dfsdm0_icr: DFSDM0_ICR,
    #[doc = "0x110 - injected channel group selection register"]
    pub dfsdm0_jchgr: DFSDM0_JCHGR,
    #[doc = "0x114 - filter control register"]
    pub dfsdm0_fcr: DFSDM0_FCR,
    #[doc = "0x118 - data register for injected group"]
    pub dfsdm0_jdatar: DFSDM0_JDATAR,
    #[doc = "0x11c - data register for the regular channel"]
    pub dfsdm0_rdatar: DFSDM0_RDATAR,
    #[doc = "0x120 - analog watchdog high threshold register"]
    pub dfsdm0_awhtr: DFSDM0_AWHTR,
    #[doc = "0x124 - analog watchdog low threshold register"]
    pub dfsdm0_awltr: DFSDM0_AWLTR,
    #[doc = "0x128 - analog watchdog status register"]
    pub dfsdm0_awsr: DFSDM0_AWSR,
    #[doc = "0x12c - analog watchdog clear flag register"]
    pub dfsdm0_awcfr: DFSDM0_AWCFR,
    #[doc = "0x130 - Extremes detector maximum register"]
    pub dfsdm0_exmax: DFSDM0_EXMAX,
    #[doc = "0x134 - Extremes detector minimum register"]
    pub dfsdm0_exmin: DFSDM0_EXMIN,
    #[doc = "0x138 - conversion timer register"]
    pub dfsdm0_cnvtimr: DFSDM0_CNVTIMR,
    _reserved8: [u8; 196usize],
    #[doc = "0x200 - control register 1"]
    pub dfsdm1_cr1: DFSDM1_CR1,
    #[doc = "0x204 - control register 2"]
    pub dfsdm1_cr2: DFSDM1_CR2,
    #[doc = "0x208 - interrupt and status register"]
    pub dfsdm1_isr: DFSDM1_ISR,
    #[doc = "0x20c - interrupt flag clear register"]
    pub dfsdm1_icr: DFSDM1_ICR,
    #[doc = "0x210 - injected channel group selection register"]
    pub dfsdm1_jchgr: DFSDM1_JCHGR,
    #[doc = "0x214 - filter control register"]
    pub dfsdm1_fcr: DFSDM1_FCR,
    #[doc = "0x218 - data register for injected group"]
    pub dfsdm1_jdatar: DFSDM1_JDATAR,
    #[doc = "0x21c - data register for the regular channel"]
    pub dfsdm1_rdatar: DFSDM1_RDATAR,
    #[doc = "0x220 - analog watchdog high threshold register"]
    pub dfsdm1_awhtr: DFSDM1_AWHTR,
    #[doc = "0x224 - analog watchdog low threshold register"]
    pub dfsdm1_awltr: DFSDM1_AWLTR,
    #[doc = "0x228 - analog watchdog status register"]
    pub dfsdm1_awsr: DFSDM1_AWSR,
    #[doc = "0x22c - analog watchdog clear flag register"]
    pub dfsdm1_awcfr: DFSDM1_AWCFR,
    #[doc = "0x230 - Extremes detector maximum register"]
    pub dfsdm1_exmax: DFSDM1_EXMAX,
    #[doc = "0x234 - Extremes detector minimum register"]
    pub dfsdm1_exmin: DFSDM1_EXMIN,
    #[doc = "0x238 - conversion timer register"]
    pub dfsdm1_cnvtimr: DFSDM1_CNVTIMR,
    _reserved9: [u8; 196usize],
    #[doc = "0x300 - control register 1"]
    pub dfsdm2_cr1: DFSDM2_CR1,
    #[doc = "0x304 - control register 2"]
    pub dfsdm2_cr2: DFSDM2_CR2,
    #[doc = "0x308 - interrupt and status register"]
    pub dfsdm2_isr: DFSDM2_ISR,
    #[doc = "0x30c - interrupt flag clear register"]
    pub dfsdm2_icr: DFSDM2_ICR,
    #[doc = "0x310 - injected channel group selection register"]
    pub dfsdm2_jchgr: DFSDM2_JCHGR,
    #[doc = "0x314 - filter control register"]
    pub dfsdm2_fcr: DFSDM2_FCR,
    #[doc = "0x318 - data register for injected group"]
    pub dfsdm2_jdatar: DFSDM2_JDATAR,
    #[doc = "0x31c - data register for the regular channel"]
    pub dfsdm2_rdatar: DFSDM2_RDATAR,
    #[doc = "0x320 - analog watchdog high threshold register"]
    pub dfsdm2_awhtr: DFSDM2_AWHTR,
    #[doc = "0x324 - analog watchdog low threshold register"]
    pub dfsdm2_awltr: DFSDM2_AWLTR,
    #[doc = "0x328 - analog watchdog status register"]
    pub dfsdm2_awsr: DFSDM2_AWSR,
    #[doc = "0x32c - analog watchdog clear flag register"]
    pub dfsdm2_awcfr: DFSDM2_AWCFR,
    #[doc = "0x330 - Extremes detector maximum register"]
    pub dfsdm2_exmax: DFSDM2_EXMAX,
    #[doc = "0x334 - Extremes detector minimum register"]
    pub dfsdm2_exmin: DFSDM2_EXMIN,
    #[doc = "0x338 - conversion timer register"]
    pub dfsdm2_cnvtimr: DFSDM2_CNVTIMR,
    _reserved10: [u8; 196usize],
    #[doc = "0x400 - control register 1"]
    pub dfsdm3_cr1: DFSDM3_CR1,
    #[doc = "0x404 - control register 2"]
    pub dfsdm3_cr2: DFSDM3_CR2,
    #[doc = "0x408 - interrupt and status register"]
    pub dfsdm3_isr: DFSDM3_ISR,
    #[doc = "0x40c - interrupt flag clear register"]
    pub dfsdm3_icr: DFSDM3_ICR,
    #[doc = "0x410 - injected channel group selection register"]
    pub dfsdm3_jchgr: DFSDM3_JCHGR,
    #[doc = "0x414 - filter control register"]
    pub dfsdm3_fcr: DFSDM3_FCR,
    #[doc = "0x418 - data register for injected group"]
    pub dfsdm3_jdatar: DFSDM3_JDATAR,
    #[doc = "0x41c - data register for the regular channel"]
    pub dfsdm3_rdatar: DFSDM3_RDATAR,
    #[doc = "0x420 - analog watchdog high threshold register"]
    pub dfsdm3_awhtr: DFSDM3_AWHTR,
    #[doc = "0x424 - analog watchdog low threshold register"]
    pub dfsdm3_awltr: DFSDM3_AWLTR,
    #[doc = "0x428 - analog watchdog status register"]
    pub dfsdm3_awsr: DFSDM3_AWSR,
    #[doc = "0x42c - analog watchdog clear flag register"]
    pub dfsdm3_awcfr: DFSDM3_AWCFR,
    #[doc = "0x430 - Extremes detector maximum register"]
    pub dfsdm3_exmax: DFSDM3_EXMAX,
    #[doc = "0x434 - Extremes detector minimum register"]
    pub dfsdm3_exmin: DFSDM3_EXMIN,
    #[doc = "0x438 - conversion timer register"]
    pub dfsdm3_cnvtimr: DFSDM3_CNVTIMR,
}
#[doc = "channel configuration y register"]
pub struct CHCFG0R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel configuration y register"]
pub mod chcfg0r1;
#[doc = "channel configuration y register"]
pub struct CHCFG0R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel configuration y register"]
pub mod chcfg0r2;
#[doc = "analog watchdog and short-circuit detector register"]
pub struct AWSCD0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog and short-circuit detector register"]
pub mod awscd0r;
#[doc = "channel watchdog filter data register"]
pub struct CHWDAT0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel watchdog filter data register"]
pub mod chwdat0r;
#[doc = "channel data input register"]
pub struct CHDATIN0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel data input register"]
pub mod chdatin0r;
#[doc = "CHCFG1R1"]
pub struct CHCFG1R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG1R1"]
pub mod chcfg1r1;
#[doc = "CHCFG1R2"]
pub struct CHCFG1R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG1R2"]
pub mod chcfg1r2;
#[doc = "AWSCD1R"]
pub struct AWSCD1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AWSCD1R"]
pub mod awscd1r;
#[doc = "CHWDAT1R"]
pub struct CHWDAT1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHWDAT1R"]
pub mod chwdat1r;
#[doc = "CHDATIN1R"]
pub struct CHDATIN1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHDATIN1R"]
pub mod chdatin1r;
#[doc = "CHCFG2R1"]
pub struct CHCFG2R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG2R1"]
pub mod chcfg2r1;
#[doc = "CHCFG2R2"]
pub struct CHCFG2R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG2R2"]
pub mod chcfg2r2;
#[doc = "AWSCD2R"]
pub struct AWSCD2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AWSCD2R"]
pub mod awscd2r;
#[doc = "CHWDAT2R"]
pub struct CHWDAT2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHWDAT2R"]
pub mod chwdat2r;
#[doc = "CHDATIN2R"]
pub struct CHDATIN2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHDATIN2R"]
pub mod chdatin2r;
#[doc = "CHCFG3R1"]
pub struct CHCFG3R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG3R1"]
pub mod chcfg3r1;
#[doc = "CHCFG3R2"]
pub struct CHCFG3R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG3R2"]
pub mod chcfg3r2;
#[doc = "AWSCD3R"]
pub struct AWSCD3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AWSCD3R"]
pub mod awscd3r;
#[doc = "CHWDAT3R"]
pub struct CHWDAT3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHWDAT3R"]
pub mod chwdat3r;
#[doc = "CHDATIN3R"]
pub struct CHDATIN3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHDATIN3R"]
pub mod chdatin3r;
#[doc = "CHCFG4R1"]
pub struct CHCFG4R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG4R1"]
pub mod chcfg4r1;
#[doc = "CHCFG4R2"]
pub struct CHCFG4R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG4R2"]
pub mod chcfg4r2;
#[doc = "AWSCD4R"]
pub struct AWSCD4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AWSCD4R"]
pub mod awscd4r;
#[doc = "CHWDAT4R"]
pub struct CHWDAT4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHWDAT4R"]
pub mod chwdat4r;
#[doc = "CHDATIN4R"]
pub struct CHDATIN4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHDATIN4R"]
pub mod chdatin4r;
#[doc = "CHCFG5R1"]
pub struct CHCFG5R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG5R1"]
pub mod chcfg5r1;
#[doc = "CHCFG5R2"]
pub struct CHCFG5R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG5R2"]
pub mod chcfg5r2;
#[doc = "AWSCD5R"]
pub struct AWSCD5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AWSCD5R"]
pub mod awscd5r;
#[doc = "CHWDAT5R"]
pub struct CHWDAT5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHWDAT5R"]
pub mod chwdat5r;
#[doc = "CHDATIN5R"]
pub struct CHDATIN5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHDATIN5R"]
pub mod chdatin5r;
#[doc = "CHCFG6R1"]
pub struct CHCFG6R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG6R1"]
pub mod chcfg6r1;
#[doc = "CHCFG6R2"]
pub struct CHCFG6R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG6R2"]
pub mod chcfg6r2;
#[doc = "AWSCD6R"]
pub struct AWSCD6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AWSCD6R"]
pub mod awscd6r;
#[doc = "CHWDAT6R"]
pub struct CHWDAT6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHWDAT6R"]
pub mod chwdat6r;
#[doc = "CHDATIN6R"]
pub struct CHDATIN6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHDATIN6R"]
pub mod chdatin6r;
#[doc = "CHCFG7R1"]
pub struct CHCFG7R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG7R1"]
pub mod chcfg7r1;
#[doc = "CHCFG7R2"]
pub struct CHCFG7R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHCFG7R2"]
pub mod chcfg7r2;
#[doc = "AWSCD7R"]
pub struct AWSCD7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AWSCD7R"]
pub mod awscd7r;
#[doc = "CHWDAT7R"]
pub struct CHWDAT7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHWDAT7R"]
pub mod chwdat7r;
#[doc = "CHDATIN7R"]
pub struct CHDATIN7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHDATIN7R"]
pub mod chdatin7r;
#[doc = "control register 1"]
pub struct DFSDM0_CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod dfsdm0_cr1;
#[doc = "control register 2"]
pub struct DFSDM0_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 2"]
pub mod dfsdm0_cr2;
#[doc = "interrupt and status register"]
pub struct DFSDM0_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt and status register"]
pub mod dfsdm0_isr;
#[doc = "interrupt flag clear register"]
pub struct DFSDM0_ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt flag clear register"]
pub mod dfsdm0_icr;
#[doc = "injected channel group selection register"]
pub struct DFSDM0_JCHGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected channel group selection register"]
pub mod dfsdm0_jchgr;
#[doc = "filter control register"]
pub struct DFSDM0_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter control register"]
pub mod dfsdm0_fcr;
#[doc = "data register for injected group"]
pub struct DFSDM0_JDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register for injected group"]
pub mod dfsdm0_jdatar;
#[doc = "data register for the regular channel"]
pub struct DFSDM0_RDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register for the regular channel"]
pub mod dfsdm0_rdatar;
#[doc = "analog watchdog high threshold register"]
pub struct DFSDM0_AWHTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm0_awhtr;
#[doc = "analog watchdog low threshold register"]
pub struct DFSDM0_AWLTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm0_awltr;
#[doc = "analog watchdog status register"]
pub struct DFSDM0_AWSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog status register"]
pub mod dfsdm0_awsr;
#[doc = "analog watchdog clear flag register"]
pub struct DFSDM0_AWCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm0_awcfr;
#[doc = "Extremes detector maximum register"]
pub struct DFSDM0_EXMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extremes detector maximum register"]
pub mod dfsdm0_exmax;
#[doc = "Extremes detector minimum register"]
pub struct DFSDM0_EXMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extremes detector minimum register"]
pub mod dfsdm0_exmin;
#[doc = "conversion timer register"]
pub struct DFSDM0_CNVTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "conversion timer register"]
pub mod dfsdm0_cnvtimr;
#[doc = "control register 1"]
pub struct DFSDM1_CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod dfsdm1_cr1;
#[doc = "control register 2"]
pub struct DFSDM1_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 2"]
pub mod dfsdm1_cr2;
#[doc = "interrupt and status register"]
pub struct DFSDM1_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt and status register"]
pub mod dfsdm1_isr;
#[doc = "interrupt flag clear register"]
pub struct DFSDM1_ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt flag clear register"]
pub mod dfsdm1_icr;
#[doc = "injected channel group selection register"]
pub struct DFSDM1_JCHGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected channel group selection register"]
pub mod dfsdm1_jchgr;
#[doc = "filter control register"]
pub struct DFSDM1_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter control register"]
pub mod dfsdm1_fcr;
#[doc = "data register for injected group"]
pub struct DFSDM1_JDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register for injected group"]
pub mod dfsdm1_jdatar;
#[doc = "data register for the regular channel"]
pub struct DFSDM1_RDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register for the regular channel"]
pub mod dfsdm1_rdatar;
#[doc = "analog watchdog high threshold register"]
pub struct DFSDM1_AWHTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm1_awhtr;
#[doc = "analog watchdog low threshold register"]
pub struct DFSDM1_AWLTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm1_awltr;
#[doc = "analog watchdog status register"]
pub struct DFSDM1_AWSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog status register"]
pub mod dfsdm1_awsr;
#[doc = "analog watchdog clear flag register"]
pub struct DFSDM1_AWCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm1_awcfr;
#[doc = "Extremes detector maximum register"]
pub struct DFSDM1_EXMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extremes detector maximum register"]
pub mod dfsdm1_exmax;
#[doc = "Extremes detector minimum register"]
pub struct DFSDM1_EXMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extremes detector minimum register"]
pub mod dfsdm1_exmin;
#[doc = "conversion timer register"]
pub struct DFSDM1_CNVTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "conversion timer register"]
pub mod dfsdm1_cnvtimr;
#[doc = "control register 1"]
pub struct DFSDM2_CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod dfsdm2_cr1;
#[doc = "control register 2"]
pub struct DFSDM2_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 2"]
pub mod dfsdm2_cr2;
#[doc = "interrupt and status register"]
pub struct DFSDM2_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt and status register"]
pub mod dfsdm2_isr;
#[doc = "interrupt flag clear register"]
pub struct DFSDM2_ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt flag clear register"]
pub mod dfsdm2_icr;
#[doc = "injected channel group selection register"]
pub struct DFSDM2_JCHGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected channel group selection register"]
pub mod dfsdm2_jchgr;
#[doc = "filter control register"]
pub struct DFSDM2_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter control register"]
pub mod dfsdm2_fcr;
#[doc = "data register for injected group"]
pub struct DFSDM2_JDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register for injected group"]
pub mod dfsdm2_jdatar;
#[doc = "data register for the regular channel"]
pub struct DFSDM2_RDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register for the regular channel"]
pub mod dfsdm2_rdatar;
#[doc = "analog watchdog high threshold register"]
pub struct DFSDM2_AWHTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm2_awhtr;
#[doc = "analog watchdog low threshold register"]
pub struct DFSDM2_AWLTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm2_awltr;
#[doc = "analog watchdog status register"]
pub struct DFSDM2_AWSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog status register"]
pub mod dfsdm2_awsr;
#[doc = "analog watchdog clear flag register"]
pub struct DFSDM2_AWCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm2_awcfr;
#[doc = "Extremes detector maximum register"]
pub struct DFSDM2_EXMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extremes detector maximum register"]
pub mod dfsdm2_exmax;
#[doc = "Extremes detector minimum register"]
pub struct DFSDM2_EXMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extremes detector minimum register"]
pub mod dfsdm2_exmin;
#[doc = "conversion timer register"]
pub struct DFSDM2_CNVTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "conversion timer register"]
pub mod dfsdm2_cnvtimr;
#[doc = "control register 1"]
pub struct DFSDM3_CR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 1"]
pub mod dfsdm3_cr1;
#[doc = "control register 2"]
pub struct DFSDM3_CR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register 2"]
pub mod dfsdm3_cr2;
#[doc = "interrupt and status register"]
pub struct DFSDM3_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt and status register"]
pub mod dfsdm3_isr;
#[doc = "interrupt flag clear register"]
pub struct DFSDM3_ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt flag clear register"]
pub mod dfsdm3_icr;
#[doc = "injected channel group selection register"]
pub struct DFSDM3_JCHGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "injected channel group selection register"]
pub mod dfsdm3_jchgr;
#[doc = "filter control register"]
pub struct DFSDM3_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "filter control register"]
pub mod dfsdm3_fcr;
#[doc = "data register for injected group"]
pub struct DFSDM3_JDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register for injected group"]
pub mod dfsdm3_jdatar;
#[doc = "data register for the regular channel"]
pub struct DFSDM3_RDATAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register for the regular channel"]
pub mod dfsdm3_rdatar;
#[doc = "analog watchdog high threshold register"]
pub struct DFSDM3_AWHTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog high threshold register"]
pub mod dfsdm3_awhtr;
#[doc = "analog watchdog low threshold register"]
pub struct DFSDM3_AWLTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog low threshold register"]
pub mod dfsdm3_awltr;
#[doc = "analog watchdog status register"]
pub struct DFSDM3_AWSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog status register"]
pub mod dfsdm3_awsr;
#[doc = "analog watchdog clear flag register"]
pub struct DFSDM3_AWCFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "analog watchdog clear flag register"]
pub mod dfsdm3_awcfr;
#[doc = "Extremes detector maximum register"]
pub struct DFSDM3_EXMAX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extremes detector maximum register"]
pub mod dfsdm3_exmax;
#[doc = "Extremes detector minimum register"]
pub struct DFSDM3_EXMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extremes detector minimum register"]
pub mod dfsdm3_exmin;
#[doc = "conversion timer register"]
pub struct DFSDM3_CNVTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "conversion timer register"]
pub mod dfsdm3_cnvtimr;
