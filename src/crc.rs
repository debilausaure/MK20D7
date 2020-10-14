#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_crc: [u8; 4usize],
    _reserved_1_gpoly: [u8; 4usize],
    _reserved_2_ctrl: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x00 - CRC_CRCLL register."]
    #[inline(always)]
    pub fn crcll(&self) -> &CRCLL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CRCLL) }
    }
    #[doc = "0x00 - CRC_CRCLL register."]
    #[inline(always)]
    pub fn crcll_mut(&self) -> &mut CRCLL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CRCLL) }
    }
    #[doc = "0x00 - CRC_CRCL register."]
    #[inline(always)]
    pub fn crcl(&self) -> &CRCL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CRCL) }
    }
    #[doc = "0x00 - CRC_CRCL register."]
    #[inline(always)]
    pub fn crcl_mut(&self) -> &mut CRCL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CRCL) }
    }
    #[doc = "0x00 - CRC Data Register"]
    #[inline(always)]
    pub fn crc(&self) -> &CRC {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CRC) }
    }
    #[doc = "0x00 - CRC Data Register"]
    #[inline(always)]
    pub fn crc_mut(&self) -> &mut CRC {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CRC) }
    }
    #[doc = "0x01 - CRC_CRCLU register."]
    #[inline(always)]
    pub fn crclu(&self) -> &CRCLU {
        unsafe { &*(((self as *const Self) as *const u8).add(1usize) as *const CRCLU) }
    }
    #[doc = "0x01 - CRC_CRCLU register."]
    #[inline(always)]
    pub fn crclu_mut(&self) -> &mut CRCLU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1usize) as *mut CRCLU) }
    }
    #[doc = "0x02 - CRC_CRCHL register."]
    #[inline(always)]
    pub fn crchl(&self) -> &CRCHL {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const CRCHL) }
    }
    #[doc = "0x02 - CRC_CRCHL register."]
    #[inline(always)]
    pub fn crchl_mut(&self) -> &mut CRCHL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2usize) as *mut CRCHL) }
    }
    #[doc = "0x02 - CRC_CRCH register."]
    #[inline(always)]
    pub fn crch(&self) -> &CRCH {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const CRCH) }
    }
    #[doc = "0x02 - CRC_CRCH register."]
    #[inline(always)]
    pub fn crch_mut(&self) -> &mut CRCH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2usize) as *mut CRCH) }
    }
    #[doc = "0x03 - CRC_CRCHU register."]
    #[inline(always)]
    pub fn crchu(&self) -> &CRCHU {
        unsafe { &*(((self as *const Self) as *const u8).add(3usize) as *const CRCHU) }
    }
    #[doc = "0x03 - CRC_CRCHU register."]
    #[inline(always)]
    pub fn crchu_mut(&self) -> &mut CRCHU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(3usize) as *mut CRCHU) }
    }
    #[doc = "0x04 - CRC_GPOLYLL register."]
    #[inline(always)]
    pub fn gpolyll(&self) -> &GPOLYLL {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const GPOLYLL) }
    }
    #[doc = "0x04 - CRC_GPOLYLL register."]
    #[inline(always)]
    pub fn gpolyll_mut(&self) -> &mut GPOLYLL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut GPOLYLL) }
    }
    #[doc = "0x04 - CRC_GPOLYL register."]
    #[inline(always)]
    pub fn gpolyl(&self) -> &GPOLYL {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const GPOLYL) }
    }
    #[doc = "0x04 - CRC_GPOLYL register."]
    #[inline(always)]
    pub fn gpolyl_mut(&self) -> &mut GPOLYL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut GPOLYL) }
    }
    #[doc = "0x04 - CRC Polynomial Register"]
    #[inline(always)]
    pub fn gpoly(&self) -> &GPOLY {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const GPOLY) }
    }
    #[doc = "0x04 - CRC Polynomial Register"]
    #[inline(always)]
    pub fn gpoly_mut(&self) -> &mut GPOLY {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut GPOLY) }
    }
    #[doc = "0x05 - CRC_GPOLYLU register."]
    #[inline(always)]
    pub fn gpolylu(&self) -> &GPOLYLU {
        unsafe { &*(((self as *const Self) as *const u8).add(5usize) as *const GPOLYLU) }
    }
    #[doc = "0x05 - CRC_GPOLYLU register."]
    #[inline(always)]
    pub fn gpolylu_mut(&self) -> &mut GPOLYLU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(5usize) as *mut GPOLYLU) }
    }
    #[doc = "0x06 - CRC_GPOLYHL register."]
    #[inline(always)]
    pub fn gpolyhl(&self) -> &GPOLYHL {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const GPOLYHL) }
    }
    #[doc = "0x06 - CRC_GPOLYHL register."]
    #[inline(always)]
    pub fn gpolyhl_mut(&self) -> &mut GPOLYHL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut GPOLYHL) }
    }
    #[doc = "0x06 - CRC_GPOLYH register."]
    #[inline(always)]
    pub fn gpolyh(&self) -> &GPOLYH {
        unsafe { &*(((self as *const Self) as *const u8).add(6usize) as *const GPOLYH) }
    }
    #[doc = "0x06 - CRC_GPOLYH register."]
    #[inline(always)]
    pub fn gpolyh_mut(&self) -> &mut GPOLYH {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(6usize) as *mut GPOLYH) }
    }
    #[doc = "0x07 - CRC_GPOLYHU register."]
    #[inline(always)]
    pub fn gpolyhu(&self) -> &GPOLYHU {
        unsafe { &*(((self as *const Self) as *const u8).add(7usize) as *const GPOLYHU) }
    }
    #[doc = "0x07 - CRC_GPOLYHU register."]
    #[inline(always)]
    pub fn gpolyhu_mut(&self) -> &mut GPOLYHU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(7usize) as *mut GPOLYHU) }
    }
    #[doc = "0x08 - CRC Control Register"]
    #[inline(always)]
    pub fn ctrl(&self) -> &CTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const CTRL) }
    }
    #[doc = "0x08 - CRC Control Register"]
    #[inline(always)]
    pub fn ctrl_mut(&self) -> &mut CTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut CTRL) }
    }
    #[doc = "0x0b - CRC_CTRLHU register."]
    #[inline(always)]
    pub fn ctrlhu(&self) -> &CTRLHU {
        unsafe { &*(((self as *const Self) as *const u8).add(11usize) as *const CTRLHU) }
    }
    #[doc = "0x0b - CRC_CTRLHU register."]
    #[inline(always)]
    pub fn ctrlhu_mut(&self) -> &mut CTRLHU {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(11usize) as *mut CTRLHU) }
    }
}
#[doc = "CRC Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc](crc) module"]
pub type CRC = crate::Reg<u32, _CRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC;
#[doc = "`read()` method returns [crc::R](crc::R) reader structure"]
impl crate::Readable for CRC {}
#[doc = "`write(|w| ..)` method takes [crc::W](crc::W) writer structure"]
impl crate::Writable for CRC {}
#[doc = "CRC Data Register"]
pub mod crc;
#[doc = "CRC_CRCL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcl](crcl) module"]
pub type CRCL = crate::Reg<u16, _CRCL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCL;
#[doc = "`read()` method returns [crcl::R](crcl::R) reader structure"]
impl crate::Readable for CRCL {}
#[doc = "`write(|w| ..)` method takes [crcl::W](crcl::W) writer structure"]
impl crate::Writable for CRCL {}
#[doc = "CRC_CRCL register."]
pub mod crcl;
#[doc = "CRC_CRCLL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcll](crcll) module"]
pub type CRCLL = crate::Reg<u8, _CRCLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCLL;
#[doc = "`read()` method returns [crcll::R](crcll::R) reader structure"]
impl crate::Readable for CRCLL {}
#[doc = "`write(|w| ..)` method takes [crcll::W](crcll::W) writer structure"]
impl crate::Writable for CRCLL {}
#[doc = "CRC_CRCLL register."]
pub mod crcll;
#[doc = "CRC_CRCLU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crclu](crclu) module"]
pub type CRCLU = crate::Reg<u8, _CRCLU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCLU;
#[doc = "`read()` method returns [crclu::R](crclu::R) reader structure"]
impl crate::Readable for CRCLU {}
#[doc = "`write(|w| ..)` method takes [crclu::W](crclu::W) writer structure"]
impl crate::Writable for CRCLU {}
#[doc = "CRC_CRCLU register."]
pub mod crclu;
#[doc = "CRC_CRCH register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crch](crch) module"]
pub type CRCH = crate::Reg<u16, _CRCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCH;
#[doc = "`read()` method returns [crch::R](crch::R) reader structure"]
impl crate::Readable for CRCH {}
#[doc = "`write(|w| ..)` method takes [crch::W](crch::W) writer structure"]
impl crate::Writable for CRCH {}
#[doc = "CRC_CRCH register."]
pub mod crch;
#[doc = "CRC_CRCHL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crchl](crchl) module"]
pub type CRCHL = crate::Reg<u8, _CRCHL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCHL;
#[doc = "`read()` method returns [crchl::R](crchl::R) reader structure"]
impl crate::Readable for CRCHL {}
#[doc = "`write(|w| ..)` method takes [crchl::W](crchl::W) writer structure"]
impl crate::Writable for CRCHL {}
#[doc = "CRC_CRCHL register."]
pub mod crchl;
#[doc = "CRC_CRCHU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crchu](crchu) module"]
pub type CRCHU = crate::Reg<u8, _CRCHU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCHU;
#[doc = "`read()` method returns [crchu::R](crchu::R) reader structure"]
impl crate::Readable for CRCHU {}
#[doc = "`write(|w| ..)` method takes [crchu::W](crchu::W) writer structure"]
impl crate::Writable for CRCHU {}
#[doc = "CRC_CRCHU register."]
pub mod crchu;
#[doc = "CRC Polynomial Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpoly](gpoly) module"]
pub type GPOLY = crate::Reg<u32, _GPOLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLY;
#[doc = "`read()` method returns [gpoly::R](gpoly::R) reader structure"]
impl crate::Readable for GPOLY {}
#[doc = "`write(|w| ..)` method takes [gpoly::W](gpoly::W) writer structure"]
impl crate::Writable for GPOLY {}
#[doc = "CRC Polynomial Register"]
pub mod gpoly;
#[doc = "CRC_GPOLYL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpolyl](gpolyl) module"]
pub type GPOLYL = crate::Reg<u16, _GPOLYL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYL;
#[doc = "`read()` method returns [gpolyl::R](gpolyl::R) reader structure"]
impl crate::Readable for GPOLYL {}
#[doc = "`write(|w| ..)` method takes [gpolyl::W](gpolyl::W) writer structure"]
impl crate::Writable for GPOLYL {}
#[doc = "CRC_GPOLYL register."]
pub mod gpolyl;
#[doc = "CRC_GPOLYLL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpolyll](gpolyll) module"]
pub type GPOLYLL = crate::Reg<u8, _GPOLYLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYLL;
#[doc = "`read()` method returns [gpolyll::R](gpolyll::R) reader structure"]
impl crate::Readable for GPOLYLL {}
#[doc = "`write(|w| ..)` method takes [gpolyll::W](gpolyll::W) writer structure"]
impl crate::Writable for GPOLYLL {}
#[doc = "CRC_GPOLYLL register."]
pub mod gpolyll;
#[doc = "CRC_GPOLYLU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpolylu](gpolylu) module"]
pub type GPOLYLU = crate::Reg<u8, _GPOLYLU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYLU;
#[doc = "`read()` method returns [gpolylu::R](gpolylu::R) reader structure"]
impl crate::Readable for GPOLYLU {}
#[doc = "`write(|w| ..)` method takes [gpolylu::W](gpolylu::W) writer structure"]
impl crate::Writable for GPOLYLU {}
#[doc = "CRC_GPOLYLU register."]
pub mod gpolylu;
#[doc = "CRC_GPOLYH register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpolyh](gpolyh) module"]
pub type GPOLYH = crate::Reg<u16, _GPOLYH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYH;
#[doc = "`read()` method returns [gpolyh::R](gpolyh::R) reader structure"]
impl crate::Readable for GPOLYH {}
#[doc = "`write(|w| ..)` method takes [gpolyh::W](gpolyh::W) writer structure"]
impl crate::Writable for GPOLYH {}
#[doc = "CRC_GPOLYH register."]
pub mod gpolyh;
#[doc = "CRC_GPOLYHL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpolyhl](gpolyhl) module"]
pub type GPOLYHL = crate::Reg<u8, _GPOLYHL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYHL;
#[doc = "`read()` method returns [gpolyhl::R](gpolyhl::R) reader structure"]
impl crate::Readable for GPOLYHL {}
#[doc = "`write(|w| ..)` method takes [gpolyhl::W](gpolyhl::W) writer structure"]
impl crate::Writable for GPOLYHL {}
#[doc = "CRC_GPOLYHL register."]
pub mod gpolyhl;
#[doc = "CRC_GPOLYHU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpolyhu](gpolyhu) module"]
pub type GPOLYHU = crate::Reg<u8, _GPOLYHU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPOLYHU;
#[doc = "`read()` method returns [gpolyhu::R](gpolyhu::R) reader structure"]
impl crate::Readable for GPOLYHU {}
#[doc = "`write(|w| ..)` method takes [gpolyhu::W](gpolyhu::W) writer structure"]
impl crate::Writable for GPOLYHU {}
#[doc = "CRC_GPOLYHU register."]
pub mod gpolyhu;
#[doc = "CRC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "CRC Control Register"]
pub mod ctrl;
#[doc = "CRC_CTRLHU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlhu](ctrlhu) module"]
pub type CTRLHU = crate::Reg<u8, _CTRLHU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLHU;
#[doc = "`read()` method returns [ctrlhu::R](ctrlhu::R) reader structure"]
impl crate::Readable for CTRLHU {}
#[doc = "`write(|w| ..)` method takes [ctrlhu::W](ctrlhu::W) writer structure"]
impl crate::Writable for CTRLHU {}
#[doc = "CRC_CTRLHU register."]
pub mod ctrlhu;
