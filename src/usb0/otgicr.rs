#[doc = "Reader of register OTGICR"]
pub type R = crate::R<u8, super::OTGICR>;
#[doc = "Writer for register OTGICR"]
pub type W = crate::W<u8, super::OTGICR>;
#[doc = "Register OTGICR `reset()`'s with value 0"]
impl crate::ResetValue for super::OTGICR {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "A VBUS Valid interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVBUSEN_A {
    #[doc = "0: The AVBUSCHG interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The AVBUSCHG interrupt is enabled"]
    _1 = 1,
}
impl From<AVBUSEN_A> for bool {
    #[inline(always)]
    fn from(variant: AVBUSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AVBUSEN`"]
pub type AVBUSEN_R = crate::R<bool, AVBUSEN_A>;
impl AVBUSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVBUSEN_A {
        match self.bits {
            false => AVBUSEN_A::_0,
            true => AVBUSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVBUSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVBUSEN_A::_1
    }
}
#[doc = "Write proxy for field `AVBUSEN`"]
pub struct AVBUSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AVBUSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVBUSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The AVBUSCHG interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVBUSEN_A::_0)
    }
    #[doc = "The AVBUSCHG interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVBUSEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "B Session END interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSESSEN_A {
    #[doc = "0: The B_SESS_CHG interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The B_SESS_CHG interrupt is enabled"]
    _1 = 1,
}
impl From<BSESSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BSESSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BSESSEN`"]
pub type BSESSEN_R = crate::R<bool, BSESSEN_A>;
impl BSESSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSESSEN_A {
        match self.bits {
            false => BSESSEN_A::_0,
            true => BSESSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSESSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSESSEN_A::_1
    }
}
#[doc = "Write proxy for field `BSESSEN`"]
pub struct BSESSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BSESSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BSESSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The B_SESS_CHG interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSESSEN_A::_0)
    }
    #[doc = "The B_SESS_CHG interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSESSEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Session valid interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SESSVLDEN_A {
    #[doc = "0: The SESSVLDCHG interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The SESSVLDCHG interrupt is enabled."]
    _1 = 1,
}
impl From<SESSVLDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SESSVLDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SESSVLDEN`"]
pub type SESSVLDEN_R = crate::R<bool, SESSVLDEN_A>;
impl SESSVLDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESSVLDEN_A {
        match self.bits {
            false => SESSVLDEN_A::_0,
            true => SESSVLDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SESSVLDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SESSVLDEN_A::_1
    }
}
#[doc = "Write proxy for field `SESSVLDEN`"]
pub struct SESSVLDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SESSVLDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SESSVLDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The SESSVLDCHG interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SESSVLDEN_A::_0)
    }
    #[doc = "The SESSVLDCHG interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SESSVLDEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Line State change interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINESTATEEN_A {
    #[doc = "0: The LINE_STAT_CHG interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The LINE_STAT_CHG interrupt is enabled"]
    _1 = 1,
}
impl From<LINESTATEEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINESTATEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINESTATEEN`"]
pub type LINESTATEEN_R = crate::R<bool, LINESTATEEN_A>;
impl LINESTATEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESTATEEN_A {
        match self.bits {
            false => LINESTATEEN_A::_0,
            true => LINESTATEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINESTATEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINESTATEEN_A::_1
    }
}
#[doc = "Write proxy for field `LINESTATEEN`"]
pub struct LINESTATEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LINESTATEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINESTATEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The LINE_STAT_CHG interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINESTATEEN_A::_0)
    }
    #[doc = "The LINE_STAT_CHG interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINESTATEEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "1 millisecond interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONEMSECEN_A {
    #[doc = "0: The 1msec timer interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The 1msec timer interrupt is enabled."]
    _1 = 1,
}
impl From<ONEMSECEN_A> for bool {
    #[inline(always)]
    fn from(variant: ONEMSECEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONEMSECEN`"]
pub type ONEMSECEN_R = crate::R<bool, ONEMSECEN_A>;
impl ONEMSECEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONEMSECEN_A {
        match self.bits {
            false => ONEMSECEN_A::_0,
            true => ONEMSECEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ONEMSECEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ONEMSECEN_A::_1
    }
}
#[doc = "Write proxy for field `ONEMSECEN`"]
pub struct ONEMSECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ONEMSECEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONEMSECEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The 1msec timer interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONEMSECEN_A::_0)
    }
    #[doc = "The 1msec timer interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONEMSECEN_A::_1)
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
#[doc = "ID interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDEN_A {
    #[doc = "0: The ID interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The ID interrupt is enabled"]
    _1 = 1,
}
impl From<IDEN_A> for bool {
    #[inline(always)]
    fn from(variant: IDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDEN`"]
pub type IDEN_R = crate::R<bool, IDEN_A>;
impl IDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDEN_A {
        match self.bits {
            false => IDEN_A::_0,
            true => IDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDEN_A::_1
    }
}
#[doc = "Write proxy for field `IDEN`"]
pub struct IDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The ID interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDEN_A::_0)
    }
    #[doc = "The ID interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDEN_A::_1)
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
    #[doc = "Bit 0 - A VBUS Valid interrupt enable"]
    #[inline(always)]
    pub fn avbusen(&self) -> AVBUSEN_R {
        AVBUSEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - B Session END interrupt enable"]
    #[inline(always)]
    pub fn bsessen(&self) -> BSESSEN_R {
        BSESSEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Session valid interrupt enable"]
    #[inline(always)]
    pub fn sessvlden(&self) -> SESSVLDEN_R {
        SESSVLDEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Line State change interrupt enable"]
    #[inline(always)]
    pub fn linestateen(&self) -> LINESTATEEN_R {
        LINESTATEEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1 millisecond interrupt enable"]
    #[inline(always)]
    pub fn onemsecen(&self) -> ONEMSECEN_R {
        ONEMSECEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ID interrupt enable"]
    #[inline(always)]
    pub fn iden(&self) -> IDEN_R {
        IDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A VBUS Valid interrupt enable"]
    #[inline(always)]
    pub fn avbusen(&mut self) -> AVBUSEN_W {
        AVBUSEN_W { w: self }
    }
    #[doc = "Bit 2 - B Session END interrupt enable"]
    #[inline(always)]
    pub fn bsessen(&mut self) -> BSESSEN_W {
        BSESSEN_W { w: self }
    }
    #[doc = "Bit 3 - Session valid interrupt enable"]
    #[inline(always)]
    pub fn sessvlden(&mut self) -> SESSVLDEN_W {
        SESSVLDEN_W { w: self }
    }
    #[doc = "Bit 5 - Line State change interrupt enable"]
    #[inline(always)]
    pub fn linestateen(&mut self) -> LINESTATEEN_W {
        LINESTATEEN_W { w: self }
    }
    #[doc = "Bit 6 - 1 millisecond interrupt enable"]
    #[inline(always)]
    pub fn onemsecen(&mut self) -> ONEMSECEN_W {
        ONEMSECEN_W { w: self }
    }
    #[doc = "Bit 7 - ID interrupt enable"]
    #[inline(always)]
    pub fn iden(&mut self) -> IDEN_W {
        IDEN_W { w: self }
    }
}
