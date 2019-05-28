#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DIER {
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
#[doc = "Possible values of the field `TDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDER {
    #[doc = "Trigger DMA request disabled"]
    DISABLED,
    #[doc = "Trigger DMA request enabled"]
    ENABLED,
}
impl TDER {
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
            TDER::DISABLED => false,
            TDER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDER {
        match value {
            false => TDER::DISABLED,
            true => TDER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TDER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TDER::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct COMDER {
    bits: bool,
}
impl COMDER {
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
#[doc = "Possible values of the field `CC4DE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4DER {
    #[doc = "CCx DMA request disabled"]
    DISABLED,
    #[doc = "CCx DMA request enabled"]
    ENABLED,
}
impl CC4DER {
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
            CC4DER::DISABLED => false,
            CC4DER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC4DER {
        match value {
            false => CC4DER::DISABLED,
            true => CC4DER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CC4DER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CC4DER::ENABLED
    }
}
#[doc = "Possible values of the field `CC3DE`"]
pub type CC3DER = CC4DER;
#[doc = "Possible values of the field `CC2DE`"]
pub type CC2DER = CC4DER;
#[doc = "Possible values of the field `CC1DE`"]
pub type CC1DER = CC4DER;
#[doc = "Possible values of the field `UDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDER {
    #[doc = "Update DMA request disabled"]
    DISABLED,
    #[doc = "Update DMA request enabled"]
    ENABLED,
}
impl UDER {
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
            UDER::DISABLED => false,
            UDER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UDER {
        match value {
            false => UDER::DISABLED,
            true => UDER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == UDER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == UDER::ENABLED
    }
}
#[doc = "Possible values of the field `TIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIER {
    #[doc = "Trigger interrupt disabled"]
    DISABLED,
    #[doc = "Trigger interrupt enabled"]
    ENABLED,
}
impl TIER {
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
            TIER::DISABLED => false,
            TIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIER {
        match value {
            false => TIER::DISABLED,
            true => TIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TIER::ENABLED
    }
}
#[doc = "Possible values of the field `CC4IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4IER {
    #[doc = "CCx interrupt disabled"]
    DISABLED,
    #[doc = "CCx interrupt enabled"]
    ENABLED,
}
impl CC4IER {
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
            CC4IER::DISABLED => false,
            CC4IER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC4IER {
        match value {
            false => CC4IER::DISABLED,
            true => CC4IER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CC4IER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CC4IER::ENABLED
    }
}
#[doc = "Possible values of the field `CC3IE`"]
pub type CC3IER = CC4IER;
#[doc = "Possible values of the field `CC2IE`"]
pub type CC2IER = CC4IER;
#[doc = "Possible values of the field `CC1IE`"]
pub type CC1IER = CC4IER;
#[doc = "Possible values of the field `UIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIER {
    #[doc = "Update interrupt disabled"]
    DISABLED,
    #[doc = "Update interrupt enabled"]
    ENABLED,
}
impl UIER {
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
            UIER::DISABLED => false,
            UIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UIER {
        match value {
            false => UIER::DISABLED,
            true => UIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == UIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == UIER::ENABLED
    }
}
#[doc = "Values that can be written to the field `TDE`"]
pub enum TDEW {
    #[doc = "Trigger DMA request disabled"]
    DISABLED,
    #[doc = "Trigger DMA request enabled"]
    ENABLED,
}
impl TDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDEW::DISABLED => false,
            TDEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger DMA request disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDEW::DISABLED)
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDEW::ENABLED)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COMDEW<'a> {
    w: &'a mut W,
}
impl<'a> _COMDEW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC4DE`"]
pub enum CC4DEW {
    #[doc = "CCx DMA request disabled"]
    DISABLED,
    #[doc = "CCx DMA request enabled"]
    ENABLED,
}
impl CC4DEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4DEW::DISABLED => false,
            CC4DEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4DEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4DEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx DMA request disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4DEW::DISABLED)
    }
    #[doc = "CCx DMA request enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4DEW::ENABLED)
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
#[doc = "Values that can be written to the field `CC3DE`"]
pub type CC3DEW = CC4DEW;
#[doc = r" Proxy"]
pub struct _CC3DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3DEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3DEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx DMA request disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4DEW::DISABLED)
    }
    #[doc = "CCx DMA request enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4DEW::ENABLED)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC2DE`"]
pub type CC2DEW = CC4DEW;
#[doc = r" Proxy"]
pub struct _CC2DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2DEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2DEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx DMA request disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4DEW::DISABLED)
    }
    #[doc = "CCx DMA request enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4DEW::ENABLED)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC1DE`"]
pub type CC1DEW = CC4DEW;
#[doc = r" Proxy"]
pub struct _CC1DEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1DEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1DEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx DMA request disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4DEW::DISABLED)
    }
    #[doc = "CCx DMA request enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4DEW::ENABLED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UDE`"]
