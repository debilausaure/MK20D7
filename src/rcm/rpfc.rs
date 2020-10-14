#[doc = "Reader of register RPFC"]
pub type R = crate::R<u8, super::RPFC>;
#[doc = "Writer for register RPFC"]
pub type W = crate::W<u8, super::RPFC>;
#[doc = "Register RPFC `reset()`'s with value 0"]
impl crate::ResetValue for super::RPFC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reset pin filter select in run and wait modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTFLTSRW_A {
    #[doc = "0: All filtering disabled"]
    _00 = 0,
    #[doc = "1: Bus clock filter enabled for normal operation"]
    _01 = 1,
    #[doc = "2: LPO clock filter enabled for normal operation"]
    _10 = 2,
    #[doc = "3: Reserved (all filtering disabled)"]
    _11 = 3,
}
impl From<RSTFLTSRW_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTFLTSRW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSTFLTSRW`"]
pub type RSTFLTSRW_R = crate::R<u8, RSTFLTSRW_A>;
impl RSTFLTSRW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFLTSRW_A {
        match self.bits {
            0 => RSTFLTSRW_A::_00,
            1 => RSTFLTSRW_A::_01,
            2 => RSTFLTSRW_A::_10,
            3 => RSTFLTSRW_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RSTFLTSRW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RSTFLTSRW_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RSTFLTSRW_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RSTFLTSRW_A::_11
    }
}
#[doc = "Write proxy for field `RSTFLTSRW`"]
pub struct RSTFLTSRW_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFLTSRW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTFLTSRW_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_00)
    }
    #[doc = "Bus clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_01)
    }
    #[doc = "LPO clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_10)
    }
    #[doc = "Reserved (all filtering disabled)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RSTFLTSRW_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u8) & 0x03);
        self.w
    }
}
#[doc = "Reset pin filter select in stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFLTSS_A {
    #[doc = "0: All filtering disabled"]
    _0 = 0,
    #[doc = "1: LPO clock filter enabled"]
    _1 = 1,
}
impl From<RSTFLTSS_A> for bool {
    #[inline(always)]
    fn from(variant: RSTFLTSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSTFLTSS`"]
pub type RSTFLTSS_R = crate::R<bool, RSTFLTSS_A>;
impl RSTFLTSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSTFLTSS_A {
        match self.bits {
            false => RSTFLTSS_A::_0,
            true => RSTFLTSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSTFLTSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSTFLTSS_A::_1
    }
}
#[doc = "Write proxy for field `RSTFLTSS`"]
pub struct RSTFLTSS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTFLTSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTFLTSS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTFLTSS_A::_0)
    }
    #[doc = "LPO clock filter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTFLTSS_A::_1)
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
impl R {
    #[doc = "Bits 0:1 - Reset pin filter select in run and wait modes"]
    #[inline(always)]
    pub fn rstfltsrw(&self) -> RSTFLTSRW_R {
        RSTFLTSRW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Reset pin filter select in stop mode"]
    #[inline(always)]
    pub fn rstfltss(&self) -> RSTFLTSS_R {
        RSTFLTSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reset pin filter select in run and wait modes"]
    #[inline(always)]
    pub fn rstfltsrw(&mut self) -> RSTFLTSRW_W {
        RSTFLTSRW_W { w: self }
    }
    #[doc = "Bit 2 - Reset pin filter select in stop mode"]
    #[inline(always)]
    pub fn rstfltss(&mut self) -> RSTFLTSS_W {
        RSTFLTSS_W { w: self }
    }
}
