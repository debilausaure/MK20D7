#[doc = "Reader of register CRCLL"]
pub type R = crate::R<u8, super::CRCLL>;
#[doc = "Writer for register CRCLL"]
pub type W = crate::W<u8, super::CRCLL>;
#[doc = "Register CRCLL `reset()`'s with value 0xff"]
impl crate::ResetValue for super::CRCLL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `CRCLL`"]
pub type CRCLL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRCLL`"]
pub struct CRCLL_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CRCLL stores the first 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crcll(&self) -> CRCLL_R {
        CRCLL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRCLL stores the first 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crcll(&mut self) -> CRCLL_W {
        CRCLL_W { w: self }
    }
}
