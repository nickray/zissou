#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
pub struct PLLSAI2RDYR {
    bits: bool,
}
impl PLLSAI2RDYR {
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
#[doc = r" Value of the field"]
pub struct PLLSAI2ONR {
    bits: bool,
}
impl PLLSAI2ONR {
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
#[doc = r" Value of the field"]
pub struct PLLSAI1RDYR {
    bits: bool,
}
impl PLLSAI1RDYR {
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
#[doc = r" Value of the field"]
pub struct PLLSAI1ONR {
    bits: bool,
}
impl PLLSAI1ONR {
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
#[doc = r" Value of the field"]
pub struct PLLRDYR {
    bits: bool,
}
impl PLLRDYR {
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
#[doc = r" Value of the field"]
pub struct PLLONR {
    bits: bool,
}
impl PLLONR {
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
#[doc = r" Value of the field"]
pub struct HSEBYPR {
    bits: bool,
}
impl HSEBYPR {
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
#[doc = r" Value of the field"]
pub struct HSERDYR {
    bits: bool,
}
impl HSERDYR {
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
#[doc = r" Value of the field"]
pub struct HSEONR {
    bits: bool,
}
impl HSEONR {
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
#[doc = r" Value of the field"]
pub struct HSIASFSR {
    bits: bool,
}
impl HSIASFSR {
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
#[doc = r" Value of the field"]
pub struct HSIRDYR {
    bits: bool,
}
impl HSIRDYR {
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
#[doc = r" Value of the field"]
pub struct HSIKERONR {
    bits: bool,
}
impl HSIKERONR {
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
#[doc = r" Value of the field"]
pub struct HSIONR {
    bits: bool,
}
impl HSIONR {
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
#[doc = "Possible values of the field `MSIRANGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSIRANGER {
    #[doc = "range 0 around 100 kHz"]
    RANGE100K,
    #[doc = "range 1 around 200 kHz"]
    RANGE200K,
    #[doc = "range 2 around 400 kHz"]
    RANGE400K,
    #[doc = "range 3 around 800 kHz"]
    RANGE800K,
    #[doc = "range 4 around 1 MHz"]
    RANGE1M,
    #[doc = "range 5 around 2 MHz"]
    RANGE2M,
    #[doc = "range 6 around 4 MHz"]
    RANGE4M,
    #[doc = "range 7 around 8 MHz"]
    RANGE8M,
    #[doc = "range 8 around 16 MHz"]
    RANGE16M,
    #[doc = "range 9 around 24 MHz"]
    RANGE24M,
    #[doc = "range 10 around 32 MHz"]
    RANGE32M,
    #[doc = "range 11 around 48 MHz"]
    RANGE48M,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MSIRANGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSIRANGER::RANGE100K => 0,
            MSIRANGER::RANGE200K => 1,
            MSIRANGER::RANGE400K => 2,
            MSIRANGER::RANGE800K => 3,
            MSIRANGER::RANGE1M => 4,
            MSIRANGER::RANGE2M => 5,
            MSIRANGER::RANGE4M => 6,
            MSIRANGER::RANGE8M => 7,
            MSIRANGER::RANGE16M => 8,
            MSIRANGER::RANGE24M => 9,
            MSIRANGER::RANGE32M => 10,
            MSIRANGER::RANGE48M => 11,
            MSIRANGER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSIRANGER {
        match value {
            0 => MSIRANGER::RANGE100K,
            1 => MSIRANGER::RANGE200K,
            2 => MSIRANGER::RANGE400K,
            3 => MSIRANGER::RANGE800K,
            4 => MSIRANGER::RANGE1M,
            5 => MSIRANGER::RANGE2M,
            6 => MSIRANGER::RANGE4M,
            7 => MSIRANGER::RANGE8M,
            8 => MSIRANGER::RANGE16M,
            9 => MSIRANGER::RANGE24M,
            10 => MSIRANGER::RANGE32M,
            11 => MSIRANGER::RANGE48M,
            i => MSIRANGER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RANGE100K`"]
    #[inline]
    pub fn is_range100k(&self) -> bool {
        *self == MSIRANGER::RANGE100K
    }
    #[doc = "Checks if the value of the field is `RANGE200K`"]
    #[inline]
    pub fn is_range200k(&self) -> bool {
        *self == MSIRANGER::RANGE200K
    }
    #[doc = "Checks if the value of the field is `RANGE400K`"]
    #[inline]
    pub fn is_range400k(&self) -> bool {
        *self == MSIRANGER::RANGE400K
    }
    #[doc = "Checks if the value of the field is `RANGE800K`"]
    #[inline]
    pub fn is_range800k(&self) -> bool {
        *self == MSIRANGER::RANGE800K
    }
    #[doc = "Checks if the value of the field is `RANGE1M`"]
    #[inline]
    pub fn is_range1m(&self) -> bool {
        *self == MSIRANGER::RANGE1M
    }
    #[doc = "Checks if the value of the field is `RANGE2M`"]
    #[inline]
    pub fn is_range2m(&self) -> bool {
        *self == MSIRANGER::RANGE2M
    }
    #[doc = "Checks if the value of the field is `RANGE4M`"]
    #[inline]
    pub fn is_range4m(&self) -> bool {
        *self == MSIRANGER::RANGE4M
    }
    #[doc = "Checks if the value of the field is `RANGE8M`"]
    #[inline]
    pub fn is_range8m(&self) -> bool {
        *self == MSIRANGER::RANGE8M
    }
    #[doc = "Checks if the value of the field is `RANGE16M`"]
    #[inline]
    pub fn is_range16m(&self) -> bool {
        *self == MSIRANGER::RANGE16M
    }
    #[doc = "Checks if the value of the field is `RANGE24M`"]
    #[inline]
    pub fn is_range24m(&self) -> bool {
        *self == MSIRANGER::RANGE24M
    }
    #[doc = "Checks if the value of the field is `RANGE32M`"]
    #[inline]
    pub fn is_range32m(&self) -> bool {
        *self == MSIRANGER::RANGE32M
    }
    #[doc = "Checks if the value of the field is `RANGE48M`"]
    #[inline]
    pub fn is_range48m(&self) -> bool {
        *self == MSIRANGER::RANGE48M
    }
}
#[doc = r" Value of the field"]
pub struct MSIPLLENR {
    bits: bool,
}
impl MSIPLLENR {
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
#[doc = r" Value of the field"]
pub struct MSIRDYR {
    bits: bool,
}
impl MSIRDYR {
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
#[doc = r" Value of the field"]
pub struct MSIONR {
    bits: bool,
}
impl MSIONR {
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
#[doc = r" Proxy"]
pub struct _PLLSAI2ONW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSAI2ONW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLLSAI1ONW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLSAI1ONW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLLONW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLONW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSSONW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSONW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSEBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _HSEBYPW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSEONW<'a> {
    w: &'a mut W,
}
impl<'a> _HSEONW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HSIASFSW<'a> {
    w: &'a mut W,
}
impl<'a> _HSIASFSW<'a> {
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
#[doc = r" Proxy"]
pub struct _HSIKERONW<'a> {
    w: &'a mut W,
}
impl<'a> _HSIKERONW<'a> {
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
#[doc = r" Proxy"]
pub struct _HSIONW<'a> {
    w: &'a mut W,
}
impl<'a> _HSIONW<'a> {
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
#[doc = "Values that can be written to the field `MSIRANGE`"]
pub enum MSIRANGEW {
    #[doc = "range 0 around 100 kHz"]
    RANGE100K,
    #[doc = "range 1 around 200 kHz"]
    RANGE200K,
    #[doc = "range 2 around 400 kHz"]
    RANGE400K,
    #[doc = "range 3 around 800 kHz"]
    RANGE800K,
    #[doc = "range 4 around 1 MHz"]
    RANGE1M,
    #[doc = "range 5 around 2 MHz"]
    RANGE2M,
    #[doc = "range 6 around 4 MHz"]
    RANGE4M,
    #[doc = "range 7 around 8 MHz"]
    RANGE8M,
    #[doc = "range 8 around 16 MHz"]
    RANGE16M,
    #[doc = "range 9 around 24 MHz"]
    RANGE24M,
    #[doc = "range 10 around 32 MHz"]
    RANGE32M,
    #[doc = "range 11 around 48 MHz"]
    RANGE48M,
}
impl MSIRANGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSIRANGEW::RANGE100K => 0,
            MSIRANGEW::RANGE200K => 1,
            MSIRANGEW::RANGE400K => 2,
            MSIRANGEW::RANGE800K => 3,
            MSIRANGEW::RANGE1M => 4,
            MSIRANGEW::RANGE2M => 5,
            MSIRANGEW::RANGE4M => 6,
            MSIRANGEW::RANGE8M => 7,
            MSIRANGEW::RANGE16M => 8,
            MSIRANGEW::RANGE24M => 9,
            MSIRANGEW::RANGE32M => 10,
            MSIRANGEW::RANGE48M => 11,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSIRANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _MSIRANGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSIRANGEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "range 0 around 100 kHz"]
    #[inline]
    pub fn range100k(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE100K)
    }
    #[doc = "range 1 around 200 kHz"]
    #[inline]
    pub fn range200k(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE200K)
    }
    #[doc = "range 2 around 400 kHz"]
    #[inline]
    pub fn range400k(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE400K)
    }
    #[doc = "range 3 around 800 kHz"]
    #[inline]
    pub fn range800k(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE800K)
    }
    #[doc = "range 4 around 1 MHz"]
    #[inline]
    pub fn range1m(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE1M)
    }
    #[doc = "range 5 around 2 MHz"]
    #[inline]
    pub fn range2m(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE2M)
    }
    #[doc = "range 6 around 4 MHz"]
    #[inline]
    pub fn range4m(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE4M)
    }
    #[doc = "range 7 around 8 MHz"]
    #[inline]
    pub fn range8m(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE8M)
    }
    #[doc = "range 8 around 16 MHz"]
    #[inline]
    pub fn range16m(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE16M)
    }
    #[doc = "range 9 around 24 MHz"]
    #[inline]
    pub fn range24m(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE24M)
    }
    #[doc = "range 10 around 32 MHz"]
    #[inline]
    pub fn range32m(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE32M)
    }
    #[doc = "range 11 around 48 MHz"]
    #[inline]
    pub fn range48m(self) -> &'a mut W {
        self.variant(MSIRANGEW::RANGE48M)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSIRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSIRGSELW<'a> {
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
#[doc = r" Proxy"]
pub struct _MSIPLLENW<'a> {
    w: &'a mut W,
}
impl<'a> _MSIPLLENW<'a> {
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
#[doc = r" Proxy"]
pub struct _MSIONW<'a> {
    w: &'a mut W,
}
impl<'a> _MSIONW<'a> {
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
    #[doc = "Bit 29 - SAI2 PLL clock ready flag"]
    #[inline]
    pub fn pllsai2rdy(&self) -> PLLSAI2RDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLSAI2RDYR { bits }
    }
    #[doc = "Bit 28 - SAI2 PLL enable"]
    #[inline]
    pub fn pllsai2on(&self) -> PLLSAI2ONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLSAI2ONR { bits }
    }
    #[doc = "Bit 27 - SAI1 PLL clock ready flag"]
    #[inline]
    pub fn pllsai1rdy(&self) -> PLLSAI1RDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLSAI1RDYR { bits }
    }
    #[doc = "Bit 26 - SAI1 PLL enable"]
    #[inline]
    pub fn pllsai1on(&self) -> PLLSAI1ONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLSAI1ONR { bits }
    }
    #[doc = "Bit 25 - Main PLL clock ready flag"]
    #[inline]
    pub fn pllrdy(&self) -> PLLRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLRDYR { bits }
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline]
    pub fn pllon(&self) -> PLLONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PLLONR { bits }
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline]
    pub fn hsebyp(&self) -> HSEBYPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSEBYPR { bits }
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline]
    pub fn hserdy(&self) -> HSERDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSERDYR { bits }
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline]
    pub fn hseon(&self) -> HSEONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSEONR { bits }
    }
    #[doc = "Bit 11 - HSI automatic start from Stop"]
    #[inline]
    pub fn hsiasfs(&self) -> HSIASFSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSIASFSR { bits }
    }
    #[doc = "Bit 10 - HSI clock ready flag"]
    #[inline]
    pub fn hsirdy(&self) -> HSIRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSIRDYR { bits }
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline]
    pub fn hsikeron(&self) -> HSIKERONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSIKERONR { bits }
    }
    #[doc = "Bit 8 - HSI clock enable"]
    #[inline]
    pub fn hsion(&self) -> HSIONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSIONR { bits }
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline]
    pub fn msirange(&self) -> MSIRANGER {
        MSIRANGER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline]
    pub fn msipllen(&self) -> MSIPLLENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSIPLLENR { bits }
    }
    #[doc = "Bit 1 - MSI clock ready flag"]
    #[inline]
    pub fn msirdy(&self) -> MSIRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSIRDYR { bits }
    }
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline]
    pub fn msion(&self) -> MSIONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSIONR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 99 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 28 - SAI2 PLL enable"]
    #[inline]
    pub fn pllsai2on(&mut self) -> _PLLSAI2ONW {
        _PLLSAI2ONW { w: self }
    }
    #[doc = "Bit 26 - SAI1 PLL enable"]
    #[inline]
    pub fn pllsai1on(&mut self) -> _PLLSAI1ONW {
        _PLLSAI1ONW { w: self }
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline]
    pub fn pllon(&mut self) -> _PLLONW {
        _PLLONW { w: self }
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline]
    pub fn csson(&mut self) -> _CSSONW {
        _CSSONW { w: self }
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline]
    pub fn hsebyp(&mut self) -> _HSEBYPW {
        _HSEBYPW { w: self }
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline]
    pub fn hseon(&mut self) -> _HSEONW {
        _HSEONW { w: self }
    }
    #[doc = "Bit 11 - HSI automatic start from Stop"]
    #[inline]
    pub fn hsiasfs(&mut self) -> _HSIASFSW {
        _HSIASFSW { w: self }
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline]
    pub fn hsikeron(&mut self) -> _HSIKERONW {
        _HSIKERONW { w: self }
    }
    #[doc = "Bit 8 - HSI clock enable"]
    #[inline]
    pub fn hsion(&mut self) -> _HSIONW {
        _HSIONW { w: self }
    }
    #[doc = "Bits 4:7 - MSI clock ranges"]
    #[inline]
    pub fn msirange(&mut self) -> _MSIRANGEW {
        _MSIRANGEW { w: self }
    }
    #[doc = "Bit 3 - MSI clock range selection"]
    #[inline]
    pub fn msirgsel(&mut self) -> _MSIRGSELW {
        _MSIRGSELW { w: self }
    }
    #[doc = "Bit 2 - MSI clock PLL enable"]
    #[inline]
    pub fn msipllen(&mut self) -> _MSIPLLENW {
        _MSIPLLENW { w: self }
    }
    #[doc = "Bit 0 - MSI clock enable"]
    #[inline]
    pub fn msion(&mut self) -> _MSIONW {
        _MSIONW { w: self }
    }
}
