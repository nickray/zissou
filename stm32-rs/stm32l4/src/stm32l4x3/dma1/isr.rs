#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ISR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `TEIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF7R {
    #[doc = "No transfer error"]
    NOERROR,
    #[doc = "A transfer error has occured"]
    ERROR,
}
impl TEIF7R {
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
            TEIF7R::NOERROR => false,
            TEIF7R::ERROR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEIF7R {
        match value {
            false => TEIF7R::NOERROR,
            true => TEIF7R::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == TEIF7R::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == TEIF7R::ERROR
    }
}
#[doc = "Possible values of the field `HTIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HTIF7R {
    #[doc = "No half transfer event"]
    NOTHALF,
    #[doc = "A half transfer event has occured"]
    HALF,
}
impl HTIF7R {
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
            HTIF7R::NOTHALF => false,
            HTIF7R::HALF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HTIF7R {
        match value {
            false => HTIF7R::NOTHALF,
            true => HTIF7R::HALF,
        }
    }
    #[doc = "Checks if the value of the field is `NOTHALF`"]
    #[inline]
    pub fn is_not_half(&self) -> bool {
        *self == HTIF7R::NOTHALF
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == HTIF7R::HALF
    }
}
#[doc = "Possible values of the field `TCIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF7R {
    #[doc = "No transfer complete event"]
    NOTCOMPLETE,
    #[doc = "A transfer complete event has occured"]
    COMPLETE,
}
impl TCIF7R {
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
            TCIF7R::NOTCOMPLETE => false,
            TCIF7R::COMPLETE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIF7R {
        match value {
            false => TCIF7R::NOTCOMPLETE,
            true => TCIF7R::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline]
    pub fn is_not_complete(&self) -> bool {
        *self == TCIF7R::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline]
    pub fn is_complete(&self) -> bool {
        *self == TCIF7R::COMPLETE
    }
}
#[doc = "Possible values of the field `GIF7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF7R {
    #[doc = "No transfer error, half event, complete event"]
    NOEVENT,
    #[doc = "A transfer error, half event or complete event has occured"]
    EVENT,
}
impl GIF7R {
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
            GIF7R::NOEVENT => false,
            GIF7R::EVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GIF7R {
        match value {
            false => GIF7R::NOEVENT,
            true => GIF7R::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline]
    pub fn is_no_event(&self) -> bool {
        *self == GIF7R::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline]
    pub fn is_event(&self) -> bool {
        *self == GIF7R::EVENT
    }
}
#[doc = "Possible values of the field `TEIF6`"]
pub type TEIF6R = TEIF7R;
#[doc = "Possible values of the field `HTIF6`"]
pub type HTIF6R = HTIF7R;
#[doc = "Possible values of the field `TCIF6`"]
pub type TCIF6R = TCIF7R;
#[doc = "Possible values of the field `GIF6`"]
pub type GIF6R = GIF7R;
#[doc = "Possible values of the field `TEIF5`"]
pub type TEIF5R = TEIF7R;
#[doc = "Possible values of the field `HTIF5`"]
pub type HTIF5R = HTIF7R;
#[doc = "Possible values of the field `TCIF5`"]
pub type TCIF5R = TCIF7R;
#[doc = "Possible values of the field `GIF5`"]
pub type GIF5R = GIF7R;
#[doc = "Possible values of the field `TEIF4`"]
pub type TEIF4R = TEIF7R;
#[doc = "Possible values of the field `HTIF4`"]
pub type HTIF4R = HTIF7R;
#[doc = "Possible values of the field `TCIF4`"]
pub type TCIF4R = TCIF7R;
#[doc = "Possible values of the field `GIF4`"]
pub type GIF4R = GIF7R;
#[doc = "Possible values of the field `TEIF3`"]
pub type TEIF3R = TEIF7R;
#[doc = "Possible values of the field `HTIF3`"]
pub type HTIF3R = HTIF7R;
#[doc = "Possible values of the field `TCIF3`"]
pub type TCIF3R = TCIF7R;
#[doc = "Possible values of the field `GIF3`"]
pub type GIF3R = GIF7R;
#[doc = "Possible values of the field `TEIF2`"]
pub type TEIF2R = TEIF7R;
#[doc = "Possible values of the field `HTIF2`"]
pub type HTIF2R = HTIF7R;
#[doc = "Possible values of the field `TCIF2`"]
pub type TCIF2R = TCIF7R;
#[doc = "Possible values of the field `GIF2`"]
pub type GIF2R = GIF7R;
#[doc = "Possible values of the field `TEIF1`"]
pub type TEIF1R = TEIF7R;
#[doc = "Possible values of the field `HTIF1`"]
pub type HTIF1R = HTIF7R;
#[doc = "Possible values of the field `TCIF1`"]
pub type TCIF1R = TCIF7R;
#[doc = "Possible values of the field `GIF1`"]
pub type GIF1R = GIF7R;
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 27 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline]
    pub fn teif7(&self) -> TEIF7R {
        TEIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline]
    pub fn htif7(&self) -> HTIF7R {
        HTIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline]
    pub fn tcif7(&self) -> TCIF7R {
        TCIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline]
    pub fn gif7(&self) -> GIF7R {
        GIF7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline]
    pub fn teif6(&self) -> TEIF6R {
        TEIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline]
    pub fn htif6(&self) -> HTIF6R {
        HTIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline]
    pub fn tcif6(&self) -> TCIF6R {
        TCIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline]
    pub fn gif6(&self) -> GIF6R {
        GIF6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline]
    pub fn teif5(&self) -> TEIF5R {
        TEIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline]
    pub fn htif5(&self) -> HTIF5R {
        HTIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline]
    pub fn tcif5(&self) -> TCIF5R {
        TCIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline]
    pub fn gif5(&self) -> GIF5R {
        GIF5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline]
    pub fn teif4(&self) -> TEIF4R {
        TEIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline]
    pub fn htif4(&self) -> HTIF4R {
        HTIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline]
    pub fn tcif4(&self) -> TCIF4R {
        TCIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline]
    pub fn gif4(&self) -> GIF4R {
        GIF4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline]
    pub fn teif3(&self) -> TEIF3R {
        TEIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline]
    pub fn htif3(&self) -> HTIF3R {
        HTIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline]
    pub fn tcif3(&self) -> TCIF3R {
        TCIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline]
    pub fn gif3(&self) -> GIF3R {
        GIF3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline]
    pub fn teif2(&self) -> TEIF2R {
        TEIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline]
    pub fn htif2(&self) -> HTIF2R {
        HTIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline]
    pub fn tcif2(&self) -> TCIF2R {
        TCIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline]
    pub fn gif2(&self) -> GIF2R {
        GIF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Channel x transfer error flag (x = 1 ..7)"]
    #[inline]
    pub fn teif1(&self) -> TEIF1R {
        TEIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel x half transfer flag (x = 1 ..7)"]
    #[inline]
    pub fn htif1(&self) -> HTIF1R {
        HTIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel x transfer complete flag (x = 1 ..7)"]
    #[inline]
    pub fn tcif1(&self) -> TCIF1R {
        TCIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Channel x global interrupt flag (x = 1 ..7)"]
    #[inline]
    pub fn gif1(&self) -> GIF1R {
        GIF1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
