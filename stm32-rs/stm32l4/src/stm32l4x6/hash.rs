#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - data input register"]
    pub din: DIN,
    #[doc = "0x08 - start register"]
    pub str: STR,
    #[doc = "0x0c - digest registers"]
    pub hr0: HR0,
    _reserved0: [u8; 16usize],
    #[doc = "0x20 - interrupt enable register"]
    pub imr: IMR,
    #[doc = "0x24 - status register"]
    pub sr: SR,
    _reserved1: [u8; 208usize],
    #[doc = "0xf8 - context swap registers"]
    pub csr0: CSR0,
    #[doc = "0xfc - context swap registers"]
    pub csr1: CSR1,
    #[doc = "0x100 - context swap registers"]
    pub csr2: CSR2,
    #[doc = "0x104 - context swap registers"]
    pub csr3: CSR3,
    #[doc = "0x108 - context swap registers"]
    pub csr4: CSR4,
    #[doc = "0x10c - context swap registers"]
    pub csr5: CSR5,
    #[doc = "0x110 - context swap registers"]
    pub csr6: CSR6,
    #[doc = "0x114 - context swap registers"]
    pub csr7: CSR7,
    #[doc = "0x118 - context swap registers"]
    pub csr8: CSR8,
    #[doc = "0x11c - context swap registers"]
    pub csr9: CSR9,
    #[doc = "0x120 - context swap registers"]
    pub csr10: CSR10,
    #[doc = "0x124 - context swap registers"]
    pub csr11: CSR11,
    #[doc = "0x128 - context swap registers"]
    pub csr12: CSR12,
    #[doc = "0x12c - context swap registers"]
    pub csr13: CSR13,
    #[doc = "0x130 - context swap registers"]
    pub csr14: CSR14,
    #[doc = "0x134 - context swap registers"]
    pub csr15: CSR15,
    #[doc = "0x138 - context swap registers"]
    pub csr16: CSR16,
    #[doc = "0x13c - context swap registers"]
    pub csr17: CSR17,
    #[doc = "0x140 - context swap registers"]
    pub csr18: CSR18,
    #[doc = "0x144 - context swap registers"]
    pub csr19: CSR19,
    #[doc = "0x148 - context swap registers"]
    pub csr20: CSR20,
    #[doc = "0x14c - context swap registers"]
    pub csr21: CSR21,
    #[doc = "0x150 - context swap registers"]
    pub csr22: CSR22,
    #[doc = "0x154 - context swap registers"]
    pub csr23: CSR23,
    #[doc = "0x158 - context swap registers"]
    pub csr24: CSR24,
    #[doc = "0x15c - context swap registers"]
    pub csr25: CSR25,
    #[doc = "0x160 - context swap registers"]
    pub csr26: CSR26,
    #[doc = "0x164 - context swap registers"]
    pub csr27: CSR27,
    #[doc = "0x168 - context swap registers"]
    pub csr28: CSR28,
    #[doc = "0x16c - context swap registers"]
    pub csr29: CSR29,
    #[doc = "0x170 - context swap registers"]
    pub csr30: CSR30,
    #[doc = "0x174 - context swap registers"]
    pub csr31: CSR31,
    #[doc = "0x178 - context swap registers"]
    pub csr32: CSR32,
    #[doc = "0x17c - context swap registers"]
    pub csr33: CSR33,
    #[doc = "0x180 - context swap registers"]
    pub csr34: CSR34,
    #[doc = "0x184 - context swap registers"]
    pub csr35: CSR35,
    #[doc = "0x188 - context swap registers"]
    pub csr36: CSR36,
    #[doc = "0x18c - context swap registers"]
    pub csr37: CSR37,
    #[doc = "0x190 - context swap registers"]
    pub csr38: CSR38,
    #[doc = "0x194 - context swap registers"]
    pub csr39: CSR39,
    #[doc = "0x198 - context swap registers"]
    pub csr40: CSR40,
    #[doc = "0x19c - context swap registers"]
    pub csr41: CSR41,
    #[doc = "0x1a0 - context swap registers"]
    pub csr42: CSR42,
    #[doc = "0x1a4 - context swap registers"]
    pub csr43: CSR43,
    #[doc = "0x1a8 - context swap registers"]
    pub csr44: CSR44,
    #[doc = "0x1ac - context swap registers"]
    pub csr45: CSR45,
    #[doc = "0x1b0 - context swap registers"]
    pub csr46: CSR46,
    #[doc = "0x1b4 - context swap registers"]
    pub csr47: CSR47,
    #[doc = "0x1b8 - context swap registers"]
    pub csr48: CSR48,
    #[doc = "0x1bc - context swap registers"]
    pub csr49: CSR49,
    #[doc = "0x1c0 - context swap registers"]
    pub csr50: CSR50,
    #[doc = "0x1c4 - context swap registers"]
    pub csr51: CSR51,
    #[doc = "0x1c8 - context swap registers"]
    pub csr52: CSR52,
    #[doc = "0x1cc - context swap registers"]
    pub csr53: CSR53,
    _reserved2: [u8; 320usize],
    #[doc = "0x310 - HASH digest register"]
    pub hash_hr0: HASH_HR0,
    #[doc = "0x314 - read-only"]
    pub hash_hr1: HASH_HR1,
    #[doc = "0x318 - read-only"]
    pub hash_hr2: HASH_HR2,
    #[doc = "0x31c - read-only"]
    pub hash_hr3: HASH_HR3,
    #[doc = "0x320 - read-only"]
    pub hash_hr4: HASH_HR4,
    #[doc = "0x324 - read-only"]
    pub hash_hr5: HASH_HR5,
    #[doc = "0x328 - read-only"]
    pub hash_hr6: HASH_HR6,
    #[doc = "0x32c - read-only"]
    pub hash_hr7: HASH_HR7,
}
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "data input register"]
pub struct DIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data input register"]
pub mod din;
#[doc = "start register"]
pub struct STR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "start register"]
pub mod str;
#[doc = "digest registers"]
pub struct HR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "digest registers"]
pub mod hr0;
#[doc = "interrupt enable register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod imr;
#[doc = "status register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "status register"]
pub mod sr;
#[doc = "context swap registers"]
pub struct CSR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr0;
#[doc = "context swap registers"]
pub struct CSR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr1;
#[doc = "context swap registers"]
pub struct CSR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr2;
#[doc = "context swap registers"]
pub struct CSR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr3;
#[doc = "context swap registers"]
pub struct CSR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr4;
#[doc = "context swap registers"]
pub struct CSR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr5;
#[doc = "context swap registers"]
pub struct CSR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr6;
#[doc = "context swap registers"]
pub struct CSR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr7;
#[doc = "context swap registers"]
pub struct CSR8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr8;
#[doc = "context swap registers"]
pub struct CSR9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr9;
#[doc = "context swap registers"]
pub struct CSR10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr10;
#[doc = "context swap registers"]
pub struct CSR11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr11;
#[doc = "context swap registers"]
pub struct CSR12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr12;
#[doc = "context swap registers"]
pub struct CSR13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr13;
#[doc = "context swap registers"]
pub struct CSR14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr14;
#[doc = "context swap registers"]
pub struct CSR15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr15;
#[doc = "context swap registers"]
pub struct CSR16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr16;
#[doc = "context swap registers"]
pub struct CSR17 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr17;
#[doc = "context swap registers"]
pub struct CSR18 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr18;
#[doc = "context swap registers"]
pub struct CSR19 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr19;
#[doc = "context swap registers"]
pub struct CSR20 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr20;
#[doc = "context swap registers"]
pub struct CSR21 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr21;
#[doc = "context swap registers"]
pub struct CSR22 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr22;
#[doc = "context swap registers"]
pub struct CSR23 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr23;
#[doc = "context swap registers"]
pub struct CSR24 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr24;
#[doc = "context swap registers"]
pub struct CSR25 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr25;
#[doc = "context swap registers"]
pub struct CSR26 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr26;
#[doc = "context swap registers"]
pub struct CSR27 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr27;
#[doc = "context swap registers"]
pub struct CSR28 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr28;
#[doc = "context swap registers"]
pub struct CSR29 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr29;
#[doc = "context swap registers"]
pub struct CSR30 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr30;
#[doc = "context swap registers"]
pub struct CSR31 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr31;
#[doc = "context swap registers"]
pub struct CSR32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr32;
#[doc = "context swap registers"]
pub struct CSR33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr33;
#[doc = "context swap registers"]
pub struct CSR34 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr34;
#[doc = "context swap registers"]
pub struct CSR35 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr35;
#[doc = "context swap registers"]
pub struct CSR36 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr36;
#[doc = "context swap registers"]
pub struct CSR37 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr37;
#[doc = "context swap registers"]
pub struct CSR38 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr38;
#[doc = "context swap registers"]
pub struct CSR39 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr39;
#[doc = "context swap registers"]
pub struct CSR40 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr40;
#[doc = "context swap registers"]
pub struct CSR41 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr41;
#[doc = "context swap registers"]
pub struct CSR42 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr42;
#[doc = "context swap registers"]
pub struct CSR43 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr43;
#[doc = "context swap registers"]
pub struct CSR44 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr44;
#[doc = "context swap registers"]
pub struct CSR45 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr45;
#[doc = "context swap registers"]
pub struct CSR46 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr46;
#[doc = "context swap registers"]
pub struct CSR47 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr47;
#[doc = "context swap registers"]
pub struct CSR48 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr48;
#[doc = "context swap registers"]
pub struct CSR49 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr49;
#[doc = "context swap registers"]
pub struct CSR50 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr50;
#[doc = "context swap registers"]
pub struct CSR51 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr51;
#[doc = "context swap registers"]
pub struct CSR52 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr52;
#[doc = "context swap registers"]
pub struct CSR53 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "context swap registers"]
pub mod csr53;
#[doc = "HASH digest register"]
pub struct HASH_HR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HASH digest register"]
pub mod hash_hr0;
#[doc = "read-only"]
pub struct HASH_HR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "read-only"]
pub mod hash_hr1;
#[doc = "read-only"]
pub struct HASH_HR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "read-only"]
pub mod hash_hr2;
#[doc = "read-only"]
pub struct HASH_HR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "read-only"]
pub mod hash_hr3;
#[doc = "read-only"]
pub struct HASH_HR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "read-only"]
pub mod hash_hr4;
#[doc = "read-only"]
pub struct HASH_HR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "read-only"]
pub mod hash_hr5;
#[doc = "read-only"]
pub struct HASH_HR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "read-only"]
pub mod hash_hr6;
#[doc = "read-only"]
pub struct HASH_HR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "read-only"]
pub mod hash_hr7;
