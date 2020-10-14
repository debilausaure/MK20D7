#[doc = "Reader of register SCGC2"]
pub type R = crate::R<u32, super::SCGC2>;
#[doc = "Writer for register SCGC2"]
pub type W = crate::W<u32, super::SCGC2>;
#[doc = "Register SCGC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCGC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DAC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DAC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DAC0`"]
pub type DAC0_R = crate::R<bool, DAC0_A>;
impl DAC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC0_A {
        match self.bits {
            false => DAC0_A::_0,
            true => DAC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAC0_A::_1
    }
}
#[doc = "Write proxy for field `DAC0`"]
pub struct DAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAC0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAC0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&self) -> DAC0_R {
        DAC0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&mut self) -> DAC0_W {
        DAC0_W { w: self }
    }
}
