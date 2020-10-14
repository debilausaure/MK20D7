#[doc = "Reader of register PEN"]
pub type R = crate::R<u32, super::PEN>;
#[doc = "Writer for register PEN"]
pub type W = crate::W<u32, super::PEN>;
#[doc = "Register PEN `reset()`'s with value 0"]
impl crate::ResetValue for super::PEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Touch Sensing Input Pin Enable Register 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN0_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN0_A> for bool {
    #[inline(always)]
    fn from(variant: PEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN0`"]
pub type PEN0_R = crate::R<bool, PEN0_A>;
impl PEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN0_A {
        match self.bits {
            false => PEN0_A::_0,
            true => PEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN0_A::_1
    }
}
#[doc = "Write proxy for field `PEN0`"]
pub struct PEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN0_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN0_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN1_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN1_A> for bool {
    #[inline(always)]
    fn from(variant: PEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN1`"]
pub type PEN1_R = crate::R<bool, PEN1_A>;
impl PEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN1_A {
        match self.bits {
            false => PEN1_A::_0,
            true => PEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN1_A::_1
    }
}
#[doc = "Write proxy for field `PEN1`"]
pub struct PEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN1_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN1_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN2_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN2_A> for bool {
    #[inline(always)]
    fn from(variant: PEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN2`"]
pub type PEN2_R = crate::R<bool, PEN2_A>;
impl PEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN2_A {
        match self.bits {
            false => PEN2_A::_0,
            true => PEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN2_A::_1
    }
}
#[doc = "Write proxy for field `PEN2`"]
pub struct PEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN2_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN2_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN3_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN3_A> for bool {
    #[inline(always)]
    fn from(variant: PEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN3`"]
pub type PEN3_R = crate::R<bool, PEN3_A>;
impl PEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN3_A {
        match self.bits {
            false => PEN3_A::_0,
            true => PEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN3_A::_1
    }
}
#[doc = "Write proxy for field `PEN3`"]
pub struct PEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN3_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN3_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN4_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN4_A> for bool {
    #[inline(always)]
    fn from(variant: PEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN4`"]
pub type PEN4_R = crate::R<bool, PEN4_A>;
impl PEN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN4_A {
        match self.bits {
            false => PEN4_A::_0,
            true => PEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN4_A::_1
    }
}
#[doc = "Write proxy for field `PEN4`"]
pub struct PEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN4_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN4_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN5_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN5_A> for bool {
    #[inline(always)]
    fn from(variant: PEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN5`"]
pub type PEN5_R = crate::R<bool, PEN5_A>;
impl PEN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN5_A {
        match self.bits {
            false => PEN5_A::_0,
            true => PEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN5_A::_1
    }
}
#[doc = "Write proxy for field `PEN5`"]
pub struct PEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN5_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN5_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN6_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN6_A> for bool {
    #[inline(always)]
    fn from(variant: PEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN6`"]
pub type PEN6_R = crate::R<bool, PEN6_A>;
impl PEN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN6_A {
        match self.bits {
            false => PEN6_A::_0,
            true => PEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN6_A::_1
    }
}
#[doc = "Write proxy for field `PEN6`"]
pub struct PEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN6_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN6_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN7_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN7_A> for bool {
    #[inline(always)]
    fn from(variant: PEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN7`"]
pub type PEN7_R = crate::R<bool, PEN7_A>;
impl PEN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN7_A {
        match self.bits {
            false => PEN7_A::_0,
            true => PEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN7_A::_1
    }
}
#[doc = "Write proxy for field `PEN7`"]
pub struct PEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN7_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN7_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN8_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN8_A> for bool {
    #[inline(always)]
    fn from(variant: PEN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN8`"]
pub type PEN8_R = crate::R<bool, PEN8_A>;
impl PEN8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN8_A {
        match self.bits {
            false => PEN8_A::_0,
            true => PEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN8_A::_1
    }
}
#[doc = "Write proxy for field `PEN8`"]
pub struct PEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN8_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN8_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Touch Sensing Input Pin Enable Register 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN9_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN9_A> for bool {
    #[inline(always)]
    fn from(variant: PEN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN9`"]
pub type PEN9_R = crate::R<bool, PEN9_A>;
impl PEN9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN9_A {
        match self.bits {
            false => PEN9_A::_0,
            true => PEN9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN9_A::_1
    }
}
#[doc = "Write proxy for field `PEN9`"]
pub struct PEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN9_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN9_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Touch Sensing Input Pin Enable Register 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN10_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN10_A> for bool {
    #[inline(always)]
    fn from(variant: PEN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN10`"]
pub type PEN10_R = crate::R<bool, PEN10_A>;
impl PEN10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN10_A {
        match self.bits {
            false => PEN10_A::_0,
            true => PEN10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN10_A::_1
    }
}
#[doc = "Write proxy for field `PEN10`"]
pub struct PEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN10_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN10_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN11_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN11_A> for bool {
    #[inline(always)]
    fn from(variant: PEN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN11`"]
pub type PEN11_R = crate::R<bool, PEN11_A>;
impl PEN11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN11_A {
        match self.bits {
            false => PEN11_A::_0,
            true => PEN11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN11_A::_1
    }
}
#[doc = "Write proxy for field `PEN11`"]
pub struct PEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN11_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN11_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN12_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN12_A> for bool {
    #[inline(always)]
    fn from(variant: PEN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN12`"]
pub type PEN12_R = crate::R<bool, PEN12_A>;
impl PEN12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN12_A {
        match self.bits {
            false => PEN12_A::_0,
            true => PEN12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN12_A::_1
    }
}
#[doc = "Write proxy for field `PEN12`"]
pub struct PEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN12_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN12_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN13_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN13_A> for bool {
    #[inline(always)]
    fn from(variant: PEN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN13`"]
pub type PEN13_R = crate::R<bool, PEN13_A>;
impl PEN13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN13_A {
        match self.bits {
            false => PEN13_A::_0,
            true => PEN13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN13_A::_1
    }
}
#[doc = "Write proxy for field `PEN13`"]
pub struct PEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN13_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN13_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN14_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN14_A> for bool {
    #[inline(always)]
    fn from(variant: PEN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN14`"]
pub type PEN14_R = crate::R<bool, PEN14_A>;
impl PEN14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN14_A {
        match self.bits {
            false => PEN14_A::_0,
            true => PEN14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN14_A::_1
    }
}
#[doc = "Write proxy for field `PEN14`"]
pub struct PEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN14_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN14_A::_1)
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
#[doc = "Touch Sensing Input Pin Enable Register 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEN15_A {
    #[doc = "0: The corresponding pin is not used by TSI."]
    _0 = 0,
    #[doc = "1: The corresponding pin is used by TSI."]
    _1 = 1,
}
impl From<PEN15_A> for bool {
    #[inline(always)]
    fn from(variant: PEN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEN15`"]
pub type PEN15_R = crate::R<bool, PEN15_A>;
impl PEN15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN15_A {
        match self.bits {
            false => PEN15_A::_0,
            true => PEN15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEN15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEN15_A::_1
    }
}
#[doc = "Write proxy for field `PEN15`"]
pub struct PEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEN15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding pin is not used by TSI."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEN15_A::_0)
    }
    #[doc = "The corresponding pin is used by TSI."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEN15_A::_1)
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
#[doc = "Low Power Scan Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPSP_A {
    #[doc = "0: TSI_IN\\[0\\]
is active in low power mode."]
    _0000 = 0,
    #[doc = "1: TSI_IN\\[1\\]
is active in low power mode."]
    _0001 = 1,
    #[doc = "2: TSI_IN\\[2\\]
is active in low power mode."]
    _0010 = 2,
    #[doc = "3: TSI_IN\\[3\\]
is active in low power mode."]
    _0011 = 3,
    #[doc = "4: TSI_IN\\[4\\]
is active in low power mode."]
    _0100 = 4,
    #[doc = "5: TSI_IN\\[5\\]
is active in low power mode."]
    _0101 = 5,
    #[doc = "6: TSI_IN\\[6\\]
is active in low power mode."]
    _0110 = 6,
    #[doc = "7: TSI_IN\\[7\\]
is active in low power mode."]
    _0111 = 7,
    #[doc = "8: TSI_IN\\[8\\]
is active in low power mode."]
    _1000 = 8,
    #[doc = "9: TSI_IN\\[9\\]
is active in low power mode."]
    _1001 = 9,
    #[doc = "10: TSI_IN\\[10\\]
is active in low power mode."]
    _1010 = 10,
    #[doc = "11: TSI_IN\\[11\\]
is active in low power mode."]
    _1011 = 11,
    #[doc = "12: TSI_IN\\[12\\]
is active in low power mode."]
    _1100 = 12,
    #[doc = "13: TSI_IN\\[13\\]
is active in low power mode."]
    _1101 = 13,
    #[doc = "14: TSI_IN\\[14\\]
is active in low power mode."]
    _1110 = 14,
    #[doc = "15: TSI_IN\\[15\\]
is active in low power mode."]
    _1111 = 15,
}
impl From<LPSP_A> for u8 {
    #[inline(always)]
    fn from(variant: LPSP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPSP`"]
pub type LPSP_R = crate::R<u8, LPSP_A>;
impl LPSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSP_A {
        match self.bits {
            0 => LPSP_A::_0000,
            1 => LPSP_A::_0001,
            2 => LPSP_A::_0010,
            3 => LPSP_A::_0011,
            4 => LPSP_A::_0100,
            5 => LPSP_A::_0101,
            6 => LPSP_A::_0110,
            7 => LPSP_A::_0111,
            8 => LPSP_A::_1000,
            9 => LPSP_A::_1001,
            10 => LPSP_A::_1010,
            11 => LPSP_A::_1011,
            12 => LPSP_A::_1100,
            13 => LPSP_A::_1101,
            14 => LPSP_A::_1110,
            15 => LPSP_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == LPSP_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == LPSP_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == LPSP_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == LPSP_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == LPSP_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == LPSP_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == LPSP_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == LPSP_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == LPSP_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == LPSP_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == LPSP_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == LPSP_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == LPSP_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == LPSP_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == LPSP_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == LPSP_A::_1111
    }
}
#[doc = "Write proxy for field `LPSP`"]
pub struct LPSP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPSP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "TSI_IN\\[0\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(LPSP_A::_0000)
    }
    #[doc = "TSI_IN\\[1\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(LPSP_A::_0001)
    }
    #[doc = "TSI_IN\\[2\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(LPSP_A::_0010)
    }
    #[doc = "TSI_IN\\[3\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(LPSP_A::_0011)
    }
    #[doc = "TSI_IN\\[4\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(LPSP_A::_0100)
    }
    #[doc = "TSI_IN\\[5\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(LPSP_A::_0101)
    }
    #[doc = "TSI_IN\\[6\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(LPSP_A::_0110)
    }
    #[doc = "TSI_IN\\[7\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(LPSP_A::_0111)
    }
    #[doc = "TSI_IN\\[8\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(LPSP_A::_1000)
    }
    #[doc = "TSI_IN\\[9\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(LPSP_A::_1001)
    }
    #[doc = "TSI_IN\\[10\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(LPSP_A::_1010)
    }
    #[doc = "TSI_IN\\[11\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(LPSP_A::_1011)
    }
    #[doc = "TSI_IN\\[12\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(LPSP_A::_1100)
    }
    #[doc = "TSI_IN\\[13\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(LPSP_A::_1101)
    }
    #[doc = "TSI_IN\\[14\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(LPSP_A::_1110)
    }
    #[doc = "TSI_IN\\[15\\]
is active in low power mode."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(LPSP_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Touch Sensing Input Pin Enable Register 0"]
    #[inline(always)]
    pub fn pen0(&self) -> PEN0_R {
        PEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Touch Sensing Input Pin Enable Register 1"]
    #[inline(always)]
    pub fn pen1(&self) -> PEN1_R {
        PEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Touch Sensing Input Pin Enable Register 2"]
    #[inline(always)]
    pub fn pen2(&self) -> PEN2_R {
        PEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Touch Sensing Input Pin Enable Register 3"]
    #[inline(always)]
    pub fn pen3(&self) -> PEN3_R {
        PEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Touch Sensing Input Pin Enable Register 4"]
    #[inline(always)]
    pub fn pen4(&self) -> PEN4_R {
        PEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Touch Sensing Input Pin Enable Register 5"]
    #[inline(always)]
    pub fn pen5(&self) -> PEN5_R {
        PEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Touch Sensing Input Pin Enable Register 6"]
    #[inline(always)]
    pub fn pen6(&self) -> PEN6_R {
        PEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Touch Sensing Input Pin Enable Register 7"]
    #[inline(always)]
    pub fn pen7(&self) -> PEN7_R {
        PEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Touch Sensing Input Pin Enable Register 8"]
    #[inline(always)]
    pub fn pen8(&self) -> PEN8_R {
        PEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Touch Sensing Input Pin Enable Register 9"]
    #[inline(always)]
    pub fn pen9(&self) -> PEN9_R {
        PEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Touch Sensing Input Pin Enable Register 10"]
    #[inline(always)]
    pub fn pen10(&self) -> PEN10_R {
        PEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Touch Sensing Input Pin Enable Register 11"]
    #[inline(always)]
    pub fn pen11(&self) -> PEN11_R {
        PEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Touch Sensing Input Pin Enable Register 12"]
    #[inline(always)]
    pub fn pen12(&self) -> PEN12_R {
        PEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Touch Sensing Input Pin Enable Register 13"]
    #[inline(always)]
    pub fn pen13(&self) -> PEN13_R {
        PEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Touch Sensing Input Pin Enable Register 14"]
    #[inline(always)]
    pub fn pen14(&self) -> PEN14_R {
        PEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Touch Sensing Input Pin Enable Register 15"]
    #[inline(always)]
    pub fn pen15(&self) -> PEN15_R {
        PEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Low Power Scan Pin"]
    #[inline(always)]
    pub fn lpsp(&self) -> LPSP_R {
        LPSP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Touch Sensing Input Pin Enable Register 0"]
    #[inline(always)]
    pub fn pen0(&mut self) -> PEN0_W {
        PEN0_W { w: self }
    }
    #[doc = "Bit 1 - Touch Sensing Input Pin Enable Register 1"]
    #[inline(always)]
    pub fn pen1(&mut self) -> PEN1_W {
        PEN1_W { w: self }
    }
    #[doc = "Bit 2 - Touch Sensing Input Pin Enable Register 2"]
    #[inline(always)]
    pub fn pen2(&mut self) -> PEN2_W {
        PEN2_W { w: self }
    }
    #[doc = "Bit 3 - Touch Sensing Input Pin Enable Register 3"]
    #[inline(always)]
    pub fn pen3(&mut self) -> PEN3_W {
        PEN3_W { w: self }
    }
    #[doc = "Bit 4 - Touch Sensing Input Pin Enable Register 4"]
    #[inline(always)]
    pub fn pen4(&mut self) -> PEN4_W {
        PEN4_W { w: self }
    }
    #[doc = "Bit 5 - Touch Sensing Input Pin Enable Register 5"]
    #[inline(always)]
    pub fn pen5(&mut self) -> PEN5_W {
        PEN5_W { w: self }
    }
    #[doc = "Bit 6 - Touch Sensing Input Pin Enable Register 6"]
    #[inline(always)]
    pub fn pen6(&mut self) -> PEN6_W {
        PEN6_W { w: self }
    }
    #[doc = "Bit 7 - Touch Sensing Input Pin Enable Register 7"]
    #[inline(always)]
    pub fn pen7(&mut self) -> PEN7_W {
        PEN7_W { w: self }
    }
    #[doc = "Bit 8 - Touch Sensing Input Pin Enable Register 8"]
    #[inline(always)]
    pub fn pen8(&mut self) -> PEN8_W {
        PEN8_W { w: self }
    }
    #[doc = "Bit 9 - Touch Sensing Input Pin Enable Register 9"]
    #[inline(always)]
    pub fn pen9(&mut self) -> PEN9_W {
        PEN9_W { w: self }
    }
    #[doc = "Bit 10 - Touch Sensing Input Pin Enable Register 10"]
    #[inline(always)]
    pub fn pen10(&mut self) -> PEN10_W {
        PEN10_W { w: self }
    }
    #[doc = "Bit 11 - Touch Sensing Input Pin Enable Register 11"]
    #[inline(always)]
    pub fn pen11(&mut self) -> PEN11_W {
        PEN11_W { w: self }
    }
    #[doc = "Bit 12 - Touch Sensing Input Pin Enable Register 12"]
    #[inline(always)]
    pub fn pen12(&mut self) -> PEN12_W {
        PEN12_W { w: self }
    }
    #[doc = "Bit 13 - Touch Sensing Input Pin Enable Register 13"]
    #[inline(always)]
    pub fn pen13(&mut self) -> PEN13_W {
        PEN13_W { w: self }
    }
    #[doc = "Bit 14 - Touch Sensing Input Pin Enable Register 14"]
    #[inline(always)]
    pub fn pen14(&mut self) -> PEN14_W {
        PEN14_W { w: self }
    }
    #[doc = "Bit 15 - Touch Sensing Input Pin Enable Register 15"]
    #[inline(always)]
    pub fn pen15(&mut self) -> PEN15_W {
        PEN15_W { w: self }
    }
    #[doc = "Bits 16:19 - Low Power Scan Pin"]
    #[inline(always)]
    pub fn lpsp(&mut self) -> LPSP_W {
        LPSP_W { w: self }
    }
}
