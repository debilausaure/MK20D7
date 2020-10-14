#[doc = "Reader of register RMR"]
pub type R = crate::R<u32, super::RMR>;
#[doc = "Writer for register RMR"]
pub type W = crate::W<u32, super::RMR>;
#[doc = "Register RMR `reset()`'s with value 0"]
impl crate::ResetValue for super::RMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive word mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RWM_A {
    #[doc = "0: Word N is enabled."]
    _0 = 0,
    #[doc = "1: Word N is masked."]
    _1 = 1,
}
impl From<RWM_A> for u32 {
    #[inline(always)]
    fn from(variant: RWM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RWM`"]
pub type RWM_R = crate::R<u32, RWM_A>;
impl RWM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, RWM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RWM_A::_0),
            1 => Val(RWM_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWM_A::_1
    }
}
#[doc = "Write proxy for field `RWM`"]
pub struct RWM_W<'a> {
    w: &'a mut W,
}
impl<'a> RWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWM_A::_0)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWM_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive word mask"]
    #[inline(always)]
    pub fn rwm(&self) -> RWM_R {
        RWM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive word mask"]
    #[inline(always)]
    pub fn rwm(&mut self) -> RWM_W {
        RWM_W { w: self }
    }
}
