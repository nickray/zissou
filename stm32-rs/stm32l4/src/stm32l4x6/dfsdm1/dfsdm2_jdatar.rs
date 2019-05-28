#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DFSDM2_JDATAR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct JDATAR {
    bits: u32,
}
impl JDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct JDATACHR {
    bits: u8,
}
impl JDATACHR {
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
    #[doc = "Bits 8:31 - Injected group conversion data"]
    #[inline]
    pub fn jdata(&self) -> JDATAR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        JDATAR { bits }
    }
    #[doc = "Bits 0:2 - Injected channel most recently converted"]
    #[inline]
    pub fn jdatach(&self) -> JDATACHR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        JDATACHR { bits }
    }
}
