#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCIPR {
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
pub struct DFSDMSELR {
    bits: bool,
}
impl DFSDMSELR {
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
pub struct SWPMI1SELR {
    bits: bool,
}
impl SWPMI1SELR {
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
pub struct ADCSELR {
    bits: u8,
}
impl ADCSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CLK48SELR {
    bits: u8,
}
impl CLK48SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAI2SELR {
    bits: u8,
}
impl SAI2SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SAI1SELR {
    bits: u8,
}
impl SAI1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPTIM2SELR {
    bits: u8,
}
impl LPTIM2SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPTIM1SELR {
    bits: u8,
}
impl LPTIM1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2C3SELR {
    bits: u8,
}
impl I2C3SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2C2SELR {
    bits: u8,
}
impl I2C2SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2C1SELR {
    bits: u8,
}
impl I2C1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LPUART1SELR {
    bits: u8,
}
impl LPUART1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UART5SELR {
    bits: u8,
}
impl UART5SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UART4SELR {
    bits: u8,
}
impl UART4SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USART3SELR {
    bits: u8,
}
impl USART3SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USART2SELR {
    bits: u8,
}
impl USART2SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USART1SELR {
    bits: u8,
}
impl USART1SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DFSDMSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DFSDMSELW<'a> {
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
pub struct _SWPMI1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SWPMI1SELW<'a> {
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
pub struct _ADCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADCSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLK48SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CLK48SELW<'a> {
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
pub struct _SAI2SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI2SELW<'a> {
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
pub struct _SAI1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SAI1SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPTIM2SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LPTIM2SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPTIM1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LPTIM1SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2C3SELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C3SELW<'a> {
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
pub struct _I2C2SELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C2SELW<'a> {
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
pub struct _I2C1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1SELW<'a> {
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
pub struct _LPUART1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _LPUART1SELW<'a> {
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
pub struct _UART5SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UART5SELW<'a> {
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
pub struct _UART4SELW<'a> {
    w: &'a mut W,
}
impl<'a> _UART4SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USART3SELW<'a> {
    w: &'a mut W,
}
impl<'a> _USART3SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USART2SELW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USART1SELW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 31 - DFSDM clock source selection"]
    #[inline]
    pub fn dfsdmsel(&self) -> DFSDMSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DFSDMSELR { bits }
    }
    #[doc = "Bit 30 - SWPMI1 clock source selection"]
    #[inline]
    pub fn swpmi1sel(&self) -> SWPMI1SELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWPMI1SELR { bits }
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline]
    pub fn adcsel(&self) -> ADCSELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADCSELR { bits }
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline]
    pub fn clk48sel(&self) -> CLK48SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CLK48SELR { bits }
    }
    #[doc = "Bits 24:25 - SAI2 clock source selection"]
    #[inline]
    pub fn sai2sel(&self) -> SAI2SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAI2SELR { bits }
    }
    #[doc = "Bits 22:23 - SAI1 clock source selection"]
    #[inline]
    pub fn sai1sel(&self) -> SAI1SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SAI1SELR { bits }
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline]
    pub fn lptim2sel(&self) -> LPTIM2SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPTIM2SELR { bits }
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline]
    pub fn lptim1sel(&self) -> LPTIM1SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPTIM1SELR { bits }
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline]
    pub fn i2c3sel(&self) -> I2C3SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2C3SELR { bits }
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline]
    pub fn i2c2sel(&self) -> I2C2SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2C2SELR { bits }
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline]
    pub fn i2c1sel(&self) -> I2C1SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        I2C1SELR { bits }
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline]
    pub fn lpuart1sel(&self) -> LPUART1SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LPUART1SELR { bits }
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline]
    pub fn uart5sel(&self) -> UART5SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UART5SELR { bits }
    }
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline]
    pub fn uart4sel(&self) -> UART4SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UART4SELR { bits }
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline]
    pub fn usart3sel(&self) -> USART3SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USART3SELR { bits }
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline]
    pub fn usart2sel(&self) -> USART2SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USART2SELR { bits }
    }
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline]
    pub fn usart1sel(&self) -> USART1SELR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USART1SELR { bits }
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
    #[doc = "Bit 31 - DFSDM clock source selection"]
    #[inline]
    pub fn dfsdmsel(&mut self) -> _DFSDMSELW {
        _DFSDMSELW { w: self }
    }
    #[doc = "Bit 30 - SWPMI1 clock source selection"]
    #[inline]
    pub fn swpmi1sel(&mut self) -> _SWPMI1SELW {
        _SWPMI1SELW { w: self }
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline]
    pub fn adcsel(&mut self) -> _ADCSELW {
        _ADCSELW { w: self }
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline]
    pub fn clk48sel(&mut self) -> _CLK48SELW {
        _CLK48SELW { w: self }
    }
    #[doc = "Bits 24:25 - SAI2 clock source selection"]
    #[inline]
    pub fn sai2sel(&mut self) -> _SAI2SELW {
        _SAI2SELW { w: self }
    }
    #[doc = "Bits 22:23 - SAI1 clock source selection"]
    #[inline]
    pub fn sai1sel(&mut self) -> _SAI1SELW {
        _SAI1SELW { w: self }
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline]
    pub fn lptim2sel(&mut self) -> _LPTIM2SELW {
        _LPTIM2SELW { w: self }
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline]
    pub fn lptim1sel(&mut self) -> _LPTIM1SELW {
        _LPTIM1SELW { w: self }
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline]
    pub fn i2c3sel(&mut self) -> _I2C3SELW {
        _I2C3SELW { w: self }
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline]
    pub fn i2c2sel(&mut self) -> _I2C2SELW {
        _I2C2SELW { w: self }
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline]
    pub fn i2c1sel(&mut self) -> _I2C1SELW {
        _I2C1SELW { w: self }
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline]
    pub fn lpuart1sel(&mut self) -> _LPUART1SELW {
        _LPUART1SELW { w: self }
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline]
    pub fn uart5sel(&mut self) -> _UART5SELW {
        _UART5SELW { w: self }
    }
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline]
    pub fn uart4sel(&mut self) -> _UART4SELW {
        _UART4SELW { w: self }
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline]
    pub fn usart3sel(&mut self) -> _USART3SELW {
        _USART3SELW { w: self }
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline]
    pub fn usart2sel(&mut self) -> _USART2SELW {
        _USART2SELW { w: self }
    }
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline]
    pub fn usart1sel(&mut self) -> _USART1SELW {
        _USART1SELW { w: self }
    }
}
