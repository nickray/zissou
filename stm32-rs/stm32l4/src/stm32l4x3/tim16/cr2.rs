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
#[doc = "Possible values of the field `OIS1N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS1NR {
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    LOW,
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    HIGH,
}
impl OIS1NR {
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
            OIS1NR::LOW => false,
            OIS1NR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OIS1NR {
        match value {
            false => OIS1NR::LOW,
            true => OIS1NR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == OIS1NR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == OIS1NR::HIGH
    }
}
#[doc = "Possible values of the field `OIS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OIS1R {
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    LOW,
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    HIGH,
}
impl OIS1R {
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
            OIS1R::LOW => false,
            OIS1R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OIS1R {
        match value {
            false => OIS1R::LOW,
            true => OIS1R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == OIS1R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == OIS1R::HIGH
    }
}
#[doc = "Possible values of the field `CCDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCDSR {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    ONCOMPARE,
    #[doc = "CCx DMA request sent when update event occurs"]
    ONUPDATE,
}
impl CCDSR {
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
            CCDSR::ONCOMPARE => false,
            CCDSR::ONUPDATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCDSR {
        match value {
            false => CCDSR::ONCOMPARE,
            true => CCDSR::ONUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `ONCOMPARE`"]
    #[inline]
    pub fn is_on_compare(&self) -> bool {
        *self == CCDSR::ONCOMPARE
    }
    #[doc = "Checks if the value of the field is `ONUPDATE`"]
    #[inline]
    pub fn is_on_update(&self) -> bool {
        *self == CCDSR::ONUPDATE
    }
}
#[doc = "Possible values of the field `CCUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCUSR {
    #[doc = "Capture/compare are updated only by setting the COMG bit"]
    DEFAULT,
    #[doc = "Capture/compare are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    WITHRISINGEDGE,
}
impl CCUSR {
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
            CCUSR::DEFAULT => false,
            CCUSR::WITHRISINGEDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCUSR {
        match value {
            false => CCUSR::DEFAULT,
            true => CCUSR::WITHRISINGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == CCUSR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `WITHRISINGEDGE`"]
    #[inline]
    pub fn is_with_rising_edge(&self) -> bool {
        *self == CCUSR::WITHRISINGEDGE
    }
}
#[doc = "Possible values of the field `CCPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPCR {
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    NOTPRELOADED,
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded"]
    PRELOADED,
}
impl CCPCR {
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
            CCPCR::NOTPRELOADED => false,
            CCPCR::PRELOADED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCPCR {
        match value {
            false => CCPCR::NOTPRELOADED,
            true => CCPCR::PRELOADED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRELOADED`"]
    #[inline]
    pub fn is_not_preloaded(&self) -> bool {
        *self == CCPCR::NOTPRELOADED
    }
    #[doc = "Checks if the value of the field is `PRELOADED`"]
    #[inline]
    pub fn is_preloaded(&self) -> bool {
        *self == CCPCR::PRELOADED
    }
}
#[doc = "Values that can be written to the field `OIS1N`"]
pub enum OIS1NW {
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    LOW,
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    HIGH,
}
impl OIS1NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OIS1NW::LOW => false,
            OIS1NW::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OIS1NW<'a> {
    w: &'a mut W,
}
impl<'a> _OIS1NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OIS1NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC1N=0 after a dead-time when MOE=0"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(OIS1NW::LOW)
    }
    #[doc = "OC1N=1 after a dead-time when MOE=0"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(OIS1NW::HIGH)
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
#[doc = "Values that can be written to the field `OIS1`"]
pub enum OIS1W {
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    LOW,
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    HIGH,
}
impl OIS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OIS1W::LOW => false,
            OIS1W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OIS1W<'a> {
    w: &'a mut W,
}
impl<'a> _OIS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OIS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OC1=0 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(OIS1W::LOW)
    }
    #[doc = "OC1=1 (after a dead-time if OC1N is implemented) when MOE=0"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(OIS1W::HIGH)
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
#[doc = "Values that can be written to the field `CCDS`"]
pub enum CCDSW {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    ONCOMPARE,
    #[doc = "CCx DMA request sent when update event occurs"]
    ONUPDATE,
}
impl CCDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCDSW::ONCOMPARE => false,
            CCDSW::ONUPDATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCDSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCDSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline]
    pub fn on_compare(self) -> &'a mut W {
        self.variant(CCDSW::ONCOMPARE)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline]
    pub fn on_update(self) -> &'a mut W {
        self.variant(CCDSW::ONUPDATE)
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
#[doc = "Values that can be written to the field `CCUS`"]
pub enum CCUSW {
    #[doc = "Capture/compare are updated only by setting the COMG bit"]
    DEFAULT,
    #[doc = "Capture/compare are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    WITHRISINGEDGE,
}
impl CCUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCUSW::DEFAULT => false,
            CCUSW::WITHRISINGEDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCUSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Capture/compare are updated only by setting the COMG bit"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(CCUSW::DEFAULT)
    }
    #[doc = "Capture/compare are updated by setting the COMG bit or when an rising edge occurs on TRGI"]
    #[inline]
    pub fn with_rising_edge(self) -> &'a mut W {
        self.variant(CCUSW::WITHRISINGEDGE)
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
#[doc = "Values that can be written to the field `CCPC`"]
pub enum CCPCW {
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    NOTPRELOADED,
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded"]
    PRELOADED,
}
impl CCPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCPCW::NOTPRELOADED => false,
            CCPCW::PRELOADED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCPCW<'a> {
    w: &'a mut W,
}
impl<'a> _CCPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCPCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCxE, CCxNE and OCxM bits are not preloaded"]
    #[inline]
    pub fn not_preloaded(self) -> &'a mut W {
        self.variant(CCPCW::NOTPRELOADED)
    }
    #[doc = "CCxE, CCxNE and OCxM bits are preloaded"]
    #[inline]
    pub fn preloaded(self) -> &'a mut W {
        self.variant(CCPCW::PRELOADED)
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
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline]
    pub fn ois1n(&self) -> OIS1NR {
        OIS1NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline]
    pub fn ois1(&self) -> OIS1R {
        OIS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline]
    pub fn ccds(&self) -> CCDSR {
        CCDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline]
    pub fn ccus(&self) -> CCUSR {
        CCUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline]
    pub fn ccpc(&self) -> CCPCR {
        CCPCR::_from({
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
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline]
    pub fn ois1n(&mut self) -> _OIS1NW {
        _OIS1NW { w: self }
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline]
    pub fn ois1(&mut self) -> _OIS1W {
        _OIS1W { w: self }
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline]
    pub fn ccds(&mut self) -> _CCDSW {
        _CCDSW { w: self }
    }
    #[doc = "Bit 2 - Capture/compare control update selection"]
    #[inline]
    pub fn ccus(&mut self) -> _CCUSW {
        _CCUSW { w: self }
    }
    #[doc = "Bit 0 - Capture/compare preloaded control"]
    #[inline]
    pub fn ccpc(&mut self) -> _CCPCW {
        _CCPCW { w: self }
    }
}
