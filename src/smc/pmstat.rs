#[doc = "Reader of register PMSTAT"]
pub type R = crate::R<u8, super::PMSTAT>;
#[doc = "Reader of field `PMSTAT`"]
pub type PMSTAT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - no description available"]
    #[inline(always)]
    pub fn pmstat(&self) -> PMSTAT_R {
        PMSTAT_R::new((self.bits & 0x7f) as u8)
    }
}
