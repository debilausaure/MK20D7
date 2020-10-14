#[doc = "Reader of register CRCHU"]
pub type R = crate::R<u8, super::CRCHU>;
#[doc = "Writer for register CRCHU"]
pub type W = crate::W<u8, super::CRCHU>;
#[doc = "Register CRCHU `reset()`'s with value 0xff"]
impl crate::ResetValue for super::CRCHU {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `CRCHU`"]
pub type CRCHU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRCHU`"]
pub struct CRCHU_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCHU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CRCHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crchu(&self) -> CRCHU_R {
        CRCHU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRCHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crchu(&mut self) -> CRCHU_W {
        CRCHU_W { w: self }
    }
}
