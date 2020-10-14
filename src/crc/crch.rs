#[doc = "Reader of register CRCH"]
pub type R = crate::R<u16, super::CRCH>;
#[doc = "Writer for register CRCH"]
pub type W = crate::W<u16, super::CRCH>;
#[doc = "Register CRCH `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CRCH {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `CRCH`"]
pub type CRCH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRCH`"]
pub struct CRCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRCH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn crch(&self) -> CRCH_R {
        CRCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRCH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn crch(&mut self) -> CRCH_W {
        CRCH_W { w: self }
    }
}
