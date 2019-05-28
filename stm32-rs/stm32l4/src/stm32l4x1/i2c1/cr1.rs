#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "Peripheral disabled"]
    DISABLED,
    #[doc = "Peripheral enabled"]
    ENABLED,
}
impl PER {
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
            PER::DISABLED => false,
            PER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::DISABLED,
            true => PER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PER::ENABLED
    }
}
#[doc = "Possible values of the field `TXIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIER {
    #[doc = "Transmit (TXIS) interrupt disabled"]
    DISABLED,
    #[doc = "Transmit (TXIS) interrupt enabled"]
    ENABLED,
}
impl TXIER {
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
            TXIER::DISABLED => false,
            TXIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXIER {
        match value {
            false => TXIER::DISABLED,
            true => TXIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXIER::ENABLED
    }
}
#[doc = "Possible values of the field `RXIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIER {
    #[doc = "Receive (RXNE) interrupt disabled"]
    DISABLED,
    #[doc = "Receive (RXNE) interrupt enabled"]
    ENABLED,
}
impl RXIER {
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
            RXIER::DISABLED => false,
            RXIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIER {
        match value {
            false => RXIER::DISABLED,
            true => RXIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXIER::ENABLED
    }
}
#[doc = "Possible values of the field `ADDRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRIER {
    #[doc = "Address match (ADDR) interrupts disabled"]
    DISABLED,
    #[doc = "Address match (ADDR) interrupts enabled"]
    ENABLED,
}
impl ADDRIER {
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
            ADDRIER::DISABLED => false,
            ADDRIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRIER {
        match value {
            false => ADDRIER::DISABLED,
            true => ADDRIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRIER::ENABLED
    }
}
#[doc = "Possible values of the field `NACKIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKIER {
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    DISABLED,
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    ENABLED,
}
impl NACKIER {
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
            NACKIER::DISABLED => false,
            NACKIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NACKIER {
        match value {
            false => NACKIER::DISABLED,
            true => NACKIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == NACKIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == NACKIER::ENABLED
    }
}
#[doc = "Possible values of the field `STOPIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPIER {
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    DISABLED,
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    ENABLED,
}
impl STOPIER {
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
            STOPIER::DISABLED => false,
            STOPIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPIER {
        match value {
            false => STOPIER::DISABLED,
            true => STOPIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == STOPIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == STOPIER::ENABLED
    }
}
#[doc = "Possible values of the field `TCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIER {
    #[doc = "Transfer Complete interrupt disabled"]
    DISABLED,
    #[doc = "Transfer Complete interrupt enabled"]
    ENABLED,
}
impl TCIER {
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
            TCIER::DISABLED => false,
            TCIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIER {
        match value {
            false => TCIER::DISABLED,
            true => TCIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TCIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TCIER::ENABLED
    }
}
#[doc = "Possible values of the field `ERRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRIER {
    #[doc = "Error detection interrupts disabled"]
    DISABLED,
    #[doc = "Error detection interrupts enabled"]
    ENABLED,
}
impl ERRIER {
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
            ERRIER::DISABLED => false,
            ERRIER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRIER {
        match value {
            false => ERRIER::DISABLED,
            true => ERRIER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIER::ENABLED
    }
}
#[doc = "Possible values of the field `DNF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNFR {
    #[doc = "Digital filter disabled"]
    NOFILTER,
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    FILTER1,
    #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    FILTER2,
    #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    FILTER3,
    #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    FILTER4,
    #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    FILTER5,
    #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    FILTER6,
    #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    FILTER7,
    #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    FILTER8,
    #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    FILTER9,
    #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    FILTER10,
    #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    FILTER11,
    #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    FILTER12,
    #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    FILTER13,
    #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    FILTER14,
    #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    FILTER15,
}
impl DNFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DNFR::NOFILTER => 0,
            DNFR::FILTER1 => 1,
            DNFR::FILTER2 => 2,
            DNFR::FILTER3 => 3,
            DNFR::FILTER4 => 4,
            DNFR::FILTER5 => 5,
            DNFR::FILTER6 => 6,
            DNFR::FILTER7 => 7,
            DNFR::FILTER8 => 8,
            DNFR::FILTER9 => 9,
            DNFR::FILTER10 => 10,
            DNFR::FILTER11 => 11,
            DNFR::FILTER12 => 12,
            DNFR::FILTER13 => 13,
            DNFR::FILTER14 => 14,
            DNFR::FILTER15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DNFR {
        match value {
            0 => DNFR::NOFILTER,
            1 => DNFR::FILTER1,
            2 => DNFR::FILTER2,
            3 => DNFR::FILTER3,
            4 => DNFR::FILTER4,
            5 => DNFR::FILTER5,
            6 => DNFR::FILTER6,
            7 => DNFR::FILTER7,
            8 => DNFR::FILTER8,
            9 => DNFR::FILTER9,
            10 => DNFR::FILTER10,
            11 => DNFR::FILTER11,
            12 => DNFR::FILTER12,
            13 => DNFR::FILTER13,
            14 => DNFR::FILTER14,
            15 => DNFR::FILTER15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOFILTER`"]
    #[inline]
    pub fn is_no_filter(&self) -> bool {
        *self == DNFR::NOFILTER
    }
    #[doc = "Checks if the value of the field is `FILTER1`"]
    #[inline]
    pub fn is_filter1(&self) -> bool {
        *self == DNFR::FILTER1
    }
    #[doc = "Checks if the value of the field is `FILTER2`"]
    #[inline]
    pub fn is_filter2(&self) -> bool {
        *self == DNFR::FILTER2
    }
    #[doc = "Checks if the value of the field is `FILTER3`"]
    #[inline]
    pub fn is_filter3(&self) -> bool {
        *self == DNFR::FILTER3
    }
    #[doc = "Checks if the value of the field is `FILTER4`"]
    #[inline]
    pub fn is_filter4(&self) -> bool {
        *self == DNFR::FILTER4
    }
    #[doc = "Checks if the value of the field is `FILTER5`"]
    #[inline]
    pub fn is_filter5(&self) -> bool {
        *self == DNFR::FILTER5
    }
    #[doc = "Checks if the value of the field is `FILTER6`"]
    #[inline]
    pub fn is_filter6(&self) -> bool {
        *self == DNFR::FILTER6
    }
    #[doc = "Checks if the value of the field is `FILTER7`"]
    #[inline]
    pub fn is_filter7(&self) -> bool {
        *self == DNFR::FILTER7
    }
    #[doc = "Checks if the value of the field is `FILTER8`"]
    #[inline]
    pub fn is_filter8(&self) -> bool {
        *self == DNFR::FILTER8
    }
    #[doc = "Checks if the value of the field is `FILTER9`"]
    #[inline]
    pub fn is_filter9(&self) -> bool {
        *self == DNFR::FILTER9
    }
    #[doc = "Checks if the value of the field is `FILTER10`"]
    #[inline]
    pub fn is_filter10(&self) -> bool {
        *self == DNFR::FILTER10
    }
    #[doc = "Checks if the value of the field is `FILTER11`"]
    #[inline]
    pub fn is_filter11(&self) -> bool {
        *self == DNFR::FILTER11
    }
    #[doc = "Checks if the value of the field is `FILTER12`"]
    #[inline]
    pub fn is_filter12(&self) -> bool {
        *self == DNFR::FILTER12
    }
    #[doc = "Checks if the value of the field is `FILTER13`"]
    #[inline]
    pub fn is_filter13(&self) -> bool {
        *self == DNFR::FILTER13
    }
    #[doc = "Checks if the value of the field is `FILTER14`"]
    #[inline]
    pub fn is_filter14(&self) -> bool {
        *self == DNFR::FILTER14
    }
    #[doc = "Checks if the value of the field is `FILTER15`"]
    #[inline]
    pub fn is_filter15(&self) -> bool {
        *self == DNFR::FILTER15
    }
}
#[doc = "Possible values of the field `ANFOFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANFOFFR {
    #[doc = "Analog noise filter enabled"]
    ENABLED,
    #[doc = "Analog noise filter disabled"]
    DISABLED,
}
impl ANFOFFR {
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
            ANFOFFR::ENABLED => false,
            ANFOFFR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANFOFFR {
        match value {
            false => ANFOFFR::ENABLED,
            true => ANFOFFR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ANFOFFR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ANFOFFR::DISABLED
    }
}
#[doc = "Possible values of the field `TXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAENR {
    #[doc = "DMA mode disabled for transmission"]
    DISABLED,
    #[doc = "DMA mode enabled for transmission"]
    ENABLED,
}
impl TXDMAENR {
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
            TXDMAENR::DISABLED => false,
            TXDMAENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDMAENR {
        match value {
            false => TXDMAENR::DISABLED,
            true => TXDMAENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAENR::ENABLED
    }
}
#[doc = "Possible values of the field `RXDMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAENR {
    #[doc = "DMA mode disabled for reception"]
    DISABLED,
    #[doc = "DMA mode enabled for reception"]
    ENABLED,
}
impl RXDMAENR {
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
            RXDMAENR::DISABLED => false,
            RXDMAENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXDMAENR {
        match value {
            false => RXDMAENR::DISABLED,
            true => RXDMAENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAENR::ENABLED
    }
}
#[doc = "Possible values of the field `SBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBCR {
    #[doc = "Slave byte control disabled"]
    DISABLED,
    #[doc = "Slave byte control enabled"]
    ENABLED,
}
impl SBCR {
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
            SBCR::DISABLED => false,
            SBCR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBCR {
        match value {
            false => SBCR::DISABLED,
            true => SBCR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SBCR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SBCR::ENABLED
    }
}
#[doc = "Possible values of the field `NOSTRETCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOSTRETCHR {
    #[doc = "Clock stretching enabled"]
    ENABLED,
    #[doc = "Clock stretching disabled"]
    DISABLED,
}
impl NOSTRETCHR {
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
            NOSTRETCHR::ENABLED => false,
            NOSTRETCHR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOSTRETCHR {
        match value {
            false => NOSTRETCHR::ENABLED,
            true => NOSTRETCHR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == NOSTRETCHR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == NOSTRETCHR::DISABLED
    }
}
#[doc = "Possible values of the field `WUPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUPENR {
    #[doc = "Wakeup from Stop mode disabled"]
    DISABLED,
    #[doc = "Wakeup from Stop mode enabled"]
    ENABLED,
}
impl WUPENR {
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
            WUPENR::DISABLED => false,
            WUPENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUPENR {
        match value {
            false => WUPENR::DISABLED,
            true => WUPENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WUPENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WUPENR::ENABLED
    }
}
#[doc = "Possible values of the field `GCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCENR {
    #[doc = "General call disabled. Address 0b00000000 is NACKed"]
    DISABLED,
    #[doc = "General call enabled. Address 0b00000000 is ACKed"]
    ENABLED,
}
impl GCENR {
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
            GCENR::DISABLED => false,
            GCENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GCENR {
        match value {
            false => GCENR::DISABLED,
            true => GCENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == GCENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == GCENR::ENABLED
    }
}
#[doc = "Possible values of the field `SMBHEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBHENR {
    #[doc = "Host address disabled. Address 0b0001000x is NACKed"]
    DISABLED,
    #[doc = "Host address enabled. Address 0b0001000x is ACKed"]
    ENABLED,
}
impl SMBHENR {
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
            SMBHENR::DISABLED => false,
            SMBHENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMBHENR {
        match value {
            false => SMBHENR::DISABLED,
            true => SMBHENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SMBHENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SMBHENR::ENABLED
    }
}
#[doc = "Possible values of the field `SMBDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBDENR {
    #[doc = "Device default address disabled. Address 0b1100001x is NACKed"]
    DISABLED,
    #[doc = "Device default address enabled. Address 0b1100001x is ACKed"]
    ENABLED,
}
impl SMBDENR {
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
            SMBDENR::DISABLED => false,
            SMBDENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMBDENR {
        match value {
            false => SMBDENR::DISABLED,
            true => SMBDENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SMBDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SMBDENR::ENABLED
    }
}
#[doc = "Possible values of the field `ALERTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTENR {
    #[doc = "In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported"]
    DISABLED,
    #[doc = "In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported"]
    ENABLED,
}
impl ALERTENR {
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
            ALERTENR::DISABLED => false,
            ALERTENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALERTENR {
        match value {
            false => ALERTENR::DISABLED,
            true => ALERTENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ALERTENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ALERTENR::ENABLED
    }
}
#[doc = "Possible values of the field `PECEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECENR {
    #[doc = "PEC calculation disabled"]
    DISABLED,
    #[doc = "PEC calculation enabled"]
    ENABLED,
}
impl PECENR {
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
            PECENR::DISABLED => false,
            PECENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PECENR {
        match value {
            false => PECENR::DISABLED,
            true => PECENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PECENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PECENR::ENABLED
    }
}
#[doc = "Values that can be written to the field `PE`"]
pub enum PEW {
    #[doc = "Peripheral disabled"]
    DISABLED,
    #[doc = "Peripheral enabled"]
    ENABLED,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::DISABLED => false,
            PEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEW::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEW::ENABLED)
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
#[doc = "Values that can be written to the field `TXIE`"]
pub enum TXIEW {
    #[doc = "Transmit (TXIS) interrupt disabled"]
    DISABLED,
    #[doc = "Transmit (TXIS) interrupt enabled"]
    ENABLED,
}
impl TXIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXIEW::DISABLED => false,
            TXIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit (TXIS) interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXIEW::DISABLED)
    }
    #[doc = "Transmit (TXIS) interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXIEW::ENABLED)
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
#[doc = "Values that can be written to the field `RXIE`"]
pub enum RXIEW {
    #[doc = "Receive (RXNE) interrupt disabled"]
    DISABLED,
    #[doc = "Receive (RXNE) interrupt enabled"]
    ENABLED,
}
impl RXIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIEW::DISABLED => false,
            RXIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive (RXNE) interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXIEW::DISABLED)
    }
    #[doc = "Receive (RXNE) interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXIEW::ENABLED)
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
#[doc = "Values that can be written to the field `ADDRIE`"]
pub enum ADDRIEW {
    #[doc = "Address match (ADDR) interrupts disabled"]
    DISABLED,
    #[doc = "Address match (ADDR) interrupts enabled"]
    ENABLED,
}
impl ADDRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRIEW::DISABLED => false,
            ADDRIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Address match (ADDR) interrupts disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRIEW::DISABLED)
    }
    #[doc = "Address match (ADDR) interrupts enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRIEW::ENABLED)
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
#[doc = "Values that can be written to the field `NACKIE`"]
pub enum NACKIEW {
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    DISABLED,
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    ENABLED,
}
impl NACKIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NACKIEW::DISABLED => false,
            NACKIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NACKIEW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NACKIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACKIEW::DISABLED)
    }
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACKIEW::ENABLED)
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
#[doc = "Values that can be written to the field `STOPIE`"]
pub enum STOPIEW {
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    DISABLED,
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    ENABLED,
}
impl STOPIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPIEW::DISABLED => false,
            STOPIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPIEW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STOPIEW::DISABLED)
    }
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STOPIEW::ENABLED)
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
#[doc = "Values that can be written to the field `TCIE`"]
pub enum TCIEW {
    #[doc = "Transfer Complete interrupt disabled"]
    DISABLED,
    #[doc = "Transfer Complete interrupt enabled"]
    ENABLED,
}
impl TCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIEW::DISABLED => false,
            TCIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer Complete interrupt disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIEW::DISABLED)
    }
    #[doc = "Transfer Complete interrupt enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIEW::ENABLED)
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
#[doc = "Values that can be written to the field `ERRIE`"]
pub enum ERRIEW {
    #[doc = "Error detection interrupts disabled"]
    DISABLED,
    #[doc = "Error detection interrupts enabled"]
    ENABLED,
}
impl ERRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRIEW::DISABLED => false,
            ERRIEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error detection interrupts disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIEW::DISABLED)
    }
    #[doc = "Error detection interrupts enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIEW::ENABLED)
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
#[doc = "Values that can be written to the field `DNF`"]
pub enum DNFW {
    #[doc = "Digital filter disabled"]
    NOFILTER,
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    FILTER1,
    #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    FILTER2,
    #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    FILTER3,
    #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    FILTER4,
    #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    FILTER5,
    #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    FILTER6,
    #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    FILTER7,
    #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    FILTER8,
    #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    FILTER9,
    #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    FILTER10,
    #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    FILTER11,
    #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    FILTER12,
    #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    FILTER13,
    #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    FILTER14,
    #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    FILTER15,
}
impl DNFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DNFW::NOFILTER => 0,
            DNFW::FILTER1 => 1,
            DNFW::FILTER2 => 2,
            DNFW::FILTER3 => 3,
            DNFW::FILTER4 => 4,
            DNFW::FILTER5 => 5,
            DNFW::FILTER6 => 6,
            DNFW::FILTER7 => 7,
            DNFW::FILTER8 => 8,
            DNFW::FILTER9 => 9,
            DNFW::FILTER10 => 10,
            DNFW::FILTER11 => 11,
            DNFW::FILTER12 => 12,
            DNFW::FILTER13 => 13,
            DNFW::FILTER14 => 14,
            DNFW::FILTER15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DNFW<'a> {
    w: &'a mut W,
}
impl<'a> _DNFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DNFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Digital filter disabled"]
    #[inline]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(DNFW::NOFILTER)
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    #[inline]
    pub fn filter1(self) -> &'a mut W {
        self.variant(DNFW::FILTER1)
    }
    #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    #[inline]
    pub fn filter2(self) -> &'a mut W {
        self.variant(DNFW::FILTER2)
    }
    #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    #[inline]
    pub fn filter3(self) -> &'a mut W {
        self.variant(DNFW::FILTER3)
    }
    #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    #[inline]
    pub fn filter4(self) -> &'a mut W {
        self.variant(DNFW::FILTER4)
    }
    #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    #[inline]
    pub fn filter5(self) -> &'a mut W {
        self.variant(DNFW::FILTER5)
    }
    #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    #[inline]
    pub fn filter6(self) -> &'a mut W {
        self.variant(DNFW::FILTER6)
    }
    #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    #[inline]
    pub fn filter7(self) -> &'a mut W {
        self.variant(DNFW::FILTER7)
    }
    #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    #[inline]
    pub fn filter8(self) -> &'a mut W {
        self.variant(DNFW::FILTER8)
    }
    #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    #[inline]
    pub fn filter9(self) -> &'a mut W {
        self.variant(DNFW::FILTER9)
    }
    #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    #[inline]
    pub fn filter10(self) -> &'a mut W {
        self.variant(DNFW::FILTER10)
    }
    #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    #[inline]
    pub fn filter11(self) -> &'a mut W {
        self.variant(DNFW::FILTER11)
    }
    #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    #[inline]
    pub fn filter12(self) -> &'a mut W {
        self.variant(DNFW::FILTER12)
    }
    #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    #[inline]
    pub fn filter13(self) -> &'a mut W {
        self.variant(DNFW::FILTER13)
    }
    #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    #[inline]
    pub fn filter14(self) -> &'a mut W {
        self.variant(DNFW::FILTER14)
    }
    #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    #[inline]
    pub fn filter15(self) -> &'a mut W {
        self.variant(DNFW::FILTER15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ANFOFF`"]
pub enum ANFOFFW {
    #[doc = "Analog noise filter enabled"]
    ENABLED,
    #[doc = "Analog noise filter disabled"]
    DISABLED,
}
impl ANFOFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANFOFFW::ENABLED => false,
            ANFOFFW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANFOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _ANFOFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANFOFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog noise filter enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ANFOFFW::ENABLED)
    }
    #[doc = "Analog noise filter disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ANFOFFW::DISABLED)
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
#[doc = "Values that can be written to the field `TXDMAEN`"]
pub enum TXDMAENW {
    #[doc = "DMA mode disabled for transmission"]
    DISABLED,
    #[doc = "DMA mode enabled for transmission"]
    ENABLED,
}
impl TXDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDMAENW::DISABLED => false,
            TXDMAENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA mode disabled for transmission"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAENW::DISABLED)
    }
    #[doc = "DMA mode enabled for transmission"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAENW::ENABLED)
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
#[doc = "Values that can be written to the field `RXDMAEN`"]
pub enum RXDMAENW {
    #[doc = "DMA mode disabled for reception"]
    DISABLED,
    #[doc = "DMA mode enabled for reception"]
    ENABLED,
}
impl RXDMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXDMAENW::DISABLED => false,
            RXDMAENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXDMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXDMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA mode disabled for reception"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAENW::DISABLED)
    }
    #[doc = "DMA mode enabled for reception"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAENW::ENABLED)
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
#[doc = "Values that can be written to the field `SBC`"]
pub enum SBCW {
    #[doc = "Slave byte control disabled"]
    DISABLED,
    #[doc = "Slave byte control enabled"]
    ENABLED,
}
impl SBCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBCW::DISABLED => false,
            SBCW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBCW<'a> {
    w: &'a mut W,
}
impl<'a> _SBCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave byte control disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SBCW::DISABLED)
    }
    #[doc = "Slave byte control enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SBCW::ENABLED)
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
#[doc = "Values that can be written to the field `NOSTRETCH`"]
pub enum NOSTRETCHW {
    #[doc = "Clock stretching enabled"]
    ENABLED,
    #[doc = "Clock stretching disabled"]
    DISABLED,
}
impl NOSTRETCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NOSTRETCHW::ENABLED => false,
            NOSTRETCHW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NOSTRETCHW<'a> {
    w: &'a mut W,
}
impl<'a> _NOSTRETCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NOSTRETCHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock stretching enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NOSTRETCHW::ENABLED)
    }
    #[doc = "Clock stretching disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NOSTRETCHW::DISABLED)
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
#[doc = "Values that can be written to the field `WUPEN`"]
pub enum WUPENW {
    #[doc = "Wakeup from Stop mode disabled"]
    DISABLED,
    #[doc = "Wakeup from Stop mode enabled"]
    ENABLED,
}
impl WUPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUPENW::DISABLED => false,
            WUPENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUPENW<'a> {
    w: &'a mut W,
}
impl<'a> _WUPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wakeup from Stop mode disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUPENW::DISABLED)
    }
    #[doc = "Wakeup from Stop mode enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUPENW::ENABLED)
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
#[doc = "Values that can be written to the field `GCEN`"]
pub enum GCENW {
    #[doc = "General call disabled. Address 0b00000000 is NACKed"]
    DISABLED,
    #[doc = "General call enabled. Address 0b00000000 is ACKed"]
    ENABLED,
}
impl GCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GCENW::DISABLED => false,
            GCENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GCENW<'a> {
    w: &'a mut W,
}
impl<'a> _GCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "General call disabled. Address 0b00000000 is NACKed"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GCENW::DISABLED)
    }
    #[doc = "General call enabled. Address 0b00000000 is ACKed"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GCENW::ENABLED)
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
#[doc = "Values that can be written to the field `SMBHEN`"]
pub enum SMBHENW {
    #[doc = "Host address disabled. Address 0b0001000x is NACKed"]
    DISABLED,
    #[doc = "Host address enabled. Address 0b0001000x is ACKed"]
    ENABLED,
}
impl SMBHENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMBHENW::DISABLED => false,
            SMBHENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMBHENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMBHENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMBHENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Host address disabled. Address 0b0001000x is NACKed"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMBHENW::DISABLED)
    }
    #[doc = "Host address enabled. Address 0b0001000x is ACKed"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMBHENW::ENABLED)
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
#[doc = "Values that can be written to the field `SMBDEN`"]
pub enum SMBDENW {
    #[doc = "Device default address disabled. Address 0b1100001x is NACKed"]
    DISABLED,
    #[doc = "Device default address enabled. Address 0b1100001x is ACKed"]
    ENABLED,
}
impl SMBDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMBDENW::DISABLED => false,
            SMBDENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMBDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMBDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMBDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Device default address disabled. Address 0b1100001x is NACKed"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMBDENW::DISABLED)
    }
    #[doc = "Device default address enabled. Address 0b1100001x is ACKed"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMBDENW::ENABLED)
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
#[doc = "Values that can be written to the field `ALERTEN`"]
pub enum ALERTENW {
    #[doc = "In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported"]
    DISABLED,
    #[doc = "In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported"]
    ENABLED,
}
impl ALERTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALERTENW::DISABLED => false,
            ALERTENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALERTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ALERTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALERTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALERTENW::DISABLED)
    }
    #[doc = "In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALERTENW::ENABLED)
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
#[doc = "Values that can be written to the field `PECEN`"]
pub enum PECENW {
    #[doc = "PEC calculation disabled"]
    DISABLED,
    #[doc = "PEC calculation enabled"]
    ENABLED,
}
impl PECENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PECENW::DISABLED => false,
            PECENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PECENW<'a> {
    w: &'a mut W,
}
impl<'a> _PECENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PECENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PEC calculation disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PECENW::DISABLED)
    }
    #[doc = "PEC calculation enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PECENW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline]
    pub fn txie(&self) -> TXIER {
        TXIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline]
    pub fn rxie(&self) -> RXIER {
        RXIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Address match interrupt enable (slave only)"]
    #[inline]
    pub fn addrie(&self) -> ADDRIER {
        ADDRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline]
    pub fn nackie(&self) -> NACKIER {
        NACKIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - STOP detection Interrupt enable"]
    #[inline]
    pub fn stopie(&self) -> STOPIER {
        STOPIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable"]
    #[inline]
    pub fn tcie(&self) -> TCIER {
        TCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline]
    pub fn errie(&self) -> ERRIER {
        ERRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Digital noise filter"]
    #[inline]
    pub fn dnf(&self) -> DNFR {
        DNFR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Analog noise filter OFF"]
    #[inline]
    pub fn anfoff(&self) -> ANFOFFR {
        ANFOFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline]
    pub fn txdmaen(&self) -> TXDMAENR {
        TXDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline]
    pub fn rxdmaen(&self) -> RXDMAENR {
        RXDMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Slave byte control"]
    #[inline]
    pub fn sbc(&self) -> SBCR {
        SBCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Clock stretching disable"]
    #[inline]
    pub fn nostretch(&self) -> NOSTRETCHR {
        NOSTRETCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Wakeup from STOP enable"]
    #[inline]
    pub fn wupen(&self) -> WUPENR {
        WUPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline]
    pub fn gcen(&self) -> GCENR {
        GCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - SMBus Host address enable"]
    #[inline]
    pub fn smbhen(&self) -> SMBHENR {
        SMBHENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - SMBus Device Default address enable"]
    #[inline]
    pub fn smbden(&self) -> SMBDENR {
        SMBDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - SMBUS alert enable"]
    #[inline]
    pub fn alerten(&self) -> ALERTENR {
        ALERTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - PEC enable"]
    #[inline]
    pub fn pecen(&self) -> PECENR {
        PECENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline]
    pub fn txie(&mut self) -> _TXIEW {
        _TXIEW { w: self }
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline]
    pub fn rxie(&mut self) -> _RXIEW {
        _RXIEW { w: self }
    }
    #[doc = "Bit 3 - Address match interrupt enable (slave only)"]
    #[inline]
    pub fn addrie(&mut self) -> _ADDRIEW {
        _ADDRIEW { w: self }
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline]
    pub fn nackie(&mut self) -> _NACKIEW {
        _NACKIEW { w: self }
    }
    #[doc = "Bit 5 - STOP detection Interrupt enable"]
    #[inline]
    pub fn stopie(&mut self) -> _STOPIEW {
        _STOPIEW { w: self }
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable"]
    #[inline]
    pub fn tcie(&mut self) -> _TCIEW {
        _TCIEW { w: self }
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline]
    pub fn errie(&mut self) -> _ERRIEW {
        _ERRIEW { w: self }
    }
    #[doc = "Bits 8:11 - Digital noise filter"]
    #[inline]
    pub fn dnf(&mut self) -> _DNFW {
        _DNFW { w: self }
    }
    #[doc = "Bit 12 - Analog noise filter OFF"]
    #[inline]
    pub fn anfoff(&mut self) -> _ANFOFFW {
        _ANFOFFW { w: self }
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline]
    pub fn txdmaen(&mut self) -> _TXDMAENW {
        _TXDMAENW { w: self }
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline]
    pub fn rxdmaen(&mut self) -> _RXDMAENW {
        _RXDMAENW { w: self }
    }
    #[doc = "Bit 16 - Slave byte control"]
    #[inline]
    pub fn sbc(&mut self) -> _SBCW {
        _SBCW { w: self }
    }
    #[doc = "Bit 17 - Clock stretching disable"]
    #[inline]
    pub fn nostretch(&mut self) -> _NOSTRETCHW {
        _NOSTRETCHW { w: self }
    }
    #[doc = "Bit 18 - Wakeup from STOP enable"]
    #[inline]
    pub fn wupen(&mut self) -> _WUPENW {
        _WUPENW { w: self }
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline]
    pub fn gcen(&mut self) -> _GCENW {
        _GCENW { w: self }
    }
    #[doc = "Bit 20 - SMBus Host address enable"]
    #[inline]
    pub fn smbhen(&mut self) -> _SMBHENW {
        _SMBHENW { w: self }
    }
    #[doc = "Bit 21 - SMBus Device Default address enable"]
    #[inline]
    pub fn smbden(&mut self) -> _SMBDENW {
        _SMBDENW { w: self }
    }
    #[doc = "Bit 22 - SMBUS alert enable"]
    #[inline]
    pub fn alerten(&mut self) -> _ALERTENW {
        _ALERTENW { w: self }
    }
    #[doc = "Bit 23 - PEC enable"]
    #[inline]
    pub fn pecen(&mut self) -> _PECENW {
        _PECENW { w: self }
    }
}
