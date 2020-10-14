#[doc = "Reader of register SC"]
pub type R = crate::R<u8, super::SC>;
#[doc = "Writer for register SC"]
pub type W = crate::W<u8, super::SC>;
#[doc = "Register SC `reset()`'s with value 0"]
impl crate::ResetValue for super::SC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Buffer Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_LV_A {
    #[doc = "0: Bandgap on only, for stabilization and startup"]
    _00 = 0,
    #[doc = "1: Low-power buffer mode enabled"]
    _01 = 1,
    #[doc = "2: Tight-regulation buffer enabled"]
    _10 = 2,
}
impl From<MODE_LV_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_LV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE_LV`"]
pub type MODE_LV_R = crate::R<u8, MODE_LV_A>;
impl MODE_LV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_LV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_LV_A::_00),
            1 => Val(MODE_LV_A::_01),
            2 => Val(MODE_LV_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MODE_LV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MODE_LV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MODE_LV_A::_10
    }
}
#[doc = "Write proxy for field `MODE_LV`"]
pub struct MODE_LV_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_LV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_LV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Bandgap on only, for stabilization and startup"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MODE_LV_A::_00)
    }
    #[doc = "Low-power buffer mode enabled"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MODE_LV_A::_01)
    }
    #[doc = "Tight-regulation buffer enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODE_LV_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Internal Voltage Reference stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFST_A {
    #[doc = "0: The module is disabled or not stable."]
    _0 = 0,
    #[doc = "1: The module is stable."]
    _1 = 1,
}
impl From<VREFST_A> for bool {
    #[inline(always)]
    fn from(variant: VREFST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFST`"]
pub type VREFST_R = crate::R<bool, VREFST_A>;
impl VREFST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFST_A {
        match self.bits {
            false => VREFST_A::_0,
            true => VREFST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFST_A::_1
    }
}
#[doc = "Regulator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGEN_A {
    #[doc = "0: Internal 1.75 V regulator is disabled."]
    _0 = 0,
    #[doc = "1: Internal 1.75 V regulator is enabled."]
    _1 = 1,
}
impl From<REGEN_A> for bool {
    #[inline(always)]
    fn from(variant: REGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGEN`"]
pub type REGEN_R = crate::R<bool, REGEN_A>;
impl REGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGEN_A {
        match self.bits {
            false => REGEN_A::_0,
            true => REGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REGEN_A::_1
    }
}
#[doc = "Write proxy for field `REGEN`"]
pub struct REGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Internal 1.75 V regulator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REGEN_A::_0)
    }
    #[doc = "Internal 1.75 V regulator is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REGEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Internal Voltage Reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFEN_A {
    #[doc = "0: The module is disabled."]
    _0 = 0,
    #[doc = "1: The module is enabled."]
    _1 = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFEN`"]
pub type VREFEN_R = crate::R<bool, VREFEN_A>;
impl VREFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::_0,
            true => VREFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFEN_A::_1
    }
}
#[doc = "Write proxy for field `VREFEN`"]
pub struct VREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFEN_A::_0)
    }
    #[doc = "The module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREFEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Buffer Mode selection"]
    #[inline(always)]
    pub fn mode_lv(&self) -> MODE_LV_R {
        MODE_LV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Internal Voltage Reference stable"]
    #[inline(always)]
    pub fn vrefst(&self) -> VREFST_R {
        VREFST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Regulator enable"]
    #[inline(always)]
    pub fn regen(&self) -> REGEN_R {
        REGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Internal Voltage Reference enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Buffer Mode selection"]
    #[inline(always)]
    pub fn mode_lv(&mut self) -> MODE_LV_W {
        MODE_LV_W { w: self }
    }
    #[doc = "Bit 6 - Regulator enable"]
    #[inline(always)]
    pub fn regen(&mut self) -> REGEN_W {
        REGEN_W { w: self }
    }
    #[doc = "Bit 7 - Internal Voltage Reference enable"]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W {
        VREFEN_W { w: self }
    }
}