pub enum UDEW {
    #[doc = "Update DMA request disabled"]
    DISABLED,
    #[doc = "Update DMA request enabled"]
    ENABLED,
}
impl UDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UDEW::DISABLED => false,
            UDEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UDEW<'a> {
    w: &'a mut W,
}
impl<'a> _UDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Update DMA request disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDEW::DISABLED)
    }
    #[doc = "Update DMA request enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDEW::ENABLED)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIE`"]
pub enum TIEW {
    #[doc = "Trigger interrupt disabled"]
    DISABLED,
    #[doc = "Trigger interrupt enabled"]
    ENABLED,
}
impl TIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIEW::DISABLED => false,
            TIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIEW::DISABLED)
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIEW::ENABLED)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC4IE`"]
pub enum CC4IEW {
    #[doc = "CCx interrupt disabled"]
    DISABLED,
    #[doc = "CCx interrupt enabled"]
    ENABLED,
}
impl CC4IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4IEW::DISABLED => false,
            CC4IEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4IEW::DISABLED)
    }
    #[doc = "CCx interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4IEW::ENABLED)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC3IE`"]
pub type CC3IEW = CC4IEW;
#[doc = r" Proxy"]
pub struct _CC3IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4IEW::DISABLED)
    }
    #[doc = "CCx interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4IEW::ENABLED)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC2IE`"]
pub type CC2IEW = CC4IEW;
#[doc = r" Proxy"]
pub struct _CC2IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4IEW::DISABLED)
    }
    #[doc = "CCx interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4IEW::ENABLED)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC1IE`"]
pub type CC1IEW = CC4IEW;
#[doc = r" Proxy"]
pub struct _CC1IEW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CC4IEW::DISABLED)
    }
    #[doc = "CCx interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CC4IEW::ENABLED)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UIE`"]
pub enum UIEW {
    #[doc = "Update interrupt disabled"]
    DISABLED,
    #[doc = "Update interrupt enabled"]
    ENABLED,
}
impl UIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UIEW::DISABLED => false,
            UIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UIEW<'a> {
    w: &'a mut W,
}
impl<'a> _UIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Update interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UIEW::DISABLED)
    }
    #[doc = "Update interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UIEW::ENABLED)
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
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline]
    pub fn tde(&self) -> TDER {
        TDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline]
    pub fn comde(&self) -> COMDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COMDER { bits }
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline]
    pub fn cc4de(&self) -> CC4DER {
        CC4DER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline]
    pub fn cc3de(&self) -> CC3DER {
        CC3DER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline]
    pub fn cc2de(&self) -> CC2DER {
        CC2DER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline]
    pub fn cc1de(&self) -> CC1DER {
        CC1DER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline]
    pub fn ude(&self) -> UDER {
        UDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline]
    pub fn tie(&self) -> TIER {
        TIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline]
    pub fn cc4ie(&self) -> CC4IER {
        CC4IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline]
    pub fn cc3ie(&self) -> CC3IER {
        CC3IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline]
    pub fn cc2ie(&self) -> CC2IER {
        CC2IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline]
    pub fn cc1ie(&self) -> CC1IER {
        CC1IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline]
    pub fn uie(&self) -> UIER {
        UIER::_from({
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
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline]
    pub fn tde(&mut self) -> _TDEW {
        _TDEW { w: self }
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline]
    pub fn comde(&mut self) -> _COMDEW {
        _COMDEW { w: self }
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline]
    pub fn cc4de(&mut self) -> _CC4DEW {
        _CC4DEW { w: self }
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline]
    pub fn cc3de(&mut self) -> _CC3DEW {
        _CC3DEW { w: self }
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline]
    pub fn cc2de(&mut self) -> _CC2DEW {
        _CC2DEW { w: self }
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline]
    pub fn cc1de(&mut self) -> _CC1DEW {
        _CC1DEW { w: self }
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline]
    pub fn ude(&mut self) -> _UDEW {
        _UDEW { w: self }
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline]
    pub fn tie(&mut self) -> _TIEW {
        _TIEW { w: self }
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline]
    pub fn cc4ie(&mut self) -> _CC4IEW {
        _CC4IEW { w: self }
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline]
    pub fn cc3ie(&mut self) -> _CC3IEW {
        _CC3IEW { w: self }
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline]
    pub fn cc2ie(&mut self) -> _CC2IEW {
        _CC2IEW { w: self }
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline]
    pub fn cc1ie(&mut self) -> _CC1IEW {
        _CC1IEW { w: self }
    }
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline]
    pub fn uie(&mut self) -> _UIEW {
        _UIEW { w: self }
    }
}
