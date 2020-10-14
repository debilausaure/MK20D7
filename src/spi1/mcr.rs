#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0x4001"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4001
    }
}
#[doc = "Halt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_A {
    #[doc = "0: Start transfers."]
    _0 = 0,
    #[doc = "1: Stop transfers."]
    _1 = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HALT`"]
pub type HALT_R = crate::R<bool, HALT_A>;
impl HALT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::_0,
            true => HALT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HALT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HALT_A::_1
    }
}
#[doc = "Write proxy for field `HALT`"]
pub struct HALT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Start transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALT_A::_0)
    }
    #[doc = "Stop transfers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALT_A::_1)
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
#[doc = "Sample Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMPL_PT_A {
    #[doc = "0: 0 system clocks between SCK edge and SIN sample"]
    _00 = 0,
    #[doc = "1: 1 system clock between SCK edge and SIN sample"]
    _01 = 1,
    #[doc = "2: 2 system clocks between SCK edge and SIN sample"]
    _10 = 2,
}
impl From<SMPL_PT_A> for u8 {
    #[inline(always)]
    fn from(variant: SMPL_PT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMPL_PT`"]
pub type SMPL_PT_R = crate::R<u8, SMPL_PT_A>;
impl SMPL_PT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SMPL_PT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SMPL_PT_A::_00),
            1 => Val(SMPL_PT_A::_01),
            2 => Val(SMPL_PT_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SMPL_PT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SMPL_PT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SMPL_PT_A::_10
    }
}
#[doc = "Write proxy for field `SMPL_PT`"]
pub struct SMPL_PT_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_PT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPL_PT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 system clocks between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SMPL_PT_A::_00)
    }
    #[doc = "1 system clock between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SMPL_PT_A::_01)
    }
    #[doc = "2 system clocks between SCK edge and SIN sample"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SMPL_PT_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_RXF_AW {
    #[doc = "0: Do not clear the Rx FIFO counter."]
    _0 = 0,
    #[doc = "1: Clear the Rx FIFO counter."]
    _1 = 1,
}
impl From<CLR_RXF_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_RXF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLR_RXF`"]
pub struct CLR_RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_RXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_RXF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not clear the Rx FIFO counter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_RXF_AW::_0)
    }
    #[doc = "Clear the Rx FIFO counter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_RXF_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Clear TX FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_TXF_AW {
    #[doc = "0: Do not clear the Tx FIFO counter."]
    _0 = 0,
    #[doc = "1: Clear the Tx FIFO counter."]
    _1 = 1,
}
impl From<CLR_TXF_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_TXF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLR_TXF`"]
pub struct CLR_TXF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_TXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_TXF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not clear the Tx FIFO counter."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_TXF_AW::_0)
    }
    #[doc = "Clear the Tx FIFO counter."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_TXF_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Disable Receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_RXF_A {
    #[doc = "0: Rx FIFO is enabled."]
    _0 = 0,
    #[doc = "1: Rx FIFO is disabled."]
    _1 = 1,
}
impl From<DIS_RXF_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_RXF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIS_RXF`"]
pub type DIS_RXF_R = crate::R<bool, DIS_RXF_A>;
impl DIS_RXF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_RXF_A {
        match self.bits {
            false => DIS_RXF_A::_0,
            true => DIS_RXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIS_RXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIS_RXF_A::_1
    }
}
#[doc = "Write proxy for field `DIS_RXF`"]
pub struct DIS_RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_RXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIS_RXF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIS_RXF_A::_0)
    }
    #[doc = "Rx FIFO is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIS_RXF_A::_1)
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
#[doc = "Disable Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_TXF_A {
    #[doc = "0: Tx FIFO is enabled."]
    _0 = 0,
    #[doc = "1: Tx FIFO is disabled."]
    _1 = 1,
}
impl From<DIS_TXF_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_TXF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIS_TXF`"]
pub type DIS_TXF_R = crate::R<bool, DIS_TXF_A>;
impl DIS_TXF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_TXF_A {
        match self.bits {
            false => DIS_TXF_A::_0,
            true => DIS_TXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIS_TXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIS_TXF_A::_1
    }
}
#[doc = "Write proxy for field `DIS_TXF`"]
pub struct DIS_TXF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TXF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIS_TXF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx FIFO is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIS_TXF_A::_0)
    }
    #[doc = "Tx FIFO is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIS_TXF_A::_1)
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
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIS_A {
    #[doc = "0: Enable DSPI clocks."]
    _0 = 0,
    #[doc = "1: Allow external logic to disable DSPI clocks."]
    _1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDIS`"]
pub type MDIS_R = crate::R<bool, MDIS_A>;
impl MDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::_0,
            true => MDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MDIS_A::_1
    }
}
#[doc = "Write proxy for field `MDIS`"]
pub struct MDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable DSPI clocks."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDIS_A::_0)
    }
    #[doc = "Allow external logic to disable DSPI clocks."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZE_A {
    #[doc = "0: Doze mode has no effect on DSPI."]
    _0 = 0,
    #[doc = "1: Doze mode disables DSPI."]
    _1 = 1,
}
impl From<DOZE_A> for bool {
    #[inline(always)]
    fn from(variant: DOZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOZE`"]
pub type DOZE_R = crate::R<bool, DOZE_A>;
impl DOZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZE_A {
        match self.bits {
            false => DOZE_A::_0,
            true => DOZE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOZE_A::_1
    }
}
#[doc = "Write proxy for field `DOZE`"]
pub struct DOZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Doze mode has no effect on DSPI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZE_A::_0)
    }
    #[doc = "Doze mode disables DSPI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Peripheral Chip Select x Inactive State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCSIS_A {
    #[doc = "0: The inactive state of PCSx is low."]
    _0 = 0,
    #[doc = "1: The inactive state of PCSx is high."]
    _1 = 1,
}
impl From<PCSIS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCSIS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCSIS`"]
pub type PCSIS_R = crate::R<u8, PCSIS_A>;
impl PCSIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PCSIS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PCSIS_A::_0),
            1 => Val(PCSIS_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCSIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCSIS_A::_1
    }
}
#[doc = "Write proxy for field `PCSIS`"]
pub struct PCSIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCSIS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The inactive state of PCSx is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSIS_A::_0)
    }
    #[doc = "The inactive state of PCSx is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSIS_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Receive FIFO Overflow Overwrite Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROOE_A {
    #[doc = "0: Incoming data is ignored."]
    _0 = 0,
    #[doc = "1: Incoming data is shifted into the shift register."]
    _1 = 1,
}
impl From<ROOE_A> for bool {
    #[inline(always)]
    fn from(variant: ROOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROOE`"]
