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
#[doc = "Values that can be written to the field `WUCF`"]
pub enum WUCFW {
    #[doc = "Clears the WUF flag in the ISR register"]
    CLEAR,
}
impl WUCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUCFW<'a> {
    w: &'a mut W,
}
impl<'a> _WUCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the WUF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(WUCFW::CLEAR)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMCF`"]
pub enum CMCFW {
    #[doc = "Clears the CMF flag in the ISR register"]
    CLEAR,
}
impl CMCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMCFW<'a> {
    w: &'a mut W,
}
impl<'a> _CMCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the CMF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMCFW::CLEAR)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTSCF`"]
pub enum CTSCFW {
    #[doc = "Clears the CTSIF flag in the ISR register"]
    CLEAR,
}
impl CTSCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTSCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTSCFW<'a> {
    w: &'a mut W,
}
impl<'a> _CTSCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTSCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the CTSIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTSCFW::CLEAR)
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
#[doc = "Values that can be written to the field `TCCF`"]
pub enum TCCFW {
    #[doc = "Clears the TC flag in the ISR register"]
    CLEAR,
}
impl TCCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCCFW<'a> {
    w: &'a mut W,
}
impl<'a> _TCCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TC flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(TCCFW::CLEAR)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDLECF`"]
pub enum IDLECFW {
    #[doc = "Clears the IDLE flag in the ISR register"]
    CLEAR,
}
impl IDLECFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLECFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECFW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the IDLE flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(IDLECFW::CLEAR)
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
#[doc = "Values that can be written to the field `ORECF`"]
pub enum ORECFW {
    #[doc = "Clears the ORE flag in the ISR register"]
    CLEAR,
}
impl ORECFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ORECFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ORECFW<'a> {
    w: &'a mut W,
}
impl<'a> _ORECFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ORECFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the ORE flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ORECFW::CLEAR)
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
#[doc = "Values that can be written to the field `NCF`"]
pub enum NCFW {
    #[doc = "Clears the NF flag in the ISR register"]
    CLEAR,
}
impl NCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NCFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NCFW<'a> {
    w: &'a mut W,
}
impl<'a> _NCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the NF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(NCFW::CLEAR)
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
#[doc = "Values that can be written to the field `FECF`"]
pub enum FECFW {
    #[doc = "Clears the FE flag in the ISR register"]
    CLEAR,
}
impl FECFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FECFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FECFW<'a> {
    w: &'a mut W,
}
impl<'a> _FECFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FECFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the FE flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(FECFW::CLEAR)
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
#[doc = "Values that can be written to the field `PECF`"]
pub enum PECFW {
    #[doc = "Clears the PE flag in the ISR register"]
    CLEAR,
}
impl PECFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PECFW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PECFW<'a> {
    w: &'a mut W,
}
impl<'a> _PECFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PECFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the PE flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(PECFW::CLEAR)
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
    #[doc = "Bit 20 - Wakeup from Stop mode clear flag"]
    #[inline]
    pub fn wucf(&mut self) -> _WUCFW {
        _WUCFW { w: self }
    }
    #[doc = "Bit 17 - Character match clear flag"]
    #[inline]
    pub fn cmcf(&mut self) -> _CMCFW {
        _CMCFW { w: self }
    }
    #[doc = "Bit 9 - CTS clear flag"]
    #[inline]
    pub fn ctscf(&mut self) -> _CTSCFW {
        _CTSCFW { w: self }
    }
    #[doc = "Bit 6 - Transmission complete clear flag"]
    #[inline]
    pub fn tccf(&mut self) -> _TCCFW {
        _TCCFW { w: self }
    }
    #[doc = "Bit 4 - Idle line detected clear flag"]
    #[inline]
    pub fn idlecf(&mut self) -> _IDLECFW {
        _IDLECFW { w: self }
    }
    #[doc = "Bit 3 - Overrun error clear flag"]
    #[inline]
    pub fn orecf(&mut self) -> _ORECFW {
        _ORECFW { w: self }
    }
    #[doc = "Bit 2 - Noise detected clear flag"]
    #[inline]
    pub fn ncf(&mut self) -> _NCFW {
        _NCFW { w: self }
    }
    #[doc = "Bit 1 - Framing error clear flag"]
    #[inline]
    pub fn fecf(&mut self) -> _FECFW {
        _FECFW { w: self }
    }
    #[doc = "Bit 0 - Parity error clear flag"]
    #[inline]
    pub fn pecf(&mut self) -> _PECFW {
        _PECFW { w: self }
    }
}
