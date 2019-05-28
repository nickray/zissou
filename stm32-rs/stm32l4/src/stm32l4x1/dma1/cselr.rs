#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSELR {
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
#[doc = "Possible values of the field `C7S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum C7SR {
    #[doc = "Default mapping"]
    NOMAPPING,
    #[doc = "Mapping 1"]
    MAP1,
    #[doc = "Mapping 2"]
    MAP2,
    #[doc = "Mapping 3"]
    MAP3,
    #[doc = "Mapping 4"]
    MAP4,
    #[doc = "Mapping 5"]
    MAP5,
    #[doc = "Mapping 6"]
    MAP6,
    #[doc = "Mapping 7"]
    MAP7,
    #[doc = "Mapping 8"]
    MAP8,
    #[doc = "Mapping 9"]
    MAP9,
    #[doc = "Mapping 10"]
    MAP10,
    #[doc = "Mapping 11"]
    MAP11,
    #[doc = "Mapping 12"]
    MAP12,
    #[doc = "Mapping 13"]
    MAP13,
    #[doc = "Mapping 14"]
    MAP14,
    #[doc = "Mapping 15"]
    MAP15,
}
impl C7SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            C7SR::NOMAPPING => 0,
            C7SR::MAP1 => 1,
            C7SR::MAP2 => 2,
            C7SR::MAP3 => 3,
            C7SR::MAP4 => 4,
            C7SR::MAP5 => 5,
            C7SR::MAP6 => 6,
            C7SR::MAP7 => 7,
            C7SR::MAP8 => 8,
            C7SR::MAP9 => 9,
            C7SR::MAP10 => 10,
            C7SR::MAP11 => 11,
            C7SR::MAP12 => 12,
            C7SR::MAP13 => 13,
            C7SR::MAP14 => 14,
            C7SR::MAP15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> C7SR {
        match value {
            0 => C7SR::NOMAPPING,
            1 => C7SR::MAP1,
            2 => C7SR::MAP2,
            3 => C7SR::MAP3,
            4 => C7SR::MAP4,
            5 => C7SR::MAP5,
            6 => C7SR::MAP6,
            7 => C7SR::MAP7,
            8 => C7SR::MAP8,
            9 => C7SR::MAP9,
            10 => C7SR::MAP10,
            11 => C7SR::MAP11,
            12 => C7SR::MAP12,
            13 => C7SR::MAP13,
            14 => C7SR::MAP14,
            15 => C7SR::MAP15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOMAPPING`"]
    #[inline]
    pub fn is_no_mapping(&self) -> bool {
        *self == C7SR::NOMAPPING
    }
    #[doc = "Checks if the value of the field is `MAP1`"]
    #[inline]
    pub fn is_map1(&self) -> bool {
        *self == C7SR::MAP1
    }
    #[doc = "Checks if the value of the field is `MAP2`"]
    #[inline]
    pub fn is_map2(&self) -> bool {
        *self == C7SR::MAP2
    }
    #[doc = "Checks if the value of the field is `MAP3`"]
    #[inline]
    pub fn is_map3(&self) -> bool {
        *self == C7SR::MAP3
    }
    #[doc = "Checks if the value of the field is `MAP4`"]
    #[inline]
    pub fn is_map4(&self) -> bool {
        *self == C7SR::MAP4
    }
    #[doc = "Checks if the value of the field is `MAP5`"]
    #[inline]
    pub fn is_map5(&self) -> bool {
        *self == C7SR::MAP5
    }
    #[doc = "Checks if the value of the field is `MAP6`"]
    #[inline]
    pub fn is_map6(&self) -> bool {
        *self == C7SR::MAP6
    }
    #[doc = "Checks if the value of the field is `MAP7`"]
    #[inline]
    pub fn is_map7(&self) -> bool {
        *self == C7SR::MAP7
    }
    #[doc = "Checks if the value of the field is `MAP8`"]
    #[inline]
    pub fn is_map8(&self) -> bool {
        *self == C7SR::MAP8
    }
    #[doc = "Checks if the value of the field is `MAP9`"]
    #[inline]
    pub fn is_map9(&self) -> bool {
        *self == C7SR::MAP9
    }
    #[doc = "Checks if the value of the field is `MAP10`"]
    #[inline]
    pub fn is_map10(&self) -> bool {
        *self == C7SR::MAP10
    }
    #[doc = "Checks if the value of the field is `MAP11`"]
    #[inline]
    pub fn is_map11(&self) -> bool {
        *self == C7SR::MAP11
    }
    #[doc = "Checks if the value of the field is `MAP12`"]
    #[inline]
    pub fn is_map12(&self) -> bool {
        *self == C7SR::MAP12
    }
    #[doc = "Checks if the value of the field is `MAP13`"]
    #[inline]
    pub fn is_map13(&self) -> bool {
        *self == C7SR::MAP13
    }
    #[doc = "Checks if the value of the field is `MAP14`"]
    #[inline]
    pub fn is_map14(&self) -> bool {
        *self == C7SR::MAP14
    }
    #[doc = "Checks if the value of the field is `MAP15`"]
    #[inline]
    pub fn is_map15(&self) -> bool {
        *self == C7SR::MAP15
    }
}
#[doc = "Possible values of the field `C6S`"]
pub type C6SR = C7SR;
#[doc = "Possible values of the field `C5S`"]
pub type C5SR = C7SR;
#[doc = "Possible values of the field `C4S`"]
pub type C4SR = C7SR;
#[doc = "Possible values of the field `C3S`"]
pub type C3SR = C7SR;
#[doc = "Possible values of the field `C2S`"]
pub type C2SR = C7SR;
#[doc = "Possible values of the field `C1S`"]
pub type C1SR = C7SR;
#[doc = "Values that can be written to the field `C7S`"]
pub enum C7SW {
    #[doc = "Default mapping"]
    NOMAPPING,
    #[doc = "Mapping 1"]
    MAP1,
    #[doc = "Mapping 2"]
    MAP2,
    #[doc = "Mapping 3"]
    MAP3,
    #[doc = "Mapping 4"]
    MAP4,
    #[doc = "Mapping 5"]
    MAP5,
    #[doc = "Mapping 6"]
    MAP6,
    #[doc = "Mapping 7"]
    MAP7,
    #[doc = "Mapping 8"]
    MAP8,
    #[doc = "Mapping 9"]
    MAP9,
    #[doc = "Mapping 10"]
    MAP10,
    #[doc = "Mapping 11"]
    MAP11,
    #[doc = "Mapping 12"]
    MAP12,
    #[doc = "Mapping 13"]
    MAP13,
    #[doc = "Mapping 14"]
    MAP14,
    #[doc = "Mapping 15"]
    MAP15,
}
impl C7SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            C7SW::NOMAPPING => 0,
            C7SW::MAP1 => 1,
            C7SW::MAP2 => 2,
            C7SW::MAP3 => 3,
            C7SW::MAP4 => 4,
            C7SW::MAP5 => 5,
            C7SW::MAP6 => 6,
            C7SW::MAP7 => 7,
            C7SW::MAP8 => 8,
            C7SW::MAP9 => 9,
            C7SW::MAP10 => 10,
            C7SW::MAP11 => 11,
            C7SW::MAP12 => 12,
            C7SW::MAP13 => 13,
            C7SW::MAP14 => 14,
            C7SW::MAP15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _C7SW<'a> {
    w: &'a mut W,
}
impl<'a> _C7SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C7SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default mapping"]
    #[inline]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7SW::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7SW::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7SW::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7SW::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7SW::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7SW::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7SW::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7SW::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7SW::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7SW::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7SW::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7SW::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7SW::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7SW::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7SW::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7SW::MAP15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C6S`"]
pub type C6SW = C7SW;
#[doc = r" Proxy"]
pub struct _C6SW<'a> {
    w: &'a mut W,
}
impl<'a> _C6SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C6SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default mapping"]
    #[inline]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7SW::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7SW::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7SW::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7SW::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7SW::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7SW::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7SW::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7SW::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7SW::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7SW::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7SW::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7SW::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7SW::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7SW::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7SW::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7SW::MAP15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C5S`"]
pub type C5SW = C7SW;
#[doc = r" Proxy"]
pub struct _C5SW<'a> {
    w: &'a mut W,
}
impl<'a> _C5SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C5SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default mapping"]
    #[inline]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7SW::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7SW::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7SW::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7SW::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7SW::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7SW::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7SW::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7SW::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7SW::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7SW::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7SW::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7SW::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7SW::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7SW::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7SW::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7SW::MAP15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `C4S`"]
