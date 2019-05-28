#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFCR {
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
#[doc = "Values that can be written to the field `CTEIF7`"]
pub enum CTEIF7W {
    #[doc = "Clears the TEIF flag in the ISR register"]
    CLEAR,
}
impl CTEIF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTEIF7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTEIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7W::CLEAR)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHTIF7`"]
pub enum CHTIF7W {
    #[doc = "Clears the HTIF flag in the ISR register"]
    CLEAR,
}
impl CHTIF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHTIF7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHTIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7W::CLEAR)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTCIF7`"]
pub enum CTCIF7W {
    #[doc = "Clears the TCIF flag in the ISR register"]
    CLEAR,
}
impl CTCIF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CTCIF7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTCIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7W::CLEAR)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CGIF7`"]
pub enum CGIF7W {
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    CLEAR,
}
impl CGIF7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CGIF7W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CGIF7W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF7W::CLEAR)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTEIF6`"]
pub type CTEIF6W = CTEIF7W;
#[doc = r" Proxy"]
pub struct _CTEIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7W::CLEAR)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHTIF6`"]
pub type CHTIF6W = CHTIF7W;
#[doc = r" Proxy"]
pub struct _CHTIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7W::CLEAR)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTCIF6`"]
pub type CTCIF6W = CTCIF7W;
#[doc = r" Proxy"]
pub struct _CTCIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7W::CLEAR)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CGIF6`"]
pub type CGIF6W = CGIF7W;
#[doc = r" Proxy"]
pub struct _CGIF6W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF5`"]
pub type CTEIF5W = CTEIF7W;
#[doc = r" Proxy"]
pub struct _CTEIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7W::CLEAR)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHTIF5`"]
pub type CHTIF5W = CHTIF7W;
#[doc = r" Proxy"]
pub struct _CHTIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7W::CLEAR)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTCIF5`"]
pub type CTCIF5W = CTCIF7W;
#[doc = r" Proxy"]
pub struct _CTCIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CGIF5`"]
pub type CGIF5W = CGIF7W;
#[doc = r" Proxy"]
pub struct _CGIF5W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF7W::CLEAR)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTEIF4`"]
pub type CTEIF4W = CTEIF7W;
#[doc = r" Proxy"]
pub struct _CTEIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF4`"]
pub type CHTIF4W = CHTIF7W;
#[doc = r" Proxy"]
pub struct _CHTIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7W::CLEAR)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTCIF4`"]
pub type CTCIF4W = CTCIF7W;
#[doc = r" Proxy"]
pub struct _CTCIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CGIF4`"]
pub type CGIF4W = CGIF7W;
#[doc = r" Proxy"]
pub struct _CGIF4W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF3`"]
pub type CTEIF3W = CTEIF7W;
#[doc = r" Proxy"]
pub struct _CTEIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF3`"]
pub type CHTIF3W = CHTIF7W;
#[doc = r" Proxy"]
pub struct _CHTIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CTCIF3`"]
pub type CTCIF3W = CTCIF7W;
#[doc = r" Proxy"]
pub struct _CTCIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CGIF3`"]
pub type CGIF3W = CGIF7W;
#[doc = r" Proxy"]
pub struct _CGIF3W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF2`"]
pub type CTEIF2W = CTEIF7W;
#[doc = r" Proxy"]
pub struct _CTEIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7W::CLEAR)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHTIF2`"]
pub type CHTIF2W = CHTIF7W;
#[doc = r" Proxy"]
pub struct _CHTIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CTCIF2`"]
pub type CTCIF2W = CTCIF7W;
#[doc = r" Proxy"]
pub struct _CTCIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CGIF2`"]
pub type CGIF2W = CGIF7W;
#[doc = r" Proxy"]
pub struct _CGIF2W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CTEIF1`"]
pub type CTEIF1W = CTEIF7W;
#[doc = r" Proxy"]
pub struct _CTEIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTEIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTEIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TEIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CHTIF1`"]
pub type CHTIF1W = CHTIF7W;
#[doc = r" Proxy"]
pub struct _CHTIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHTIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHTIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the HTIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CTCIF1`"]
pub type CTCIF1W = CTCIF7W;
#[doc = r" Proxy"]
pub struct _CTCIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CTCIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the TCIF flag in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF7W::CLEAR)
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
#[doc = "Values that can be written to the field `CGIF1`"]
pub type CGIF1W = CGIF7W;
#[doc = r" Proxy"]
pub struct _CGIF1W<'a> {
    w: &'a mut W,
}
impl<'a> _CGIF1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGIF1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF7W::CLEAR)
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
    #[doc = "Bit 27 - Channel x transfer error clear (x = 1 ..7)"]
    #[inline]
    pub fn cteif7(&mut self) -> _CTEIF7W {
        _CTEIF7W { w: self }
    }
    #[doc = "Bit 26 - Channel x half transfer clear (x = 1 ..7)"]
    #[inline]
    pub fn chtif7(&mut self) -> _CHTIF7W {
        _CHTIF7W { w: self }
    }
    #[doc = "Bit 25 - Channel x transfer complete clear (x = 1 ..7)"]
    #[inline]
    pub fn ctcif7(&mut self) -> _CTCIF7W {
        _CTCIF7W { w: self }
    }
    #[doc = "Bit 24 - Channel x global interrupt clear (x = 1 ..7)"]
    #[inline]
    pub fn cgif7(&mut self) -> _CGIF7W {
        _CGIF7W { w: self }
    }
    #[doc = "Bit 23 - Channel x transfer error clear (x = 1 ..7)"]
    #[inline]
    pub fn cteif6(&mut self) -> _CTEIF6W {
        _CTEIF6W { w: self }
    }
    #[doc = "Bit 22 - Channel x half transfer clear (x = 1 ..7)"]
    #[inline]
    pub fn chtif6(&mut self) -> _CHTIF6W {
        _CHTIF6W { w: self }
    }
    #[doc = "Bit 21 - Channel x transfer complete clear (x = 1 ..7)"]
    #[inline]
    pub fn ctcif6(&mut self) -> _CTCIF6W {
        _CTCIF6W { w: self }
    }
    #[doc = "Bit 20 - Channel x global interrupt clear (x = 1 ..7)"]
    #[inline]
    pub fn cgif6(&mut self) -> _CGIF6W {
        _CGIF6W { w: self }
    }
    #[doc = "Bit 19 - Channel x transfer error clear (x = 1 ..7)"]
    #[inline]
    pub fn cteif5(&mut self) -> _CTEIF5W {
        _CTEIF5W { w: self }
    }
    #[doc = "Bit 18 - Channel x half transfer clear (x = 1 ..7)"]
    #[inline]
    pub fn chtif5(&mut self) -> _CHTIF5W {
        _CHTIF5W { w: self }
    }
    #[doc = "Bit 17 - Channel x transfer complete clear (x = 1 ..7)"]
    #[inline]
    pub fn ctcif5(&mut self) -> _CTCIF5W {
        _CTCIF5W { w: self }
    }
    #[doc = "Bit 16 - Channel x global interrupt clear (x = 1 ..7)"]
    #[inline]
    pub fn cgif5(&mut self) -> _CGIF5W {
        _CGIF5W { w: self }
    }
    #[doc = "Bit 15 - Channel x transfer error clear (x = 1 ..7)"]
    #[inline]
    pub fn cteif4(&mut self) -> _CTEIF4W {
        _CTEIF4W { w: self }
    }
    #[doc = "Bit 14 - Channel x half transfer clear (x = 1 ..7)"]
    #[inline]
    pub fn chtif4(&mut self) -> _CHTIF4W {
        _CHTIF4W { w: self }
    }
    #[doc = "Bit 13 - Channel x transfer complete clear (x = 1 ..7)"]
    #[inline]
    pub fn ctcif4(&mut self) -> _CTCIF4W {
        _CTCIF4W { w: self }
    }
    #[doc = "Bit 12 - Channel x global interrupt clear (x = 1 ..7)"]
    #[inline]
    pub fn cgif4(&mut self) -> _CGIF4W {
        _CGIF4W { w: self }
    }
    #[doc = "Bit 11 - Channel x transfer error clear (x = 1 ..7)"]
    #[inline]
    pub fn cteif3(&mut self) -> _CTEIF3W {
        _CTEIF3W { w: self }
    }
    #[doc = "Bit 10 - Channel x half transfer clear (x = 1 ..7)"]
    #[inline]
    pub fn chtif3(&mut self) -> _CHTIF3W {
        _CHTIF3W { w: self }
    }
    #[doc = "Bit 9 - Channel x transfer complete clear (x = 1 ..7)"]
    #[inline]
    pub fn ctcif3(&mut self) -> _CTCIF3W {
        _CTCIF3W { w: self }
    }
    #[doc = "Bit 8 - Channel x global interrupt clear (x = 1 ..7)"]
    #[inline]
    pub fn cgif3(&mut self) -> _CGIF3W {
        _CGIF3W { w: self }
    }
    #[doc = "Bit 7 - Channel x transfer error clear (x = 1 ..7)"]
    #[inline]
    pub fn cteif2(&mut self) -> _CTEIF2W {
        _CTEIF2W { w: self }
    }
    #[doc = "Bit 6 - Channel x half transfer clear (x = 1 ..7)"]
    #[inline]
    pub fn chtif2(&mut self) -> _CHTIF2W {
        _CHTIF2W { w: self }
    }
    #[doc = "Bit 5 - Channel x transfer complete clear (x = 1 ..7)"]
    #[inline]
    pub fn ctcif2(&mut self) -> _CTCIF2W {
        _CTCIF2W { w: self }
    }
    #[doc = "Bit 4 - Channel x global interrupt clear (x = 1 ..7)"]
    #[inline]
    pub fn cgif2(&mut self) -> _CGIF2W {
        _CGIF2W { w: self }
    }
    #[doc = "Bit 3 - Channel x transfer error clear (x = 1 ..7)"]
    #[inline]
    pub fn cteif1(&mut self) -> _CTEIF1W {
        _CTEIF1W { w: self }
    }
    #[doc = "Bit 2 - Channel x half transfer clear (x = 1 ..7)"]
    #[inline]
    pub fn chtif1(&mut self) -> _CHTIF1W {
        _CHTIF1W { w: self }
    }
    #[doc = "Bit 1 - Channel x transfer complete clear (x = 1 ..7)"]
    #[inline]
    pub fn ctcif1(&mut self) -> _CTCIF1W {
        _CTCIF1W { w: self }
    }
    #[doc = "Bit 0 - Channel x global interrupt clear (x = 1 ..7)"]
    #[inline]
    pub fn cgif1(&mut self) -> _CGIF1W {
        _CGIF1W { w: self }
    }
}
