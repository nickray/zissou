#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCMR2_INPUT {
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
pub struct IC4FR {
    bits: u8,
}
impl IC4FR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IC4PSCR {
    bits: u8,
}
impl IC4PSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CC4S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC4SR {
    #[doc = "CC4 channel is configured as output"]
    OUTPUT,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    TI4,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    TI3,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC"]
    TRC,
}
impl CC4SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC4SR::OUTPUT => 0,
            CC4SR::TI4 => 1,
            CC4SR::TI3 => 2,
            CC4SR::TRC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC4SR {
        match value {
            0 => CC4SR::OUTPUT,
            1 => CC4SR::TI4,
            2 => CC4SR::TI3,
            3 => CC4SR::TRC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == CC4SR::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TI4`"]
    #[inline]
    pub fn is_ti4(&self) -> bool {
        *self == CC4SR::TI4
    }
    #[doc = "Checks if the value of the field is `TI3`"]
    #[inline]
    pub fn is_ti3(&self) -> bool {
        *self == CC4SR::TI3
    }
    #[doc = "Checks if the value of the field is `TRC`"]
    #[inline]
    pub fn is_trc(&self) -> bool {
        *self == CC4SR::TRC
    }
}
#[doc = r" Value of the field"]
pub struct IC3FR {
    bits: u8,
}
impl IC3FR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IC3PSCR {
    bits: u8,
}
impl IC3PSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CC3S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3SR {
    #[doc = "CC3 channel is configured as output"]
    OUTPUT,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    TI3,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    TI4,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC"]
    TRC,
}
impl CC3SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC3SR::OUTPUT => 0,
            CC3SR::TI3 => 1,
            CC3SR::TI4 => 2,
            CC3SR::TRC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC3SR {
        match value {
            0 => CC3SR::OUTPUT,
            1 => CC3SR::TI3,
            2 => CC3SR::TI4,
            3 => CC3SR::TRC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == CC3SR::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TI3`"]
    #[inline]
    pub fn is_ti3(&self) -> bool {
        *self == CC3SR::TI3
    }
    #[doc = "Checks if the value of the field is `TI4`"]
    #[inline]
    pub fn is_ti4(&self) -> bool {
        *self == CC3SR::TI4
    }
    #[doc = "Checks if the value of the field is `TRC`"]
    #[inline]
    pub fn is_trc(&self) -> bool {
        *self == CC3SR::TRC
    }
}
#[doc = r" Proxy"]
pub struct _IC4FW<'a> {
    w: &'a mut W,
}
impl<'a> _IC4FW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IC4PSCW<'a> {
    w: &'a mut W,
}
impl<'a> _IC4PSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC4S`"]
pub enum CC4SW {
    #[doc = "CC4 channel is configured as output"]
    OUTPUT,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    TI4,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    TI3,
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC"]
    TRC,
}
impl CC4SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC4SW::OUTPUT => 0,
            CC4SW::TI4 => 1,
            CC4SW::TI3 => 2,
            CC4SW::TRC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC4SW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC4SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC4 channel is configured as output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(CC4SW::OUTPUT)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI4"]
    #[inline]
    pub fn ti4(self) -> &'a mut W {
        self.variant(CC4SW::TI4)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TI3"]
    #[inline]
    pub fn ti3(self) -> &'a mut W {
        self.variant(CC4SW::TI3)
    }
    #[doc = "CC4 channel is configured as input, IC4 is mapped on TRC"]
    #[inline]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC4SW::TRC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IC3FW<'a> {
    w: &'a mut W,
}
impl<'a> _IC3FW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IC3PSCW<'a> {
    w: &'a mut W,
}
impl<'a> _IC3PSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CC3S`"]
pub enum CC3SW {
    #[doc = "CC3 channel is configured as output"]
    OUTPUT,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    TI3,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    TI4,
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC"]
    TRC,
}
impl CC3SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC3SW::OUTPUT => 0,
            CC3SW::TI3 => 1,
            CC3SW::TI4 => 2,
            CC3SW::TRC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3SW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC3 channel is configured as output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(CC3SW::OUTPUT)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI3"]
    #[inline]
    pub fn ti3(self) -> &'a mut W {
        self.variant(CC3SW::TI3)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TI4"]
    #[inline]
    pub fn ti4(self) -> &'a mut W {
        self.variant(CC3SW::TI4)
    }
    #[doc = "CC3 channel is configured as input, IC3 is mapped on TRC"]
    #[inline]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC3SW::TRC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline]
    pub fn ic4f(&self) -> IC4FR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IC4FR { bits }
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline]
    pub fn ic4psc(&self) -> IC4PSCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IC4PSCR { bits }
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline]
    pub fn cc4s(&self) -> CC4SR {
        CC4SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline]
    pub fn ic3f(&self) -> IC3FR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IC3FR { bits }
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline]
    pub fn ic3psc(&self) -> IC3PSCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IC3PSCR { bits }
    }
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline]
    pub fn cc3s(&self) -> CC3SR {
        CC3SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 12:15 - Input capture 4 filter"]
    #[inline]
    pub fn ic4f(&mut self) -> _IC4FW {
        _IC4FW { w: self }
    }
    #[doc = "Bits 10:11 - Input capture 4 prescaler"]
    #[inline]
    pub fn ic4psc(&mut self) -> _IC4PSCW {
        _IC4PSCW { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline]
    pub fn cc4s(&mut self) -> _CC4SW {
        _CC4SW { w: self }
    }
    #[doc = "Bits 4:7 - Input capture 3 filter"]
    #[inline]
    pub fn ic3f(&mut self) -> _IC3FW {
        _IC3FW { w: self }
    }
    #[doc = "Bits 2:3 - Input capture 3 prescaler"]
    #[inline]
    pub fn ic3psc(&mut self) -> _IC3PSCW {
        _IC3PSCW { w: self }
    }
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline]
    pub fn cc3s(&mut self) -> _CC3SW {
        _CC3SW { w: self }
    }
}
