#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PR {
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
#[doc = "Possible values of the field `PR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRR {
    #[doc = "Divider /4"]
    DIVIDEBY4,
    #[doc = "Divider /8"]
    DIVIDEBY8,
    #[doc = "Divider /16"]
    DIVIDEBY16,
    #[doc = "Divider /32"]
    DIVIDEBY32,
    #[doc = "Divider /64"]
    DIVIDEBY64,
    #[doc = "Divider /128"]
    DIVIDEBY128,
    #[doc = "Divider /256"]
    DIVIDEBY256,
    #[doc = "Divider /256"]
    DIVIDEBY256BIS,
}
impl PRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRR::DIVIDEBY4 => 0,
            PRR::DIVIDEBY8 => 1,
            PRR::DIVIDEBY16 => 2,
            PRR::DIVIDEBY32 => 3,
            PRR::DIVIDEBY64 => 4,
            PRR::DIVIDEBY128 => 5,
            PRR::DIVIDEBY256 => 6,
            PRR::DIVIDEBY256BIS => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRR {
        match value {
            0 => PRR::DIVIDEBY4,
            1 => PRR::DIVIDEBY8,
            2 => PRR::DIVIDEBY16,
            3 => PRR::DIVIDEBY32,
            4 => PRR::DIVIDEBY64,
            5 => PRR::DIVIDEBY128,
            6 => PRR::DIVIDEBY256,
            7 => PRR::DIVIDEBY256BIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY4`"]
    #[inline]
    pub fn is_divide_by4(&self) -> bool {
        *self == PRR::DIVIDEBY4
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY8`"]
    #[inline]
    pub fn is_divide_by8(&self) -> bool {
        *self == PRR::DIVIDEBY8
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY16`"]
    #[inline]
    pub fn is_divide_by16(&self) -> bool {
        *self == PRR::DIVIDEBY16
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY32`"]
    #[inline]
    pub fn is_divide_by32(&self) -> bool {
        *self == PRR::DIVIDEBY32
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY64`"]
    #[inline]
    pub fn is_divide_by64(&self) -> bool {
        *self == PRR::DIVIDEBY64
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY128`"]
    #[inline]
    pub fn is_divide_by128(&self) -> bool {
        *self == PRR::DIVIDEBY128
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY256`"]
    #[inline]
    pub fn is_divide_by256(&self) -> bool {
        *self == PRR::DIVIDEBY256
    }
    #[doc = "Checks if the value of the field is `DIVIDEBY256BIS`"]
    #[inline]
    pub fn is_divide_by256bis(&self) -> bool {
        *self == PRR::DIVIDEBY256BIS
    }
}
#[doc = "Values that can be written to the field `PR`"]
pub enum PRW {
    #[doc = "Divider /4"]
    DIVIDEBY4,
    #[doc = "Divider /8"]
    DIVIDEBY8,
    #[doc = "Divider /16"]
    DIVIDEBY16,
    #[doc = "Divider /32"]
    DIVIDEBY32,
    #[doc = "Divider /64"]
    DIVIDEBY64,
    #[doc = "Divider /128"]
    DIVIDEBY128,
    #[doc = "Divider /256"]
    DIVIDEBY256,
    #[doc = "Divider /256"]
    DIVIDEBY256BIS,
}
impl PRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRW::DIVIDEBY4 => 0,
            PRW::DIVIDEBY8 => 1,
            PRW::DIVIDEBY16 => 2,
            PRW::DIVIDEBY32 => 3,
            PRW::DIVIDEBY64 => 4,
            PRW::DIVIDEBY128 => 5,
            PRW::DIVIDEBY256 => 6,
            PRW::DIVIDEBY256BIS => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRW<'a> {
    w: &'a mut W,
}
impl<'a> _PRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divider /4"]
    #[inline]
    pub fn divide_by4(self) -> &'a mut W {
        self.variant(PRW::DIVIDEBY4)
    }
    #[doc = "Divider /8"]
    #[inline]
    pub fn divide_by8(self) -> &'a mut W {
        self.variant(PRW::DIVIDEBY8)
    }
    #[doc = "Divider /16"]
    #[inline]
    pub fn divide_by16(self) -> &'a mut W {
        self.variant(PRW::DIVIDEBY16)
    }
    #[doc = "Divider /32"]
    #[inline]
    pub fn divide_by32(self) -> &'a mut W {
        self.variant(PRW::DIVIDEBY32)
    }
    #[doc = "Divider /64"]
    #[inline]
    pub fn divide_by64(self) -> &'a mut W {
        self.variant(PRW::DIVIDEBY64)
    }
    #[doc = "Divider /128"]
    #[inline]
    pub fn divide_by128(self) -> &'a mut W {
        self.variant(PRW::DIVIDEBY128)
    }
    #[doc = "Divider /256"]
    #[inline]
    pub fn divide_by256(self) -> &'a mut W {
        self.variant(PRW::DIVIDEBY256)
    }
    #[doc = "Divider /256"]
    #[inline]
    pub fn divide_by256bis(self) -> &'a mut W {
        self.variant(PRW::DIVIDEBY256BIS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Prescaler divider"]
    #[inline]
    pub fn pr(&self) -> PRR {
        PRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:2 - Prescaler divider"]
    #[inline]
    pub fn pr(&mut self) -> _PRW {
        _PRW { w: self }
    }
}