pub type C4SW = C7SW;
#[doc = r" Proxy"]
pub struct _C4SW<'a> {
    w: &'a mut W,
}
impl<'a> _C4SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C4SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default mapping"]
    #[inline]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7SW::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7SW::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7SW::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7SW::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7SW::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7SW::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7SW::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7SW::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7SW::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7SW::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7SW::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7SW::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7SW::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7SW::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7SW::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7SW::MAP15)
    }
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
#[doc = "Values that can be written to the field `C3S`"]
pub type C3SW = C7SW;
#[doc = r" Proxy"]
pub struct _C3SW<'a> {
    w: &'a mut W,
}
impl<'a> _C3SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C3SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default mapping"]
    #[inline]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7SW::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7SW::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7SW::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7SW::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7SW::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7SW::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7SW::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7SW::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7SW::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7SW::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7SW::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7SW::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7SW::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7SW::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7SW::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7SW::MAP15)
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
#[doc = "Values that can be written to the field `C2S`"]
pub type C2SW = C7SW;
#[doc = r" Proxy"]
pub struct _C2SW<'a> {
    w: &'a mut W,
}
impl<'a> _C2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C2SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default mapping"]
    #[inline]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7SW::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7SW::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7SW::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7SW::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7SW::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7SW::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7SW::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7SW::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7SW::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7SW::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7SW::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7SW::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7SW::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7SW::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7SW::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7SW::MAP15)
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
#[doc = "Values that can be written to the field `C1S`"]
pub type C1SW = C7SW;
#[doc = r" Proxy"]
pub struct _C1SW<'a> {
    w: &'a mut W,
}
impl<'a> _C1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: C1SW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default mapping"]
    #[inline]
    pub fn no_mapping(self) -> &'a mut W {
        self.variant(C7SW::NOMAPPING)
    }
    #[doc = "Mapping 1"]
    #[inline]
    pub fn map1(self) -> &'a mut W {
        self.variant(C7SW::MAP1)
    }
    #[doc = "Mapping 2"]
    #[inline]
    pub fn map2(self) -> &'a mut W {
        self.variant(C7SW::MAP2)
    }
    #[doc = "Mapping 3"]
    #[inline]
    pub fn map3(self) -> &'a mut W {
        self.variant(C7SW::MAP3)
    }
    #[doc = "Mapping 4"]
    #[inline]
    pub fn map4(self) -> &'a mut W {
        self.variant(C7SW::MAP4)
    }
    #[doc = "Mapping 5"]
    #[inline]
    pub fn map5(self) -> &'a mut W {
        self.variant(C7SW::MAP5)
    }
    #[doc = "Mapping 6"]
    #[inline]
    pub fn map6(self) -> &'a mut W {
        self.variant(C7SW::MAP6)
    }
    #[doc = "Mapping 7"]
    #[inline]
    pub fn map7(self) -> &'a mut W {
        self.variant(C7SW::MAP7)
    }
    #[doc = "Mapping 8"]
    #[inline]
    pub fn map8(self) -> &'a mut W {
        self.variant(C7SW::MAP8)
    }
    #[doc = "Mapping 9"]
    #[inline]
    pub fn map9(self) -> &'a mut W {
        self.variant(C7SW::MAP9)
    }
    #[doc = "Mapping 10"]
    #[inline]
    pub fn map10(self) -> &'a mut W {
        self.variant(C7SW::MAP10)
    }
    #[doc = "Mapping 11"]
    #[inline]
    pub fn map11(self) -> &'a mut W {
        self.variant(C7SW::MAP11)
    }
    #[doc = "Mapping 12"]
    #[inline]
    pub fn map12(self) -> &'a mut W {
        self.variant(C7SW::MAP12)
    }
    #[doc = "Mapping 13"]
    #[inline]
    pub fn map13(self) -> &'a mut W {
        self.variant(C7SW::MAP13)
    }
    #[doc = "Mapping 14"]
    #[inline]
    pub fn map14(self) -> &'a mut W {
        self.variant(C7SW::MAP14)
    }
    #[doc = "Mapping 15"]
    #[inline]
    pub fn map15(self) -> &'a mut W {
        self.variant(C7SW::MAP15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline]
    pub fn c7s(&self) -> C7SR {
        C7SR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline]
    pub fn c6s(&self) -> C6SR {
        C6SR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline]
    pub fn c5s(&self) -> C5SR {
        C5SR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline]
    pub fn c4s(&self) -> C4SR {
        C4SR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline]
    pub fn c3s(&self) -> C3SR {
        C3SR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline]
    pub fn c2s(&self) -> C2SR {
        C2SR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline]
    pub fn c1s(&self) -> C1SR {
        C1SR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline]
    pub fn c7s(&mut self) -> _C7SW {
        _C7SW { w: self }
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline]
    pub fn c6s(&mut self) -> _C6SW {
        _C6SW { w: self }
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline]
    pub fn c5s(&mut self) -> _C5SW {
        _C5SW { w: self }
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline]
    pub fn c4s(&mut self) -> _C4SW {
        _C4SW { w: self }
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline]
    pub fn c3s(&mut self) -> _C3SW {
        _C3SW { w: self }
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline]
    pub fn c2s(&mut self) -> _C2SW {
        _C2SW { w: self }
    }
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline]
    pub fn c1s(&mut self) -> _C1SW {
        _C1SW { w: self }
    }
}
