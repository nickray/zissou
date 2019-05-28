#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR2 {
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
#[doc = "Possible values of the field `MMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMSR {
    #[doc = "Use UG bit from TIMx_EGR register"]
    RESET,
    #[doc = "Use CNT bit from TIMx_CEN register"]
    ENABLE,
    #[doc = "Use the update event"]
    UPDATE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MMSR::RESET => 0,
            MMSR::ENABLE => 1,
            MMSR::UPDATE => 2,
            MMSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MMSR {
        match value {
            0 => MMSR::RESET,
            1 => MMSR::ENABLE,
            2 => MMSR::UPDATE,
            i => MMSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == MMSR::RESET
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MMSR::ENABLE
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline]
    pub fn is_update(&self) -> bool {
        *self == MMSR::UPDATE
    }
}
#[doc = "Values that can be written to the field `MMS`"]
pub enum MMSW {
    #[doc = "Use UG bit from TIMx_EGR register"]
    RESET,
    #[doc = "Use CNT bit from TIMx_CEN register"]
    ENABLE,
    #[doc = "Use the update event"]
    UPDATE,
}
impl MMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MMSW::RESET => 0,
            MMSW::ENABLE => 1,
            MMSW::UPDATE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMSW<'a> {
    w: &'a mut W,
}
impl<'a> _MMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use UG bit from TIMx_EGR register"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMSW::RESET)
    }
    #[doc = "Use CNT bit from TIMx_CEN register"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMSW::ENABLE)
    }
    #[doc = "Use the update event"]
    #[inline]
    pub fn update(self) -> &'a mut W {
        self.variant(MMSW::UPDATE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline]
    pub fn mms(&self) -> MMSR {
        MMSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline]
    pub fn mms(&mut self) -> _MMSW {
        _MMSW { w: self }
    }
}
