#[doc = "Reader of register RXIMR%s"]
pub type R = crate::R<u32, super::RXIMR>;
#[doc = "Writer for register RXIMR%s"]
pub type W = crate::W<u32, super::RXIMR>;
#[doc = "Register RXIMR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::RXIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Individual Mask Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum MI_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<MI_A> for u32 {
    #[inline(always)]
    fn from(variant: MI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MI`"]
pub type MI_R = crate::R<u32, MI_A>;
impl MI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, MI_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MI_A::_0),
            1 => Val(MI_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MI_A::_1
    }
}
#[doc = "Write proxy for field `MI`"]
pub struct MI_W<'a> {
    w: &'a mut W,
}
impl<'a> MI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MI_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MI_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MI_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi(&self) -> MI_R {
        MI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Individual Mask Bits"]
    #[inline(always)]
    pub fn mi(&mut self) -> MI_W {
        MI_W { w: self }
    }
}
