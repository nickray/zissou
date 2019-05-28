#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR4 {
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
#[doc = "Possible values of the field `MEM2MEM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEM2MEMR {
    #[doc = "Memory to memory mode disabled"]
    DISABLED,
    #[doc = "Memory to memory mode enabled"]
    ENABLED,
}
impl MEM2MEMR {
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
            MEM2MEMR::DISABLED => false,
            MEM2MEMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEM2MEMR {
        match value {
            false => MEM2MEMR::DISABLED,
            true => MEM2MEMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MEM2MEMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MEM2MEMR::ENABLED
    }
}
#[doc = "Possible values of the field `PL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLR {
    #[doc = "Low priority"]
    LOW,
    #[doc = "Medium priority"]
    MEDIUM,
    #[doc = "High priority"]
    HIGH,
    #[doc = "Very high priority"]
    VERYHIGH,
}
impl PLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLR::LOW => 0,
            PLR::MEDIUM => 1,
            PLR::HIGH => 2,
            PLR::VERYHIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLR {
        match value {
            0 => PLR::LOW,
            1 => PLR::MEDIUM,
            2 => PLR::HIGH,
            3 => PLR::VERYHIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PLR::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline]
    pub fn is_medium(&self) -> bool {
        *self == PLR::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PLR::HIGH
    }
    #[doc = "Checks if the value of the field is `VERYHIGH`"]
    #[inline]
    pub fn is_very_high(&self) -> bool {
        *self == PLR::VERYHIGH
    }
}
#[doc = "Possible values of the field `MSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSIZER {
    #[doc = "8-bit size"]
    BIT8,
    #[doc = "16-bit size"]
    BIT16,
    #[doc = "32-bit size"]
    BIT32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSIZER::BIT8 => 0,
            MSIZER::BIT16 => 1,
            MSIZER::BIT32 => 2,
            MSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSIZER {
        match value {
            0 => MSIZER::BIT8,
            1 => MSIZER::BIT16,
            2 => MSIZER::BIT32,
            i => MSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BIT8`"]
    #[inline]
    pub fn is_bit8(&self) -> bool {
        *self == MSIZER::BIT8
    }
    #[doc = "Checks if the value of the field is `BIT16`"]
    #[inline]
    pub fn is_bit16(&self) -> bool {
        *self == MSIZER::BIT16
    }
    #[doc = "Checks if the value of the field is `BIT32`"]
    #[inline]
    pub fn is_bit32(&self) -> bool {
        *self == MSIZER::BIT32
    }
}
#[doc = "Possible values of the field `PSIZE`"]
pub type PSIZER = MSIZER;
#[doc = "Possible values of the field `MINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MINCR {
    #[doc = "Increment mode disabled"]
    DISABLED,
    #[doc = "Increment mode enabled"]
    ENABLED,
}
impl MINCR {
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
            MINCR::DISABLED => false,
            MINCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MINCR {
        match value {
            false => MINCR::DISABLED,
            true => MINCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == MINCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == MINCR::ENABLED
    }
}
#[doc = "Possible values of the field `PINC`"]
pub type PINCR = MINCR;
#[doc = "Possible values of the field `CIRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIRCR {
    #[doc = "Circular buffer disabled"]
    DISABLED,
    #[doc = "Circular buffer enabled"]
    ENABLED,
}
impl CIRCR {
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
            CIRCR::DISABLED => false,
            CIRCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIRCR {
        match value {
            false => CIRCR::DISABLED,
            true => CIRCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CIRCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CIRCR::ENABLED
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Read from peripheral"]
    FROMPERIPHERAL,
    #[doc = "Read from memory"]
    FROMMEMORY,
}
impl DIRR {
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
            DIRR::FROMPERIPHERAL => false,
            DIRR::FROMMEMORY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRR {
        match value {
            false => DIRR::FROMPERIPHERAL,
            true => DIRR::FROMMEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `FROMPERIPHERAL`"]
    #[inline]
    pub fn is_from_peripheral(&self) -> bool {
        *self == DIRR::FROMPERIPHERAL
    }
    #[doc = "Checks if the value of the field is `FROMMEMORY`"]
    #[inline]
    pub fn is_from_memory(&self) -> bool {
        *self == DIRR::FROMMEMORY
    }
}
#[doc = "Possible values of the field `TEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIER {
    #[doc = "Transfer Error interrupt disabled"]
    DISABLED,
    #[doc = "Transfer Error interrupt enabled"]
    ENABLED,
}
impl TEIER {
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
            TEIER::DISABLED => false,
            TEIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEIER {
        match value {
            false => TEIER::DISABLED,
            true => TEIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TEIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TEIER::ENABLED
    }
}
#[doc = "Possible values of the field `HTIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIER {
    #[doc = "Half Transfer interrupt disabled"]
    DISABLED,
    #[doc = "Half Transfer interrupt enabled"]
    ENABLED,
}
impl HTIER {
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
            HTIER::DISABLED => false,
            HTIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HTIER {
        match value {
            false => HTIER::DISABLED,
            true => HTIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HTIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HTIER::ENABLED
    }
}
#[doc = "Possible values of the field `TCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIER {
    #[doc = "Transfer Complete interrupt disabled"]
    DISABLED,
    #[doc = "Transfer Complete interrupt enabled"]
    ENABLED,
}
impl TCIER {
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
            TCIER::DISABLED => false,
            TCIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIER {
        match value {
            false => TCIER::DISABLED,
            true => TCIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TCIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TCIER::ENABLED
    }
}
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "Channel disabled"]
    DISABLED,
    #[doc = "Channel enabled"]
    ENABLED,
}
impl ENR {
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
            ENR::DISABLED => false,
            ENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::DISABLED,
            true => ENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `MEM2MEM`"]
pub enum MEM2MEMW {
    #[doc = "Memory to memory mode disabled"]
    DISABLED,
    #[doc = "Memory to memory mode enabled"]
    ENABLED,
}
impl MEM2MEMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEM2MEMW::DISABLED => false,
            MEM2MEMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEM2MEMW<'a> {
    w: &'a mut W,
}
impl<'a> _MEM2MEMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEM2MEMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Memory to memory mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MEM2MEMW::DISABLED)
    }
    #[doc = "Memory to memory mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MEM2MEMW::ENABLED)
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
#[doc = "Values that can be written to the field `PL`"]
pub enum PLW {
    #[doc = "Low priority"]
    LOW,
    #[doc = "Medium priority"]
    MEDIUM,
    #[doc = "High priority"]
    HIGH,
    #[doc = "Very high priority"]
    VERYHIGH,
}
impl PLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLW::LOW => 0,
            PLW::MEDIUM => 1,
            PLW::HIGH => 2,
            PLW::VERYHIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLW<'a> {
    w: &'a mut W,
}
impl<'a> _PLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low priority"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PLW::LOW)
    }
    #[doc = "Medium priority"]
    #[inline]
    pub fn medium(self) -> &'a mut W {
        self.variant(PLW::MEDIUM)
    }
    #[doc = "High priority"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PLW::HIGH)
    }
    #[doc = "Very high priority"]
    #[inline]
    pub fn very_high(self) -> &'a mut W {
        self.variant(PLW::VERYHIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MSIZE`"]
pub enum MSIZEW {
    #[doc = "8-bit size"]
    BIT8,
    #[doc = "16-bit size"]
    BIT16,
    #[doc = "32-bit size"]
    BIT32,
}
impl MSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSIZEW::BIT8 => 0,
            MSIZEW::BIT16 => 1,
            MSIZEW::BIT32 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bit size"]
    #[inline]
    pub fn bit8(self) -> &'a mut W {
        self.variant(MSIZEW::BIT8)
    }
    #[doc = "16-bit size"]
    #[inline]
    pub fn bit16(self) -> &'a mut W {
        self.variant(MSIZEW::BIT16)
    }
    #[doc = "32-bit size"]
    #[inline]
    pub fn bit32(self) -> &'a mut W {
        self.variant(MSIZEW::BIT32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSIZE`"]
pub type PSIZEW = MSIZEW;
#[doc = r" Proxy"]
pub struct _PSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bit size"]
    #[inline]
    pub fn bit8(self) -> &'a mut W {
        self.variant(MSIZEW::BIT8)
    }
    #[doc = "16-bit size"]
    #[inline]
    pub fn bit16(self) -> &'a mut W {
        self.variant(MSIZEW::BIT16)
    }
    #[doc = "32-bit size"]
    #[inline]
    pub fn bit32(self) -> &'a mut W {
        self.variant(MSIZEW::BIT32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MINC`"]
pub enum MINCW {
    #[doc = "Increment mode disabled"]
    DISABLED,
    #[doc = "Increment mode enabled"]
    ENABLED,
}
impl MINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MINCW::DISABLED => false,
            MINCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MINCW<'a> {
    w: &'a mut W,
}
impl<'a> _MINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Increment mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MINCW::DISABLED)
    }
    #[doc = "Increment mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MINCW::ENABLED)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PINC`"]
pub type PINCW = MINCW;
#[doc = r" Proxy"]
pub struct _PINCW<'a> {
    w: &'a mut W,
}
impl<'a> _PINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Increment mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MINCW::DISABLED)
    }
    #[doc = "Increment mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MINCW::ENABLED)
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
#[doc = "Values that can be written to the field `CIRC`"]
pub enum CIRCW {
    #[doc = "Circular buffer disabled"]
    DISABLED,
    #[doc = "Circular buffer enabled"]
    ENABLED,
}
impl CIRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIRCW::DISABLED => false,
            CIRCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CIRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Circular buffer disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CIRCW::DISABLED)
    }
    #[doc = "Circular buffer enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CIRCW::ENABLED)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIR`"]
