#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EGR {
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
#[doc = "Values that can be written to the field `TG`"]
pub enum TGW {
    #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    TRIGGER,
}
impl TGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TGW::TRIGGER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TGW<'a> {
    w: &'a mut W,
}
impl<'a> _TGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled."]
    #[inline]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TGW::TRIGGER)
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
#[doc = "Values that can be written to the field `CC4G`"]
pub enum CC4GW {
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    TRIGGER,
}
impl CC4GW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CC4GW::TRIGGER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4GW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4GW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4GW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC4GW::TRIGGER)
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
#[doc = "Values that can be written to the field `CC3G`"]
pub type CC3GW = CC4GW;
#[doc = r" Proxy"]
pub struct _CC3GW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3GW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3GW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC4GW::TRIGGER)
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
#[doc = "Values that can be written to the field `CC2G`"]
pub type CC2GW = CC4GW;
#[doc = r" Proxy"]
pub struct _CC2GW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2GW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2GW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC4GW::TRIGGER)
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
#[doc = "Values that can be written to the field `CC1G`"]
pub type CC1GW = CC4GW;
#[doc = r" Proxy"]
pub struct _CC1GW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1GW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1GW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register."]
    #[inline]
    pub fn trigger(self) -> &'a mut W {
        self.variant(CC4GW::TRIGGER)
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
#[doc = "Values that can be written to the field `UG`"]
pub enum UGW {
    #[doc = "Re-initializes the timer counter and generates an update of the reigsters."]
    UPDATE,
}
impl UGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UGW::UPDATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UGW<'a> {
    w: &'a mut W,
}
impl<'a> _UGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Re-initializes the timer counter and generates an update of the reigsters."]
    #[inline]
    pub fn update(self) -> &'a mut W {
        self.variant(UGW::UPDATE)
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
    #[doc = "Bit 6 - Trigger generation"]
    #[inline]
    pub fn tg(&mut self) -> _TGW {
        _TGW { w: self }
    }
    #[doc = "Bit 4 - Capture/compare 4 generation"]
    #[inline]
    pub fn cc4g(&mut self) -> _CC4GW {
        _CC4GW { w: self }
    }
    #[doc = "Bit 3 - Capture/compare 3 generation"]
    #[inline]
    pub fn cc3g(&mut self) -> _CC3GW {
        _CC3GW { w: self }
    }
    #[doc = "Bit 2 - Capture/compare 2 generation"]
    #[inline]
    pub fn cc2g(&mut self) -> _CC2GW {
        _CC2GW { w: self }
    }
    #[doc = "Bit 1 - Capture/compare 1 generation"]
    #[inline]
    pub fn cc1g(&mut self) -> _CC1GW {
        _CC1GW { w: self }
    }
    #[doc = "Bit 0 - Update generation"]
    #[inline]
    pub fn ug(&mut self) -> _UGW {
        _UGW { w: self }
    }
}
