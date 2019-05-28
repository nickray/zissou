#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISR {
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
pub struct ADDCODER {
    bits: u8,
}
impl ADDCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Write transfer, slave enters receiver mode"]
    WRITE,
    #[doc = "Read transfer, slave enters transmitter mode"]
    READ,
}
impl DIRR {
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
            DIRR::WRITE => false,
            DIRR::READ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRR {
        match value {
            false => DIRR::WRITE,
            true => DIRR::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline]
    pub fn is_write(&self) -> bool {
        *self == DIRR::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == DIRR::READ
    }
}
#[doc = "Possible values of the field `BUSY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSYR {
    #[doc = "No communication is in progress on the bus"]
    NOTBUSY,
    #[doc = "A communication is in progress on the bus"]
    BUSY,
}
impl BUSYR {
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
            BUSYR::NOTBUSY => false,
            BUSYR::BUSY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUSYR {
        match value {
            false => BUSYR::NOTBUSY,
            true => BUSYR::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBUSY`"]
    #[inline]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSYR::NOTBUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == BUSYR::BUSY
    }
}
#[doc = "Possible values of the field `ALERT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTR {
    #[doc = "SMBA alert is not detected"]
    NOALERT,
    #[doc = "SMBA alert event is detected on SMBA pin"]
    ALERT,
}
impl ALERTR {
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
            ALERTR::NOALERT => false,
            ALERTR::ALERT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALERTR {
        match value {
            false => ALERTR::NOALERT,
            true => ALERTR::ALERT,
        }
    }
    #[doc = "Checks if the value of the field is `NOALERT`"]
    #[inline]
    pub fn is_no_alert(&self) -> bool {
        *self == ALERTR::NOALERT
    }
    #[doc = "Checks if the value of the field is `ALERT`"]
    #[inline]
    pub fn is_alert(&self) -> bool {
        *self == ALERTR::ALERT
    }
}
#[doc = "Possible values of the field `TIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTR {
    #[doc = "No timeout occured"]
    NOTIMEOUT,
    #[doc = "Timeout occured"]
    TIMEOUT,
}
impl TIMEOUTR {
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
            TIMEOUTR::NOTIMEOUT => false,
            TIMEOUTR::TIMEOUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMEOUTR {
        match value {
            false => TIMEOUTR::NOTIMEOUT,
            true => TIMEOUTR::TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTIMEOUT`"]
    #[inline]
    pub fn is_no_timeout(&self) -> bool {
        *self == TIMEOUTR::NOTIMEOUT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline]
    pub fn is_timeout(&self) -> bool {
        *self == TIMEOUTR::TIMEOUT
    }
}
#[doc = "Possible values of the field `PECERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECERRR {
    #[doc = "Received PEC does match with PEC register"]
    MATCH,
    #[doc = "Received PEC does not match with PEC register"]
    NOMATCH,
}
impl PECERRR {
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
            PECERRR::MATCH => false,
            PECERRR::NOMATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PECERRR {
        match value {
            false => PECERRR::MATCH,
            true => PECERRR::NOMATCH,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == PECERRR::MATCH
    }
    #[doc = "Checks if the value of the field is `NOMATCH`"]
    #[inline]
    pub fn is_no_match(&self) -> bool {
        *self == PECERRR::NOMATCH
    }
}
#[doc = "Possible values of the field `OVR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRR {
    #[doc = "No overrun/underrun error occurs"]
    NOOVERRUN,
    #[doc = "slave mode with NOSTRETCH=1, when an overrun/underrun error occurs"]
    OVERRUN,
}
impl OVRR {
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
            OVRR::NOOVERRUN => false,
            OVRR::OVERRUN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVRR {
        match value {
            false => OVRR::NOOVERRUN,
            true => OVRR::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR::OVERRUN
    }
}
#[doc = "Possible values of the field `ARLO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARLOR {
    #[doc = "No arbitration lost"]
    NOTLOST,
    #[doc = "Arbitration lost"]
    LOST,
}
impl ARLOR {
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
            ARLOR::NOTLOST => false,
            ARLOR::LOST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARLOR {
        match value {
            false => ARLOR::NOTLOST,
            true => ARLOR::LOST,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLOST`"]
    #[inline]
    pub fn is_not_lost(&self) -> bool {
        *self == ARLOR::NOTLOST
    }
    #[doc = "Checks if the value of the field is `LOST`"]
    #[inline]
    pub fn is_lost(&self) -> bool {
        *self == ARLOR::LOST
    }
}
#[doc = "Possible values of the field `BERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BERRR {
    #[doc = "No bus error"]
    NOERROR,
    #[doc = "Misplaced Start and Stop condition is detected"]
    ERROR,
}
impl BERRR {
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
            BERRR::NOERROR => false,
            BERRR::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BERRR {
        match value {
            false => BERRR::NOERROR,
            true => BERRR::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == BERRR::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == BERRR::ERROR
    }
}
#[doc = "Possible values of the field `TCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRR {
    #[doc = "Transfer is not complete"]
    NOTCOMPLETE,
    #[doc = "NBYTES has been transfered"]
    COMPLETE,
}
impl TCRR {
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
            TCRR::NOTCOMPLETE => false,
            TCRR::COMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCRR {
        match value {
            false => TCRR::NOTCOMPLETE,
            true => TCRR::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == TCRR::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == TCRR::COMPLETE
    }
}
#[doc = "Possible values of the field `TC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCR {
    #[doc = "Transfer is not complete"]
    NOTCOMPLETE,
    #[doc = "NBYTES has been transfered"]
    COMPLETE,
}
impl TCR {
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
            TCR::NOTCOMPLETE => false,
            TCR::COMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCR {
        match value {
            false => TCR::NOTCOMPLETE,
            true => TCR::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == TCR::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == TCR::COMPLETE
    }
}
#[doc = "Possible values of the field `STOPF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPFR {
    #[doc = "No Stop condition detected"]
    NOSTOP,
    #[doc = "Stop condition detected"]
    STOP,
}
impl STOPFR {
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
            STOPFR::NOSTOP => false,
            STOPFR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPFR {
        match value {
            false => STOPFR::NOSTOP,
            true => STOPFR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTOP`"]
    #[inline]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPFR::NOSTOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == STOPFR::STOP
    }
}
#[doc = "Possible values of the field `NACKF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKFR {
    #[doc = "No NACK has been received"]
    NONACK,
    #[doc = "NACK has been received"]
    NACK,
}
impl NACKFR {
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
            NACKFR::NONACK => false,
            NACKFR::NACK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NACKFR {
        match value {
            false => NACKFR::NONACK,
            true => NACKFR::NACK,
        }
    }
    #[doc = "Checks if the value of the field is `NONACK`"]
    #[inline]
    pub fn is_no_nack(&self) -> bool {
        *self == NACKFR::NONACK
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline]
    pub fn is_nack(&self) -> bool {
        *self == NACKFR::NACK
    }
}
#[doc = "Possible values of the field `ADDR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRR {
    #[doc = "Adress mismatched or not received"]
    NOTMATCH,
    #[doc = "Received slave address matched with one of the enabled slave addresses"]
    MATCH,
}
impl ADDRR {
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
            ADDRR::NOTMATCH => false,
            ADDRR::MATCH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRR {
        match value {
            false => ADDRR::NOTMATCH,
            true => ADDRR::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NOTMATCH`"]
    #[inline]
    pub fn is_not_match(&self) -> bool {
        *self == ADDRR::NOTMATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline]
    pub fn is_match_(&self) -> bool {
        *self == ADDRR::MATCH
    }
}
#[doc = "Possible values of the field `RXNE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXNER {
    #[doc = "The RXDR register is empty"]
    EMPTY,
    #[doc = "Received data is copied into the RXDR register, and is ready to be read"]
    NOTEMPTY,
}
impl RXNER {
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
            RXNER::EMPTY => false,
            RXNER::NOTEMPTY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXNER {
        match value {
            false => RXNER::EMPTY,
            true => RXNER::NOTEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == RXNER::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNER::NOTEMPTY
    }
}
#[doc = "Possible values of the field `TXIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXISR {
    #[doc = "The TXDR register is not empty"]
    NOTEMPTY,
    #[doc = "The TXDR register is empty and the data to be transmitted must be written in the TXDR register"]
    EMPTY,
}
impl TXISR {
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
            TXISR::NOTEMPTY => false,
            TXISR::EMPTY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXISR {
        match value {
            false => TXISR::NOTEMPTY,
            true => TXISR::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline]
    pub fn is_not_empty(&self) -> bool {
        *self == TXISR::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == TXISR::EMPTY
    }
}
#[doc = "Possible values of the field `TXE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXER {
    #[doc = "TXDR register not empty"]
    NOTEMPTY,
    #[doc = "TXDR register empty"]
    EMPTY,
}
impl TXER {
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
            TXER::NOTEMPTY => false,
            TXER::EMPTY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXER {
        match value {
            false => TXER::NOTEMPTY,
            true => TXER::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline]
    pub fn is_not_empty(&self) -> bool {
        *self == TXER::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline]
    pub fn is_empty(&self) -> bool {
        *self == TXER::EMPTY
    }
}
#[doc = "Values that can be written to the field `TXIS`"]
pub enum TXISW {
    #[doc = "The TXDR register is not empty"]
    NOTEMPTY,
    #[doc = "The TXDR register is empty and the data to be transmitted must be written in the TXDR register"]
    EMPTY,
}
impl TXISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXISW::NOTEMPTY => false,
            TXISW::EMPTY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXISW<'a> {
    w: &'a mut W,
}
impl<'a> _TXISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The TXDR register is not empty"]
    #[inline]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TXISW::NOTEMPTY)
    }
    #[doc = "The TXDR register is empty and the data to be transmitted must be written in the TXDR register"]
    #[inline]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXISW::EMPTY)
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
#[doc = "Values that can be written to the field `TXE`"]
pub enum TXEW {
    #[doc = "TXDR register not empty"]
    NOTEMPTY,
    #[doc = "TXDR register empty"]
    EMPTY,
}
impl TXEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXEW::NOTEMPTY => false,
            TXEW::EMPTY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXDR register not empty"]
    #[inline]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TXEW::NOTEMPTY)
    }
    #[doc = "TXDR register empty"]
    #[inline]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXEW::EMPTY)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 17:23 - Address match code (Slave mode)"]
    #[inline]
    pub fn addcode(&self) -> ADDCODER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDCODER { bits }
    }
    #[doc = "Bit 16 - Transfer direction (Slave mode)"]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Bus busy"]
    #[inline]
    pub fn busy(&self) -> BUSYR {
        BUSYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline]
    pub fn alert(&self) -> ALERTR {
        ALERTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Timeout or t_low detection flag"]
    #[inline]
    pub fn timeout(&self) -> TIMEOUTR {
        TIMEOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - PEC Error in reception"]
    #[inline]
    pub fn pecerr(&self) -> PECERRR {
        PECERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Overrun/Underrun (slave mode)"]
    #[inline]
    pub fn ovr(&self) -> OVRR {
        OVRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Arbitration lost"]
    #[inline]
    pub fn arlo(&self) -> ARLOR {
        ARLOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline]
    pub fn berr(&self) -> BERRR {
        BERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Transfer Complete Reload"]
    #[inline]
    pub fn tcr(&self) -> TCRR {
        TCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transfer Complete (master mode)"]
    #[inline]
    pub fn tc(&self) -> TCR {
        TCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Stop detection flag"]
    #[inline]
    pub fn stopf(&self) -> STOPFR {
        STOPFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Not acknowledge received flag"]
    #[inline]
    pub fn nackf(&self) -> NACKFR {
        NACKFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Address matched (slave mode)"]
    #[inline]
    pub fn addr(&self) -> ADDRR {
        ADDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Receive data register not empty (receivers)"]
    #[inline]
    pub fn rxne(&self) -> RXNER {
        RXNER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transmit interrupt status (transmitters)"]
    #[inline]
    pub fn txis(&self) -> TXISR {
        TXISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Transmit data register empty (transmitters)"]
    #[inline]
    pub fn txe(&self) -> TXER {
        TXER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Transmit interrupt status (transmitters)"]
    #[inline]
    pub fn txis(&mut self) -> _TXISW {
        _TXISW { w: self }
    }
    #[doc = "Bit 0 - Transmit data register empty (transmitters)"]
    #[inline]
    pub fn txe(&mut self) -> _TXEW {
        _TXEW { w: self }
    }
}
