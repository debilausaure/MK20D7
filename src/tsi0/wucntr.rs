#[doc = "Reader of register WUCNTR"]
pub type R = crate::R<u32, super::WUCNTR>;
#[doc = "Reader of field `WUCNT`"]
pub type WUCNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - TouchSensing wake-up Channel 16bit counter value"]
    #[inline(always)]
    pub fn wucnt(&self) -> WUCNT_R {
        WUCNT_R::new((self.bits & 0xffff) as u16)
    }
}
