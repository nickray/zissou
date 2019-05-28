#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct BYTE3R {
    bits: u8,
}
impl BYTE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BYTE2R {
    bits: u8,
}
impl BYTE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BYTE1R {
    bits: u8,
}
impl BYTE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BYTE0R {
    bits: u8,
}
impl BYTE0R {
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
    #[doc = "Bits 24:31 - Data byte 3"]
    #[inline]
    pub fn byte3(&self) -> BYTE3R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BYTE3R { bits }
    }
    #[doc = "Bits 16:23 - Data byte 2"]
    #[inline]
    pub fn byte2(&self) -> BYTE2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BYTE2R { bits }
    }
    #[doc = "Bits 8:15 - Data byte 1"]
    #[inline]
    pub fn byte1(&self) -> BYTE1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BYTE1R { bits }
    }
    #[doc = "Bits 0:7 - Data byte 0"]
    #[inline]
    pub fn byte0(&self) -> BYTE0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BYTE0R { bits }
    }
}
