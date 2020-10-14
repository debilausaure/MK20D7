#[doc = "Reader of register FLT"]
pub type R = crate::R<u8, super::FLT>;
#[doc = "Writer for register FLT"]
pub type W = crate::W<u8, super::FLT>;
#[doc = "Register FLT `reset()`'s with value 0"]
impl crate::ResetValue for super::FLT {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I2C programmable filter factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLT_A {
    #[doc = "0: No filter/bypass"]
    _0 = 0,
}
impl From<FLT_A> for u8 {
    #[inline(always)]
    fn from(variant: FLT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLT`"]
pub type FLT_R = crate::R<u8, FLT_A>;
impl FLT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLT_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT_A::_0
    }
}
#[doc = "Write proxy for field `FLT`"]
pub struct FLT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No filter/bypass"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT_A::_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - I2C programmable filter factor"]
    #[inline(always)]
    pub fn flt(&self) -> FLT_R {
        FLT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - I2C programmable filter factor"]
    #[inline(always)]
    pub fn flt(&mut self) -> FLT_W {
        FLT_W { w: self }
    }
}