pub enum DIRW {
    #[doc = "Read from peripheral"]
    FROMPERIPHERAL,
    #[doc = "Read from memory"]
    FROMMEMORY,
}
impl DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRW::FROMPERIPHERAL => false,
            DIRW::FROMMEMORY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read from peripheral"]
    #[inline]
    pub fn from_peripheral(self) -> &'a mut W {
        self.variant(DIRW::FROMPERIPHERAL)
    }
    #[doc = "Read from memory"]
    #[inline]
    pub fn from_memory(self) -> &'a mut W {
        self.variant(DIRW::FROMMEMORY)
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
#[doc = "Values that can be written to the field `TEIE`"]
pub enum TEIEW {
    #[doc = "Transfer Error interrupt disabled"]
    DISABLED,
    #[doc = "Transfer Error interrupt enabled"]
    ENABLED,
}
impl TEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEIEW::DISABLED => false,
            TEIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer Error interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEIEW::DISABLED)
    }
    #[doc = "Transfer Error interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEIEW::ENABLED)
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
#[doc = "Values that can be written to the field `HTIE`"]
pub enum HTIEW {
    #[doc = "Half Transfer interrupt disabled"]
    DISABLED,
    #[doc = "Half Transfer interrupt enabled"]
    ENABLED,
}
impl HTIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HTIEW::DISABLED => false,
            HTIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HTIEW<'a> {
    w: &'a mut W,
}
impl<'a> _HTIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HTIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Half Transfer interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HTIEW::DISABLED)
    }
    #[doc = "Half Transfer interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HTIEW::ENABLED)
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
#[doc = "Values that can be written to the field `TCIE`"]
pub enum TCIEW {
    #[doc = "Transfer Complete interrupt disabled"]
    DISABLED,
    #[doc = "Transfer Complete interrupt enabled"]
    ENABLED,
}
impl TCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIEW::DISABLED => false,
            TCIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer Complete interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIEW::DISABLED)
    }
    #[doc = "Transfer Complete interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIEW::ENABLED)
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
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "Channel disabled"]
    DISABLED,
    #[doc = "Channel enabled"]
    ENABLED,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::DISABLED => false,
            ENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENW::DISABLED)
    }
    #[doc = "Channel enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENW::ENABLED)
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
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline]
    pub fn mem2mem(&self) -> MEM2MEMR {
        MEM2MEMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - Channel priority level"]
    #[inline]
    pub fn pl(&self) -> PLR {
        PLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Memory size"]
    #[inline]
    pub fn msize(&self) -> MSIZER {
        MSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Peripheral size"]
    #[inline]
    pub fn psize(&self) -> PSIZER {
        PSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline]
    pub fn minc(&self) -> MINCR {
        MINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline]
    pub fn pinc(&self) -> PINCR {
        PINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Circular mode"]
    #[inline]
    pub fn circ(&self) -> CIRCR {
        CIRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline]
    pub fn teie(&self) -> TEIER {
        TEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Half transfer interrupt enable"]
    #[inline]
    pub fn htie(&self) -> HTIER {
        HTIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline]
    pub fn tcie(&self) -> TCIER {
        TCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Channel enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
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
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline]
    pub fn mem2mem(&mut self) -> _MEM2MEMW {
        _MEM2MEMW { w: self }
    }
    #[doc = "Bits 12:13 - Channel priority level"]
    #[inline]
    pub fn pl(&mut self) -> _PLW {
        _PLW { w: self }
    }
    #[doc = "Bits 10:11 - Memory size"]
    #[inline]
    pub fn msize(&mut self) -> _MSIZEW {
        _MSIZEW { w: self }
    }
    #[doc = "Bits 8:9 - Peripheral size"]
    #[inline]
    pub fn psize(&mut self) -> _PSIZEW {
        _PSIZEW { w: self }
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline]
    pub fn minc(&mut self) -> _MINCW {
        _MINCW { w: self }
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline]
    pub fn pinc(&mut self) -> _PINCW {
        _PINCW { w: self }
    }
    #[doc = "Bit 5 - Circular mode"]
    #[inline]
    pub fn circ(&mut self) -> _CIRCW {
        _CIRCW { w: self }
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline]
    pub fn teie(&mut self) -> _TEIEW {
        _TEIEW { w: self }
    }
    #[doc = "Bit 2 - Half transfer interrupt enable"]
    #[inline]
    pub fn htie(&mut self) -> _HTIEW {
        _HTIEW { w: self }
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline]
    pub fn tcie(&mut self) -> _TCIEW {
        _TCIEW { w: self }
    }
    #[doc = "Bit 0 - Channel enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
}
