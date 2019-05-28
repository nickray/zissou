#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FS_DSTS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SUSPSTSR {
    bits: bool,
}
impl SUSPSTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ENUMSPDR {
    bits: u8,
}
impl ENUMSPDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EERRR {
    bits: bool,
}
impl EERRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FNSOFR {
    bits: u16,
}
impl FNSOFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Suspend status"]
    #[inline]
    pub fn suspsts(&self) -> SUSPSTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SUSPSTSR { bits }
    }
    #[doc = "Bits 1:2 - Enumerated speed"]
    #[inline]
    pub fn enumspd(&self) -> ENUMSPDR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ENUMSPDR { bits }
    }
    #[doc = "Bit 3 - Erratic error"]
    #[inline]
    pub fn eerr(&self) -> EERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EERRR { bits }
    }
    #[doc = "Bits 8:21 - Frame number of the received SOF"]
    #[inline]
    pub fn fnsof(&self) -> FNSOFR {
        let bits = {
            const MASK: u16 = 16383;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FNSOFR { bits }
    }
}
