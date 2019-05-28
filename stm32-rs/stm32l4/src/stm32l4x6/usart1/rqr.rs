#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RQR {
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
}
#[doc = "Values that can be written to the field `TXFRQ`"]
pub enum TXFRQW {
    #[doc = "Set the TXE flags. This allows to discard the transmit data"]
    DISCARD,
}
impl TXFRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFRQW::DISCARD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFRQW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Set the TXE flags. This allows to discard the transmit data"]
    #[inline]
    pub fn discard(self) -> &'a mut W {
        self.variant(TXFRQW::DISCARD)
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
#[doc = "Values that can be written to the field `RXFRQ`"]
pub enum RXFRQW {
    #[doc = "clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    DISCARD,
}
impl RXFRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFRQW::DISCARD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFRQW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "clears the RXNE flag. This allows to discard the received data without reading it, and avoid an overrun condition"]
    #[inline]
    pub fn discard(self) -> &'a mut W {
        self.variant(RXFRQW::DISCARD)
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
#[doc = "Values that can be written to the field `MMRQ`"]
pub enum MMRQW {
    #[doc = "Puts the USART in mute mode and sets the RWU flag"]
    MUTE,
}
impl MMRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MMRQW::MUTE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMRQW<'a> {
    w: &'a mut W,
}
impl<'a> _MMRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Puts the USART in mute mode and sets the RWU flag"]
    #[inline]
    pub fn mute(self) -> &'a mut W {
        self.variant(MMRQW::MUTE)
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
#[doc = "Values that can be written to the field `SBKRQ`"]
pub enum SBKRQW {
    #[doc = "sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    BREAK,
}
impl SBKRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBKRQW::BREAK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBKRQW<'a> {
    w: &'a mut W,
}
impl<'a> _SBKRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBKRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "sets the SBKF flag and request to send a BREAK on the line, as soon as the transmit machine is available"]
    #[inline]
    pub fn break_(self) -> &'a mut W {
        self.variant(SBKRQW::BREAK)
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
#[doc = "Values that can be written to the field `ABRRQ`"]
pub enum ABRRQW {
    #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    REQUEST,
}
impl ABRRQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABRRQW::REQUEST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABRRQW<'a> {
    w: &'a mut W,
}
impl<'a> _ABRRQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABRRQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "resets the ABRF flag in the USART_ISR and request an automatic baud rate measurement on the next received data frame"]
    #[inline]
    pub fn request(self) -> &'a mut W {
        self.variant(ABRRQW::REQUEST)
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
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline]
    pub fn txfrq(&mut self) -> _TXFRQW {
        _TXFRQW { w: self }
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline]
    pub fn rxfrq(&mut self) -> _RXFRQW {
        _RXFRQW { w: self }
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline]
    pub fn mmrq(&mut self) -> _MMRQW {
        _MMRQW { w: self }
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline]
    pub fn sbkrq(&mut self) -> _SBKRQW {
        _SBKRQW { w: self }
    }
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline]
    pub fn abrrq(&mut self) -> _ABRRQW {
        _ABRRQW { w: self }
    }
}
