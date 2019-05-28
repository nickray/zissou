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
#[doc = "Possible values of the field `PECBYTE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECBYTER {
    #[doc = "No PEC transfer"]
    NOPEC,
    #[doc = "PEC transmission/reception is requested"]
    PEC,
}
impl PECBYTER {
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
            PECBYTER::NOPEC => false,
            PECBYTER::PEC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PECBYTER {
        match value {
            false => PECBYTER::NOPEC,
            true => PECBYTER::PEC,
        }
    }
    #[doc = "Checks if the value of the field is `NOPEC`"]
    #[inline]
    pub fn is_no_pec(&self) -> bool {
        *self == PECBYTER::NOPEC
    }
    #[doc = "Checks if the value of the field is `PEC`"]
    #[inline]
    pub fn is_pec(&self) -> bool {
        *self == PECBYTER::PEC
    }
}
#[doc = "Possible values of the field `AUTOEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOENDR {
    #[doc = "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    SOFTWARE,
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    AUTOMATIC,
}
impl AUTOENDR {
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
            AUTOENDR::SOFTWARE => false,
            AUTOENDR::AUTOMATIC => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTOENDR {
        match value {
            false => AUTOENDR::SOFTWARE,
            true => AUTOENDR::AUTOMATIC,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline]
    pub fn is_software(&self) -> bool {
        *self == AUTOENDR::SOFTWARE
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline]
    pub fn is_automatic(&self) -> bool {
        *self == AUTOENDR::AUTOMATIC
    }
}
#[doc = "Possible values of the field `RELOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RELOADR {
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    COMPLETED,
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    NOTCOMPETED,
}
impl RELOADR {
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
            RELOADR::COMPLETED => false,
            RELOADR::NOTCOMPETED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RELOADR {
        match value {
            false => RELOADR::COMPLETED,
            true => RELOADR::NOTCOMPETED,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline]
    pub fn is_completed(&self) -> bool {
        *self == RELOADR::COMPLETED
    }
    #[doc = "Checks if the value of the field is `NOTCOMPETED`"]
    #[inline]
    pub fn is_not_competed(&self) -> bool {
        *self == RELOADR::NOTCOMPETED
    }
}
#[doc = r" Value of the field"]
pub struct NBYTESR {
    bits: u8,
}
impl NBYTESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `NACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKR {
    #[doc = "an ACK is sent after current received byte"]
    ACK,
    #[doc = "a NACK is sent after current received byte"]
    NACK,
}
impl NACKR {
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
            NACKR::ACK => false,
            NACKR::NACK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NACKR {
        match value {
            false => NACKR::ACK,
            true => NACKR::NACK,
        }
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline]
    pub fn is_ack(&self) -> bool {
        *self == NACKR::ACK
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline]
    pub fn is_nack(&self) -> bool {
        *self == NACKR::NACK
    }
}
#[doc = "Possible values of the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPR {
    #[doc = "No Stop generation"]
    NOSTOP,
    #[doc = "Stop generation after current byte transfer"]
    STOP,
}
impl STOPR {
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
            STOPR::NOSTOP => false,
            STOPR::STOP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPR {
        match value {
            false => STOPR::NOSTOP,
            true => STOPR::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTOP`"]
    #[inline]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPR::NOSTOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == STOPR::STOP
    }
}
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "No Start generation"]
    NOSTART,
    #[doc = "Restart/Start generation"]
    START,
}
impl STARTR {
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
            STARTR::NOSTART => false,
            STARTR::START => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTR {
        match value {
            false => STARTR::NOSTART,
            true => STARTR::START,
        }
    }
    #[doc = "Checks if the value of the field is `NOSTART`"]
    #[inline]
    pub fn is_no_start(&self) -> bool {
        *self == STARTR::NOSTART
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline]
    pub fn is_start(&self) -> bool {
        *self == STARTR::START
    }
}
#[doc = "Possible values of the field `HEAD10R`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HEAD10RR {
    #[doc = "The master sends the complete 10 bit slave address read sequence"]
    COMPLETE,
    #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    PARTIAL,
}
impl HEAD10RR {
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
            HEAD10RR::COMPLETE => false,
            HEAD10RR::PARTIAL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HEAD10RR {
        match value {
            false => HEAD10RR::COMPLETE,
            true => HEAD10RR::PARTIAL,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == HEAD10RR::COMPLETE
    }
    #[doc = "Checks if the value of the field is `PARTIAL`"]
    #[inline]
    pub fn is_partial(&self) -> bool {
        *self == HEAD10RR::PARTIAL
    }
}
#[doc = "Possible values of the field `ADD10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADD10R {
    #[doc = "The master operates in 7-bit addressing mode"]
    BIT7,
    #[doc = "The master operates in 10-bit addressing mode"]
    BIT10,
}
impl ADD10R {
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
            ADD10R::BIT7 => false,
            ADD10R::BIT10 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADD10R {
        match value {
            false => ADD10R::BIT7,
            true => ADD10R::BIT10,
        }
    }
    #[doc = "Checks if the value of the field is `BIT7`"]
    #[inline]
    pub fn is_bit7(&self) -> bool {
        *self == ADD10R::BIT7
    }
    #[doc = "Checks if the value of the field is `BIT10`"]
    #[inline]
    pub fn is_bit10(&self) -> bool {
        *self == ADD10R::BIT10
    }
}
#[doc = "Possible values of the field `RD_WRN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RD_WRNR {
    #[doc = "Master requests a write transfer"]
    WRITE,
    #[doc = "Master requests a read transfer"]
    READ,
}
impl RD_WRNR {
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
            RD_WRNR::WRITE => false,
            RD_WRNR::READ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RD_WRNR {
        match value {
            false => RD_WRNR::WRITE,
            true => RD_WRNR::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline]
    pub fn is_write(&self) -> bool {
        *self == RD_WRNR::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline]
    pub fn is_read(&self) -> bool {
        *self == RD_WRNR::READ
    }
}
#[doc = r" Value of the field"]
pub struct SADDR {
    bits: u16,
}
impl SADDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `PECBYTE`"]
pub enum PECBYTEW {
    #[doc = "No PEC transfer"]
    NOPEC,
    #[doc = "PEC transmission/reception is requested"]
    PEC,
}
impl PECBYTEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PECBYTEW::NOPEC => false,
            PECBYTEW::PEC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PECBYTEW<'a> {
    w: &'a mut W,
}
impl<'a> _PECBYTEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PECBYTEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No PEC transfer"]
    #[inline]
    pub fn no_pec(self) -> &'a mut W {
        self.variant(PECBYTEW::NOPEC)
    }
    #[doc = "PEC transmission/reception is requested"]
    #[inline]
    pub fn pec(self) -> &'a mut W {
        self.variant(PECBYTEW::PEC)
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
#[doc = "Values that can be written to the field `AUTOEND`"]
pub enum AUTOENDW {
    #[doc = "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    SOFTWARE,
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    AUTOMATIC,
}
impl AUTOENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AUTOENDW::SOFTWARE => false,
            AUTOENDW::AUTOMATIC => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOENDW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    #[inline]
    pub fn software(self) -> &'a mut W {
        self.variant(AUTOENDW::SOFTWARE)
    }
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    #[inline]
    pub fn automatic(self) -> &'a mut W {
        self.variant(AUTOENDW::AUTOMATIC)
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
#[doc = "Values that can be written to the field `RELOAD`"]
pub enum RELOADW {
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    COMPLETED,
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    NOTCOMPETED,
}
impl RELOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RELOADW::COMPLETED => false,
            RELOADW::NOTCOMPETED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RELOADW<'a> {
    w: &'a mut W,
}
impl<'a> _RELOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RELOADW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    #[inline]
    pub fn completed(self) -> &'a mut W {
        self.variant(RELOADW::COMPLETED)
    }
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    #[inline]
    pub fn not_competed(self) -> &'a mut W {
        self.variant(RELOADW::NOTCOMPETED)
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
#[doc = r" Proxy"]
pub struct _NBYTESW<'a> {
    w: &'a mut W,
}
impl<'a> _NBYTESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NACK`"]
pub enum NACKW {
    #[doc = "an ACK is sent after current received byte"]
    ACK,
    #[doc = "a NACK is sent after current received byte"]
    NACK,
}
impl NACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NACKW::ACK => false,
            NACKW::NACK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NACKW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "an ACK is sent after current received byte"]
    #[inline]
    pub fn ack(self) -> &'a mut W {
        self.variant(NACKW::ACK)
    }
    #[doc = "a NACK is sent after current received byte"]
    #[inline]
    pub fn nack(self) -> &'a mut W {
        self.variant(NACKW::NACK)
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
    #[doc = "No Stop generation"]
    NOSTOP,
    #[doc = "Stop generation after current byte transfer"]
    STOP,
}
impl STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPW::NOSTOP => false,
            STOPW::STOP => true,
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
            self.bit(variant._bits())
        }
    }
    #[doc = "No Stop generation"]
    #[inline]
    pub fn no_stop(self) -> &'a mut W {
        self.variant(STOPW::NOSTOP)
    }
    #[doc = "Stop generation after current byte transfer"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOPW::STOP)
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
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "No Start generation"]
    NOSTART,
    #[doc = "Restart/Start generation"]
    START,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTW::NOSTART => false,
            STARTW::START => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Start generation"]
    #[inline]
    pub fn no_start(self) -> &'a mut W {
        self.variant(STARTW::NOSTART)
    }
    #[doc = "Restart/Start generation"]
    #[inline]
    pub fn start(self) -> &'a mut W {
        self.variant(STARTW::START)
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
#[doc = "Values that can be written to the field `HEAD10R`"]
pub enum HEAD10RW {
    #[doc = "The master sends the complete 10 bit slave address read sequence"]
    COMPLETE,
    #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    PARTIAL,
}
impl HEAD10RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HEAD10RW::COMPLETE => false,
            HEAD10RW::PARTIAL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HEAD10RW<'a> {
    w: &'a mut W,
}
impl<'a> _HEAD10RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HEAD10RW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The master sends the complete 10 bit slave address read sequence"]
    #[inline]
    pub fn complete(self) -> &'a mut W {
        self.variant(HEAD10RW::COMPLETE)
    }
    #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    #[inline]
    pub fn partial(self) -> &'a mut W {
        self.variant(HEAD10RW::PARTIAL)
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
#[doc = "Values that can be written to the field `ADD10`"]
pub enum ADD10W {
    #[doc = "The master operates in 7-bit addressing mode"]
    BIT7,
    #[doc = "The master operates in 10-bit addressing mode"]
    BIT10,
}
impl ADD10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADD10W::BIT7 => false,
            ADD10W::BIT10 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADD10W<'a> {
    w: &'a mut W,
}
impl<'a> _ADD10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADD10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The master operates in 7-bit addressing mode"]
    #[inline]
    pub fn bit7(self) -> &'a mut W {
        self.variant(ADD10W::BIT7)
    }
    #[doc = "The master operates in 10-bit addressing mode"]
    #[inline]
    pub fn bit10(self) -> &'a mut W {
        self.variant(ADD10W::BIT10)
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
#[doc = "Values that can be written to the field `RD_WRN`"]
pub enum RD_WRNW {
    #[doc = "Master requests a write transfer"]
    WRITE,
    #[doc = "Master requests a read transfer"]
    READ,
}
impl RD_WRNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RD_WRNW::WRITE => false,
            RD_WRNW::READ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RD_WRNW<'a> {
    w: &'a mut W,
}
impl<'a> _RD_WRNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RD_WRNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master requests a write transfer"]
    #[inline]
    pub fn write(self) -> &'a mut W {
        self.variant(RD_WRNW::WRITE)
    }
    #[doc = "Master requests a read transfer"]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(RD_WRNW::READ)
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
#[doc = r" Proxy"]
pub struct _SADDW<'a> {
    w: &'a mut W,
}
impl<'a> _SADDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
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
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline]
    pub fn pecbyte(&self) -> PECBYTER {
        PECBYTER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline]
    pub fn autoend(&self) -> AUTOENDR {
        AUTOENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline]
    pub fn reload(&self) -> RELOADR {
        RELOADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline]
    pub fn nbytes(&self) -> NBYTESR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NBYTESR { bits }
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline]
    pub fn nack(&self) -> NACKR {
        NACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline]
    pub fn stop(&self) -> STOPR {
        STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline]
    pub fn head10r(&self) -> HEAD10RR {
        HEAD10RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline]
    pub fn add10(&self) -> ADD10R {
        ADD10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline]
    pub fn rd_wrn(&self) -> RD_WRNR {
        RD_WRNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 0:9 - Slave address bit (master mode)"]
    #[inline]
    pub fn sadd(&self) -> SADDR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SADDR { bits }
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
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline]
    pub fn pecbyte(&mut self) -> _PECBYTEW {
        _PECBYTEW { w: self }
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline]
    pub fn autoend(&mut self) -> _AUTOENDW {
        _AUTOENDW { w: self }
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline]
    pub fn reload(&mut self) -> _RELOADW {
        _RELOADW { w: self }
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline]
    pub fn nbytes(&mut self) -> _NBYTESW {
        _NBYTESW { w: self }
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline]
    pub fn nack(&mut self) -> _NACKW {
        _NACKW { w: self }
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline]
    pub fn head10r(&mut self) -> _HEAD10RW {
        _HEAD10RW { w: self }
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline]
    pub fn add10(&mut self) -> _ADD10W {
        _ADD10W { w: self }
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline]
    pub fn rd_wrn(&mut self) -> _RD_WRNW {
        _RD_WRNW { w: self }
    }
    #[doc = "Bits 0:9 - Slave address bit (master mode)"]
    #[inline]
    pub fn sadd(&mut self) -> _SADDW {
        _SADDW { w: self }
    }
}
