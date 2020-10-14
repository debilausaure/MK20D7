#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Control and Status Register"]
    pub gencs: GENCS,
    #[doc = "0x04 - SCAN Control Register"]
    pub scanc: SCANC,
    #[doc = "0x08 - Pin Enable Register"]
    pub pen: PEN,
    #[doc = "0x0c - Wake-Up Channel Counter Register"]
    pub wucntr: WUCNTR,
    _reserved4: [u8; 240usize],
    #[doc = "0x100 - Counter Register"]
    pub cntr1: CNTR,
    #[doc = "0x104 - Counter Register"]
    pub cntr3: CNTR,
    #[doc = "0x108 - Counter Register"]
    pub cntr5: CNTR,
    #[doc = "0x10c - Counter Register"]
    pub cntr7: CNTR,
    #[doc = "0x110 - Counter Register"]
    pub cntr9: CNTR,
    #[doc = "0x114 - Counter Register"]
    pub cntr11: CNTR,
    #[doc = "0x118 - Counter Register"]
    pub cntr13: CNTR,
    #[doc = "0x11c - Counter Register"]
    pub cntr15: CNTR,
    #[doc = "0x120 - Low Power Channel Threshold Register"]
    pub threshold: THRESHOLD,
}
#[doc = "General Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gencs](gencs) module"]
pub type GENCS = crate::Reg<u32, _GENCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GENCS;
#[doc = "`read()` method returns [gencs::R](gencs::R) reader structure"]
impl crate::Readable for GENCS {}
#[doc = "`write(|w| ..)` method takes [gencs::W](gencs::W) writer structure"]
impl crate::Writable for GENCS {}
#[doc = "General Control and Status Register"]
pub mod gencs;
#[doc = "SCAN Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanc](scanc) module"]
pub type SCANC = crate::Reg<u32, _SCANC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCANC;
#[doc = "`read()` method returns [scanc::R](scanc::R) reader structure"]
impl crate::Readable for SCANC {}
#[doc = "`write(|w| ..)` method takes [scanc::W](scanc::W) writer structure"]
impl crate::Writable for SCANC {}
#[doc = "SCAN Control Register"]
pub mod scanc;
#[doc = "Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pen](pen) module"]
pub type PEN = crate::Reg<u32, _PEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PEN;
#[doc = "`read()` method returns [pen::R](pen::R) reader structure"]
impl crate::Readable for PEN {}
#[doc = "`write(|w| ..)` method takes [pen::W](pen::W) writer structure"]
impl crate::Writable for PEN {}
#[doc = "Pin Enable Register"]
pub mod pen;
#[doc = "Wake-Up Channel Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wucntr](wucntr) module"]
pub type WUCNTR = crate::Reg<u32, _WUCNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WUCNTR;
#[doc = "`read()` method returns [wucntr::R](wucntr::R) reader structure"]
impl crate::Readable for WUCNTR {}
#[doc = "Wake-Up Channel Counter Register"]
pub mod wucntr;
#[doc = "Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](cntr) module"]
pub type CNTR = crate::Reg<u32, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
#[doc = "`read()` method returns [cntr::R](cntr::R) reader structure"]
impl crate::Readable for CNTR {}
#[doc = "Counter Register"]
pub mod cntr;
#[doc = "Low Power Channel Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [threshold](threshold) module"]
pub type THRESHOLD = crate::Reg<u32, _THRESHOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THRESHOLD;
#[doc = "`read()` method returns [threshold::R](threshold::R) reader structure"]
impl crate::Readable for THRESHOLD {}
#[doc = "`write(|w| ..)` method takes [threshold::W](threshold::W) writer structure"]
impl crate::Writable for THRESHOLD {}
#[doc = "Low Power Channel Threshold Register"]
pub mod threshold;
