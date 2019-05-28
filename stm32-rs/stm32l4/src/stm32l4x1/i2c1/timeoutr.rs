#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMEOUTR {
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
pub struct TIMEOUTAR {
    bits: u16,
}
impl TIMEOUTAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `TIDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIDLER {
    #[doc = "TIMEOUTA is used to detect SCL low timeout"]
    DISABLED,
    #[doc = "TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    ENABLED,
}
impl TIDLER {
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
            TIDLER::DISABLED => false,
            TIDLER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIDLER {
        match value {
            false => TIDLER::DISABLED,
            true => TIDLER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TIDLER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TIDLER::ENABLED
    }
}
#[doc = "Possible values of the field `TIMOUTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMOUTENR {
    #[doc = "SCL timeout detection is disabled"]
    DISABLED,
    #[doc = "SCL timeout detection is enabled"]
    ENABLED,
}
impl TIMOUTENR {
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
            TIMOUTENR::DISABLED => false,
            TIMOUTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMOUTENR {
        match value {
            false => TIMOUTENR::DISABLED,
            true => TIMOUTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TIMOUTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TIMOUTENR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct TIMEOUTBR {
    bits: u16,
}
impl TIMEOUTBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `TEXTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEXTENR {
    #[doc = "Extended clock timeout detection is disabled"]
    DISABLED,
    #[doc = "Extended clock timeout detection is enabled"]
    ENABLED,
}
impl TEXTENR {
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
            TEXTENR::DISABLED => false,
            TEXTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEXTENR {
        match value {
            false => TEXTENR::DISABLED,
            true => TEXTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TEXTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TEXTENR::ENABLED
    }
}
#[doc = r" Proxy"]
pub struct _TIMEOUTAW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIDLE`"]
pub enum TIDLEW {
    #[doc = "TIMEOUTA is used to detect SCL low timeout"]
    DISABLED,
    #[doc = "TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    ENABLED,
}
impl TIDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIDLEW::DISABLED => false,
            TIDLEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TIMEOUTA is used to detect SCL low timeout"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIDLEW::DISABLED)
    }
    #[doc = "TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIDLEW::ENABLED)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMOUTEN`"]
pub enum TIMOUTENW {
    #[doc = "SCL timeout detection is disabled"]
    DISABLED,
    #[doc = "SCL timeout detection is enabled"]
    ENABLED,
}
impl TIMOUTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMOUTENW::DISABLED => false,
            TIMOUTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMOUTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMOUTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMOUTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCL timeout detection is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIMOUTENW::DISABLED)
    }
    #[doc = "SCL timeout detection is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIMOUTENW::ENABLED)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMEOUTBW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TEXTEN`"]
pub enum TEXTENW {
    #[doc = "Extended clock timeout detection is disabled"]
    DISABLED,
    #[doc = "Extended clock timeout detection is enabled"]
    ENABLED,
}
impl TEXTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEXTENW::DISABLED => false,
            TEXTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEXTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TEXTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEXTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Extended clock timeout detection is disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEXTENW::DISABLED)
    }
    #[doc = "Extended clock timeout detection is enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEXTENW::ENABLED)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:11 - Bus timeout A"]
    #[inline]
    pub fn timeouta(&self) -> TIMEOUTAR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TIMEOUTAR { bits }
    }
    #[doc = "Bit 12 - Idle clock timeout detection"]
    #[inline]
    pub fn tidle(&self) -> TIDLER {
        TIDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Clock timeout enable"]
    #[inline]
    pub fn timouten(&self) -> TIMOUTENR {
        TIMOUTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:27 - Bus timeout B"]
    #[inline]
    pub fn timeoutb(&self) -> TIMEOUTBR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TIMEOUTBR { bits }
    }
    #[doc = "Bit 31 - Extended clock timeout enable"]
    #[inline]
    pub fn texten(&self) -> TEXTENR {
        TEXTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:11 - Bus timeout A"]
    #[inline]
    pub fn timeouta(&mut self) -> _TIMEOUTAW {
        _TIMEOUTAW { w: self }
    }
    #[doc = "Bit 12 - Idle clock timeout detection"]
    #[inline]
    pub fn tidle(&mut self) -> _TIDLEW {
        _TIDLEW { w: self }
    }
    #[doc = "Bit 15 - Clock timeout enable"]
    #[inline]
    pub fn timouten(&mut self) -> _TIMOUTENW {
        _TIMOUTENW { w: self }
    }
    #[doc = "Bits 16:27 - Bus timeout B"]
    #[inline]
    pub fn timeoutb(&mut self) -> _TIMEOUTBW {
        _TIMEOUTBW { w: self }
    }
    #[doc = "Bit 31 - Extended clock timeout enable"]
    #[inline]
    pub fn texten(&mut self) -> _TEXTENW {
        _TEXTENW { w: self }
    }
}
