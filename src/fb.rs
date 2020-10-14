#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip select address register"]
    pub csar0: CSAR,
    #[doc = "0x04 - Chip select mask register"]
    pub csmr0: CSMR,
    #[doc = "0x08 - Chip select control register"]
    pub cscr0: CSCR,
    #[doc = "0x0c - Chip select address register"]
    pub csar1: CSAR,
    #[doc = "0x10 - Chip select mask register"]
    pub csmr1: CSMR,
    #[doc = "0x14 - Chip select control register"]
    pub cscr1: CSCR,
    #[doc = "0x18 - Chip select address register"]
    pub csar2: CSAR,
    #[doc = "0x1c - Chip select mask register"]
    pub csmr2: CSMR,
    #[doc = "0x20 - Chip select control register"]
    pub cscr2: CSCR,
    #[doc = "0x24 - Chip select address register"]
    pub csar3: CSAR,
    #[doc = "0x28 - Chip select mask register"]
    pub csmr3: CSMR,
    #[doc = "0x2c - Chip select control register"]
    pub cscr3: CSCR,
    #[doc = "0x30 - Chip select address register"]
    pub csar4: CSAR,
    #[doc = "0x34 - Chip select mask register"]
    pub csmr4: CSMR,
    #[doc = "0x38 - Chip select control register"]
    pub cscr4: CSCR,
    #[doc = "0x3c - Chip select address register"]
    pub csar5: CSAR,
    #[doc = "0x40 - Chip select mask register"]
    pub csmr5: CSMR,
    #[doc = "0x44 - Chip select control register"]
    pub cscr5: CSCR,
    _reserved18: [u8; 24usize],
    #[doc = "0x60 - Chip select port multiplexing control register"]
    pub cspmcr: CSPMCR,
}
#[doc = "Chip select address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csar](csar) module"]
pub type CSAR = crate::Reg<u32, _CSAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSAR;
#[doc = "`read()` method returns [csar::R](csar::R) reader structure"]
impl crate::Readable for CSAR {}
#[doc = "`write(|w| ..)` method takes [csar::W](csar::W) writer structure"]
impl crate::Writable for CSAR {}
#[doc = "Chip select address register"]
pub mod csar;
#[doc = "Chip select mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csmr](csmr) module"]
pub type CSMR = crate::Reg<u32, _CSMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSMR;
#[doc = "`read()` method returns [csmr::R](csmr::R) reader structure"]
impl crate::Readable for CSMR {}
#[doc = "`write(|w| ..)` method takes [csmr::W](csmr::W) writer structure"]
impl crate::Writable for CSMR {}
#[doc = "Chip select mask register"]
pub mod csmr;
#[doc = "Chip select control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cscr](cscr) module"]
pub type CSCR = crate::Reg<u32, _CSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSCR;
#[doc = "`read()` method returns [cscr::R](cscr::R) reader structure"]
impl crate::Readable for CSCR {}
#[doc = "`write(|w| ..)` method takes [cscr::W](cscr::W) writer structure"]
impl crate::Writable for CSCR {}
#[doc = "Chip select control register"]
pub mod cscr;
#[doc = "Chip select port multiplexing control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspmcr](cspmcr) module"]
pub type CSPMCR = crate::Reg<u32, _CSPMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSPMCR;
#[doc = "`read()` method returns [cspmcr::R](cspmcr::R) reader structure"]
impl crate::Readable for CSPMCR {}
#[doc = "`write(|w| ..)` method takes [cspmcr::W](cspmcr::W) writer structure"]
impl crate::Writable for CSPMCR {}
#[doc = "Chip select port multiplexing control register"]
pub mod cspmcr;
