#[doc = "Reader of register C7"]
pub type R = crate::R<u8, super::C7>;
#[doc = "Writer for register C7"]
pub type W = crate::W<u8, super::C7>;
#[doc = "Register C7 `reset()`'s with value 0"]
impl crate::ResetValue for super::C7 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCG OSC Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSEL_A {
    #[doc = "0: Selects System Oscillator (OSCCLK)."]
    _0 = 0,
    #[doc = "1: Selects 32 kHz RTC Oscillator."]
    _1 = 1,
}
impl From<OSCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OSCSEL`"]
pub type OSCSEL_R = crate::R<bool, OSCSEL_A>;
impl OSCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSEL_A {
        match self.bits {
            false => OSCSEL_A::_0,
            true => OSCSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OSCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OSCSEL_A::_1
    }
}
#[doc = "Write proxy for field `OSCSEL`"]
pub struct OSCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects System Oscillator (OSCCLK)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OSCSEL_A::_0)
    }
    #[doc = "Selects 32 kHz RTC Oscillator."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OSCSEL_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MCG OSC Clock Select"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCG OSC Clock Select"]
    #[inline(always)]
    pub fn oscsel(&mut self) -> OSCSEL_W {
        OSCSEL_W { w: self }
    }
}
