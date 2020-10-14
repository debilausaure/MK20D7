#[doc = "Reader of register CNTR%s"]
pub type R = crate::R<u32, super::CNTR>;
#[doc = "Reader of field `CTN1`"]
pub type CTN1_R = crate::R<u16, u16>;
#[doc = "Reader of field `CTN`"]
pub type CTN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - TouchSensing Channel n-1 16-bit counter value"]
    #[inline(always)]
    pub fn ctn1(&self) -> CTN1_R {
        CTN1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TouchSensing Channel n 16-bit counter value"]
    #[inline(always)]
    pub fn ctn(&self) -> CTN_R {
        CTN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
