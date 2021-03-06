#[doc = "Reader of register IMASK1"]
pub type R = crate::R<u32, super::IMASK1>;
#[doc = "Writer for register IMASK1"]
pub type W = crate::W<u32, super::IMASK1>;
#[doc = "Register IMASK1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IMASK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Buffer MBi Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum BUFLM_A {
    #[doc = "0: The corresponding buffer Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The corresponding buffer Interrupt is enabled."]
    _1 = 1,
}
impl From<BUFLM_A> for u32 {
    #[inline(always)]
    fn from(variant: BUFLM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BUFLM`"]
pub type BUFLM_R = crate::R<u32, BUFLM_A>;
impl BUFLM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, BUFLM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BUFLM_A::_0),
            1 => Val(BUFLM_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFLM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFLM_A::_1
    }
}
#[doc = "Write proxy for field `BUFLM`"]
pub struct BUFLM_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFLM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFLM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding buffer Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFLM_A::_0)
    }
    #[doc = "The corresponding buffer Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFLM_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer MBi Mask"]
    #[inline(always)]
    pub fn buflm(&self) -> BUFLM_R {
        BUFLM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MBi Mask"]
    #[inline(always)]
    pub fn buflm(&mut self) -> BUFLM_W {
        BUFLM_W { w: self }
    }
}
