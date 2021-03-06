#[doc = "Reader of register DFSR"]
pub type R = crate::R<u32, super::DFSR>;
#[doc = "Writer for register DFSR"]
pub type W = crate::W<u32, super::DFSR>;
#[doc = "Register DFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::DFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALTED_A {
    #[doc = "0: No active halt request debug event"]
    _0 = 0,
    #[doc = "1: Halt request debug event active"]
    _1 = 1,
}
impl From<HALTED_A> for bool {
    #[inline(always)]
    fn from(variant: HALTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HALTED`"]
pub type HALTED_R = crate::R<bool, HALTED_A>;
impl HALTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALTED_A {
        match self.bits {
            false => HALTED_A::_0,
            true => HALTED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HALTED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HALTED_A::_1
    }
}
#[doc = "Write proxy for field `HALTED`"]
pub struct HALTED_W<'a> {
    w: &'a mut W,
}
impl<'a> HALTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALTED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No active halt request debug event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALTED_A::_0)
    }
    #[doc = "Halt request debug event active"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALTED_A::_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKPT_A {
    #[doc = "0: No current breakpoint debug event"]
    _0 = 0,
    #[doc = "1: At least one current breakpoint debug event"]
    _1 = 1,
}
impl From<BKPT_A> for bool {
    #[inline(always)]
    fn from(variant: BKPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKPT`"]
pub type BKPT_R = crate::R<bool, BKPT_A>;
impl BKPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKPT_A {
        match self.bits {
            false => BKPT_A::_0,
            true => BKPT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BKPT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BKPT_A::_1
    }
}
#[doc = "Write proxy for field `BKPT`"]
pub struct BKPT_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKPT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No current breakpoint debug event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BKPT_A::_0)
    }
    #[doc = "At least one current breakpoint debug event"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BKPT_A::_1)
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
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWTTRAP_A {
    #[doc = "0: No current debug events generated by the DWT"]
    _0 = 0,
    #[doc = "1: At least one current debug event generated by the DWT"]
    _1 = 1,
}
impl From<DWTTRAP_A> for bool {
    #[inline(always)]
    fn from(variant: DWTTRAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DWTTRAP`"]
pub type DWTTRAP_R = crate::R<bool, DWTTRAP_A>;
impl DWTTRAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DWTTRAP_A {
        match self.bits {
            false => DWTTRAP_A::_0,
            true => DWTTRAP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DWTTRAP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DWTTRAP_A::_1
    }
}
#[doc = "Write proxy for field `DWTTRAP`"]
pub struct DWTTRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DWTTRAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DWTTRAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No current debug events generated by the DWT"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DWTTRAP_A::_0)
    }
    #[doc = "At least one current debug event generated by the DWT"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DWTTRAP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCATCH_A {
    #[doc = "0: No Vector catch triggered"]
    _0 = 0,
    #[doc = "1: Vector catch triggered"]
    _1 = 1,
}
impl From<VCATCH_A> for bool {
    #[inline(always)]
    fn from(variant: VCATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VCATCH`"]
pub type VCATCH_R = crate::R<bool, VCATCH_A>;
impl VCATCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCATCH_A {
        match self.bits {
            false => VCATCH_A::_0,
            true => VCATCH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VCATCH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VCATCH_A::_1
    }
}
#[doc = "Write proxy for field `VCATCH`"]
pub struct VCATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VCATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCATCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Vector catch triggered"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VCATCH_A::_0)
    }
    #[doc = "Vector catch triggered"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VCATCH_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTERNAL_A {
    #[doc = "0: No EDBGRQ debug event"]
    _0 = 0,
    #[doc = "1: EDBGRQ debug event"]
    _1 = 1,
}
impl From<EXTERNAL_A> for bool {
    #[inline(always)]
    fn from(variant: EXTERNAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTERNAL`"]
pub type EXTERNAL_R = crate::R<bool, EXTERNAL_A>;
impl EXTERNAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTERNAL_A {
        match self.bits {
            false => EXTERNAL_A::_0,
            true => EXTERNAL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXTERNAL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXTERNAL_A::_1
    }
}
#[doc = "Write proxy for field `EXTERNAL`"]
pub struct EXTERNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTERNAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No EDBGRQ debug event"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTERNAL_A::_0)
    }
    #[doc = "EDBGRQ debug event"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTERNAL_A::_1)
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
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bkpt(&self) -> BKPT_R {
        BKPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn dwttrap(&self) -> DWTTRAP_R {
        DWTTRAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn vcatch(&self) -> VCATCH_R {
        VCATCH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn halted(&mut self) -> HALTED_W {
        HALTED_W { w: self }
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bkpt(&mut self) -> BKPT_W {
        BKPT_W { w: self }
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn dwttrap(&mut self) -> DWTTRAP_W {
        DWTTRAP_W { w: self }
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn vcatch(&mut self) -> VCATCH_W {
        VCATCH_W { w: self }
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn external(&mut self) -> EXTERNAL_W {
        EXTERNAL_W { w: self }
    }
}
