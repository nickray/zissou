#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::KR {
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
#[doc = "Values that can be written to the field `KEY`"]
pub enum KEYW {
    #[doc = "Enable access to PR, RLR and WINR registers (0x5555)"]
    ENABLE,
    #[doc = "Reset the watchdog value (0xAAAA)"]
    RESET,
    #[doc = "Start the watchdog (0xCCCC)"]
    START,
}
impl KEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            KEYW::ENABLE => 21845,
            KEYW::RESET => 43690,
            KEYW::START => 52428,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Enable access to PR, RLR and WINR registers (0x5555)"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(KEYW::ENABLE)
    }
    #[doc = "Reset the watchdog value (0xAAAA)"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(KEYW::RESET)
    }
    #[doc = "Start the watchdog (0xCCCC)"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(KEYW::START)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:15 - Key value (write only, read 0x0000)"]
    #[inline]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
}
