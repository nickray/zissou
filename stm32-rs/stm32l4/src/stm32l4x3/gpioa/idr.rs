#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `IDR15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDR15R {
    #[doc = "Input is logic high"]
    HIGH,
    #[doc = "Input is logic low"]
    LOW,
}
impl IDR15R {
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
            IDR15R::HIGH => true,
            IDR15R::LOW => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDR15R {
        match value {
            true => IDR15R::HIGH,
            false => IDR15R::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == IDR15R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == IDR15R::LOW
    }
}
#[doc = "Possible values of the field `IDR14`"]
pub type IDR14R = IDR15R;
#[doc = "Possible values of the field `IDR13`"]
pub type IDR13R = IDR15R;
#[doc = "Possible values of the field `IDR12`"]
pub type IDR12R = IDR15R;
#[doc = "Possible values of the field `IDR11`"]
pub type IDR11R = IDR15R;
#[doc = "Possible values of the field `IDR10`"]
pub type IDR10R = IDR15R;
#[doc = "Possible values of the field `IDR9`"]
pub type IDR9R = IDR15R;
#[doc = "Possible values of the field `IDR8`"]
pub type IDR8R = IDR15R;
#[doc = "Possible values of the field `IDR7`"]
pub type IDR7R = IDR15R;
#[doc = "Possible values of the field `IDR6`"]
pub type IDR6R = IDR15R;
#[doc = "Possible values of the field `IDR5`"]
pub type IDR5R = IDR15R;
#[doc = "Possible values of the field `IDR4`"]
pub type IDR4R = IDR15R;
#[doc = "Possible values of the field `IDR3`"]
pub type IDR3R = IDR15R;
#[doc = "Possible values of the field `IDR2`"]
pub type IDR2R = IDR15R;
#[doc = "Possible values of the field `IDR1`"]
pub type IDR1R = IDR15R;
#[doc = "Possible values of the field `IDR0`"]
pub type IDR0R = IDR15R;
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 15 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr15(&self) -> IDR15R {
        IDR15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr14(&self) -> IDR14R {
        IDR14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr13(&self) -> IDR13R {
        IDR13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr12(&self) -> IDR12R {
        IDR12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr11(&self) -> IDR11R {
        IDR11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr10(&self) -> IDR10R {
        IDR10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr9(&self) -> IDR9R {
        IDR9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr8(&self) -> IDR8R {
        IDR8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr7(&self) -> IDR7R {
        IDR7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr6(&self) -> IDR6R {
        IDR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr5(&self) -> IDR5R {
        IDR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr4(&self) -> IDR4R {
        IDR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr3(&self) -> IDR3R {
        IDR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr2(&self) -> IDR2R {
        IDR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr1(&self) -> IDR1R {
        IDR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Port input data (y = 0..15)"]
    #[inline]
    pub fn idr0(&self) -> IDR0R {
        IDR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
