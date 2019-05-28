#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OAR2 {
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
pub struct OA2R {
    bits: u8,
}
impl OA2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `OA2MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA2MSKR {
    #[doc = "No mask"]
    NOMASK,
    #[doc = "OA2\\[1\\] is masked and don\u{2019}t care. Only OA2\\[7:2\\] are compared"]
    MASK1,
    #[doc = "OA2\\[2:1\\] are masked and don\u{2019}t care. Only OA2\\[7:3\\] are compared"]
    MASK2,
    #[doc = "OA2\\[3:1\\] are masked and don\u{2019}t care. Only OA2\\[7:4\\] are compared"]
    MASK3,
    #[doc = "OA2\\[4:1\\] are masked and don\u{2019}t care. Only OA2\\[7:5\\] are compared"]
    MASK4,
    #[doc = "OA2\\[5:1\\] are masked and don\u{2019}t care. Only OA2\\[7:6\\] are compared"]
    MASK5,
    #[doc = "OA2\\[6:1\\] are masked and don\u{2019}t care. Only OA2\\[7\\] is compared."]
    MASK6,
    #[doc = "OA2\\[7:1\\] are masked and don\u{2019}t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
    MASK7,
}
impl OA2MSKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OA2MSKR::NOMASK => 0,
            OA2MSKR::MASK1 => 1,
            OA2MSKR::MASK2 => 2,
            OA2MSKR::MASK3 => 3,
            OA2MSKR::MASK4 => 4,
            OA2MSKR::MASK5 => 5,
            OA2MSKR::MASK6 => 6,
            OA2MSKR::MASK7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OA2MSKR {
        match value {
            0 => OA2MSKR::NOMASK,
            1 => OA2MSKR::MASK1,
            2 => OA2MSKR::MASK2,
            3 => OA2MSKR::MASK3,
            4 => OA2MSKR::MASK4,
            5 => OA2MSKR::MASK5,
            6 => OA2MSKR::MASK6,
            7 => OA2MSKR::MASK7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOMASK`"]
    #[inline]
    pub fn is_no_mask(&self) -> bool {
        *self == OA2MSKR::NOMASK
    }
    #[doc = "Checks if the value of the field is `MASK1`"]
    #[inline]
    pub fn is_mask1(&self) -> bool {
        *self == OA2MSKR::MASK1
    }
    #[doc = "Checks if the value of the field is `MASK2`"]
    #[inline]
    pub fn is_mask2(&self) -> bool {
        *self == OA2MSKR::MASK2
    }
    #[doc = "Checks if the value of the field is `MASK3`"]
    #[inline]
    pub fn is_mask3(&self) -> bool {
        *self == OA2MSKR::MASK3
    }
    #[doc = "Checks if the value of the field is `MASK4`"]
    #[inline]
    pub fn is_mask4(&self) -> bool {
        *self == OA2MSKR::MASK4
    }
    #[doc = "Checks if the value of the field is `MASK5`"]
    #[inline]
    pub fn is_mask5(&self) -> bool {
        *self == OA2MSKR::MASK5
    }
    #[doc = "Checks if the value of the field is `MASK6`"]
    #[inline]
    pub fn is_mask6(&self) -> bool {
        *self == OA2MSKR::MASK6
    }
    #[doc = "Checks if the value of the field is `MASK7`"]
    #[inline]
    pub fn is_mask7(&self) -> bool {
        *self == OA2MSKR::MASK7
    }
}
#[doc = "Possible values of the field `OA2EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA2ENR {
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed"]
    DISABLED,
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed"]
    ENABLED,
}
impl OA2ENR {
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
            OA2ENR::DISABLED => false,
            OA2ENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OA2ENR {
        match value {
            false => OA2ENR::DISABLED,
            true => OA2ENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OA2ENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OA2ENR::ENABLED
    }
}
#[doc = r" Proxy"]
pub struct _OA2W<'a> {
    w: &'a mut W,
}
impl<'a> _OA2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OA2MSK`"]
pub enum OA2MSKW {
    #[doc = "No mask"]
    NOMASK,
    #[doc = "OA2\\[1\\] is masked and don\u{2019}t care. Only OA2\\[7:2\\] are compared"]
    MASK1,
    #[doc = "OA2\\[2:1\\] are masked and don\u{2019}t care. Only OA2\\[7:3\\] are compared"]
    MASK2,
    #[doc = "OA2\\[3:1\\] are masked and don\u{2019}t care. Only OA2\\[7:4\\] are compared"]
    MASK3,
    #[doc = "OA2\\[4:1\\] are masked and don\u{2019}t care. Only OA2\\[7:5\\] are compared"]
    MASK4,
    #[doc = "OA2\\[5:1\\] are masked and don\u{2019}t care. Only OA2\\[7:6\\] are compared"]
    MASK5,
    #[doc = "OA2\\[6:1\\] are masked and don\u{2019}t care. Only OA2\\[7\\] is compared."]
    MASK6,
    #[doc = "OA2\\[7:1\\] are masked and don\u{2019}t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
    MASK7,
}
impl OA2MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OA2MSKW::NOMASK => 0,
            OA2MSKW::MASK1 => 1,
            OA2MSKW::MASK2 => 2,
            OA2MSKW::MASK3 => 3,
            OA2MSKW::MASK4 => 4,
            OA2MSKW::MASK5 => 5,
            OA2MSKW::MASK6 => 6,
            OA2MSKW::MASK7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OA2MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _OA2MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OA2MSKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No mask"]
    #[inline]
    pub fn no_mask(self) -> &'a mut W {
        self.variant(OA2MSKW::NOMASK)
    }
    #[doc = "OA2\\[1\\] is masked and don\u{2019}t care. Only OA2\\[7:2\\] are compared"]
    #[inline]
    pub fn mask1(self) -> &'a mut W {
        self.variant(OA2MSKW::MASK1)
    }
    #[doc = "OA2\\[2:1\\] are masked and don\u{2019}t care. Only OA2\\[7:3\\] are compared"]
    #[inline]
    pub fn mask2(self) -> &'a mut W {
        self.variant(OA2MSKW::MASK2)
    }
    #[doc = "OA2\\[3:1\\] are masked and don\u{2019}t care. Only OA2\\[7:4\\] are compared"]
    #[inline]
    pub fn mask3(self) -> &'a mut W {
        self.variant(OA2MSKW::MASK3)
    }
    #[doc = "OA2\\[4:1\\] are masked and don\u{2019}t care. Only OA2\\[7:5\\] are compared"]
    #[inline]
    pub fn mask4(self) -> &'a mut W {
        self.variant(OA2MSKW::MASK4)
    }
    #[doc = "OA2\\[5:1\\] are masked and don\u{2019}t care. Only OA2\\[7:6\\] are compared"]
    #[inline]
    pub fn mask5(self) -> &'a mut W {
        self.variant(OA2MSKW::MASK5)
    }
    #[doc = "OA2\\[6:1\\] are masked and don\u{2019}t care. Only OA2\\[7\\] is compared."]
    #[inline]
    pub fn mask6(self) -> &'a mut W {
        self.variant(OA2MSKW::MASK6)
    }
    #[doc = "OA2\\[7:1\\] are masked and don\u{2019}t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged"]
    #[inline]
    pub fn mask7(self) -> &'a mut W {
        self.variant(OA2MSKW::MASK7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OA2EN`"]
pub enum OA2ENW {
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed"]
    DISABLED,
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed"]
    ENABLED,
}
impl OA2ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OA2ENW::DISABLED => false,
            OA2ENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OA2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _OA2ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OA2ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OA2ENW::DISABLED)
    }
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OA2ENW::ENABLED)
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
    #[doc = "Bits 1:7 - Interface address"]
    #[inline]
    pub fn oa2(&self) -> OA2R {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OA2R { bits }
    }
    #[doc = "Bits 8:10 - Own Address 2 masks"]
    #[inline]
    pub fn oa2msk(&self) -> OA2MSKR {
        OA2MSKR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline]
    pub fn oa2en(&self) -> OA2ENR {
        OA2ENR::_from({
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
    #[doc = "Bits 1:7 - Interface address"]
    #[inline]
    pub fn oa2(&mut self) -> _OA2W {
        _OA2W { w: self }
    }
    #[doc = "Bits 8:10 - Own Address 2 masks"]
    #[inline]
    pub fn oa2msk(&mut self) -> _OA2MSKW {
        _OA2MSKW { w: self }
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline]
    pub fn oa2en(&mut self) -> _OA2ENW {
        _OA2ENW { w: self }
    }
}
