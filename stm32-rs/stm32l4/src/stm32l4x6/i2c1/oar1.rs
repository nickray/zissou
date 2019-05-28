#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OAR1 {
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
pub struct OA1R {
    bits: u16,
}
impl OA1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `OA1MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA1MODER {
    #[doc = "Own address 1 is a 7-bit address"]
    BIT7,
    #[doc = "Own address 1 is a 10-bit address"]
    BIT10,
}
impl OA1MODER {
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
            OA1MODER::BIT7 => false,
            OA1MODER::BIT10 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OA1MODER {
        match value {
            false => OA1MODER::BIT7,
            true => OA1MODER::BIT10,
        }
    }
    #[doc = "Checks if the value of the field is `BIT7`"]
    #[inline]
    pub fn is_bit7(&self) -> bool {
        *self == OA1MODER::BIT7
    }
    #[doc = "Checks if the value of the field is `BIT10`"]
    #[inline]
    pub fn is_bit10(&self) -> bool {
        *self == OA1MODER::BIT10
    }
}
#[doc = "Possible values of the field `OA1EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA1ENR {
    #[doc = "Own address 1 disabled. The received slave address OA1 is NACKed"]
    DIASBLED,
    #[doc = "Own address 1 enabled. The received slave address OA1 is ACKed"]
    ENABLED,
}
impl OA1ENR {
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
            OA1ENR::DIASBLED => false,
            OA1ENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OA1ENR {
        match value {
            false => OA1ENR::DIASBLED,
            true => OA1ENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DIASBLED`"]
    #[inline]
    pub fn is_diasbled(&self) -> bool {
        *self == OA1ENR::DIASBLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OA1ENR::ENABLED
    }
}
#[doc = r" Proxy"]
pub struct _OA1W<'a> {
    w: &'a mut W,
}
impl<'a> _OA1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OA1MODE`"]
pub enum OA1MODEW {
    #[doc = "Own address 1 is a 7-bit address"]
    BIT7,
    #[doc = "Own address 1 is a 10-bit address"]
    BIT10,
}
impl OA1MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OA1MODEW::BIT7 => false,
            OA1MODEW::BIT10 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OA1MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _OA1MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OA1MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Own address 1 is a 7-bit address"]
    #[inline]
    pub fn bit7(self) -> &'a mut W {
        self.variant(OA1MODEW::BIT7)
    }
    #[doc = "Own address 1 is a 10-bit address"]
    #[inline]
    pub fn bit10(self) -> &'a mut W {
        self.variant(OA1MODEW::BIT10)
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
#[doc = "Values that can be written to the field `OA1EN`"]
pub enum OA1ENW {
    #[doc = "Own address 1 disabled. The received slave address OA1 is NACKed"]
    DIASBLED,
    #[doc = "Own address 1 enabled. The received slave address OA1 is ACKed"]
    ENABLED,
}
impl OA1ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OA1ENW::DIASBLED => false,
            OA1ENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OA1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OA1ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OA1ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Own address 1 disabled. The received slave address OA1 is NACKed"]
    #[inline]
    pub fn diasbled(self) -> &'a mut W {
        self.variant(OA1ENW::DIASBLED)
    }
    #[doc = "Own address 1 enabled. The received slave address OA1 is ACKed"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OA1ENW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - Interface address"]
    #[inline]
    pub fn oa1(&self) -> OA1R {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        OA1R { bits }
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode"]
    #[inline]
    pub fn oa1mode(&self) -> OA1MODER {
        OA1MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline]
    pub fn oa1en(&self) -> OA1ENR {
        OA1ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
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
    #[doc = "Bits 0:9 - Interface address"]
    #[inline]
    pub fn oa1(&mut self) -> _OA1W {
        _OA1W { w: self }
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode"]
    #[inline]
    pub fn oa1mode(&mut self) -> _OA1MODEW {
        _OA1MODEW { w: self }
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline]
    pub fn oa1en(&mut self) -> _OA1ENW {
        _OA1ENW { w: self }
    }
}
