#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DFSDM2_AWSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct AWHTFR {
    bits: u8,
}
impl AWHTFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct AWLTFR {
    bits: u8,
}
impl AWLTFR {
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
    #[doc = "Bits 8:15 - Analog watchdog high threshold flag"]
    #[inline]
    pub fn awhtf(&self) -> AWHTFR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AWHTFR { bits }
    }
    #[doc = "Bits 0:7 - Analog watchdog low threshold flag"]
    #[inline]
    pub fn awltf(&self) -> AWLTFR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AWLTFR { bits }
    }
}
