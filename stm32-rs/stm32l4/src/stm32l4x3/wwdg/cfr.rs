#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFR {
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
#[doc = "Possible values of the field `EWI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWIR {
    #[doc = "interrupt occurs whenever the counter reaches the value 0x40"]
    ENABLE,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EWIR {
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
            EWIR::ENABLE => true,
            EWIR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EWIR {
        match value {
            true => EWIR::ENABLE,
            i => EWIR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == EWIR::ENABLE
    }
}
#[doc = r" Value of the field"]
pub struct WR {
    bits: u8,
}
impl WR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WDGTB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDGTBR {
    #[doc = "Counter clock (PCLK1 div 4096) div 1"]
    DIV1,
    #[doc = "Counter clock (PCLK1 div 4096) div 2"]
    DIV2,
    #[doc = "Counter clock (PCLK1 div 4096) div 4"]
    DIV4,
    #[doc = "Counter clock (PCLK1 div 4096) div 8"]
    DIV8,
}
impl WDGTBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WDGTBR::DIV1 => 0,
            WDGTBR::DIV2 => 1,
            WDGTBR::DIV4 => 2,
            WDGTBR::DIV8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WDGTBR {
        match value {
            0 => WDGTBR::DIV1,
            1 => WDGTBR::DIV2,
            2 => WDGTBR::DIV4,
            3 => WDGTBR::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == WDGTBR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == WDGTBR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == WDGTBR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == WDGTBR::DIV8
    }
}
#[doc = "Values that can be written to the field `EWI`"]
pub enum EWIW {
    #[doc = "interrupt occurs whenever the counter reaches the value 0x40"]
    ENABLE,
}
impl EWIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EWIW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EWIW<'a> {
    w: &'a mut W,
}
impl<'a> _EWIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EWIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "interrupt occurs whenever the counter reaches the value 0x40"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(EWIW::ENABLE)
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
#[doc = r" Proxy"]
pub struct _WW<'a> {
    w: &'a mut W,
}
impl<'a> _WW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDGTB`"]
pub enum WDGTBW {
    #[doc = "Counter clock (PCLK1 div 4096) div 1"]
    DIV1,
    #[doc = "Counter clock (PCLK1 div 4096) div 2"]
    DIV2,
    #[doc = "Counter clock (PCLK1 div 4096) div 4"]
    DIV4,
    #[doc = "Counter clock (PCLK1 div 4096) div 8"]
    DIV8,
}
impl WDGTBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WDGTBW::DIV1 => 0,
            WDGTBW::DIV2 => 1,
            WDGTBW::DIV4 => 2,
            WDGTBW::DIV8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDGTBW<'a> {
    w: &'a mut W,
}
impl<'a> _WDGTBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDGTBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 1"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(WDGTBW::DIV1)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(WDGTBW::DIV2)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(WDGTBW::DIV4)
    }
    #[doc = "Counter clock (PCLK1 div 4096) div 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(WDGTBW::DIV8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline]
    pub fn ewi(&self) -> EWIR {
        EWIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline]
    pub fn w(&self) -> WR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WR { bits }
    }
    #[doc = "Bits 7:8 - Timer base"]
    #[inline]
    pub fn wdgtb(&self) -> WDGTBR {
        WDGTBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 127 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 9 - Early wakeup interrupt"]
    #[inline]
    pub fn ewi(&mut self) -> _EWIW {
        _EWIW { w: self }
    }
    #[doc = "Bits 0:6 - 7-bit window value"]
    #[inline]
    pub fn w(&mut self) -> _WW {
        _WW { w: self }
    }
    #[doc = "Bits 7:8 - Timer base"]
    #[inline]
    pub fn wdgtb(&mut self) -> _WDGTBW {
        _WDGTBW { w: self }
    }
}
