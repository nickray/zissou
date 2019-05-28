#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
pub struct DDRMR {
    bits: bool,
}
impl DDRMR {
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
pub struct DHHCR {
    bits: bool,
}
impl DHHCR {
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
pub struct SIOOR {
    bits: bool,
}
impl SIOOR {
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
pub struct FMODER {
    bits: u8,
}
impl FMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMODER {
    bits: u8,
}
impl DMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCYCR {
    bits: u8,
}
impl DCYCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ABSIZER {
    bits: u8,
}
impl ABSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ABMODER {
    bits: u8,
}
impl ABMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADSIZER {
    bits: u8,
}
impl ADSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ADMODER {
    bits: u8,
}
impl ADMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IMODER {
    bits: u8,
}
impl IMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INSTRUCTIONR {
    bits: u8,
}
impl INSTRUCTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DDRMW<'a> {
    w: &'a mut W,
}
impl<'a> _DDRMW<'a> {
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DHHCW<'a> {
    w: &'a mut W,
}
impl<'a> _DHHCW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SIOOW<'a> {
    w: &'a mut W,
}
impl<'a> _SIOOW<'a> {
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
pub struct _FMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCYCW<'a> {
    w: &'a mut W,
}
impl<'a> _DCYCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ABSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _ABSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ABMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ABMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADMODEW<'a> {
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
#[doc = r" Proxy"]
pub struct _IMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _IMODEW<'a> {
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
#[doc = r" Proxy"]
pub struct _INSTRUCTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _INSTRUCTIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 31 - Double data rate mode"]
    #[inline]
    pub fn ddrm(&self) -> DDRMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDRMR { bits }
    }
    #[doc = "Bit 30 - DDR hold half cycle"]
    #[inline]
    pub fn dhhc(&self) -> DHHCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DHHCR { bits }
    }
    #[doc = "Bit 28 - Send instruction only once mode"]
    #[inline]
    pub fn sioo(&self) -> SIOOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SIOOR { bits }
    }
    #[doc = "Bits 26:27 - Functional mode"]
    #[inline]
    pub fn fmode(&self) -> FMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FMODER { bits }
    }
    #[doc = "Bits 24:25 - Data mode"]
    #[inline]
    pub fn dmode(&self) -> DMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMODER { bits }
    }
    #[doc = "Bits 18:22 - Number of dummy cycles"]
    #[inline]
    pub fn dcyc(&self) -> DCYCR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCYCR { bits }
    }
    #[doc = "Bits 16:17 - Alternate bytes size"]
    #[inline]
    pub fn absize(&self) -> ABSIZER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ABSIZER { bits }
    }
    #[doc = "Bits 14:15 - Alternate bytes mode"]
    #[inline]
    pub fn abmode(&self) -> ABMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ABMODER { bits }
    }
    #[doc = "Bits 12:13 - Address size"]
    #[inline]
    pub fn adsize(&self) -> ADSIZER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADSIZER { bits }
    }
    #[doc = "Bits 10:11 - Address mode"]
    #[inline]
    pub fn admode(&self) -> ADMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADMODER { bits }
    }
    #[doc = "Bits 8:9 - Instruction mode"]
    #[inline]
    pub fn imode(&self) -> IMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IMODER { bits }
    }
    #[doc = "Bits 0:7 - Instruction"]
    #[inline]
    pub fn instruction(&self) -> INSTRUCTIONR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INSTRUCTIONR { bits }
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
    #[doc = "Bit 31 - Double data rate mode"]
    #[inline]
    pub fn ddrm(&mut self) -> _DDRMW {
        _DDRMW { w: self }
    }
    #[doc = "Bit 30 - DDR hold half cycle"]
    #[inline]
    pub fn dhhc(&mut self) -> _DHHCW {
        _DHHCW { w: self }
    }
    #[doc = "Bit 28 - Send instruction only once mode"]
    #[inline]
    pub fn sioo(&mut self) -> _SIOOW {
        _SIOOW { w: self }
    }
    #[doc = "Bits 26:27 - Functional mode"]
    #[inline]
    pub fn fmode(&mut self) -> _FMODEW {
        _FMODEW { w: self }
    }
    #[doc = "Bits 24:25 - Data mode"]
    #[inline]
    pub fn dmode(&mut self) -> _DMODEW {
        _DMODEW { w: self }
    }
    #[doc = "Bits 18:22 - Number of dummy cycles"]
    #[inline]
    pub fn dcyc(&mut self) -> _DCYCW {
        _DCYCW { w: self }
    }
    #[doc = "Bits 16:17 - Alternate bytes size"]
    #[inline]
    pub fn absize(&mut self) -> _ABSIZEW {
        _ABSIZEW { w: self }
    }
    #[doc = "Bits 14:15 - Alternate bytes mode"]
    #[inline]
    pub fn abmode(&mut self) -> _ABMODEW {
        _ABMODEW { w: self }
    }
    #[doc = "Bits 12:13 - Address size"]
    #[inline]
    pub fn adsize(&mut self) -> _ADSIZEW {
        _ADSIZEW { w: self }
    }
    #[doc = "Bits 10:11 - Address mode"]
    #[inline]
    pub fn admode(&mut self) -> _ADMODEW {
        _ADMODEW { w: self }
    }
    #[doc = "Bits 8:9 - Instruction mode"]
    #[inline]
    pub fn imode(&mut self) -> _IMODEW {
        _IMODEW { w: self }
    }
    #[doc = "Bits 0:7 - Instruction"]
    #[inline]
    pub fn instruction(&mut self) -> _INSTRUCTIONW {
        _INSTRUCTIONW { w: self }
    }
}
