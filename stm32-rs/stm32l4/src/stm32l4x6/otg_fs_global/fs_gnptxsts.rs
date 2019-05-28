#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FS_GNPTXSTS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NPTXFSAVR {
    bits: u16,
}
impl NPTXFSAVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NPTQXSAVR {
    bits: u8,
}
impl NPTQXSAVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NPTXQTOPR {
    bits: u8,
}
impl NPTXQTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Non-periodic TxFIFO space available"]
    #[inline]
    pub fn nptxfsav(&self) -> NPTXFSAVR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        NPTXFSAVR { bits }
    }
    #[doc = "Bits 16:23 - Non-periodic transmit request queue space available"]
    #[inline]
    pub fn nptqxsav(&self) -> NPTQXSAVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NPTQXSAVR { bits }
    }
    #[doc = "Bits 24:30 - Top of the non-periodic transmit request queue"]
    #[inline]
    pub fn nptxqtop(&self) -> NPTXQTOPR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NPTXQTOPR { bits }
    }
}
