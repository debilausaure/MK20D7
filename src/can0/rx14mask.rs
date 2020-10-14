#[doc = "Reader of register RX14MASK"]
pub type R = crate::R<u32, super::RX14MASK>;
#[doc = "Writer for register RX14MASK"]
pub type W = crate::W<u32, super::RX14MASK>;
#[doc = "Register RX14MASK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::RX14MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Rx Buffer 14 Mask Bits\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RX14M_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care.\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked."]
    _1 = 1,
}
impl From<RX14M_A> for u32 {
    #[inline(always)]
    fn from(variant: RX14M_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RX14M`"]
pub type RX14M_R = crate::R<u32, RX14M_A>;
impl RX14M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, RX14M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX14M_A::_0),
            1 => Val(RX14M_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX14M_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX14M_A::_1
    }
}
#[doc = "Write proxy for field `RX14M`"]
pub struct RX14M_W<'a> {
    w: &'a mut W,
}
impl<'a> RX14M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX14M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The corresponding bit in the filter is \"don't care.\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX14M_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX14M_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m(&self) -> RX14M_R {
        RX14M_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Rx Buffer 14 Mask Bits"]
    #[inline(always)]
    pub fn rx14m(&mut self) -> RX14M_W {
        RX14M_W { w: self }
    }
}