pub type ROOE_R = crate::R<bool, ROOE_A>;
impl ROOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROOE_A {
        match self.bits {
            false => ROOE_A::_0,
            true => ROOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ROOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ROOE_A::_1
    }
}
#[doc = "Write proxy for field `ROOE`"]
pub struct ROOE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Incoming data is ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ROOE_A::_0)
    }
    #[doc = "Incoming data is shifted into the shift register."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ROOE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Peripheral Chip Select Strobe Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSSE_A {
    #[doc = "0: PCS\\[5\\]/PCSS is used as the Peripheral Chip Select\\[5\\]
signal."]
    _0 = 0,
    #[doc = "1: PCS\\[5\\]/PCSS is used as an active-low PCS Strobe signal."]
    _1 = 1,
}
impl From<PCSSE_A> for bool {
    #[inline(always)]
    fn from(variant: PCSSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PCSSE`"]
pub type PCSSE_R = crate::R<bool, PCSSE_A>;
impl PCSSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSSE_A {
        match self.bits {
            false => PCSSE_A::_0,
            true => PCSSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCSSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCSSE_A::_1
    }
}
#[doc = "Write proxy for field `PCSSE`"]
pub struct PCSSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCSSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PCS\\[5\\]/PCSS is used as the Peripheral Chip Select\\[5\\]
signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSSE_A::_0)
    }
    #[doc = "PCS\\[5\\]/PCSS is used as an active-low PCS Strobe signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSSE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Modified Timing Format Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTFE_A {
    #[doc = "0: Modified SPI transfer format disabled."]
    _0 = 0,
    #[doc = "1: Modified SPI transfer format enabled."]
    _1 = 1,
}
impl From<MTFE_A> for bool {
    #[inline(always)]
    fn from(variant: MTFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MTFE`"]
pub type MTFE_R = crate::R<bool, MTFE_A>;
impl MTFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTFE_A {
        match self.bits {
            false => MTFE_A::_0,
            true => MTFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTFE_A::_1
    }
}
#[doc = "Write proxy for field `MTFE`"]
pub struct MTFE_W<'a> {
    w: &'a mut W,
}
impl<'a> MTFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Modified SPI transfer format disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MTFE_A::_0)
    }
    #[doc = "Modified SPI transfer format enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MTFE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRZ_A {
    #[doc = "0: Do not halt serial transfers in debug mode."]
    _0 = 0,
    #[doc = "1: Halt serial transfers in debug mode."]
    _1 = 1,
}
impl From<FRZ_A> for bool {
    #[inline(always)]
    fn from(variant: FRZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRZ`"]
pub type FRZ_R = crate::R<bool, FRZ_A>;
impl FRZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZ_A {
        match self.bits {
            false => FRZ_A::_0,
            true => FRZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRZ_A::_1
    }
}
#[doc = "Write proxy for field `FRZ`"]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not halt serial transfers in debug mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRZ_A::_0)
    }
    #[doc = "Halt serial transfers in debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRZ_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "DSPI Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCONF_A {
    #[doc = "0: SPI"]
    _00 = 0,
}
impl From<DCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: DCONF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCONF`"]
pub type DCONF_R = crate::R<u8, DCONF_A>;
impl DCONF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DCONF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DCONF_A::_00),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DCONF_A::_00
    }
}
#[doc = "Write proxy for field `DCONF`"]
pub struct DCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCONF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DCONF_A::_00)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Continuous SCK Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONT_SCKE_A {
    #[doc = "0: Continuous SCK disabled."]
    _0 = 0,
    #[doc = "1: Continuous SCK enabled."]
    _1 = 1,
}
impl From<CONT_SCKE_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_SCKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONT_SCKE`"]
pub type CONT_SCKE_R = crate::R<bool, CONT_SCKE_A>;
impl CONT_SCKE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_SCKE_A {
        match self.bits {
            false => CONT_SCKE_A::_0,
            true => CONT_SCKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CONT_SCKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CONT_SCKE_A::_1
    }
}
#[doc = "Write proxy for field `CONT_SCKE`"]
pub struct CONT_SCKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_SCKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONT_SCKE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Continuous SCK disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONT_SCKE_A::_0)
    }
    #[doc = "Continuous SCK enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONT_SCKE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Master/Slave Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTR_A {
    #[doc = "0: DSPI is in slave mode."]
    _0 = 0,
    #[doc = "1: DSPI is in master mode."]
    _1 = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTR`"]
