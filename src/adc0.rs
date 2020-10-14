#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC status and control registers 1"]
    pub sc1a: SC1,
    #[doc = "0x04 - ADC status and control registers 1"]
    pub sc1b: SC1,
    #[doc = "0x08 - ADC configuration register 1"]
    pub cfg1: CFG1,
    #[doc = "0x0c - Configuration register 2"]
    pub cfg2: CFG2,
    #[doc = "0x10 - ADC data result register"]
    pub ra: R,
    #[doc = "0x14 - ADC data result register"]
    pub rb: R,
    #[doc = "0x18 - Compare value registers"]
    pub cv1: CV,
    #[doc = "0x1c - Compare value registers"]
    pub cv2: CV,
    #[doc = "0x20 - Status and control register 2"]
    pub sc2: SC2,
    #[doc = "0x24 - Status and control register 3"]
    pub sc3: SC3,
    #[doc = "0x28 - ADC offset correction register"]
    pub ofs: OFS,
    #[doc = "0x2c - ADC plus-side gain register"]
    pub pg: PG,
    #[doc = "0x30 - ADC minus-side gain register"]
    pub mg: MG,
    #[doc = "0x34 - ADC plus-side general calibration value register"]
    pub clpd: CLPD,
    #[doc = "0x38 - ADC plus-side general calibration value register"]
    pub clps: CLPS,
    #[doc = "0x3c - ADC plus-side general calibration value register"]
    pub clp4: CLP4,
    #[doc = "0x40 - ADC plus-side general calibration value register"]
    pub clp3: CLP3,
    #[doc = "0x44 - ADC plus-side general calibration value register"]
    pub clp2: CLP2,
    #[doc = "0x48 - ADC plus-side general calibration value register"]
    pub clp1: CLP1,
    #[doc = "0x4c - ADC plus-side general calibration value register"]
    pub clp0: CLP0,
    #[doc = "0x50 - ADC PGA register"]
    pub pga: PGA,
    #[doc = "0x54 - ADC minus-side general calibration value register"]
    pub clmd: CLMD,
    #[doc = "0x58 - ADC minus-side general calibration value register"]
    pub clms: CLMS,
    #[doc = "0x5c - ADC minus-side general calibration value register"]
    pub clm4: CLM4,
    #[doc = "0x60 - ADC minus-side general calibration value register"]
    pub clm3: CLM3,
    #[doc = "0x64 - ADC minus-side general calibration value register"]
    pub clm2: CLM2,
    #[doc = "0x68 - ADC minus-side general calibration value register"]
    pub clm1: CLM1,
    #[doc = "0x6c - ADC minus-side general calibration value register"]
    pub clm0: CLM0,
}
#[doc = "ADC status and control registers 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc1](sc1) module"]
pub type SC1 = crate::Reg<u32, _SC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC1;
#[doc = "`read()` method returns [sc1::R](sc1::R) reader structure"]
impl crate::Readable for SC1 {}
#[doc = "`write(|w| ..)` method takes [sc1::W](sc1::W) writer structure"]
impl crate::Writable for SC1 {}
#[doc = "ADC status and control registers 1"]
pub mod sc1;
#[doc = "ADC configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "ADC configuration register 1"]
pub mod cfg1;
#[doc = "Configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](cfg2) module"]
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
#[doc = "`read()` method returns [cfg2::R](cfg2::R) reader structure"]
impl crate::Readable for CFG2 {}
#[doc = "`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure"]
impl crate::Writable for CFG2 {}
#[doc = "Configuration register 2"]
pub mod cfg2;
#[doc = "ADC data result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r](r) module"]
pub type R = crate::Reg<u32, _R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R;
#[doc = "`read()` method returns [r::R](r::R) reader structure"]
impl crate::Readable for R {}
#[doc = "ADC data result register"]
pub mod r;
#[doc = "Compare value registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](cv) module"]
pub type CV = crate::Reg<u32, _CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CV;
#[doc = "`read()` method returns [cv::R](cv::R) reader structure"]
impl crate::Readable for CV {}
#[doc = "`write(|w| ..)` method takes [cv::W](cv::W) writer structure"]
impl crate::Writable for CV {}
#[doc = "Compare value registers"]
pub mod cv;
#[doc = "Status and control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc2](sc2) module"]
pub type SC2 = crate::Reg<u32, _SC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC2;
#[doc = "`read()` method returns [sc2::R](sc2::R) reader structure"]
impl crate::Readable for SC2 {}
#[doc = "`write(|w| ..)` method takes [sc2::W](sc2::W) writer structure"]
impl crate::Writable for SC2 {}
#[doc = "Status and control register 2"]
pub mod sc2;
#[doc = "Status and control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc3](sc3) module"]
pub type SC3 = crate::Reg<u32, _SC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SC3;
#[doc = "`read()` method returns [sc3::R](sc3::R) reader structure"]
impl crate::Readable for SC3 {}
#[doc = "`write(|w| ..)` method takes [sc3::W](sc3::W) writer structure"]
impl crate::Writable for SC3 {}
#[doc = "Status and control register 3"]
pub mod sc3;
#[doc = "ADC offset correction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofs](ofs) module"]
pub type OFS = crate::Reg<u32, _OFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFS;
#[doc = "`read()` method returns [ofs::R](ofs::R) reader structure"]
impl crate::Readable for OFS {}
#[doc = "`write(|w| ..)` method takes [ofs::W](ofs::W) writer structure"]
impl crate::Writable for OFS {}
#[doc = "ADC offset correction register"]
pub mod ofs;
#[doc = "ADC plus-side gain register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg](pg) module"]
pub type PG = crate::Reg<u32, _PG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PG;
#[doc = "`read()` method returns [pg::R](pg::R) reader structure"]
impl crate::Readable for PG {}
#[doc = "`write(|w| ..)` method takes [pg::W](pg::W) writer structure"]
impl crate::Writable for PG {}
#[doc = "ADC plus-side gain register"]
pub mod pg;
#[doc = "ADC minus-side gain register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mg](mg) module"]
pub type MG = crate::Reg<u32, _MG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MG;
#[doc = "`read()` method returns [mg::R](mg::R) reader structure"]
impl crate::Readable for MG {}
#[doc = "`write(|w| ..)` method takes [mg::W](mg::W) writer structure"]
impl crate::Writable for MG {}
#[doc = "ADC minus-side gain register"]
pub mod mg;
#[doc = "ADC plus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clpd](clpd) module"]
pub type CLPD = crate::Reg<u32, _CLPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLPD;
#[doc = "`read()` method returns [clpd::R](clpd::R) reader structure"]
impl crate::Readable for CLPD {}
#[doc = "`write(|w| ..)` method takes [clpd::W](clpd::W) writer structure"]
impl crate::Writable for CLPD {}
#[doc = "ADC plus-side general calibration value register"]
pub mod clpd;
#[doc = "ADC plus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clps](clps) module"]
pub type CLPS = crate::Reg<u32, _CLPS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLPS;
#[doc = "`read()` method returns [clps::R](clps::R) reader structure"]
impl crate::Readable for CLPS {}
#[doc = "`write(|w| ..)` method takes [clps::W](clps::W) writer structure"]
impl crate::Writable for CLPS {}
#[doc = "ADC plus-side general calibration value register"]
pub mod clps;
#[doc = "ADC plus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp4](clp4) module"]
pub type CLP4 = crate::Reg<u32, _CLP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP4;
#[doc = "`read()` method returns [clp4::R](clp4::R) reader structure"]
impl crate::Readable for CLP4 {}
#[doc = "`write(|w| ..)` method takes [clp4::W](clp4::W) writer structure"]
impl crate::Writable for CLP4 {}
#[doc = "ADC plus-side general calibration value register"]
pub mod clp4;
#[doc = "ADC plus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp3](clp3) module"]
pub type CLP3 = crate::Reg<u32, _CLP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP3;
#[doc = "`read()` method returns [clp3::R](clp3::R) reader structure"]
impl crate::Readable for CLP3 {}
#[doc = "`write(|w| ..)` method takes [clp3::W](clp3::W) writer structure"]
impl crate::Writable for CLP3 {}
#[doc = "ADC plus-side general calibration value register"]
pub mod clp3;
#[doc = "ADC plus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp2](clp2) module"]
pub type CLP2 = crate::Reg<u32, _CLP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP2;
#[doc = "`read()` method returns [clp2::R](clp2::R) reader structure"]
impl crate::Readable for CLP2 {}
#[doc = "`write(|w| ..)` method takes [clp2::W](clp2::W) writer structure"]
impl crate::Writable for CLP2 {}
#[doc = "ADC plus-side general calibration value register"]
pub mod clp2;
#[doc = "ADC plus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp1](clp1) module"]
pub type CLP1 = crate::Reg<u32, _CLP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP1;
#[doc = "`read()` method returns [clp1::R](clp1::R) reader structure"]
impl crate::Readable for CLP1 {}
#[doc = "`write(|w| ..)` method takes [clp1::W](clp1::W) writer structure"]
impl crate::Writable for CLP1 {}
#[doc = "ADC plus-side general calibration value register"]
pub mod clp1;
#[doc = "ADC plus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp0](clp0) module"]
pub type CLP0 = crate::Reg<u32, _CLP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLP0;
#[doc = "`read()` method returns [clp0::R](clp0::R) reader structure"]
impl crate::Readable for CLP0 {}
#[doc = "`write(|w| ..)` method takes [clp0::W](clp0::W) writer structure"]
impl crate::Writable for CLP0 {}
#[doc = "ADC plus-side general calibration value register"]
pub mod clp0;
#[doc = "ADC PGA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pga](pga) module"]
pub type PGA = crate::Reg<u32, _PGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PGA;
#[doc = "`read()` method returns [pga::R](pga::R) reader structure"]
impl crate::Readable for PGA {}
#[doc = "`write(|w| ..)` method takes [pga::W](pga::W) writer structure"]
impl crate::Writable for PGA {}
#[doc = "ADC PGA register"]
pub mod pga;
#[doc = "ADC minus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clmd](clmd) module"]
pub type CLMD = crate::Reg<u32, _CLMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLMD;
#[doc = "`read()` method returns [clmd::R](clmd::R) reader structure"]
impl crate::Readable for CLMD {}
#[doc = "`write(|w| ..)` method takes [clmd::W](clmd::W) writer structure"]
impl crate::Writable for CLMD {}
#[doc = "ADC minus-side general calibration value register"]
pub mod clmd;
#[doc = "ADC minus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clms](clms) module"]
pub type CLMS = crate::Reg<u32, _CLMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLMS;
#[doc = "`read()` method returns [clms::R](clms::R) reader structure"]
impl crate::Readable for CLMS {}
#[doc = "`write(|w| ..)` method takes [clms::W](clms::W) writer structure"]
impl crate::Writable for CLMS {}
#[doc = "ADC minus-side general calibration value register"]
pub mod clms;
#[doc = "ADC minus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clm4](clm4) module"]
pub type CLM4 = crate::Reg<u32, _CLM4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLM4;
#[doc = "`read()` method returns [clm4::R](clm4::R) reader structure"]
impl crate::Readable for CLM4 {}
#[doc = "`write(|w| ..)` method takes [clm4::W](clm4::W) writer structure"]
impl crate::Writable for CLM4 {}
#[doc = "ADC minus-side general calibration value register"]
pub mod clm4;
#[doc = "ADC minus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clm3](clm3) module"]
pub type CLM3 = crate::Reg<u32, _CLM3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLM3;
#[doc = "`read()` method returns [clm3::R](clm3::R) reader structure"]
impl crate::Readable for CLM3 {}
#[doc = "`write(|w| ..)` method takes [clm3::W](clm3::W) writer structure"]
impl crate::Writable for CLM3 {}
#[doc = "ADC minus-side general calibration value register"]
pub mod clm3;
#[doc = "ADC minus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clm2](clm2) module"]
pub type CLM2 = crate::Reg<u32, _CLM2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLM2;
#[doc = "`read()` method returns [clm2::R](clm2::R) reader structure"]
impl crate::Readable for CLM2 {}
#[doc = "`write(|w| ..)` method takes [clm2::W](clm2::W) writer structure"]
impl crate::Writable for CLM2 {}
#[doc = "ADC minus-side general calibration value register"]
pub mod clm2;
#[doc = "ADC minus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clm1](clm1) module"]
pub type CLM1 = crate::Reg<u32, _CLM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLM1;
#[doc = "`read()` method returns [clm1::R](clm1::R) reader structure"]
impl crate::Readable for CLM1 {}
#[doc = "`write(|w| ..)` method takes [clm1::W](clm1::W) writer structure"]
impl crate::Writable for CLM1 {}
#[doc = "ADC minus-side general calibration value register"]
pub mod clm1;
#[doc = "ADC minus-side general calibration value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clm0](clm0) module"]
pub type CLM0 = crate::Reg<u32, _CLM0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLM0;
#[doc = "`read()` method returns [clm0::R](clm0::R) reader structure"]
impl crate::Readable for CLM0 {}
#[doc = "`write(|w| ..)` method takes [clm0::W](clm0::W) writer structure"]
impl crate::Writable for CLM0 {}
#[doc = "ADC minus-side general calibration value register"]
pub mod clm0;
