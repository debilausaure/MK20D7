#[doc = "Reader of register THRESHOLD"]
pub type R = crate::R<u32, super::THRESHOLD>;
#[doc = "Writer for register THRESHOLD"]
pub type W = crate::W<u32, super::THRESHOLD>;
#[doc = "Register THRESHOLD `reset()`'s with value 0"]
impl crate::ResetValue for super::THRESHOLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HTHH`"]
pub type HTHH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HTHH`"]
pub struct HTHH_W<'a> {
    w: &'a mut W,
}
impl<'a> HTHH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `LTHH`"]
pub type LTHH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LTHH`"]
pub struct LTHH_W<'a> {
    w: &'a mut W,
}
impl<'a> LTHH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Touch Sensing Channel High Threshold value"]
    #[inline(always)]
    pub fn hthh(&self) -> HTHH_R {
        HTHH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Touch Sensing Channel Low Threshold value"]
    #[inline(always)]
    pub fn lthh(&self) -> LTHH_R {
        LTHH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Touch Sensing Channel High Threshold value"]
    #[inline(always)]
    pub fn hthh(&mut self) -> HTHH_W {
        HTHH_W { w: self }
    }
    #[doc = "Bits 16:31 - Touch Sensing Channel Low Threshold value"]
    #[inline(always)]
    pub fn lthh(&mut self) -> LTHH_W {
        LTHH_W { w: self }
    }
}
