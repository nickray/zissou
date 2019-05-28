#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCMR1_INPUT {
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
pub struct IC2FR {
    bits: u8,
}
impl IC2FR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct IC2PSCR {
    bits: u8,
}
impl IC2PSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CC2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2SR {
    #[doc = "CC2 channel is configured as output"]
    OUTPUT,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"]
    TI2,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"]
    TI1,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC"]
    TRC,
}
impl CC2SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC2SR::OUTPUT => 0,
            CC2SR::TI2 => 1,
            CC2SR::TI1 => 2,
            CC2SR::TRC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC2SR {
        match value {
            0 => CC2SR::OUTPUT,
            1 => CC2SR::TI2,
            2 => CC2SR::TI1,
            3 => CC2SR::TRC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == CC2SR::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TI2`"]
    #[inline]
    pub fn is_ti2(&self) -> bool {
        *self == CC2SR::TI2
    }
    #[doc = "Checks if the value of the field is `TI1`"]
    #[inline]
    pub fn is_ti1(&self) -> bool {
        *self == CC2SR::TI1
    }
    #[doc = "Checks if the value of the field is `TRC`"]
    #[inline]
    pub fn is_trc(&self) -> bool {
        *self == CC2SR::TRC
    }
}
#[doc = "Possible values of the field `IC1F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IC1FR {
    #[doc = "No filter, sampling is done at fDTS"]
    NOFILTER,
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    FCK_INT_N2,
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    FCK_INT_N4,
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    FCK_INT_N8,
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    FDTS_DIV2_N6,
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    FDTS_DIV2_N8,
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    FDTS_DIV4_N6,
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    FDTS_DIV4_N8,
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    FDTS_DIV8_N6,
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    FDTS_DIV8_N8,
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    FDTS_DIV16_N5,
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    FDTS_DIV16_N6,
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    FDTS_DIV16_N8,
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    FDTS_DIV32_N5,
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    FDTS_DIV32_N6,
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    FDTS_DIV32_N8,
}
impl IC1FR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IC1FR::NOFILTER => 0,
            IC1FR::FCK_INT_N2 => 1,
            IC1FR::FCK_INT_N4 => 2,
            IC1FR::FCK_INT_N8 => 3,
            IC1FR::FDTS_DIV2_N6 => 4,
            IC1FR::FDTS_DIV2_N8 => 5,
            IC1FR::FDTS_DIV4_N6 => 6,
            IC1FR::FDTS_DIV4_N8 => 7,
            IC1FR::FDTS_DIV8_N6 => 8,
            IC1FR::FDTS_DIV8_N8 => 9,
            IC1FR::FDTS_DIV16_N5 => 10,
            IC1FR::FDTS_DIV16_N6 => 11,
            IC1FR::FDTS_DIV16_N8 => 12,
            IC1FR::FDTS_DIV32_N5 => 13,
            IC1FR::FDTS_DIV32_N6 => 14,
            IC1FR::FDTS_DIV32_N8 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IC1FR {
        match value {
            0 => IC1FR::NOFILTER,
            1 => IC1FR::FCK_INT_N2,
            2 => IC1FR::FCK_INT_N4,
            3 => IC1FR::FCK_INT_N8,
            4 => IC1FR::FDTS_DIV2_N6,
            5 => IC1FR::FDTS_DIV2_N8,
            6 => IC1FR::FDTS_DIV4_N6,
            7 => IC1FR::FDTS_DIV4_N8,
            8 => IC1FR::FDTS_DIV8_N6,
            9 => IC1FR::FDTS_DIV8_N8,
            10 => IC1FR::FDTS_DIV16_N5,
            11 => IC1FR::FDTS_DIV16_N6,
            12 => IC1FR::FDTS_DIV16_N8,
            13 => IC1FR::FDTS_DIV32_N5,
            14 => IC1FR::FDTS_DIV32_N6,
            15 => IC1FR::FDTS_DIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOFILTER`"]
    #[inline]
    pub fn is_no_filter(&self) -> bool {
        *self == IC1FR::NOFILTER
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N2`"]
    #[inline]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == IC1FR::FCK_INT_N2
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N4`"]
    #[inline]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == IC1FR::FCK_INT_N4
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N8`"]
    #[inline]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == IC1FR::FCK_INT_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV2_N6`"]
    #[inline]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == IC1FR::FDTS_DIV2_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV2_N8`"]
    #[inline]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == IC1FR::FDTS_DIV2_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV4_N6`"]
    #[inline]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == IC1FR::FDTS_DIV4_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV4_N8`"]
    #[inline]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == IC1FR::FDTS_DIV4_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV8_N6`"]
    #[inline]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == IC1FR::FDTS_DIV8_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV8_N8`"]
    #[inline]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == IC1FR::FDTS_DIV8_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N5`"]
    #[inline]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == IC1FR::FDTS_DIV16_N5
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N6`"]
    #[inline]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == IC1FR::FDTS_DIV16_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N8`"]
    #[inline]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == IC1FR::FDTS_DIV16_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N5`"]
    #[inline]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == IC1FR::FDTS_DIV32_N5
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N6`"]
    #[inline]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == IC1FR::FDTS_DIV32_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N8`"]
    #[inline]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == IC1FR::FDTS_DIV32_N8
    }
}
#[doc = r" Value of the field"]
pub struct IC1PSCR {
    bits: u8,
}
impl IC1PSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CC1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1SR {
    #[doc = "CC1 channel is configured as output"]
    OUTPUT,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    TI1,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    TI2,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"]
    TRC,
}
impl CC1SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC1SR::OUTPUT => 0,
            CC1SR::TI1 => 1,
            CC1SR::TI2 => 2,
            CC1SR::TRC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC1SR {
        match value {
            0 => CC1SR::OUTPUT,
            1 => CC1SR::TI1,
            2 => CC1SR::TI2,
            3 => CC1SR::TRC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == CC1SR::OUTPUT
    }
    #[doc = "Checks if the value of the field is `TI1`"]
    #[inline]
    pub fn is_ti1(&self) -> bool {
        *self == CC1SR::TI1
    }
    #[doc = "Checks if the value of the field is `TI2`"]
    #[inline]
    pub fn is_ti2(&self) -> bool {
        *self == CC1SR::TI2
    }
    #[doc = "Checks if the value of the field is `TRC`"]
    #[inline]
    pub fn is_trc(&self) -> bool {
        *self == CC1SR::TRC
    }
}
#[doc = r" Proxy"]
pub struct _IC2FW<'a> {
    w: &'a mut W,
}
impl<'a> _IC2FW<'a> {
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
pub struct _IC2PSCW<'a> {
    w: &'a mut W,
}
impl<'a> _IC2PSCW<'a> {
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
#[doc = "Values that can be written to the field `CC2S`"]
pub enum CC2SW {
    #[doc = "CC2 channel is configured as output"]
    OUTPUT,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"]
    TI2,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"]
    TI1,
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC"]
    TRC,
}
impl CC2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC2SW::OUTPUT => 0,
            CC2SW::TI2 => 1,
            CC2SW::TI1 => 2,
            CC2SW::TRC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2SW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC2 channel is configured as output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(CC2SW::OUTPUT)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"]
    #[inline]
    pub fn ti2(self) -> &'a mut W {
        self.variant(CC2SW::TI2)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"]
    #[inline]
    pub fn ti1(self) -> &'a mut W {
        self.variant(CC2SW::TI1)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC"]
    #[inline]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC2SW::TRC)
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
#[doc = "Values that can be written to the field `IC1F`"]
pub enum IC1FW {
    #[doc = "No filter, sampling is done at fDTS"]
    NOFILTER,
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    FCK_INT_N2,
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    FCK_INT_N4,
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    FCK_INT_N8,
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    FDTS_DIV2_N6,
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    FDTS_DIV2_N8,
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    FDTS_DIV4_N6,
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    FDTS_DIV4_N8,
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    FDTS_DIV8_N6,
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    FDTS_DIV8_N8,
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    FDTS_DIV16_N5,
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    FDTS_DIV16_N6,
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    FDTS_DIV16_N8,
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    FDTS_DIV32_N5,
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    FDTS_DIV32_N6,
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    FDTS_DIV32_N8,
}
impl IC1FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IC1FW::NOFILTER => 0,
            IC1FW::FCK_INT_N2 => 1,
            IC1FW::FCK_INT_N4 => 2,
            IC1FW::FCK_INT_N8 => 3,
            IC1FW::FDTS_DIV2_N6 => 4,
            IC1FW::FDTS_DIV2_N8 => 5,
            IC1FW::FDTS_DIV4_N6 => 6,
            IC1FW::FDTS_DIV4_N8 => 7,
            IC1FW::FDTS_DIV8_N6 => 8,
            IC1FW::FDTS_DIV8_N8 => 9,
            IC1FW::FDTS_DIV16_N5 => 10,
            IC1FW::FDTS_DIV16_N6 => 11,
            IC1FW::FDTS_DIV16_N8 => 12,
            IC1FW::FDTS_DIV32_N5 => 13,
            IC1FW::FDTS_DIV32_N6 => 14,
            IC1FW::FDTS_DIV32_N8 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IC1FW<'a> {
    w: &'a mut W,
}
impl<'a> _IC1FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IC1FW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC1FW::NOFILTER)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(IC1FW::FCK_INT_N2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(IC1FW::FCK_INT_N4)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(IC1FW::FCK_INT_N8)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV2_N6)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV2_N8)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV4_N6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV4_N8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV8_N6)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV8_N8)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV16_N5)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV16_N6)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV16_N8)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV32_N5)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV32_N6)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(IC1FW::FDTS_DIV32_N8)
    }
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
pub struct _IC1PSCW<'a> {
    w: &'a mut W,
}
impl<'a> _IC1PSCW<'a> {
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
#[doc = "Values that can be written to the field `CC1S`"]
pub enum CC1SW {
    #[doc = "CC1 channel is configured as output"]
    OUTPUT,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    TI1,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    TI2,
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"]
    TRC,
}
impl CC1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC1SW::OUTPUT => 0,
            CC1SW::TI1 => 1,
            CC1SW::TI2 => 2,
            CC1SW::TRC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1SW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CC1 channel is configured as output"]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(CC1SW::OUTPUT)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline]
    pub fn ti1(self) -> &'a mut W {
        self.variant(CC1SW::TI1)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    #[inline]
    pub fn ti2(self) -> &'a mut W {
        self.variant(CC1SW::TI2)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"]
    #[inline]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC1SW::TRC)
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
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline]
    pub fn ic2f(&self) -> IC2FR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IC2FR { bits }
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline]
    pub fn ic2psc(&self) -> IC2PSCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IC2PSCR { bits }
    }
    #[doc = "Bits 8:9 - Capture/compare 2 selection"]
    #[inline]
    pub fn cc2s(&self) -> CC2SR {
        CC2SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline]
    pub fn ic1f(&self) -> IC1FR {
        IC1FR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline]
    pub fn ic1psc(&self) -> IC1PSCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        IC1PSCR { bits }
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline]
    pub fn cc1s(&self) -> CC1SR {
        CC1SR::_from({
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
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline]
    pub fn ic2f(&mut self) -> _IC2FW {
        _IC2FW { w: self }
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline]
    pub fn ic2psc(&mut self) -> _IC2PSCW {
        _IC2PSCW { w: self }
    }
    #[doc = "Bits 8:9 - Capture/compare 2 selection"]
    #[inline]
    pub fn cc2s(&mut self) -> _CC2SW {
        _CC2SW { w: self }
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline]
    pub fn ic1f(&mut self) -> _IC1FW {
        _IC1FW { w: self }
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline]
    pub fn ic1psc(&mut self) -> _IC1PSCW {
        _IC1PSCW { w: self }
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline]
    pub fn cc1s(&mut self) -> _CC1SW {
        _CC1SW { w: self }
    }
}
