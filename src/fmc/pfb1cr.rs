#[doc = "Reader of register PFB1CR"]
pub type R = crate::R<u32, super::PFB1CR>;
#[doc = "Writer for register PFB1CR"]
pub type W = crate::W<u32, super::PFB1CR>;
#[doc = "Register PFB1CR `reset()`'s with value 0x3002_001f"]
impl crate::ResetValue for super::PFB1CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3002_001f
    }
}
#[doc = "Bank 1 Single Entry Buffer Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B1SEBE_A {
    #[doc = "0: Single entry buffer is disabled."]
    _0 = 0,
    #[doc = "1: Single entry buffer is enabled."]
    _1 = 1,
}
impl From<B1SEBE_A> for bool {
    #[inline(always)]
    fn from(variant: B1SEBE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `B1SEBE`"]
pub type B1SEBE_R = crate::R<bool, B1SEBE_A>;
impl B1SEBE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B1SEBE_A {
        match self.bits {
            false => B1SEBE_A::_0,
            true => B1SEBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1SEBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1SEBE_A::_1
    }
}
#[doc = "Write proxy for field `B1SEBE`"]
pub struct B1SEBE_W<'a> {
    w: &'a mut W,
}
impl<'a> B1SEBE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B1SEBE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single entry buffer is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1SEBE_A::_0)
    }
    #[doc = "Single entry buffer is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1SEBE_A::_1)
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
#[doc = "Bank 1 Instruction Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B1IPE_A {
    #[doc = "0: Do not prefetch in response to instruction fetches."]
    _0 = 0,
    #[doc = "1: Enable prefetches in response to instruction fetches."]
    _1 = 1,
}
impl From<B1IPE_A> for bool {
    #[inline(always)]
    fn from(variant: B1IPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `B1IPE`"]
pub type B1IPE_R = crate::R<bool, B1IPE_A>;
impl B1IPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B1IPE_A {
        match self.bits {
            false => B1IPE_A::_0,
            true => B1IPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1IPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1IPE_A::_1
    }
}
#[doc = "Write proxy for field `B1IPE`"]
pub struct B1IPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B1IPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B1IPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not prefetch in response to instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1IPE_A::_0)
    }
    #[doc = "Enable prefetches in response to instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1IPE_A::_1)
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
#[doc = "Bank 1 Data Prefetch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B1DPE_A {
    #[doc = "0: Do not prefetch in response to data references."]
    _0 = 0,
    #[doc = "1: Enable prefetches in response to data references."]
    _1 = 1,
}
impl From<B1DPE_A> for bool {
    #[inline(always)]
    fn from(variant: B1DPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `B1DPE`"]
pub type B1DPE_R = crate::R<bool, B1DPE_A>;
impl B1DPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B1DPE_A {
        match self.bits {
            false => B1DPE_A::_0,
            true => B1DPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1DPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1DPE_A::_1
    }
}
#[doc = "Write proxy for field `B1DPE`"]
pub struct B1DPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B1DPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B1DPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not prefetch in response to data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1DPE_A::_0)
    }
    #[doc = "Enable prefetches in response to data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1DPE_A::_1)
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
#[doc = "Bank 1 Instruction Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B1ICE_A {
    #[doc = "0: Do not cache instruction fetches."]
    _0 = 0,
    #[doc = "1: Cache instruction fetches."]
    _1 = 1,
}
impl From<B1ICE_A> for bool {
    #[inline(always)]
    fn from(variant: B1ICE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `B1ICE`"]
pub type B1ICE_R = crate::R<bool, B1ICE_A>;
impl B1ICE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B1ICE_A {
        match self.bits {
            false => B1ICE_A::_0,
            true => B1ICE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1ICE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1ICE_A::_1
    }
}
#[doc = "Write proxy for field `B1ICE`"]
pub struct B1ICE_W<'a> {
    w: &'a mut W,
}
impl<'a> B1ICE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B1ICE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not cache instruction fetches."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1ICE_A::_0)
    }
    #[doc = "Cache instruction fetches."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1ICE_A::_1)
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
#[doc = "Bank 1 Data Cache Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum B1DCE_A {
    #[doc = "0: Do not cache data references."]
    _0 = 0,
    #[doc = "1: Cache data references."]
    _1 = 1,
}
impl From<B1DCE_A> for bool {
    #[inline(always)]
    fn from(variant: B1DCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `B1DCE`"]
pub type B1DCE_R = crate::R<bool, B1DCE_A>;
impl B1DCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> B1DCE_A {
        match self.bits {
            false => B1DCE_A::_0,
            true => B1DCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == B1DCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == B1DCE_A::_1
    }
}
#[doc = "Write proxy for field `B1DCE`"]
pub struct B1DCE_W<'a> {
    w: &'a mut W,
}
impl<'a> B1DCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: B1DCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not cache data references."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(B1DCE_A::_0)
    }
    #[doc = "Cache data references."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(B1DCE_A::_1)
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
#[doc = "Bank 1 Memory Width\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum B1MW_A {
    #[doc = "0: 32 bits"]
    _00 = 0,
    #[doc = "1: 64 bits"]
    _01 = 1,
}
impl From<B1MW_A> for u8 {
    #[inline(always)]
    fn from(variant: B1MW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `B1MW`"]
pub type B1MW_R = crate::R<u8, B1MW_A>;
impl B1MW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, B1MW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(B1MW_A::_00),
            1 => Val(B1MW_A::_01),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == B1MW_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == B1MW_A::_01
    }
}
#[doc = "Reader of field `B1RWSC`"]
pub type B1RWSC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Bank 1 Single Entry Buffer Enable"]
    #[inline(always)]
    pub fn b1sebe(&self) -> B1SEBE_R {
        B1SEBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bank 1 Instruction Prefetch Enable"]
    #[inline(always)]
    pub fn b1ipe(&self) -> B1IPE_R {
        B1IPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bank 1 Data Prefetch Enable"]
    #[inline(always)]
    pub fn b1dpe(&self) -> B1DPE_R {
        B1DPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bank 1 Instruction Cache Enable"]
    #[inline(always)]
    pub fn b1ice(&self) -> B1ICE_R {
        B1ICE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bank 1 Data Cache Enable"]
    #[inline(always)]
    pub fn b1dce(&self) -> B1DCE_R {
        B1DCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Bank 1 Memory Width"]
    #[inline(always)]
    pub fn b1mw(&self) -> B1MW_R {
        B1MW_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 28:31 - Bank 1 Read Wait State Control"]
    #[inline(always)]
    pub fn b1rwsc(&self) -> B1RWSC_R {
        B1RWSC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 Single Entry Buffer Enable"]
    #[inline(always)]
    pub fn b1sebe(&mut self) -> B1SEBE_W {
        B1SEBE_W { w: self }
    }
    #[doc = "Bit 1 - Bank 1 Instruction Prefetch Enable"]
    #[inline(always)]
    pub fn b1ipe(&mut self) -> B1IPE_W {
        B1IPE_W { w: self }
    }
    #[doc = "Bit 2 - Bank 1 Data Prefetch Enable"]
    #[inline(always)]
    pub fn b1dpe(&mut self) -> B1DPE_W {
        B1DPE_W { w: self }
    }
    #[doc = "Bit 3 - Bank 1 Instruction Cache Enable"]
    #[inline(always)]
    pub fn b1ice(&mut self) -> B1ICE_W {
        B1ICE_W { w: self }
    }
    #[doc = "Bit 4 - Bank 1 Data Cache Enable"]
    #[inline(always)]
    pub fn b1dce(&mut self) -> B1DCE_W {
        B1DCE_W { w: self }
    }
}
