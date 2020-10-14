#[doc = "Reader of register VLLSCTRL"]
pub type R = crate::R<u8, super::VLLSCTRL>;
#[doc = "Writer for register VLLSCTRL"]
pub type W = crate::W<u8, super::VLLSCTRL>;
#[doc = "Register VLLSCTRL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::VLLSCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "VLLS Mode Control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VLLSM_A {
    #[doc = "1: VLLS1"]
    _001 = 1,
    #[doc = "2: VLLS2"]
    _010 = 2,
    #[doc = "3: VLLS3"]
    _011 = 3,
}
impl From<VLLSM_A> for u8 {
    #[inline(always)]
    fn from(variant: VLLSM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VLLSM`"]
pub type VLLSM_R = crate::R<u8, VLLSM_A>;
impl VLLSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VLLSM_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(VLLSM_A::_001),
            2 => Val(VLLSM_A::_010),
            3 => Val(VLLSM_A::_011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == VLLSM_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == VLLSM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == VLLSM_A::_011
    }
}
#[doc = "Write proxy for field `VLLSM`"]
pub struct VLLSM_W<'a> {
    w: &'a mut W,
}
impl<'a> VLLSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLLSM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VLLS1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(VLLSM_A::_001)
    }
    #[doc = "VLLS2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(VLLSM_A::_010)
    }
    #[doc = "VLLS3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(VLLSM_A::_011)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline(always)]
    pub fn vllsm(&self) -> VLLSM_R {
        VLLSM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline(always)]
    pub fn vllsm(&mut self) -> VLLSM_W {
        VLLSM_W { w: self }
    }
}
