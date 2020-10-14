#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Control Register n"]
    pub pcr: [PCR; 32],
    #[doc = "0x80 - Global Pin Control Low Register"]
    pub gpclr: GPCLR,
    #[doc = "0x84 - Global Pin Control High Register"]
    pub gpchr: GPCHR,
    _reserved3: [u8; 24usize],
    #[doc = "0xa0 - Interrupt Status Flag Register"]
    pub isfr: ISFR,
    _reserved4: [u8; 28usize],
    #[doc = "0xc0 - Digital Filter Enable Register"]
    pub dfer: DFER,
    #[doc = "0xc4 - Digital Filter Clock Register"]
    pub dfcr: DFCR,
    #[doc = "0xc8 - Digital Filter Width Register"]
    pub dfwr: DFWR,
}
#[doc = "Pin Control Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](pcr) module"]
pub type PCR = crate::Reg<u32, _PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCR;
#[doc = "`read()` method returns [pcr::R](pcr::R) reader structure"]
impl crate::Readable for PCR {}
#[doc = "`write(|w| ..)` method takes [pcr::W](pcr::W) writer structure"]
impl crate::Writable for PCR {}
#[doc = "Pin Control Register n"]
pub mod pcr;
#[doc = "Global Pin Control Low Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpclr](gpclr) module"]
pub type GPCLR = crate::Reg<u32, _GPCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCLR;
#[doc = "`write(|w| ..)` method takes [gpclr::W](gpclr::W) writer structure"]
impl crate::Writable for GPCLR {}
#[doc = "Global Pin Control Low Register"]
pub mod gpclr;
#[doc = "Global Pin Control High Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpchr](gpchr) module"]
pub type GPCHR = crate::Reg<u32, _GPCHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCHR;
#[doc = "`write(|w| ..)` method takes [gpchr::W](gpchr::W) writer structure"]
impl crate::Writable for GPCHR {}
#[doc = "Global Pin Control High Register"]
pub mod gpchr;
#[doc = "Interrupt Status Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isfr](isfr) module"]
pub type ISFR = crate::Reg<u32, _ISFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISFR;
#[doc = "`read()` method returns [isfr::R](isfr::R) reader structure"]
impl crate::Readable for ISFR {}
#[doc = "`write(|w| ..)` method takes [isfr::W](isfr::W) writer structure"]
impl crate::Writable for ISFR {}
#[doc = "Interrupt Status Flag Register"]
pub mod isfr;
#[doc = "Digital Filter Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfer](dfer) module"]
pub type DFER = crate::Reg<u32, _DFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFER;
#[doc = "`read()` method returns [dfer::R](dfer::R) reader structure"]
impl crate::Readable for DFER {}
#[doc = "`write(|w| ..)` method takes [dfer::W](dfer::W) writer structure"]
impl crate::Writable for DFER {}
#[doc = "Digital Filter Enable Register"]
pub mod dfer;
#[doc = "Digital Filter Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfcr](dfcr) module"]
pub type DFCR = crate::Reg<u32, _DFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFCR;
#[doc = "`read()` method returns [dfcr::R](dfcr::R) reader structure"]
impl crate::Readable for DFCR {}
#[doc = "`write(|w| ..)` method takes [dfcr::W](dfcr::W) writer structure"]
impl crate::Writable for DFCR {}
#[doc = "Digital Filter Clock Register"]
pub mod dfcr;
#[doc = "Digital Filter Width Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfwr](dfwr) module"]
pub type DFWR = crate::Reg<u32, _DFWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFWR;
#[doc = "`read()` method returns [dfwr::R](dfwr::R) reader structure"]
impl crate::Readable for DFWR {}
#[doc = "`write(|w| ..)` method takes [dfwr::W](dfwr::W) writer structure"]
impl crate::Writable for DFWR {}
#[doc = "Digital Filter Width Register"]
pub mod dfwr;
