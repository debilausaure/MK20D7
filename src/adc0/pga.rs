#[doc = "Reader of register PGA"]
pub type R = crate::R<u32, super::PGA>;
#[doc = "Writer for register PGA"]
pub type W = crate::W<u32, super::PGA>;
#[doc = "Register PGA `reset()`'s with value 0"]
impl crate::ResetValue for super::PGA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PGA gain setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PGAG_A {
    #[doc = "0: 1"]
    _0000 = 0,
    #[doc = "1: 2"]
    _0001 = 1,
    #[doc = "2: 4"]
    _0010 = 2,
    #[doc = "3: 8"]
    _0011 = 3,
    #[doc = "4: 16"]
    _0100 = 4,
    #[doc = "5: 32"]
    _0101 = 5,
    #[doc = "6: 64"]
    _0110 = 6,
}
impl From<PGAG_A> for u8 {
    #[inline(always)]
    fn from(variant: PGAG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PGAG`"]
pub type PGAG_R = crate::R<u8, PGAG_A>;
impl PGAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PGAG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PGAG_A::_0000),
            1 => Val(PGAG_A::_0001),
            2 => Val(PGAG_A::_0010),
            3 => Val(PGAG_A::_0011),
            4 => Val(PGAG_A::_0100),
            5 => Val(PGAG_A::_0101),
            6 => Val(PGAG_A::_0110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == PGAG_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == PGAG_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PGAG_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PGAG_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PGAG_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PGAG_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PGAG_A::_0110
    }
}
#[doc = "Write proxy for field `PGAG`"]
pub struct PGAG_W<'a> {
    w: &'a mut W,
}
impl<'a> PGAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGAG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PGAG_A::_0000)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PGAG_A::_0001)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(PGAG_A::_0010)
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(PGAG_A::_0011)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PGAG_A::_0100)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PGAG_A::_0101)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PGAG_A::_0110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "PGA low-power mode control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGALPB_A {
    #[doc = "0: PGA runs in low power mode."]
    _0 = 0,
    #[doc = "1: PGA runs in normal power mode."]
    _1 = 1,
}
impl From<PGALPB_A> for bool {
    #[inline(always)]
    fn from(variant: PGALPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PGALPb`"]
pub type PGALPB_R = crate::R<bool, PGALPB_A>;
impl PGALPB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGALPB_A {
        match self.bits {
            false => PGALPB_A::_0,
            true => PGALPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGALPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGALPB_A::_1
    }
}
#[doc = "Write proxy for field `PGALPb`"]
pub struct PGALPB_W<'a> {
    w: &'a mut W,
}
impl<'a> PGALPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGALPB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PGA runs in low power mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGALPB_A::_0)
    }
    #[doc = "PGA runs in normal power mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGALPB_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "PGA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGAEN_A {
    #[doc = "0: PGA disabled."]
    _0 = 0,
    #[doc = "1: PGA enabled."]
    _1 = 1,
}
impl From<PGAEN_A> for bool {
    #[inline(always)]
    fn from(variant: PGAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PGAEN`"]
pub type PGAEN_R = crate::R<bool, PGAEN_A>;
impl PGAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGAEN_A {
        match self.bits {
            false => PGAEN_A::_0,
            true => PGAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PGAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PGAEN_A::_1
    }
}
#[doc = "Write proxy for field `PGAEN`"]
pub struct PGAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PGAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PGA disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PGAEN_A::_0)
    }
    #[doc = "PGA enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PGAEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - PGA gain setting"]
    #[inline(always)]
    pub fn pgag(&self) -> PGAG_R {
        PGAG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - PGA low-power mode control"]
    #[inline(always)]
    pub fn pgalpb(&self) -> PGALPB_R {
        PGALPB_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 23 - PGA enable"]
    #[inline(always)]
    pub fn pgaen(&self) -> PGAEN_R {
        PGAEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:19 - PGA gain setting"]
    #[inline(always)]
    pub fn pgag(&mut self) -> PGAG_W {
        PGAG_W { w: self }
    }
    #[doc = "Bit 20 - PGA low-power mode control"]
    #[inline(always)]
    pub fn pgalpb(&mut self) -> PGALPB_W {
        PGALPB_W { w: self }
    }
    #[doc = "Bit 23 - PGA enable"]
    #[inline(always)]
    pub fn pgaen(&mut self) -> PGAEN_W {
        PGAEN_W { w: self }
    }
}
