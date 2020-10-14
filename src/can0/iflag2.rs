#[doc = "Reader of register IFLAG2"]
pub type R = crate::R<u32, super::IFLAG2>;
#[doc = "Writer for register IFLAG2"]
pub type W = crate::W<u32, super::IFLAG2>;
#[doc = "Register IFLAG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::IFLAG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Buffer MBi Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum BUFHI_A {
    #[doc = "0: The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    _0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception."]
    _1 = 1,
}
impl From<BUFHI_A> for u32 {
    #[inline(always)]
    fn from(variant: BUFHI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BUFHI`"]
pub type BUFHI_R = crate::R<u32, BUFHI_A>;
impl BUFHI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, BUFHI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BUFHI_A::_0),
            1 => Val(BUFHI_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUFHI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUFHI_A::_1
    }
}
#[doc = "Write proxy for field `BUFHI`"]
pub struct BUFHI_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFHI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUFHI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding buffer has no occurrence of successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BUFHI_A::_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BUFHI_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn bufhi(&self) -> BUFHI_R {
        BUFHI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn bufhi(&mut self) -> BUFHI_W {
        BUFHI_W { w: self }
    }
}
