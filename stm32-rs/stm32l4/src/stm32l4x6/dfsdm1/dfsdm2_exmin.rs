#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DFSDM2_EXMIN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct EXMINR {
    bits: u32,
}
impl EXMINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXMINCHR {
    bits: u8,
}
impl EXMINCHR {
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
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline]
    pub fn exmin(&self) -> EXMINR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        EXMINR { bits }
    }
    #[doc = "Bits 0:2 - Extremes detector minimum data channel"]
    #[inline]
    pub fn exminch(&self) -> EXMINCHR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EXMINCHR { bits }
    }
}
