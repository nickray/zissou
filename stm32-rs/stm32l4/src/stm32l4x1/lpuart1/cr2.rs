#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR2 {
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
#[doc = "Possible values of the field `MSBFIRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBFIRSTR {
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit"]
    LSB,
    #[doc = "data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    MSB,
}
impl MSBFIRSTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MSBFIRSTR::LSB => false,
            MSBFIRSTR::MSB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSBFIRSTR {
        match value {
            false => MSBFIRSTR::LSB,
            true => MSBFIRSTR::MSB,
        }
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline]
    pub fn is_lsb(&self) -> bool {
        *self == MSBFIRSTR::LSB
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline]
    pub fn is_msb(&self) -> bool {
        *self == MSBFIRSTR::MSB
    }
}
#[doc = "Possible values of the field `DATAINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATAINVR {
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    POSITIVE,
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    NEGATIVE,
}
impl DATAINVR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DATAINVR::POSITIVE => false,
            DATAINVR::NEGATIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATAINVR {
        match value {
            false => DATAINVR::POSITIVE,
            true => DATAINVR::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline]
    pub fn is_positive(&self) -> bool {
        *self == DATAINVR::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline]
    pub fn is_negative(&self) -> bool {
        *self == DATAINVR::NEGATIVE
    }
}
#[doc = "Possible values of the field `TXINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXINVR {
    #[doc = "TX pin signal works using the standard logic levels"]
    STANDARD,
    #[doc = "TX pin signal values are inverted"]
    INVERTED,
}
impl TXINVR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TXINVR::STANDARD => false,
            TXINVR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXINVR {
        match value {
            false => TXINVR::STANDARD,
            true => TXINVR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == TXINVR::STANDARD
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == TXINVR::INVERTED
    }
}
#[doc = "Possible values of the field `RXINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINVR {
    #[doc = "RX pin signal works using the standard logic levels"]
    STANDARD,
    #[doc = "RX pin signal values are inverted"]
    INVERTED,
}
impl RXINVR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RXINVR::STANDARD => false,
            RXINVR::INVERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXINVR {
        match value {
            false => RXINVR::STANDARD,
            true => RXINVR::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == RXINVR::STANDARD
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline]
    pub fn is_inverted(&self) -> bool {
        *self == RXINVR::INVERTED
    }
}
#[doc = "Possible values of the field `SWAP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWAPR {
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    STANDARD,
    #[doc = "The TX and RX pins functions are swapped"]
    SWAPPED,
}
impl SWAPR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SWAPR::STANDARD => false,
            SWAPR::SWAPPED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWAPR {
        match value {
            false => SWAPR::STANDARD,
            true => SWAPR::SWAPPED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline]
    pub fn is_standard(&self) -> bool {
        *self == SWAPR::STANDARD
    }
    #[doc = "Checks if the value of the field is `SWAPPED`"]
    #[inline]
    pub fn is_swapped(&self) -> bool {
        *self == SWAPR::SWAPPED
    }
}
#[doc = "Possible values of the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPR {
    #[doc = "1 stop bit"]
    STOP1,
    #[doc = "0.5 stop bit"]
    STOP0P5,
    #[doc = "2 stop bit"]
    STOP2,
    #[doc = "1.5 stop bit"]
    STOP1P5,
}
impl STOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STOPR::STOP1 => 0,
            STOPR::STOP0P5 => 1,
            STOPR::STOP2 => 2,
            STOPR::STOP1P5 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STOPR {
        match value {
            0 => STOPR::STOP1,
            1 => STOPR::STOP0P5,
            2 => STOPR::STOP2,
            3 => STOPR::STOP1P5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STOP1`"]
    #[inline]
    pub fn is_stop1(&self) -> bool {
        *self == STOPR::STOP1
    }
    #[doc = "Checks if the value of the field is `STOP0P5`"]
    #[inline]
    pub fn is_stop0p5(&self) -> bool {
        *self == STOPR::STOP0P5
    }
    #[doc = "Checks if the value of the field is `STOP2`"]
    #[inline]
    pub fn is_stop2(&self) -> bool {
        *self == STOPR::STOP2
    }
    #[doc = "Checks if the value of the field is `STOP1P5`"]
    #[inline]
    pub fn is_stop1p5(&self) -> bool {
        *self == STOPR::STOP1P5
    }
}
#[doc = "Possible values of the field `CLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKENR {
    #[doc = "CK pin disabled"]
    DISABLED,
    #[doc = "CK pin enabled"]
    ENABLED,
}
impl CLKENR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CLKENR::DISABLED => false,
            CLKENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKENR {
        match value {
            false => CLKENR::DISABLED,
            true => CLKENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CLKENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CLKENR::ENABLED
    }
}
#[doc = "Possible values of the field `ADDM7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDM7R {
    #[doc = "4-bit address detection"]
    BIT4,
    #[doc = "7-bit address detection"]
    BIT7,
}
impl ADDM7R {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ADDM7R::BIT4 => false,
            ADDM7R::BIT7 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDM7R {
        match value {
            false => ADDM7R::BIT4,
            true => ADDM7R::BIT7,
        }
    }
    #[doc = "Checks if the value of the field is `BIT4`"]
    #[inline]
    pub fn is_bit4(&self) -> bool {
        *self == ADDM7R::BIT4
    }
    #[doc = "Checks if the value of the field is `BIT7`"]
    #[inline]
    pub fn is_bit7(&self) -> bool {
        *self == ADDM7R::BIT7
    }
}
#[doc = r" Value of the field"]
pub struct ADDR {
    bits: u8,
}
impl ADDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MSBFIRST`"]
pub enum MSBFIRSTW {
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit"]
    LSB,
    #[doc = "data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    MSB,
}
impl MSBFIRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSBFIRSTW::LSB => false,
            MSBFIRSTW::MSB => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSBFIRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBFIRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSBFIRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "data is transmitted/received with data bit 0 first, following the start bit"]
    #[inline]
    pub fn lsb(self) -> &'a mut W {
        self.variant(MSBFIRSTW::LSB)
    }
    #[doc = "data is transmitted/received with MSB (bit 7/8/9) first, following the start bit"]
    #[inline]
    pub fn msb(self) -> &'a mut W {
        self.variant(MSBFIRSTW::MSB)
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
#[doc = "Values that can be written to the field `DATAINV`"]
pub enum DATAINVW {
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    POSITIVE,
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    NEGATIVE,
}
impl DATAINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATAINVW::POSITIVE => false,
            DATAINVW::NEGATIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATAINVW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATAINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Logical data from the data register are send/received in positive/direct logic"]
    #[inline]
    pub fn positive(self) -> &'a mut W {
        self.variant(DATAINVW::POSITIVE)
    }
    #[doc = "Logical data from the data register are send/received in negative/inverse logic"]
    #[inline]
    pub fn negative(self) -> &'a mut W {
        self.variant(DATAINVW::NEGATIVE)
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
#[doc = "Values that can be written to the field `TXINV`"]
pub enum TXINVW {
    #[doc = "TX pin signal works using the standard logic levels"]
    STANDARD,
    #[doc = "TX pin signal values are inverted"]
    INVERTED,
}
impl TXINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXINVW::STANDARD => false,
            TXINVW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TXINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX pin signal works using the standard logic levels"]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(TXINVW::STANDARD)
    }
    #[doc = "TX pin signal values are inverted"]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TXINVW::INVERTED)
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
#[doc = "Values that can be written to the field `RXINV`"]
pub enum RXINVW {
    #[doc = "RX pin signal works using the standard logic levels"]
    STANDARD,
    #[doc = "RX pin signal values are inverted"]
    INVERTED,
}
impl RXINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXINVW::STANDARD => false,
            RXINVW::INVERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _RXINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RX pin signal works using the standard logic levels"]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(RXINVW::STANDARD)
    }
    #[doc = "RX pin signal values are inverted"]
    #[inline]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RXINVW::INVERTED)
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
#[doc = "Values that can be written to the field `SWAP`"]
pub enum SWAPW {
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    STANDARD,
    #[doc = "The TX and RX pins functions are swapped"]
    SWAPPED,
}
impl SWAPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWAPW::STANDARD => false,
            SWAPW::SWAPPED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWAPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TX/RX pins are used as defined in standard pinout"]
    #[inline]
    pub fn standard(self) -> &'a mut W {
        self.variant(SWAPW::STANDARD)
    }
    #[doc = "The TX and RX pins functions are swapped"]
    #[inline]
    pub fn swapped(self) -> &'a mut W {
        self.variant(SWAPW::SWAPPED)
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
#[doc = "Values that can be written to the field `STOP`"]
pub enum STOPW {
    #[doc = "1 stop bit"]
    STOP1,
    #[doc = "0.5 stop bit"]
    STOP0P5,
    #[doc = "2 stop bit"]
    STOP2,
    #[doc = "1.5 stop bit"]
    STOP1P5,
}
impl STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STOPW::STOP1 => 0,
            STOPW::STOP0P5 => 1,
            STOPW::STOP2 => 2,
            STOPW::STOP1P5 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 stop bit"]
    #[inline]
    pub fn stop1(self) -> &'a mut W {
        self.variant(STOPW::STOP1)
    }
    #[doc = "0.5 stop bit"]
    #[inline]
    pub fn stop0p5(self) -> &'a mut W {
        self.variant(STOPW::STOP0P5)
    }
    #[doc = "2 stop bit"]
    #[inline]
    pub fn stop2(self) -> &'a mut W {
        self.variant(STOPW::STOP2)
    }
    #[doc = "1.5 stop bit"]
    #[inline]
    pub fn stop1p5(self) -> &'a mut W {
        self.variant(STOPW::STOP1P5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKEN`"]
pub enum CLKENW {
    #[doc = "CK pin disabled"]
    DISABLED,
    #[doc = "CK pin enabled"]
    ENABLED,
}
impl CLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKENW::DISABLED => false,
            CLKENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CK pin disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKENW::DISABLED)
    }
    #[doc = "CK pin enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKENW::ENABLED)
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
#[doc = "Values that can be written to the field `ADDM7`"]
pub enum ADDM7W {
    #[doc = "4-bit address detection"]
    BIT4,
    #[doc = "7-bit address detection"]
    BIT7,
}
impl ADDM7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDM7W::BIT4 => false,
            ADDM7W::BIT7 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDM7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADDM7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDM7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "4-bit address detection"]
    #[inline]
    pub fn bit4(self) -> &'a mut W {
        self.variant(ADDM7W::BIT4)
    }
    #[doc = "7-bit address detection"]
    #[inline]
    pub fn bit7(self) -> &'a mut W {
        self.variant(ADDM7W::BIT7)
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
#[doc = r" Proxy"]
pub struct _ADDW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline]
    pub fn msbfirst(&self) -> MSBFIRSTR {
        MSBFIRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline]
    pub fn datainv(&self) -> DATAINVR {
        DATAINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline]
    pub fn txinv(&self) -> TXINVR {
        TXINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline]
    pub fn rxinv(&self) -> RXINVR {
        RXINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline]
    pub fn swap(&self) -> SWAPR {
        SWAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline]
    pub fn stop(&self) -> STOPR {
        STOPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline]
    pub fn clken(&self) -> CLKENR {
        CLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline]
    pub fn addm7(&self) -> ADDM7R {
        ADDM7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline]
    pub fn add(&self) -> ADDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDR { bits }
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
    #[doc = "Bit 19 - Most significant bit first"]
    #[inline]
    pub fn msbfirst(&mut self) -> _MSBFIRSTW {
        _MSBFIRSTW { w: self }
    }
    #[doc = "Bit 18 - Binary data inversion"]
    #[inline]
    pub fn datainv(&mut self) -> _DATAINVW {
        _DATAINVW { w: self }
    }
    #[doc = "Bit 17 - TX pin active level inversion"]
    #[inline]
    pub fn txinv(&mut self) -> _TXINVW {
        _TXINVW { w: self }
    }
    #[doc = "Bit 16 - RX pin active level inversion"]
    #[inline]
    pub fn rxinv(&mut self) -> _RXINVW {
        _RXINVW { w: self }
    }
    #[doc = "Bit 15 - Swap TX/RX pins"]
    #[inline]
    pub fn swap(&mut self) -> _SWAPW {
        _SWAPW { w: self }
    }
    #[doc = "Bits 12:13 - STOP bits"]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline]
    pub fn clken(&mut self) -> _CLKENW {
        _CLKENW { w: self }
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection"]
    #[inline]
    pub fn addm7(&mut self) -> _ADDM7W {
        _ADDM7W { w: self }
    }
    #[doc = "Bits 24:31 - Address of the USART node"]
    #[inline]
    pub fn add(&mut self) -> _ADDW {
        _ADDW { w: self }
    }
}
