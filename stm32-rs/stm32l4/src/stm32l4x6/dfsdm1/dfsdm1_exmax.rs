#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DFSDM1_EXMAX {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct EXMAXR {
    bits: u32,
}
impl EXMAXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXMAXCHR {
    bits: u8,
}
impl EXMAXCHR {
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
    #[doc = "Bits 8:31 - Extremes detector maximum value"]
    #[inline]
    pub fn exmax(&self) -> EXMAXR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        EXMAXR { bits }
    }
    #[doc = "Bits 0:2 - Extremes detector maximum data channel"]
    #[inline]
    pub fn exmaxch(&self) -> EXMAXCHR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXMAXCHR { bits }
    }
}
