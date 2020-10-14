#[doc = "Reader of register GENCS"]
pub type R = crate::R<u32, super::GENCS>;
#[doc = "Writer for register GENCS"]
pub type W = crate::W<u32, super::GENCS>;
#[doc = "Register GENCS `reset()`'s with value 0"]
impl crate::ResetValue for super::GENCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPE_A {
    #[doc = "0: Disable TSI when MCU goes into low power modes."]
    _0 = 0,
    #[doc = "1: Allows TSI to continue running in all low power modes."]
    _1 = 1,
}
impl From<STPE_A> for bool {
    #[inline(always)]
    fn from(variant: STPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STPE`"]
pub type STPE_R = crate::R<bool, STPE_A>;
impl STPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPE_A {
        match self.bits {
            false => STPE_A::_0,
            true => STPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STPE_A::_1
    }
}
#[doc = "Write proxy for field `STPE`"]
pub struct STPE_W<'a> {
    w: &'a mut W,
}
impl<'a> STPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable TSI when MCU goes into low power modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STPE_A::_0)
    }
    #[doc = "Allows TSI to continue running in all low power modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STPE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Scan Trigger Mode. This bit-field can only be changed if the TSI module is disabled (TSIEN bit = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STM_A {
    #[doc = "0: Software trigger scan."]
    _0 = 0,
    #[doc = "1: Periodical Scan."]
    _1 = 1,
}
impl From<STM_A> for bool {
    #[inline(always)]
    fn from(variant: STM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STM`"]
pub type STM_R = crate::R<bool, STM_A>;
impl STM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STM_A {
        match self.bits {
            false => STM_A::_0,
            true => STM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STM_A::_1
    }
}
#[doc = "Write proxy for field `STM`"]
pub struct STM_W<'a> {
    w: &'a mut W,
}
impl<'a> STM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software trigger scan."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STM_A::_0)
    }
    #[doc = "Periodical Scan."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STM_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "End-of-Scan or Out-of-Range Interrupt select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESOR_A {
    #[doc = "0: Out-of-Range interrupt is allowed."]
    _0 = 0,
    #[doc = "1: End-of-Scan interrupt is allowed."]
    _1 = 1,
}
impl From<ESOR_A> for bool {
    #[inline(always)]
    fn from(variant: ESOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ESOR`"]
pub type ESOR_R = crate::R<bool, ESOR_A>;
impl ESOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESOR_A {
        match self.bits {
            false => ESOR_A::_0,
            true => ESOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESOR_A::_1
    }
}
#[doc = "Write proxy for field `ESOR`"]
pub struct ESOR_W<'a> {
    w: &'a mut W,
}
impl<'a> ESOR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESOR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Out-of-Range interrupt is allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESOR_A::_0)
    }
    #[doc = "End-of-Scan interrupt is allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESOR_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERIE_A {
    #[doc = "0: Interrupt disabled for error."]
    _0 = 0,
    #[doc = "1: Interrupt enabled for error."]
    _1 = 1,
}
impl From<ERIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERIE`"]
pub type ERIE_R = crate::R<bool, ERIE_A>;
impl ERIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERIE_A {
        match self.bits {
            false => ERIE_A::_0,
            true => ERIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERIE_A::_1
    }
}
#[doc = "Write proxy for field `ERIE`"]
pub struct ERIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt disabled for error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERIE_A::_0)
    }
    #[doc = "Interrupt enabled for error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERIE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Touch Sensing Input Interrupt Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIIE_A {
    #[doc = "0: Interrupt from TSI is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt from TSI is enabled"]
    _1 = 1,
}
impl From<TSIIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSIIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSIIE`"]
pub type TSIIE_R = crate::R<bool, TSIIE_A>;
impl TSIIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIIE_A {
        match self.bits {
            false => TSIIE_A::_0,
            true => TSIIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSIIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSIIE_A::_1
    }
}
#[doc = "Write proxy for field `TSIIE`"]
pub struct TSIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt from TSI is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIIE_A::_0)
    }
    #[doc = "Interrupt from TSI is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIIE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Touch Sensing Input Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSIEN_A {
    #[doc = "0: TSI module is disabled"]
    _0 = 0,
    #[doc = "1: TSI module is enabled"]
    _1 = 1,
}
impl From<TSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSIEN`"]
pub type TSIEN_R = crate::R<bool, TSIEN_A>;
impl TSIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIEN_A {
        match self.bits {
            false => TSIEN_A::_0,
            true => TSIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSIEN_A::_1
    }
}
#[doc = "Write proxy for field `TSIEN`"]
pub struct TSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TSI module is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSIEN_A::_0)
    }
    #[doc = "TSI module is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSIEN_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `SWTS`"]
