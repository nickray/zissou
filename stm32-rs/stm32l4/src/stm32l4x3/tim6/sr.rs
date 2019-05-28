#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR {
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
#[doc = "Possible values of the field `UIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIFR {
    #[doc = "No update occurred"]
    CLEAR,
    #[doc = "Update interrupt pending."]
    UPDATEPENDING,
}
impl UIFR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UIFR::CLEAR => false,
            UIFR::UPDATEPENDING => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UIFR {
        match value {
            false => UIFR::CLEAR,
            true => UIFR::UPDATEPENDING,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == UIFR::CLEAR
    }
    #[doc = "Checks if the value of the field is `UPDATEPENDING`"]
    #[inline]
    pub fn is_update_pending(&self) -> bool {
        *self == UIFR::UPDATEPENDING
    }
}
#[doc = "Values that can be written to the field `UIF`"]
pub enum UIFW {
    #[doc = "No update occurred"]
    CLEAR,
    #[doc = "Update interrupt pending."]
    UPDATEPENDING,
}
impl UIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UIFW::CLEAR => false,
            UIFW::UPDATEPENDING => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UIFW<'a> {
    w: &'a mut W,
}
impl<'a> _UIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No update occurred"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(UIFW::CLEAR)
    }
    #[doc = "Update interrupt pending."]
    #[inline]
    pub fn update_pending(self) -> &'a mut W {
        self.variant(UIFW::UPDATEPENDING)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
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
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline]
    pub fn uif(&self) -> UIFR {
        UIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline]
    pub fn uif(&mut self) -> _UIFW {
        _UIFW { w: self }
    }
}