pub type MSTR_R = crate::R<bool, MSTR_A>;
impl MSTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::_0,
            true => MSTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTR_A::_1
    }
}
#[doc = "Write proxy for field `MSTR`"]
pub struct MSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSPI is in slave mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTR_A::_0)
    }
    #[doc = "DSPI is in master mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Halt"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Sample Point"]
    #[inline(always)]
    pub fn smpl_pt(&self) -> SMPL_PT_R {
        SMPL_PT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Disable Receive FIFO"]
    #[inline(always)]
    pub fn dis_rxf(&self) -> DIS_RXF_R {
        DIS_RXF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Disable Transmit FIFO"]
    #[inline(always)]
    pub fn dis_txf(&self) -> DIS_TXF_R {
        DIS_TXF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline(always)]
    pub fn doze(&self) -> DOZE_R {
        DOZE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis(&self) -> PCSIS_R {
        PCSIS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Receive FIFO Overflow Overwrite Enable"]
    #[inline(always)]
    pub fn rooe(&self) -> ROOE_R {
        ROOE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Peripheral Chip Select Strobe Enable"]
    #[inline(always)]
    pub fn pcsse(&self) -> PCSSE_R {
        PCSSE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Modified Timing Format Enable"]
    #[inline(always)]
    pub fn mtfe(&self) -> MTFE_R {
        MTFE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Freeze"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - DSPI Configuration"]
    #[inline(always)]
    pub fn dconf(&self) -> DCONF_R {
        DCONF_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Continuous SCK Enable"]
    #[inline(always)]
    pub fn cont_scke(&self) -> CONT_SCKE_R {
        CONT_SCKE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt"]
    #[inline(always)]
    pub fn halt(&mut self) -> HALT_W {
        HALT_W { w: self }
    }
    #[doc = "Bits 8:9 - Sample Point"]
    #[inline(always)]
    pub fn smpl_pt(&mut self) -> SMPL_PT_W {
        SMPL_PT_W { w: self }
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    pub fn clr_rxf(&mut self) -> CLR_RXF_W {
        CLR_RXF_W { w: self }
    }
    #[doc = "Bit 11 - Clear TX FIFO"]
    #[inline(always)]
    pub fn clr_txf(&mut self) -> CLR_TXF_W {
        CLR_TXF_W { w: self }
    }
    #[doc = "Bit 12 - Disable Receive FIFO"]
    #[inline(always)]
    pub fn dis_rxf(&mut self) -> DIS_RXF_W {
        DIS_RXF_W { w: self }
    }
    #[doc = "Bit 13 - Disable Transmit FIFO"]
    #[inline(always)]
    pub fn dis_txf(&mut self) -> DIS_TXF_W {
        DIS_TXF_W { w: self }
    }
    #[doc = "Bit 14 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&mut self) -> MDIS_W {
        MDIS_W { w: self }
    }
    #[doc = "Bit 15 - Doze Enable"]
    #[inline(always)]
    pub fn doze(&mut self) -> DOZE_W {
        DOZE_W { w: self }
    }
    #[doc = "Bits 16:21 - Peripheral Chip Select x Inactive State"]
    #[inline(always)]
    pub fn pcsis(&mut self) -> PCSIS_W {
        PCSIS_W { w: self }
    }
    #[doc = "Bit 24 - Receive FIFO Overflow Overwrite Enable"]
    #[inline(always)]
    pub fn rooe(&mut self) -> ROOE_W {
        ROOE_W { w: self }
    }
    #[doc = "Bit 25 - Peripheral Chip Select Strobe Enable"]
    #[inline(always)]
    pub fn pcsse(&mut self) -> PCSSE_W {
        PCSSE_W { w: self }
    }
    #[doc = "Bit 26 - Modified Timing Format Enable"]
    #[inline(always)]
    pub fn mtfe(&mut self) -> MTFE_W {
        MTFE_W { w: self }
    }
    #[doc = "Bit 27 - Freeze"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bits 28:29 - DSPI Configuration"]
    #[inline(always)]
    pub fn dconf(&mut self) -> DCONF_W {
        DCONF_W { w: self }
    }
    #[doc = "Bit 30 - Continuous SCK Enable"]
    #[inline(always)]
    pub fn cont_scke(&mut self) -> CONT_SCKE_W {
        CONT_SCKE_W { w: self }
    }
    #[doc = "Bit 31 - Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W {
        MSTR_W { w: self }
    }
}
