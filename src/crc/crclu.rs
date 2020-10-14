#[doc = "Reader of register CRCLU"]
pub type R = crate::R<u8, super::CRCLU>;
#[doc = "Writer for register CRCLU"]
pub type W = crate::W<u8, super::CRCLU>;
#[doc = "Register CRCLU `reset()`'s with value 0xff"]
impl crate::ResetValue for super::CRCLU {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `CRCLU`"]
pub type CRCLU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRCLU`"]
pub struct CRCLU_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCLU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CRCLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crclu(&self) -> CRCLU_R {
        CRCLU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRCLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crclu(&mut self) -> CRCLU_W {
        CRCLU_W { w: self }
    }
}
