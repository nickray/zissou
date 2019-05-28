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
#[doc = "Possible values of the field `CC4OF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4OFR {
    #[doc = "The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set"]
    OVERCAPTURE,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CC4OFR {
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
            CC4OFR::OVERCAPTURE => true,
            CC4OFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC4OFR {
        match value {
            true => CC4OFR::OVERCAPTURE,
            i => CC4OFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OVERCAPTURE`"]
    #[inline]
    pub fn is_overcapture(&self) -> bool {
        *self == CC4OFR::OVERCAPTURE
    }
}
#[doc = "Possible values of the field `CC3OF`"]
pub type CC3OFR = CC4OFR;
#[doc = "Possible values of the field `CC2OF`"]
pub type CC2OFR = CC4OFR;
#[doc = "Possible values of the field `CC1OF`"]
pub type CC1OFR = CC4OFR;
#[doc = "Possible values of the field `TIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIFR {
    #[doc = "No trigger event occurred"]
    NOTRIGGER,
    #[doc = "Trigger interrupt pending"]
    TRIGGER,
}
impl TIFR {
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
            TIFR::NOTRIGGER => false,
            TIFR::TRIGGER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIFR {
        match value {
            false => TIFR::NOTRIGGER,
            true => TIFR::TRIGGER,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRIGGER`"]
    #[inline]
    pub fn is_no_trigger(&self) -> bool {
        *self == TIFR::NOTRIGGER
    }
    #[doc = "Checks if the value of the field is `TRIGGER`"]
    #[inline]
    pub fn is_trigger(&self) -> bool {
        *self == TIFR::TRIGGER
    }
}
#[doc = "Possible values of the field `CC4IF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4IFR {
    #[doc = "If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register."]
    MATCH,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CC4IFR {
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
            CC4IFR::MATCH => true,
            CC4IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CC4IFR {
        match value {
            true => CC4IFR::MATCH,
            i => CC4IFR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == CC4IFR::MATCH
    }
}
#[doc = "Possible values of the field `CC3IF`"]
pub type CC3IFR = CC4IFR;
#[doc = "Possible values of the field `CC2IF`"]
pub type CC2IFR = CC4IFR;
#[doc = "Possible values of the field `CC1IF`"]
pub type CC1IFR = CC4IFR;
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
#[doc = "Values that can be written to the field `CC4OF`"]
pub enum CC4OFW {
    #[doc = "Clear flag"]
    CLEAR,
}
impl CC4OFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4OFW::CLEAR => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4OFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4OFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4OFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC4OFW::CLEAR)
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
#[doc = "Values that can be written to the field `CC3OF`"]
pub type CC3OFW = CC4OFW;
#[doc = r" Proxy"]
pub struct _CC3OFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3OFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3OFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC4OFW::CLEAR)
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
#[doc = "Values that can be written to the field `CC2OF`"]
pub type CC2OFW = CC4OFW;
#[doc = r" Proxy"]
pub struct _CC2OFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2OFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2OFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC4OFW::CLEAR)
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
#[doc = "Values that can be written to the field `CC1OF`"]
pub type CC1OFW = CC4OFW;
#[doc = r" Proxy"]
pub struct _CC1OFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1OFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1OFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC4OFW::CLEAR)
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
#[doc = "Values that can be written to the field `TIF`"]
pub enum TIFW {
    #[doc = "Clear flag"]
    CLEAR,
}
impl TIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIFW::CLEAR => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIFW<'a> {
    w: &'a mut W,
}
impl<'a> _TIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIFW::CLEAR)
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
#[doc = "Values that can be written to the field `CC4IF`"]
pub enum CC4IFW {
    #[doc = "Clear flag"]
    CLEAR,
}
impl CC4IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4IFW::CLEAR => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4IFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4IFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC4IFW::CLEAR)
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
#[doc = "Values that can be written to the field `CC3IF`"]
pub type CC3IFW = CC4IFW;
#[doc = r" Proxy"]
pub struct _CC3IFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3IFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC4IFW::CLEAR)
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
#[doc = "Values that can be written to the field `CC2IF`"]
pub type CC2IFW = CC4IFW;
#[doc = r" Proxy"]
pub struct _CC2IFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2IFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC4IFW::CLEAR)
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
#[doc = "Values that can be written to the field `CC1IF`"]
pub type CC1IFW = CC4IFW;
#[doc = r" Proxy"]
pub struct _CC1IFW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1IFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clear flag"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CC4IFW::CLEAR)
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
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline]
    pub fn cc4of(&self) -> CC4OFR {
        CC4OFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline]
    pub fn cc3of(&self) -> CC3OFR {
        CC3OFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline]
    pub fn cc2of(&self) -> CC2OFR {
        CC2OFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline]
    pub fn cc1of(&self) -> CC1OFR {
        CC1OFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline]
    pub fn tif(&self) -> TIFR {
        TIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline]
    pub fn cc4if(&self) -> CC4IFR {
        CC4IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline]
    pub fn cc3if(&self) -> CC3IFR {
        CC3IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline]
    pub fn cc2if(&self) -> CC2IFR {
        CC2IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline]
    pub fn cc1if(&self) -> CC1IFR {
        CC1IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 12 - Capture/Compare 4 overcapture flag"]
    #[inline]
    pub fn cc4of(&mut self) -> _CC4OFW {
        _CC4OFW { w: self }
    }
    #[doc = "Bit 11 - Capture/Compare 3 overcapture flag"]
    #[inline]
    pub fn cc3of(&mut self) -> _CC3OFW {
        _CC3OFW { w: self }
    }
    #[doc = "Bit 10 - Capture/compare 2 overcapture flag"]
    #[inline]
    pub fn cc2of(&mut self) -> _CC2OFW {
        _CC2OFW { w: self }
    }
    #[doc = "Bit 9 - Capture/Compare 1 overcapture flag"]
    #[inline]
    pub fn cc1of(&mut self) -> _CC1OFW {
        _CC1OFW { w: self }
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline]
    pub fn tif(&mut self) -> _TIFW {
        _TIFW { w: self }
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt flag"]
    #[inline]
    pub fn cc4if(&mut self) -> _CC4IFW {
        _CC4IFW { w: self }
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt flag"]
    #[inline]
    pub fn cc3if(&mut self) -> _CC3IFW {
        _CC3IFW { w: self }
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt flag"]
    #[inline]
    pub fn cc2if(&mut self) -> _CC2IFW {
        _CC2IFW { w: self }
    }
    #[doc = "Bit 1 - Capture/compare 1 interrupt flag"]
    #[inline]
    pub fn cc1if(&mut self) -> _CC1IFW {
        _CC1IFW { w: self }
    }
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline]
    pub fn uif(&mut self) -> _UIFW {
        _UIFW { w: self }
    }
}
