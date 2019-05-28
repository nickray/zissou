#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FS_DAINT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct IEPINTR {
    bits: u16,
}
impl IEPINTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OEPINTR {
    bits: u16,
}
impl OEPINTR {
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
    #[doc = "Bits 0:15 - IN endpoint interrupt bits"]
    #[inline]
    pub fn iepint(&self) -> IEPINTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        IEPINTR { bits }
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline]
    pub fn oepint(&self) -> OEPINTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        OEPINTR { bits }
    }
}
