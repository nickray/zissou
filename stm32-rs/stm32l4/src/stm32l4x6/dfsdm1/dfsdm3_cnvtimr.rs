#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DFSDM3_CNVTIMR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CNVCNTR {
    bits: u32,
}
impl CNVCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT\\[27:0\\] / fDFSDM_CKIN"]
    #[inline]
    pub fn cnvcnt(&self) -> CNVCNTR {
        let bits = {
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CNVCNTR { bits }
    }
}
