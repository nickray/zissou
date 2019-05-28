#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DFSDM1_FCR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct FORDR {
    bits: u8,
}
impl FORDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FOSRR {
    bits: u16,
}
impl FOSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IOSRR {
    bits: u8,
}
impl IOSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _FORDW<'a> {
    w: &'a mut W,
}
impl<'a> _FORDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FOSRW<'a> {
    w: &'a mut W,
}
impl<'a> _FOSRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IOSRW<'a> {
    w: &'a mut W,
}
impl<'a> _IOSRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 29:31 - Sinc filter order"]
    #[inline]
    pub fn ford(&self) -> FORDR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FORDR { bits }
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)"]
    #[inline]
    pub fn fosr(&self) -> FOSRR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FOSRR { bits }
    }
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)"]
    #[inline]
    pub fn iosr(&self) -> IOSRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IOSRR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 29:31 - Sinc filter order"]
    #[inline]
    pub fn ford(&mut self) -> _FORDW {
        _FORDW { w: self }
    }
    #[doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)"]
    #[inline]
    pub fn fosr(&mut self) -> _FOSRW {
        _FOSRW { w: self }
    }
    #[doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)"]
    #[inline]
    pub fn iosr(&mut self) -> _IOSRW {
        _IOSRW { w: self }
    }
}
