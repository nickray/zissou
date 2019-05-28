#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
#[doc = "Possible values of the field `CKD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKDR {
    #[doc = "CK_INT not divided"]
    NOTDIVIDED,
    #[doc = "CK_INT divided by 2"]
    DIVIDEDBY2,
    #[doc = "CK_INT divided by 4"]
    DIVIDEDBY4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CKDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKDR::NOTDIVIDED => 0,
            CKDR::DIVIDEDBY2 => 1,
            CKDR::DIVIDEDBY4 => 2,
            CKDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKDR {
        match value {
            0 => CKDR::NOTDIVIDED,
            1 => CKDR::DIVIDEDBY2,
            2 => CKDR::DIVIDEDBY4,
            i => CKDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTDIVIDED`"]
    #[inline]
    pub fn is_not_divided(&self) -> bool {
        *self == CKDR::NOTDIVIDED
    }
    #[doc = "Checks if the value of the field is `DIVIDEDBY2`"]
    #[inline]
    pub fn is_divided_by2(&self) -> bool {
        *self == CKDR::DIVIDEDBY2
    }
    #[doc = "Checks if the value of the field is `DIVIDEDBY4`"]
    #[inline]
    pub fn is_divided_by4(&self) -> bool {
        *self == CKDR::DIVIDEDBY4
    }
}
#[doc = "Possible values of the field `ARPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARPER {
    #[doc = "TIMx_APRR register is not buffered"]
    DISABLED,
    #[doc = "TIMx_APRR register is buffered"]
    ENABLED,
}
impl ARPER {
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
            ARPER::DISABLED => false,
            ARPER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARPER {
        match value {
            false => ARPER::DISABLED,
            true => ARPER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ARPER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ARPER::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct CMSR {
    bits: u8,
}
impl CMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DIRR {
    bits: bool,
}
impl DIRR {
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
#[doc = "Possible values of the field `OPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPMR {
    #[doc = "Counter is not stopped at update event"]
    DISABLED,
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    ENABLED,
}
impl OPMR {
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
            OPMR::DISABLED => false,
            OPMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OPMR {
        match value {
            false => OPMR::DISABLED,
            true => OPMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OPMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OPMR::ENABLED
    }
}
#[doc = "Possible values of the field `URS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URSR {
    #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    ANYEVENT,
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    COUNTERONLY,
}
impl URSR {
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
            URSR::ANYEVENT => false,
            URSR::COUNTERONLY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> URSR {
        match value {
            false => URSR::ANYEVENT,
            true => URSR::COUNTERONLY,
        }
    }
    #[doc = "Checks if the value of the field is `ANYEVENT`"]
    #[inline]
    pub fn is_any_event(&self) -> bool {
        *self == URSR::ANYEVENT
    }
    #[doc = "Checks if the value of the field is `COUNTERONLY`"]
    #[inline]
    pub fn is_counter_only(&self) -> bool {
        *self == URSR::COUNTERONLY
    }
}
#[doc = "Possible values of the field `UDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDISR {
    #[doc = "Update event enabled"]
    ENABLED,
    #[doc = "Update event disabled"]
    DISABLED,
}
impl UDISR {
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
            UDISR::ENABLED => false,
            UDISR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UDISR {
        match value {
            false => UDISR::ENABLED,
            true => UDISR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == UDISR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == UDISR::DISABLED
    }
}
#[doc = "Possible values of the field `CEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CENR {
    #[doc = "Counter disabled"]
    DISABLED,
    #[doc = "Counter enabled"]
    ENABLED,
}
impl CENR {
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
            CENR::DISABLED => false,
            CENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CENR {
        match value {
            false => CENR::DISABLED,
            true => CENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `CKD`"]
pub enum CKDW {
    #[doc = "CK_INT not divided"]
    NOTDIVIDED,
    #[doc = "CK_INT divided by 2"]
    DIVIDEDBY2,
    #[doc = "CK_INT divided by 4"]
    DIVIDEDBY4,
}
impl CKDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKDW::NOTDIVIDED => 0,
            CKDW::DIVIDEDBY2 => 1,
            CKDW::DIVIDEDBY4 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKDW<'a> {
    w: &'a mut W,
}
impl<'a> _CKDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CK_INT not divided"]
    #[inline]
    pub fn not_divided(self) -> &'a mut W {
        self.variant(CKDW::NOTDIVIDED)
    }
    #[doc = "CK_INT divided by 2"]
    #[inline]
    pub fn divided_by2(self) -> &'a mut W {
        self.variant(CKDW::DIVIDEDBY2)
    }
    #[doc = "CK_INT divided by 4"]
    #[inline]
    pub fn divided_by4(self) -> &'a mut W {
        self.variant(CKDW::DIVIDEDBY4)
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
#[doc = "Values that can be written to the field `ARPE`"]
pub enum ARPEW {
    #[doc = "TIMx_APRR register is not buffered"]
    DISABLED,
    #[doc = "TIMx_APRR register is buffered"]
    ENABLED,
}
impl ARPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARPEW::DISABLED => false,
            ARPEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARPEW<'a> {
    w: &'a mut W,
}
impl<'a> _ARPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TIMx_APRR register is not buffered"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARPEW::DISABLED)
    }
    #[doc = "TIMx_APRR register is buffered"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARPEW::ENABLED)
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
#[doc = r" Proxy"]
pub struct _CMSW<'a> {
    w: &'a mut W,
}
impl<'a> _CMSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
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
#[doc = "Values that can be written to the field `OPM`"]
pub enum OPMW {
    #[doc = "Counter is not stopped at update event"]
    DISABLED,
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    ENABLED,
}
impl OPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OPMW::DISABLED => false,
            OPMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPMW<'a> {
    w: &'a mut W,
}
impl<'a> _OPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter is not stopped at update event"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPMW::DISABLED)
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPMW::ENABLED)
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
#[doc = "Values that can be written to the field `URS`"]
pub enum URSW {
    #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    ANYEVENT,
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    COUNTERONLY,
}
impl URSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            URSW::ANYEVENT => false,
            URSW::COUNTERONLY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _URSW<'a> {
    w: &'a mut W,
}
impl<'a> _URSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: URSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    #[inline]
    pub fn any_event(self) -> &'a mut W {
        self.variant(URSW::ANYEVENT)
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    #[inline]
    pub fn counter_only(self) -> &'a mut W {
        self.variant(URSW::COUNTERONLY)
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
#[doc = "Values that can be written to the field `UDIS`"]
pub enum UDISW {
    #[doc = "Update event enabled"]
    ENABLED,
    #[doc = "Update event disabled"]
    DISABLED,
}
impl UDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UDISW::ENABLED => false,
            UDISW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UDISW<'a> {
    w: &'a mut W,
}
impl<'a> _UDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Update event enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDISW::ENABLED)
    }
    #[doc = "Update event disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDISW::DISABLED)
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
#[doc = "Values that can be written to the field `CEN`"]
pub enum CENW {
    #[doc = "Counter disabled"]
    DISABLED,
    #[doc = "Counter enabled"]
    ENABLED,
}
impl CENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CENW::DISABLED => false,
            CENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CENW<'a> {
    w: &'a mut W,
}
impl<'a> _CENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Counter disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CENW::DISABLED)
    }
    #[doc = "Counter enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CENW::ENABLED)
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
    #[doc = "Bits 8:9 - Clock division"]
    #[inline]
    pub fn ckd(&self) -> CKDR {
        CKDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline]
    pub fn arpe(&self) -> ARPER {
        ARPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline]
    pub fn cms(&self) -> CMSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMSR { bits }
    }
    #[doc = "Bit 4 - Direction"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DIRR { bits }
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline]
    pub fn opm(&self) -> OPMR {
        OPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline]
    pub fn urs(&self) -> URSR {
        URSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline]
    pub fn udis(&self) -> UDISR {
        UDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline]
    pub fn cen(&self) -> CENR {
        CENR::_from({
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
    #[doc = "Bits 8:9 - Clock division"]
    #[inline]
    pub fn ckd(&mut self) -> _CKDW {
        _CKDW { w: self }
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline]
    pub fn arpe(&mut self) -> _ARPEW {
        _ARPEW { w: self }
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline]
    pub fn cms(&mut self) -> _CMSW {
        _CMSW { w: self }
    }
    #[doc = "Bit 4 - Direction"]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline]
    pub fn opm(&mut self) -> _OPMW {
        _OPMW { w: self }
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline]
    pub fn urs(&mut self) -> _URSW {
        _URSW { w: self }
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline]
    pub fn udis(&mut self) -> _UDISW {
        _UDISW { w: self }
    }
    #[doc = "Bit 0 - Counter enable"]
    #[inline]
    pub fn cen(&mut self) -> _CENW {
        _CENW { w: self }
    }
}
