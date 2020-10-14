#[doc = "Reader of register HRS"]
pub type R = crate::R<u32, super::HRS>;
#[doc = "Writer for register HRS"]
pub type W = crate::W<u32, super::HRS>;
#[doc = "Register HRS `reset()`'s with value 0"]
impl crate::ResetValue for super::HRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Hardware Request Status Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS0_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS0_A> for bool {
    #[inline(always)]
    fn from(variant: HRS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS0`"]
pub type HRS0_R = crate::R<bool, HRS0_A>;
impl HRS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS0_A {
        match self.bits {
            false => HRS0_A::_0,
            true => HRS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS0_A::_1
    }
}
#[doc = "Write proxy for field `HRS0`"]
pub struct HRS0_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS0_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS0_A::_1)
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
#[doc = "Hardware Request Status Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS1_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS1_A> for bool {
    #[inline(always)]
    fn from(variant: HRS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS1`"]
pub type HRS1_R = crate::R<bool, HRS1_A>;
impl HRS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS1_A {
        match self.bits {
            false => HRS1_A::_0,
            true => HRS1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS1_A::_1
    }
}
#[doc = "Write proxy for field `HRS1`"]
pub struct HRS1_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS1_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS1_A::_1)
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
#[doc = "Hardware Request Status Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS2_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS2_A> for bool {
    #[inline(always)]
    fn from(variant: HRS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS2`"]
pub type HRS2_R = crate::R<bool, HRS2_A>;
impl HRS2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS2_A {
        match self.bits {
            false => HRS2_A::_0,
            true => HRS2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS2_A::_1
    }
}
#[doc = "Write proxy for field `HRS2`"]
pub struct HRS2_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS2_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS2_A::_1)
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
#[doc = "Hardware Request Status Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS3_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS3_A> for bool {
    #[inline(always)]
    fn from(variant: HRS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS3`"]
pub type HRS3_R = crate::R<bool, HRS3_A>;
impl HRS3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS3_A {
        match self.bits {
            false => HRS3_A::_0,
            true => HRS3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS3_A::_1
    }
}
#[doc = "Write proxy for field `HRS3`"]
pub struct HRS3_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS3_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS3_A::_1)
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
#[doc = "Hardware Request Status Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS4_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS4_A> for bool {
    #[inline(always)]
    fn from(variant: HRS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS4`"]
pub type HRS4_R = crate::R<bool, HRS4_A>;
impl HRS4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS4_A {
        match self.bits {
            false => HRS4_A::_0,
            true => HRS4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS4_A::_1
    }
}
#[doc = "Write proxy for field `HRS4`"]
pub struct HRS4_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS4_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS4_A::_1)
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
#[doc = "Hardware Request Status Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS5_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS5_A> for bool {
    #[inline(always)]
    fn from(variant: HRS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS5`"]
pub type HRS5_R = crate::R<bool, HRS5_A>;
impl HRS5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS5_A {
        match self.bits {
            false => HRS5_A::_0,
            true => HRS5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS5_A::_1
    }
}
#[doc = "Write proxy for field `HRS5`"]
pub struct HRS5_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS5_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS5_A::_1)
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
#[doc = "Hardware Request Status Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS6_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS6_A> for bool {
    #[inline(always)]
    fn from(variant: HRS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS6`"]
pub type HRS6_R = crate::R<bool, HRS6_A>;
impl HRS6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS6_A {
        match self.bits {
            false => HRS6_A::_0,
            true => HRS6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS6_A::_1
    }
}
#[doc = "Write proxy for field `HRS6`"]
pub struct HRS6_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS6_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS6_A::_1)
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
#[doc = "Hardware Request Status Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS7_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS7_A> for bool {
    #[inline(always)]
    fn from(variant: HRS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS7`"]
pub type HRS7_R = crate::R<bool, HRS7_A>;
impl HRS7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS7_A {
        match self.bits {
            false => HRS7_A::_0,
            true => HRS7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS7_A::_1
    }
}
#[doc = "Write proxy for field `HRS7`"]
pub struct HRS7_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS7_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS7_A::_1)
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
#[doc = "Hardware Request Status Channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS8_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS8_A> for bool {
    #[inline(always)]
    fn from(variant: HRS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS8`"]
pub type HRS8_R = crate::R<bool, HRS8_A>;
impl HRS8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS8_A {
        match self.bits {
            false => HRS8_A::_0,
            true => HRS8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS8_A::_1
    }
}
#[doc = "Write proxy for field `HRS8`"]
pub struct HRS8_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS8_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS8_A::_1)
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
#[doc = "Hardware Request Status Channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS9_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS9_A> for bool {
    #[inline(always)]
    fn from(variant: HRS9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS9`"]
pub type HRS9_R = crate::R<bool, HRS9_A>;
impl HRS9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS9_A {
        match self.bits {
            false => HRS9_A::_0,
            true => HRS9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS9_A::_1
    }
}
#[doc = "Write proxy for field `HRS9`"]
pub struct HRS9_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS9_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS9_A::_1)
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
#[doc = "Hardware Request Status Channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS10_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS10_A> for bool {
    #[inline(always)]
    fn from(variant: HRS10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS10`"]
pub type HRS10_R = crate::R<bool, HRS10_A>;
impl HRS10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS10_A {
        match self.bits {
            false => HRS10_A::_0,
            true => HRS10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS10_A::_1
    }
}
#[doc = "Write proxy for field `HRS10`"]
pub struct HRS10_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS10_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS10_A::_1)
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
#[doc = "Hardware Request Status Channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS11_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS11_A> for bool {
    #[inline(always)]
    fn from(variant: HRS11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS11`"]
pub type HRS11_R = crate::R<bool, HRS11_A>;
impl HRS11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS11_A {
        match self.bits {
            false => HRS11_A::_0,
            true => HRS11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS11_A::_1
    }
}
#[doc = "Write proxy for field `HRS11`"]
pub struct HRS11_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS11_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS11_A::_1)
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
#[doc = "Hardware Request Status Channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS12_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS12_A> for bool {
    #[inline(always)]
    fn from(variant: HRS12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS12`"]
pub type HRS12_R = crate::R<bool, HRS12_A>;
impl HRS12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS12_A {
        match self.bits {
            false => HRS12_A::_0,
            true => HRS12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS12_A::_1
    }
}
#[doc = "Write proxy for field `HRS12`"]
pub struct HRS12_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS12_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS12_A::_1)
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
#[doc = "Hardware Request Status Channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS13_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS13_A> for bool {
    #[inline(always)]
    fn from(variant: HRS13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS13`"]
pub type HRS13_R = crate::R<bool, HRS13_A>;
impl HRS13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS13_A {
        match self.bits {
            false => HRS13_A::_0,
            true => HRS13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS13_A::_1
    }
}
#[doc = "Write proxy for field `HRS13`"]
pub struct HRS13_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS13_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS13_A::_1)
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
#[doc = "Hardware Request Status Channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS14_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS14_A> for bool {
    #[inline(always)]
    fn from(variant: HRS14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS14`"]
pub type HRS14_R = crate::R<bool, HRS14_A>;
impl HRS14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS14_A {
        match self.bits {
            false => HRS14_A::_0,
            true => HRS14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS14_A::_1
    }
}
#[doc = "Write proxy for field `HRS14`"]
pub struct HRS14_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS14_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS14_A::_1)
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
#[doc = "Hardware Request Status Channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRS15_A {
    #[doc = "0: A hardware service request for the corresponding channel is not present"]
    _0 = 0,
    #[doc = "1: A hardware service request for the corresponding channel is present"]
    _1 = 1,
}
impl From<HRS15_A> for bool {
    #[inline(always)]
    fn from(variant: HRS15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRS15`"]
pub type HRS15_R = crate::R<bool, HRS15_A>;
impl HRS15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRS15_A {
        match self.bits {
            false => HRS15_A::_0,
            true => HRS15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HRS15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HRS15_A::_1
    }
}
#[doc = "Write proxy for field `HRS15`"]
pub struct HRS15_W<'a> {
    w: &'a mut W,
}
impl<'a> HRS15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRS15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A hardware service request for the corresponding channel is not present"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRS15_A::_0)
    }
    #[doc = "A hardware service request for the corresponding channel is present"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRS15_A::_1)
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
impl R {
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline(always)]
    pub fn hrs0(&self) -> HRS0_R {
        HRS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline(always)]
    pub fn hrs1(&self) -> HRS1_R {
        HRS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline(always)]
    pub fn hrs2(&self) -> HRS2_R {
        HRS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline(always)]
    pub fn hrs3(&self) -> HRS3_R {
        HRS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline(always)]
    pub fn hrs4(&self) -> HRS4_R {
        HRS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline(always)]
    pub fn hrs5(&self) -> HRS5_R {
        HRS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline(always)]
    pub fn hrs6(&self) -> HRS6_R {
        HRS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline(always)]
    pub fn hrs7(&self) -> HRS7_R {
        HRS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline(always)]
    pub fn hrs8(&self) -> HRS8_R {
        HRS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline(always)]
    pub fn hrs9(&self) -> HRS9_R {
        HRS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline(always)]
    pub fn hrs10(&self) -> HRS10_R {
        HRS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline(always)]
    pub fn hrs11(&self) -> HRS11_R {
        HRS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline(always)]
    pub fn hrs12(&self) -> HRS12_R {
        HRS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline(always)]
    pub fn hrs13(&self) -> HRS13_R {
        HRS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline(always)]
    pub fn hrs14(&self) -> HRS14_R {
        HRS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline(always)]
    pub fn hrs15(&self) -> HRS15_R {
        HRS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware Request Status Channel 0"]
    #[inline(always)]
    pub fn hrs0(&mut self) -> HRS0_W {
        HRS0_W { w: self }
    }
    #[doc = "Bit 1 - Hardware Request Status Channel 1"]
    #[inline(always)]
    pub fn hrs1(&mut self) -> HRS1_W {
        HRS1_W { w: self }
    }
    #[doc = "Bit 2 - Hardware Request Status Channel 2"]
    #[inline(always)]
    pub fn hrs2(&mut self) -> HRS2_W {
        HRS2_W { w: self }
    }
    #[doc = "Bit 3 - Hardware Request Status Channel 3"]
    #[inline(always)]
    pub fn hrs3(&mut self) -> HRS3_W {
        HRS3_W { w: self }
    }
    #[doc = "Bit 4 - Hardware Request Status Channel 4"]
    #[inline(always)]
    pub fn hrs4(&mut self) -> HRS4_W {
        HRS4_W { w: self }
    }
    #[doc = "Bit 5 - Hardware Request Status Channel 5"]
    #[inline(always)]
    pub fn hrs5(&mut self) -> HRS5_W {
        HRS5_W { w: self }
    }
    #[doc = "Bit 6 - Hardware Request Status Channel 6"]
    #[inline(always)]
    pub fn hrs6(&mut self) -> HRS6_W {
        HRS6_W { w: self }
    }
    #[doc = "Bit 7 - Hardware Request Status Channel 7"]
    #[inline(always)]
    pub fn hrs7(&mut self) -> HRS7_W {
        HRS7_W { w: self }
    }
    #[doc = "Bit 8 - Hardware Request Status Channel 8"]
    #[inline(always)]
    pub fn hrs8(&mut self) -> HRS8_W {
        HRS8_W { w: self }
    }
    #[doc = "Bit 9 - Hardware Request Status Channel 9"]
    #[inline(always)]
    pub fn hrs9(&mut self) -> HRS9_W {
        HRS9_W { w: self }
    }
    #[doc = "Bit 10 - Hardware Request Status Channel 10"]
    #[inline(always)]
    pub fn hrs10(&mut self) -> HRS10_W {
        HRS10_W { w: self }
    }
    #[doc = "Bit 11 - Hardware Request Status Channel 11"]
    #[inline(always)]
    pub fn hrs11(&mut self) -> HRS11_W {
        HRS11_W { w: self }
    }
    #[doc = "Bit 12 - Hardware Request Status Channel 12"]
    #[inline(always)]
    pub fn hrs12(&mut self) -> HRS12_W {
        HRS12_W { w: self }
    }
    #[doc = "Bit 13 - Hardware Request Status Channel 13"]
    #[inline(always)]
    pub fn hrs13(&mut self) -> HRS13_W {
        HRS13_W { w: self }
    }
    #[doc = "Bit 14 - Hardware Request Status Channel 14"]
    #[inline(always)]
    pub fn hrs14(&mut self) -> HRS14_W {
        HRS14_W { w: self }
    }
    #[doc = "Bit 15 - Hardware Request Status Channel 15"]
    #[inline(always)]
    pub fn hrs15(&mut self) -> HRS15_W {
        HRS15_W { w: self }
    }
}
