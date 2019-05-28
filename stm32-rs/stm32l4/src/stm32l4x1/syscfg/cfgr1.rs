#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGR1 {
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
pub struct FPU_IER {
    bits: u8,
}
impl FPU_IER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2C3_FMPR {
    bits: bool,
}
impl I2C3_FMPR {
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
pub struct I2C2_FMPR {
    bits: bool,
}
impl I2C2_FMPR {
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
pub struct I2C1_FMPR {
    bits: bool,
}
impl I2C1_FMPR {
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
pub struct I2C_PB9_FMPR {
    bits: bool,
}
impl I2C_PB9_FMPR {
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
pub struct I2C_PB8_FMPR {
    bits: bool,
}
impl I2C_PB8_FMPR {
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
pub struct I2C_PB7_FMPR {
    bits: bool,
}
impl I2C_PB7_FMPR {
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
pub struct I2C_PB6_FMPR {
    bits: bool,
}
impl I2C_PB6_FMPR {
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
pub struct BOOSTENR {
    bits: bool,
}
impl BOOSTENR {
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
pub struct FWDISR {
    bits: bool,
}
impl FWDISR {
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
pub struct _FPU_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _FPU_IEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C3_FMPW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C3_FMPW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C2_FMPW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2_FMPW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C1_FMPW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1_FMPW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C_PB9_FMPW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_PB9_FMPW<'a> {
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
pub struct _I2C_PB8_FMPW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_PB8_FMPW<'a> {
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
pub struct _I2C_PB7_FMPW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_PB7_FMPW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C_PB6_FMPW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_PB6_FMPW<'a> {
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
pub struct _BOOSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOSTENW<'a> {
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
#[doc = r" Proxy"]
pub struct _FWDISW<'a> {
    w: &'a mut W,
}
impl<'a> _FWDISW<'a> {
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
    #[doc = "Bits 26:31 - Floating Point Unit interrupts enable bits"]
    #[inline]
    pub fn fpu_ie(&self) -> FPU_IER {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FPU_IER { bits }
    }
    #[doc = "Bit 22 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline]
    pub fn i2c3_fmp(&self) -> I2C3_FMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        I2C3_FMPR { bits }
    }
    #[doc = "Bit 21 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline]
    pub fn i2c2_fmp(&self) -> I2C2_FMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        I2C2_FMPR { bits }
    }
    #[doc = "Bit 20 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline]
    pub fn i2c1_fmp(&self) -> I2C1_FMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        I2C1_FMPR { bits }
    }
    #[doc = "Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline]
    pub fn i2c_pb9_fmp(&self) -> I2C_PB9_FMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        I2C_PB9_FMPR { bits }
    }
    #[doc = "Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline]
    pub fn i2c_pb8_fmp(&self) -> I2C_PB8_FMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        I2C_PB8_FMPR { bits }
    }
    #[doc = "Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline]
    pub fn i2c_pb7_fmp(&self) -> I2C_PB7_FMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        I2C_PB7_FMPR { bits }
    }
    #[doc = "Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline]
    pub fn i2c_pb6_fmp(&self) -> I2C_PB6_FMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        I2C_PB6_FMPR { bits }
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline]
    pub fn boosten(&self) -> BOOSTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOOSTENR { bits }
    }
    #[doc = "Bit 0 - Firewall disable"]
    #[inline]
    pub fn fwdis(&self) -> FWDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FWDISR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2080374785 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 26:31 - Floating Point Unit interrupts enable bits"]
    #[inline]
    pub fn fpu_ie(&mut self) -> _FPU_IEW {
        _FPU_IEW { w: self }
    }
    #[doc = "Bit 22 - I2C3 Fast-mode Plus driving capability activation"]
    #[inline]
    pub fn i2c3_fmp(&mut self) -> _I2C3_FMPW {
        _I2C3_FMPW { w: self }
    }
    #[doc = "Bit 21 - I2C2 Fast-mode Plus driving capability activation"]
    #[inline]
    pub fn i2c2_fmp(&mut self) -> _I2C2_FMPW {
        _I2C2_FMPW { w: self }
    }
    #[doc = "Bit 20 - I2C1 Fast-mode Plus driving capability activation"]
    #[inline]
    pub fn i2c1_fmp(&mut self) -> _I2C1_FMPW {
        _I2C1_FMPW { w: self }
    }
    #[doc = "Bit 19 - Fast-mode Plus (Fm+) driving capability activation on PB9"]
    #[inline]
    pub fn i2c_pb9_fmp(&mut self) -> _I2C_PB9_FMPW {
        _I2C_PB9_FMPW { w: self }
    }
    #[doc = "Bit 18 - Fast-mode Plus (Fm+) driving capability activation on PB8"]
    #[inline]
    pub fn i2c_pb8_fmp(&mut self) -> _I2C_PB8_FMPW {
        _I2C_PB8_FMPW { w: self }
    }
    #[doc = "Bit 17 - Fast-mode Plus (Fm+) driving capability activation on PB7"]
    #[inline]
    pub fn i2c_pb7_fmp(&mut self) -> _I2C_PB7_FMPW {
        _I2C_PB7_FMPW { w: self }
    }
    #[doc = "Bit 16 - Fast-mode Plus (Fm+) driving capability activation on PB6"]
    #[inline]
    pub fn i2c_pb6_fmp(&mut self) -> _I2C_PB6_FMPW {
        _I2C_PB6_FMPW { w: self }
    }
    #[doc = "Bit 8 - I/O analog switch voltage booster enable"]
    #[inline]
    pub fn boosten(&mut self) -> _BOOSTENW {
        _BOOSTENW { w: self }
    }
    #[doc = "Bit 0 - Firewall disable"]
    #[inline]
    pub fn fwdis(&mut self) -> _FWDISW {
        _FWDISW { w: self }
    }
}
