#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICR {
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
#[doc = "Values that can be written to the field `ALERTCF`"]
pub enum ALERTCFW {
    #[doc = "Clears the ALERT flag in ISR register"]
    CLEAR,
}
impl ALERTCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALERTCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALERTCFW<'a> {
    w: &'a mut W,
}
impl<'a> _ALERTCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALERTCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the ALERT flag in ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALERTCFW::CLEAR)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMOUTCF`"]
pub enum TIMOUTCFW {
    #[doc = "Clears the TIMOUT flag in ISR register"]
    CLEAR,
}
impl TIMOUTCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIMOUTCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMOUTCFW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMOUTCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMOUTCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TIMOUT flag in ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIMOUTCFW::CLEAR)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PECCF`"]
pub enum PECCFW {
    #[doc = "Clears the PEC flag in ISR register"]
    CLEAR,
}
impl PECCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PECCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PECCFW<'a> {
    w: &'a mut W,
}
impl<'a> _PECCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PECCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the PEC flag in ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECCFW::CLEAR)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OVRCF`"]
pub enum OVRCFW {
    #[doc = "Clears the OVR flag in ISR register"]
    CLEAR,
}
impl OVRCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVRCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVRCFW<'a> {
    w: &'a mut W,
}
impl<'a> _OVRCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVRCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the OVR flag in ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVRCFW::CLEAR)
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
#[doc = "Values that can be written to the field `ARLOCF`"]
pub enum ARLOCFW {
    #[doc = "Clears the ARLO flag in ISR register"]
    CLEAR,
}
impl ARLOCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARLOCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARLOCFW<'a> {
    w: &'a mut W,
}
impl<'a> _ARLOCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARLOCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the ARLO flag in ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ARLOCFW::CLEAR)
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
#[doc = "Values that can be written to the field `BERRCF`"]
pub enum BERRCFW {
    #[doc = "Clears the BERR flag in ISR register"]
    CLEAR,
}
impl BERRCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BERRCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BERRCFW<'a> {
    w: &'a mut W,
}
impl<'a> _BERRCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BERRCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the BERR flag in ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(BERRCFW::CLEAR)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STOPCF`"]
pub enum STOPCFW {
    #[doc = "Clears the STOP flag in ISR register"]
    CLEAR,
}
impl STOPCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPCFW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the STOP flag in ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(STOPCFW::CLEAR)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NACKCF`"]
pub enum NACKCFW {
    #[doc = "Clears the NACK flag in ISR register"]
    CLEAR,
}
impl NACKCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NACKCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NACKCFW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NACKCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the NACK flag in ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(NACKCFW::CLEAR)
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
#[doc = "Values that can be written to the field `ADDRCF`"]
pub enum ADDRCFW {
    #[doc = "Clears the ADDR flag in ISR register"]
    CLEAR,
}
impl ADDRCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRCFW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the ADDR flag in ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADDRCFW::CLEAR)
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
    #[doc = "Bit 13 - Alert flag clear"]
    #[inline]
    pub fn alertcf(&mut self) -> _ALERTCFW {
        _ALERTCFW { w: self }
    }
    #[doc = "Bit 12 - Timeout detection flag clear"]
    #[inline]
    pub fn timoutcf(&mut self) -> _TIMOUTCFW {
        _TIMOUTCFW { w: self }
    }
    #[doc = "Bit 11 - PEC Error flag clear"]
    #[inline]
    pub fn peccf(&mut self) -> _PECCFW {
        _PECCFW { w: self }
    }
    #[doc = "Bit 10 - Overrun/Underrun flag clear"]
    #[inline]
    pub fn ovrcf(&mut self) -> _OVRCFW {
        _OVRCFW { w: self }
    }
    #[doc = "Bit 9 - Arbitration lost flag clear"]
    #[inline]
    pub fn arlocf(&mut self) -> _ARLOCFW {
        _ARLOCFW { w: self }
    }
    #[doc = "Bit 8 - Bus error flag clear"]
    #[inline]
    pub fn berrcf(&mut self) -> _BERRCFW {
        _BERRCFW { w: self }
    }
    #[doc = "Bit 5 - Stop detection flag clear"]
    #[inline]
    pub fn stopcf(&mut self) -> _STOPCFW {
        _STOPCFW { w: self }
    }
    #[doc = "Bit 4 - Not Acknowledge flag clear"]
    #[inline]
    pub fn nackcf(&mut self) -> _NACKCFW {
        _NACKCFW { w: self }
    }
    #[doc = "Bit 3 - Address Matched flag clear"]
    #[inline]
    pub fn addrcf(&mut self) -> _ADDRCFW {
        _ADDRCFW { w: self }
    }
}
