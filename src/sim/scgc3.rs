#[doc = "Reader of register SCGC3"]
pub type R = crate::R<u32, super::SCGC3>;
#[doc = "Writer for register SCGC3"]
pub type W = crate::W<u32, super::SCGC3>;
#[doc = "Register SCGC3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCGC3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FTM2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FTM2_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTM2_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FTM2`"]
pub type FTM2_R = crate::R<bool, FTM2_A>;
impl FTM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2_A {
        match self.bits {
            false => FTM2_A::_0,
            true => FTM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2_A::_1
    }
}
#[doc = "Write proxy for field `FTM2`"]
pub struct FTM2_W<'a> {
    w: &'a mut W,
}
impl<'a> FTM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "ADC1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<ADC1_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC1`"]
pub type ADC1_R = crate::R<bool, ADC1_A>;
impl ADC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1_A {
        match self.bits {
            false => ADC1_A::_0,
            true => ADC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC1_A::_1
    }
}
#[doc = "Write proxy for field `ADC1`"]
pub struct ADC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC1_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&self) -> FTM2_R {
        FTM2_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ADC1 Clock Gate Control"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&mut self) -> FTM2_W {
        FTM2_W { w: self }
    }
    #[doc = "Bit 27 - ADC1 Clock Gate Control"]
    #[inline(always)]
    pub fn adc1(&mut self) -> ADC1_W {
        ADC1_W { w: self }
    }
}