pub struct SWTS_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCNIP`"]
pub type SCNIP_R = crate::R<bool, bool>;
#[doc = "Overrun error Flag. This flag is set when a scan trigger occurs while a scan is still in progress. Write \"1\", when this flag is set, to clear it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRF_A {
    #[doc = "0: No over run."]
    _0 = 0,
    #[doc = "1: Over Run occurred."]
    _1 = 1,
}
impl From<OVRF_A> for bool {
    #[inline(always)]
    fn from(variant: OVRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVRF`"]
pub type OVRF_R = crate::R<bool, OVRF_A>;
impl OVRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRF_A {
        match self.bits {
            false => OVRF_A::_0,
            true => OVRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRF_A::_1
    }
}
#[doc = "Write proxy for field `OVRF`"]
pub struct OVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No over run."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVRF_A::_0)
    }
    #[doc = "Over Run occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVRF_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "External Electrode error occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTERF_A {
    #[doc = "0: No fault happend on TSI electrodes"]
    _0 = 0,
    #[doc = "1: Short to VDD or VSS was detected on one or more electrodes."]
    _1 = 1,
}
impl From<EXTERF_A> for bool {
    #[inline(always)]
    fn from(variant: EXTERF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTERF`"]
pub type EXTERF_R = crate::R<bool, EXTERF_A>;
impl EXTERF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTERF_A {
        match self.bits {
            false => EXTERF_A::_0,
            true => EXTERF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXTERF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXTERF_A::_1
    }
}
#[doc = "Write proxy for field `EXTERF`"]
pub struct EXTERF_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTERF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No fault happend on TSI electrodes"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTERF_A::_0)
    }
    #[doc = "Short to VDD or VSS was detected on one or more electrodes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTERF_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `OUTRGF`"]
pub type OUTRGF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTRGF`"]
pub struct OUTRGF_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTRGF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `EOSF`"]
pub type EOSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOSF`"]
pub struct EOSF_W<'a> {
    w: &'a mut W,
}
impl<'a> EOSF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Electrode Oscillator prescaler. .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: Electrode Oscillator Frequency divided by 1"]
    _000 = 0,
    #[doc = "1: Electrode Oscillator Frequency divided by 2"]
    _001 = 1,
    #[doc = "2: Electrode Oscillator Frequency divided by 4"]
    _010 = 2,
    #[doc = "3: Electrode Oscillator Frequency divided by 8"]
    _011 = 3,
    #[doc = "4: Electrode Oscillator Frequency divided by 16"]
    _100 = 4,
    #[doc = "5: Electrode Oscillator Frequency divided by 32"]
    _101 = 5,
    #[doc = "6: Electrode Oscillator Frequency divided by 64"]
    _110 = 6,
    #[doc = "7: Electrode Oscillator Frequency divided by 128"]
    _111 = 7,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, PS_A>;
impl PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::_000,
            1 => PS_A::_001,
            2 => PS_A::_010,
            3 => PS_A::_011,
            4 => PS_A::_100,
            5 => PS_A::_101,
            6 => PS_A::_110,
            7 => PS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PS_A::_111
    }
}
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Electrode Oscillator Frequency divided by 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PS_A::_000)
    }
    #[doc = "Electrode Oscillator Frequency divided by 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PS_A::_001)
    }
    #[doc = "Electrode Oscillator Frequency divided by 4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PS_A::_010)
    }
    #[doc = "Electrode Oscillator Frequency divided by 8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PS_A::_011)
    }
    #[doc = "Electrode Oscillator Frequency divided by 16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PS_A::_100)
    }
    #[doc = "Electrode Oscillator Frequency divided by 32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PS_A::_101)
    }
    #[doc = "Electrode Oscillator Frequency divided by 64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PS_A::_110)
    }
    #[doc = "Electrode Oscillator Frequency divided by 128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PS_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Number of Consecutive Scans per Electrode electrode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NSCN_A {
    #[doc = "0: Once per electrode"]
    _00000 = 0,
    #[doc = "1: Twice per electrode"]
    _00001 = 1,
    #[doc = "2: 3 times per electrode"]
    _00010 = 2,
    #[doc = "3: 4 times per electrode"]
    _00011 = 3,
    #[doc = "4: 5 times per electrode"]
    _00100 = 4,
    #[doc = "5: 6 times per electrode"]
    _00101 = 5,
    #[doc = "6: 7 times per electrode"]
    _00110 = 6,
    #[doc = "7: 8 times per electrode"]
    _00111 = 7,
    #[doc = "8: 9 times per electrode"]
    _01000 = 8,
    #[doc = "9: 10 times per electrode"]
    _01001 = 9,
    #[doc = "10: 11 times per electrode"]
    _01010 = 10,
    #[doc = "11: 12 times per electrode"]
    _01011 = 11,
    #[doc = "12: 13 times per electrode"]
    _01100 = 12,
    #[doc = "13: 14 times per electrode"]
    _01101 = 13,
    #[doc = "14: 15 times per electrode"]
    _01110 = 14,
    #[doc = "15: 16 times per electrode"]
    _01111 = 15,
    #[doc = "16: 17 times per electrode"]
    _10000 = 16,
    #[doc = "17: 18 times per electrode"]
    _10001 = 17,
    #[doc = "18: 19 times per electrode"]
    _10010 = 18,
    #[doc = "19: 20 times per electrode"]
    _10011 = 19,
    #[doc = "20: 21 times per electrode"]
    _10100 = 20,
    #[doc = "21: 22 times per electrode"]
    _10101 = 21,
    #[doc = "22: 23 times per electrode"]
    _10110 = 22,
    #[doc = "23: 24 times per electrode"]
    _10111 = 23,
    #[doc = "24: 25 times per electrode"]
    _11000 = 24,
    #[doc = "25: 26 times per electrode"]
    _11001 = 25,
    #[doc = "26: 27 times per electrode"]
    _11010 = 26,
    #[doc = "27: 28 times per electrode"]
    _11011 = 27,
    #[doc = "28: 29 times per electrode"]
    _11100 = 28,
    #[doc = "29: 30 times per electrode"]
    _11101 = 29,
    #[doc = "30: 31 times per electrode"]
    _11110 = 30,
    #[doc = "31: 32 times per electrode"]
    _11111 = 31,
}
impl From<NSCN_A> for u8 {
    #[inline(always)]
    fn from(variant: NSCN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NSCN`"]
pub type NSCN_R = crate::R<u8, NSCN_A>;
impl NSCN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSCN_A {
        match self.bits {
            0 => NSCN_A::_00000,
            1 => NSCN_A::_00001,
            2 => NSCN_A::_00010,
            3 => NSCN_A::_00011,
            4 => NSCN_A::_00100,
            5 => NSCN_A::_00101,
            6 => NSCN_A::_00110,
            7 => NSCN_A::_00111,
            8 => NSCN_A::_01000,
            9 => NSCN_A::_01001,
            10 => NSCN_A::_01010,
            11 => NSCN_A::_01011,
            12 => NSCN_A::_01100,
            13 => NSCN_A::_01101,
            14 => NSCN_A::_01110,
            15 => NSCN_A::_01111,
            16 => NSCN_A::_10000,
            17 => NSCN_A::_10001,
            18 => NSCN_A::_10010,
            19 => NSCN_A::_10011,
            20 => NSCN_A::_10100,
            21 => NSCN_A::_10101,
            22 => NSCN_A::_10110,
            23 => NSCN_A::_10111,
            24 => NSCN_A::_11000,
            25 => NSCN_A::_11001,
            26 => NSCN_A::_11010,
            27 => NSCN_A::_11011,
            28 => NSCN_A::_11100,
            29 => NSCN_A::_11101,
            30 => NSCN_A::_11110,
            31 => NSCN_A::_11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == NSCN_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == NSCN_A::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == NSCN_A::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == NSCN_A::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == NSCN_A::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == NSCN_A::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == NSCN_A::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == NSCN_A::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == NSCN_A::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == NSCN_A::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == NSCN_A::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == NSCN_A::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == NSCN_A::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == NSCN_A::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == NSCN_A::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == NSCN_A::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == NSCN_A::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == NSCN_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == NSCN_A::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == NSCN_A::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == NSCN_A::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == NSCN_A::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == NSCN_A::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == NSCN_A::_10111
    }
    #[doc = "Checks if the value of the field is `_11000`"]
    #[inline(always)]
    pub fn is_11000(&self) -> bool {
        *self == NSCN_A::_11000
    }
    #[doc = "Checks if the value of the field is `_11001`"]
    #[inline(always)]
    pub fn is_11001(&self) -> bool {
        *self == NSCN_A::_11001
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == NSCN_A::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == NSCN_A::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline(always)]
    pub fn is_11100(&self) -> bool {
        *self == NSCN_A::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == NSCN_A::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == NSCN_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == NSCN_A::_11111
    }
}
#[doc = "Write proxy for field `NSCN`"]
pub struct NSCN_W<'a> {
    w: &'a mut W,
}
impl<'a> NSCN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSCN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Once per electrode"]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(NSCN_A::_00000)
    }
    #[doc = "Twice per electrode"]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(NSCN_A::_00001)
    }
    #[doc = "3 times per electrode"]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut W {
        self.variant(NSCN_A::_00010)
    }
    #[doc = "4 times per electrode"]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut W {
        self.variant(NSCN_A::_00011)
    }
    #[doc = "5 times per electrode"]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut W {
        self.variant(NSCN_A::_00100)
    }
    #[doc = "6 times per electrode"]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut W {
        self.variant(NSCN_A::_00101)
    }
    #[doc = "7 times per electrode"]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut W {
        self.variant(NSCN_A::_00110)
    }
    #[doc = "8 times per electrode"]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut W {
        self.variant(NSCN_A::_00111)
    }
    #[doc = "9 times per electrode"]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut W {
        self.variant(NSCN_A::_01000)
    }
    #[doc = "10 times per electrode"]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(NSCN_A::_01001)
    }
    #[doc = "11 times per electrode"]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut W {
        self.variant(NSCN_A::_01010)
    }
    #[doc = "12 times per electrode"]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut W {
        self.variant(NSCN_A::_01011)
    }
    #[doc = "13 times per electrode"]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut W {
        self.variant(NSCN_A::_01100)
    }
    #[doc = "14 times per electrode"]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut W {
        self.variant(NSCN_A::_01101)
    }
    #[doc = "15 times per electrode"]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut W {
        self.variant(NSCN_A::_01110)
    }
    #[doc = "16 times per electrode"]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut W {
        self.variant(NSCN_A::_01111)
    }
    #[doc = "17 times per electrode"]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(NSCN_A::_10000)
    }
    #[doc = "18 times per electrode"]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut W {
        self.variant(NSCN_A::_10001)
    }
    #[doc = "19 times per electrode"]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(NSCN_A::_10010)
    }
    #[doc = "20 times per electrode"]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut W {
        self.variant(NSCN_A::_10011)
    }
    #[doc = "21 times per electrode"]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut W {
        self.variant(NSCN_A::_10100)
    }
    #[doc = "22 times per electrode"]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut W {
        self.variant(NSCN_A::_10101)
    }
    #[doc = "23 times per electrode"]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut W {
        self.variant(NSCN_A::_10110)
    }
    #[doc = "24 times per electrode"]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut W {
        self.variant(NSCN_A::_10111)
    }
    #[doc = "25 times per electrode"]
    #[inline(always)]
    pub fn _11000(self) -> &'a mut W {
        self.variant(NSCN_A::_11000)
    }
    #[doc = "26 times per electrode"]
    #[inline(always)]
    pub fn _11001(self) -> &'a mut W {
        self.variant(NSCN_A::_11001)
    }
    #[doc = "27 times per electrode"]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut W {
        self.variant(NSCN_A::_11010)
    }
    #[doc = "28 times per electrode"]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut W {
        self.variant(NSCN_A::_11011)
    }
    #[doc = "29 times per electrode"]
    #[inline(always)]
    pub fn _11100(self) -> &'a mut W {
        self.variant(NSCN_A::_11100)
    }
    #[doc = "30 times per electrode"]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut W {
        self.variant(NSCN_A::_11101)
    }
    #[doc = "31 times per electrode"]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(NSCN_A::_11110)
    }
    #[doc = "32 times per electrode"]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(NSCN_A::_11111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "TSI Low Power Mode Scan Interval.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPSCNITV_A {
    #[doc = "0: 1 ms scan interval"]
    _0000 = 0,
    #[doc = "1: 5 ms scan interval"]
    _0001 = 1,
    #[doc = "2: 10 ms scan interval"]
    _0010 = 2,
    #[doc = "3: 15 ms scan interval"]
    _0011 = 3,
    #[doc = "4: 20 ms scan interval"]
    _0100 = 4,
    #[doc = "5: 30 ms scan interval"]
    _0101 = 5,
    #[doc = "6: 40 ms scan interval"]
    _0110 = 6,
    #[doc = "7: 50 ms scan interval"]
    _0111 = 7,
    #[doc = "8: 75 ms scan interval"]
    _1000 = 8,
    #[doc = "9: 100 ms scan interval"]
    _1001 = 9,
    #[doc = "10: 125 ms scan interval"]
    _1010 = 10,
    #[doc = "11: 150 ms scan interval"]
    _1011 = 11,
    #[doc = "12: 200 ms scan interval"]
    _1100 = 12,
    #[doc = "13: 300 ms scan interval"]
    _1101 = 13,
    #[doc = "14: 400 ms scan interval"]
    _1110 = 14,
    #[doc = "15: 500 ms scan interval"]
    _1111 = 15,
}
impl From<LPSCNITV_A> for u8 {
    #[inline(always)]
    fn from(variant: LPSCNITV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPSCNITV`"]
pub type LPSCNITV_R = crate::R<u8, LPSCNITV_A>;
impl LPSCNITV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSCNITV_A {
        match self.bits {
            0 => LPSCNITV_A::_0000,
            1 => LPSCNITV_A::_0001,
            2 => LPSCNITV_A::_0010,
            3 => LPSCNITV_A::_0011,
            4 => LPSCNITV_A::_0100,
            5 => LPSCNITV_A::_0101,
            6 => LPSCNITV_A::_0110,
            7 => LPSCNITV_A::_0111,
            8 => LPSCNITV_A::_1000,
            9 => LPSCNITV_A::_1001,
            10 => LPSCNITV_A::_1010,
            11 => LPSCNITV_A::_1011,
            12 => LPSCNITV_A::_1100,
            13 => LPSCNITV_A::_1101,
            14 => LPSCNITV_A::_1110,
            15 => LPSCNITV_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == LPSCNITV_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == LPSCNITV_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == LPSCNITV_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == LPSCNITV_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == LPSCNITV_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == LPSCNITV_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == LPSCNITV_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == LPSCNITV_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == LPSCNITV_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == LPSCNITV_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == LPSCNITV_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == LPSCNITV_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == LPSCNITV_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == LPSCNITV_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == LPSCNITV_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == LPSCNITV_A::_1111
    }
}
#[doc = "Write proxy for field `LPSCNITV`"]
pub struct LPSCNITV_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSCNITV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSCNITV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "1 ms scan interval"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_0000)
    }
    #[doc = "5 ms scan interval"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_0001)
    }
    #[doc = "10 ms scan interval"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_0010)
    }
    #[doc = "15 ms scan interval"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_0011)
    }
    #[doc = "20 ms scan interval"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_0100)
    }
    #[doc = "30 ms scan interval"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_0101)
    }
    #[doc = "40 ms scan interval"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_0110)
    }
    #[doc = "50 ms scan interval"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_0111)
    }
    #[doc = "75 ms scan interval"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_1000)
    }
    #[doc = "100 ms scan interval"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_1001)
    }
    #[doc = "125 ms scan interval"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_1010)
    }
    #[doc = "150 ms scan interval"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_1011)
    }
    #[doc = "200 ms scan interval"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_1100)
    }
    #[doc = "300 ms scan interval"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_1101)
    }
    #[doc = "400 ms scan interval"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_1110)
    }
    #[doc = "500 ms scan interval"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(LPSCNITV_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Low Power Mode Clock Source Selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCLKS_A {
    #[doc = "0: LPOCLK is selected to determine the scan period in low power mode"]
    _0 = 0,
    #[doc = "1: VLPOSCCLK is selected to determine the scan period in low power mode"]
    _1 = 1,
}
impl From<LPCLKS_A> for bool {
    #[inline(always)]
    fn from(variant: LPCLKS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPCLKS`"]
pub type LPCLKS_R = crate::R<bool, LPCLKS_A>;
impl LPCLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCLKS_A {
        match self.bits {
            false => LPCLKS_A::_0,
            true => LPCLKS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPCLKS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPCLKS_A::_1
    }
}
#[doc = "Write proxy for field `LPCLKS`"]
pub struct LPCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCLKS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LPOCLK is selected to determine the scan period in low power mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPCLKS_A::_0)
    }
    #[doc = "VLPOSCCLK is selected to determine the scan period in low power mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPCLKS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn stpe(&self) -> STPE_R {
        STPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Scan Trigger Mode. This bit-field can only be changed if the TSI module is disabled (TSIEN bit = 0)."]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End-of-Scan or Out-of-Range Interrupt select"]
    #[inline(always)]
    pub fn esor(&self) -> ESOR_R {
        ESOR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Touch Sensing Input Interrupt Module Enable"]
    #[inline(always)]
    pub fn tsiie(&self) -> TSIIE_R {
        TSIIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Touch Sensing Input Module Enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TSIEN_R {
        TSIEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Scan In Progress status"]
    #[inline(always)]
    pub fn scnip(&self) -> SCNIP_R {
        SCNIP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Overrun error Flag. This flag is set when a scan trigger occurs while a scan is still in progress. Write \"1\", when this flag is set, to clear it."]
    #[inline(always)]
    pub fn ovrf(&self) -> OVRF_R {
        OVRF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Electrode error occurred"]
    #[inline(always)]
    pub fn exterf(&self) -> EXTERF_R {
        EXTERF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Out of Range Flag."]
    #[inline(always)]
    pub fn outrgf(&self) -> OUTRGF_R {
        OUTRGF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - End of Scan Flag."]
    #[inline(always)]
    pub fn eosf(&self) -> EOSF_R {
        EOSF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Electrode Oscillator prescaler. ."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:23 - Number of Consecutive Scans per Electrode electrode."]
    #[inline(always)]
    pub fn nscn(&self) -> NSCN_R {
        NSCN_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - TSI Low Power Mode Scan Interval."]
    #[inline(always)]
    pub fn lpscnitv(&self) -> LPSCNITV_R {
        LPSCNITV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Low Power Mode Clock Source Selection."]
    #[inline(always)]
    pub fn lpclks(&self) -> LPCLKS_R {
        LPCLKS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn stpe(&mut self) -> STPE_W {
        STPE_W { w: self }
    }
    #[doc = "Bit 1 - Scan Trigger Mode. This bit-field can only be changed if the TSI module is disabled (TSIEN bit = 0)."]
    #[inline(always)]
    pub fn stm(&mut self) -> STM_W {
        STM_W { w: self }
    }
    #[doc = "Bit 4 - End-of-Scan or Out-of-Range Interrupt select"]
    #[inline(always)]
    pub fn esor(&mut self) -> ESOR_W {
        ESOR_W { w: self }
    }
    #[doc = "Bit 5 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W {
        ERIE_W { w: self }
    }
    #[doc = "Bit 6 - Touch Sensing Input Interrupt Module Enable"]
    #[inline(always)]
    pub fn tsiie(&mut self) -> TSIIE_W {
        TSIIE_W { w: self }
    }
    #[doc = "Bit 7 - Touch Sensing Input Module Enable"]
    #[inline(always)]
    pub fn tsien(&mut self) -> TSIEN_W {
        TSIEN_W { w: self }
    }
    #[doc = "Bit 8 - Software Trigger Start"]
    #[inline(always)]
    pub fn swts(&mut self) -> SWTS_W {
        SWTS_W { w: self }
    }
    #[doc = "Bit 12 - Overrun error Flag. This flag is set when a scan trigger occurs while a scan is still in progress. Write \"1\", when this flag is set, to clear it."]
    #[inline(always)]
    pub fn ovrf(&mut self) -> OVRF_W {
        OVRF_W { w: self }
    }
    #[doc = "Bit 13 - External Electrode error occurred"]
    #[inline(always)]
    pub fn exterf(&mut self) -> EXTERF_W {
        EXTERF_W { w: self }
    }
    #[doc = "Bit 14 - Out of Range Flag."]
    #[inline(always)]
    pub fn outrgf(&mut self) -> OUTRGF_W {
        OUTRGF_W { w: self }
    }
    #[doc = "Bit 15 - End of Scan Flag."]
    #[inline(always)]
    pub fn eosf(&mut self) -> EOSF_W {
        EOSF_W { w: self }
    }
    #[doc = "Bits 16:18 - Electrode Oscillator prescaler. ."]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bits 19:23 - Number of Consecutive Scans per Electrode electrode."]
    #[inline(always)]
    pub fn nscn(&mut self) -> NSCN_W {
        NSCN_W { w: self }
    }
    #[doc = "Bits 24:27 - TSI Low Power Mode Scan Interval."]
    #[inline(always)]
    pub fn lpscnitv(&mut self) -> LPSCNITV_W {
        LPSCNITV_W { w: self }
    }
    #[doc = "Bit 28 - Low Power Mode Clock Source Selection."]
    #[inline(always)]
    pub fn lpclks(&mut self) -> LPCLKS_W {
        LPCLKS_W { w: self }
    }
}
