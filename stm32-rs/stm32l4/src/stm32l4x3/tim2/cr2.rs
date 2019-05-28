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
#[doc = "Possible values of the field `TI1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TI1SR {
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    NORMAL,
    #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    XOR,
}
impl TI1SR {
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
            TI1SR::NORMAL => false,
            TI1SR::XOR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TI1SR {
        match value {
            false => TI1SR::NORMAL,
            true => TI1SR::XOR,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == TI1SR::NORMAL
    }
    #[doc = "Checks if the value of the field is `XOR`"]
    #[inline]
    pub fn is_xor(&self) -> bool {
        *self == TI1SR::XOR
    }
}
#[doc = "Possible values of the field `MMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMSR {
    #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
    RESET,
    #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
    ENABLE,
    #[doc = "The update event is selected as trigger output"]
    UPDATE,
    #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
    COMPAREPULSE,
    #[doc = "OC1REF signal is used as trigger output"]
    COMPAREOC1,
    #[doc = "OC2REF signal is used as trigger output"]
    COMPAREOC2,
    #[doc = "OC3REF signal is used as trigger output"]
    COMPAREOC3,
    #[doc = "OC4REF signal is used as trigger output"]
    COMPAREOC4,
}
impl MMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MMSR::RESET => 0,
            MMSR::ENABLE => 1,
            MMSR::UPDATE => 2,
            MMSR::COMPAREPULSE => 3,
            MMSR::COMPAREOC1 => 4,
            MMSR::COMPAREOC2 => 5,
            MMSR::COMPAREOC3 => 6,
            MMSR::COMPAREOC4 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MMSR {
        match value {
            0 => MMSR::RESET,
            1 => MMSR::ENABLE,
            2 => MMSR::UPDATE,
            3 => MMSR::COMPAREPULSE,
            4 => MMSR::COMPAREOC1,
            5 => MMSR::COMPAREOC2,
            6 => MMSR::COMPAREOC3,
            7 => MMSR::COMPAREOC4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == MMSR::RESET
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MMSR::ENABLE
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline]
    pub fn is_update(&self) -> bool {
        *self == MMSR::UPDATE
    }
    #[doc = "Checks if the value of the field is `COMPAREPULSE`"]
    #[inline]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMSR::COMPAREPULSE
    }
    #[doc = "Checks if the value of the field is `COMPAREOC1`"]
    #[inline]
    pub fn is_compare_oc1(&self) -> bool {
        *self == MMSR::COMPAREOC1
    }
    #[doc = "Checks if the value of the field is `COMPAREOC2`"]
    #[inline]
    pub fn is_compare_oc2(&self) -> bool {
        *self == MMSR::COMPAREOC2
    }
    #[doc = "Checks if the value of the field is `COMPAREOC3`"]
    #[inline]
    pub fn is_compare_oc3(&self) -> bool {
        *self == MMSR::COMPAREOC3
    }
    #[doc = "Checks if the value of the field is `COMPAREOC4`"]
    #[inline]
    pub fn is_compare_oc4(&self) -> bool {
        *self == MMSR::COMPAREOC4
    }
}
#[doc = "Possible values of the field `CCDS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCDSR {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    ONCOMPARE,
    #[doc = "CCx DMA request sent when update event occurs"]
    ONUPDATE,
}
impl CCDSR {
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
            CCDSR::ONCOMPARE => false,
            CCDSR::ONUPDATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CCDSR {
        match value {
            false => CCDSR::ONCOMPARE,
            true => CCDSR::ONUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `ONCOMPARE`"]
    #[inline]
    pub fn is_on_compare(&self) -> bool {
        *self == CCDSR::ONCOMPARE
    }
    #[doc = "Checks if the value of the field is `ONUPDATE`"]
    #[inline]
    pub fn is_on_update(&self) -> bool {
        *self == CCDSR::ONUPDATE
    }
}
#[doc = "Values that can be written to the field `TI1S`"]
pub enum TI1SW {
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    NORMAL,
    #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    XOR,
}
impl TI1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TI1SW::NORMAL => false,
            TI1SW::XOR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TI1SW<'a> {
    w: &'a mut W,
}
impl<'a> _TI1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TI1SW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(TI1SW::NORMAL)
    }
    #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    #[inline]
    pub fn xor(self) -> &'a mut W {
        self.variant(TI1SW::XOR)
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
#[doc = "Values that can be written to the field `MMS`"]
pub enum MMSW {
    #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
    RESET,
    #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
    ENABLE,
    #[doc = "The update event is selected as trigger output"]
    UPDATE,
    #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
    COMPAREPULSE,
    #[doc = "OC1REF signal is used as trigger output"]
    COMPAREOC1,
    #[doc = "OC2REF signal is used as trigger output"]
    COMPAREOC2,
    #[doc = "OC3REF signal is used as trigger output"]
    COMPAREOC3,
    #[doc = "OC4REF signal is used as trigger output"]
    COMPAREOC4,
}
impl MMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MMSW::RESET => 0,
            MMSW::ENABLE => 1,
            MMSW::UPDATE => 2,
            MMSW::COMPAREPULSE => 3,
            MMSW::COMPAREOC1 => 4,
            MMSW::COMPAREOC2 => 5,
            MMSW::COMPAREOC3 => 6,
            MMSW::COMPAREOC4 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMSW<'a> {
    w: &'a mut W,
}
impl<'a> _MMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMSW::RESET)
    }
    #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMSW::ENABLE)
    }
    #[doc = "The update event is selected as trigger output"]
    #[inline]
    pub fn update(self) -> &'a mut W {
        self.variant(MMSW::UPDATE)
    }
    #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
    #[inline]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMSW::COMPAREPULSE)
    }
    #[doc = "OC1REF signal is used as trigger output"]
    #[inline]
    pub fn compare_oc1(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC1)
    }
    #[doc = "OC2REF signal is used as trigger output"]
    #[inline]
    pub fn compare_oc2(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC2)
    }
    #[doc = "OC3REF signal is used as trigger output"]
    #[inline]
    pub fn compare_oc3(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC3)
    }
    #[doc = "OC4REF signal is used as trigger output"]
    #[inline]
    pub fn compare_oc4(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CCDS`"]
pub enum CCDSW {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    ONCOMPARE,
    #[doc = "CCx DMA request sent when update event occurs"]
    ONUPDATE,
}
impl CCDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CCDSW::ONCOMPARE => false,
            CCDSW::ONUPDATE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CCDSW<'a> {
    w: &'a mut W,
}
impl<'a> _CCDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CCDSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline]
    pub fn on_compare(self) -> &'a mut W {
        self.variant(CCDSW::ONCOMPARE)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline]
    pub fn on_update(self) -> &'a mut W {
        self.variant(CCDSW::ONUPDATE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline]
    pub fn ti1s(&self) -> TI1SR {
        TI1SR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline]
    pub fn mms(&self) -> MMSR {
        MMSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline]
    pub fn ccds(&self) -> CCDSR {
        CCDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 7 - TI1 selection"]
    #[inline]
    pub fn ti1s(&mut self) -> _TI1SW {
        _TI1SW { w: self }
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline]
    pub fn mms(&mut self) -> _MMSW {
        _MMSW { w: self }
    }
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline]
    pub fn ccds(&mut self) -> _CCDSW {
        _CCDSW { w: self }
    }
}
