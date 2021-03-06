#[doc = "Reader of register CMD2"]
pub type R = crate::R<u8, super::CMD2>;
#[doc = "Writer for register CMD2"]
pub type W = crate::W<u8, super::CMD2>;
#[doc = "Register CMD2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MB`"]
pub type MB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MB`"]
pub struct MB_W<'a> {
    w: &'a mut W,
}
impl<'a> MB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W {
        MB_W { w: self }
    }
}
