#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub control: CONTROL,
    #[doc = "0x04 - Clock Register"]
    pub clock: CLOCK,
    #[doc = "0x08 - Status Register"]
    pub status: STATUS,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - TIMER0 Register"]
    pub timer0: TIMER0,
    #[doc = "0x14 - no description available"]
    pub timer1: TIMER1,
    #[doc = "0x18 - no description available"]
    pub timer2: TIMER2,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "Control Register"]
pub mod control;
#[doc = "Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock](clock) module"]
pub type CLOCK = crate::Reg<u32, _CLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCK;
#[doc = "`read()` method returns [clock::R](clock::R) reader structure"]
impl crate::Readable for CLOCK {}
#[doc = "`write(|w| ..)` method takes [clock::W](clock::W) writer structure"]
impl crate::Writable for CLOCK {}
#[doc = "Clock Register"]
pub mod clock;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "TIMER0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0](timer0) module"]
pub type TIMER0 = crate::Reg<u32, _TIMER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER0;
#[doc = "`read()` method returns [timer0::R](timer0::R) reader structure"]
impl crate::Readable for TIMER0 {}
#[doc = "`write(|w| ..)` method takes [timer0::W](timer0::W) writer structure"]
impl crate::Writable for TIMER0 {}
#[doc = "TIMER0 Register"]
pub mod timer0;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1](timer1) module"]
pub type TIMER1 = crate::Reg<u32, _TIMER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER1;
#[doc = "`read()` method returns [timer1::R](timer1::R) reader structure"]
impl crate::Readable for TIMER1 {}
#[doc = "`write(|w| ..)` method takes [timer1::W](timer1::W) writer structure"]
impl crate::Writable for TIMER1 {}
#[doc = "no description available"]
pub mod timer1;
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2](timer2) module"]
pub type TIMER2 = crate::Reg<u32, _TIMER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMER2;
#[doc = "`read()` method returns [timer2::R](timer2::R) reader structure"]
impl crate::Readable for TIMER2 {}
#[doc = "`write(|w| ..)` method takes [timer2::W](timer2::W) writer structure"]
impl crate::Writable for TIMER2 {}
#[doc = "no description available"]
pub mod timer2;
