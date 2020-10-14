#[doc = "Reader of register CRCHL"]
pub type R = crate::R<u8, super::CRCHL>;
#[doc = "Writer for register CRCHL"]
pub type W = crate::W<u8, super::CRCHL>;
#[doc = "Register CRCHL `reset()`'s with value 0xff"]
impl crate::ResetValue for super::CRCHL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `CRCHL`"]
pub type CRCHL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRCHL`"]
pub struct CRCHL_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCHL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CRCHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crchl(&self) -> CRCHL_R {
        CRCHL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CRCHL stores the third 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn crchl(&mut self) -> CRCHL_W {
        CRCHL_W { w: self }
    }
}
