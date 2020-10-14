#[doc = "Reader of register CRCL"]
pub type R = crate::R<u16, super::CRCL>;
#[doc = "Writer for register CRCL"]
pub type W = crate::W<u16, super::CRCL>;
#[doc = "Register CRCL `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CRCL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `CRCL`"]
pub type CRCL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRCL`"]
pub struct CRCL_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRCL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn crcl(&self) -> CRCL_R {
        CRCL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRCL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn crcl(&mut self) -> CRCL_W {
        CRCL_W { w: self }
    }
}
