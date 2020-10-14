#[doc = "Reader of register CSPMCR"]
pub type R = crate::R<u32, super::CSPMCR>;
#[doc = "Writer for register CSPMCR"]
pub type W = crate::W<u32, super::CSPMCR>;
#[doc = "Register CSPMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSPMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FlexBus signal group 5 multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GROUP5_A {
    #[doc = "0: FB_TA"]
    _0000 = 0,
    #[doc = "1: FB_CS3. You must also set CSCRn\\[AA\\]."]
    _0001 = 1,
    #[doc = "2: FB_BE_7_0. You must also set CSCRn\\[AA\\]."]
    _0010 = 2,
}
impl From<GROUP5_A> for u8 {
    #[inline(always)]
    fn from(variant: GROUP5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GROUP5`"]
pub type GROUP5_R = crate::R<u8, GROUP5_A>;
impl GROUP5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GROUP5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GROUP5_A::_0000),
            1 => Val(GROUP5_A::_0001),
            2 => Val(GROUP5_A::_0010),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == GROUP5_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == GROUP5_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == GROUP5_A::_0010
    }
}
#[doc = "Write proxy for field `GROUP5`"]
pub struct GROUP5_W<'a> {
    w: &'a mut W,
}
impl<'a> GROUP5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROUP5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FB_TA"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP5_A::_0000)
    }
    #[doc = "FB_CS3. You must also set CSCRn\\[AA\\]."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP5_A::_0001)
    }
    #[doc = "FB_BE_7_0. You must also set CSCRn\\[AA\\]."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP5_A::_0010)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "FlexBus signal group 4 multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GROUP4_A {
    #[doc = "0: FB_TBST"]
    _0000 = 0,
    #[doc = "1: FB_CS2"]
    _0001 = 1,
    #[doc = "2: FB_BE_15_8"]
    _0010 = 2,
}
impl From<GROUP4_A> for u8 {
    #[inline(always)]
    fn from(variant: GROUP4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GROUP4`"]
pub type GROUP4_R = crate::R<u8, GROUP4_A>;
impl GROUP4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GROUP4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GROUP4_A::_0000),
            1 => Val(GROUP4_A::_0001),
            2 => Val(GROUP4_A::_0010),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == GROUP4_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == GROUP4_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == GROUP4_A::_0010
    }
}
#[doc = "Write proxy for field `GROUP4`"]
pub struct GROUP4_W<'a> {
    w: &'a mut W,
}
impl<'a> GROUP4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROUP4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FB_TBST"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP4_A::_0000)
    }
    #[doc = "FB_CS2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP4_A::_0001)
    }
    #[doc = "FB_BE_15_8"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP4_A::_0010)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "FlexBus signal group 3 multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GROUP3_A {
    #[doc = "0: FB_CS5"]
    _0000 = 0,
    #[doc = "1: FB_TSIZ1"]
    _0001 = 1,
    #[doc = "2: FB_BE_23_16"]
    _0010 = 2,
}
impl From<GROUP3_A> for u8 {
    #[inline(always)]
    fn from(variant: GROUP3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GROUP3`"]
pub type GROUP3_R = crate::R<u8, GROUP3_A>;
impl GROUP3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GROUP3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GROUP3_A::_0000),
            1 => Val(GROUP3_A::_0001),
            2 => Val(GROUP3_A::_0010),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == GROUP3_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == GROUP3_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == GROUP3_A::_0010
    }
}
#[doc = "Write proxy for field `GROUP3`"]
pub struct GROUP3_W<'a> {
    w: &'a mut W,
}
impl<'a> GROUP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROUP3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FB_CS5"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP3_A::_0000)
    }
    #[doc = "FB_TSIZ1"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP3_A::_0001)
    }
    #[doc = "FB_BE_23_16"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP3_A::_0010)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "FlexBus signal group 2 multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GROUP2_A {
    #[doc = "0: FB_CS4"]
    _0000 = 0,
    #[doc = "1: FB_TSIZ0"]
    _0001 = 1,
    #[doc = "2: FB_BE_31_24"]
    _0010 = 2,
}
impl From<GROUP2_A> for u8 {
    #[inline(always)]
    fn from(variant: GROUP2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GROUP2`"]
pub type GROUP2_R = crate::R<u8, GROUP2_A>;
impl GROUP2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GROUP2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GROUP2_A::_0000),
            1 => Val(GROUP2_A::_0001),
            2 => Val(GROUP2_A::_0010),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == GROUP2_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == GROUP2_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == GROUP2_A::_0010
    }
}
#[doc = "Write proxy for field `GROUP2`"]
pub struct GROUP2_W<'a> {
    w: &'a mut W,
}
impl<'a> GROUP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROUP2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FB_CS4"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP2_A::_0000)
    }
    #[doc = "FB_TSIZ0"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP2_A::_0001)
    }
    #[doc = "FB_BE_31_24"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP2_A::_0010)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "FlexBus signal group 1 multiplex control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GROUP1_A {
    #[doc = "0: FB_ALE"]
    _0000 = 0,
    #[doc = "1: FB_CS1"]
    _0001 = 1,
    #[doc = "2: FB_TS"]
    _0010 = 2,
}
impl From<GROUP1_A> for u8 {
    #[inline(always)]
    fn from(variant: GROUP1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GROUP1`"]
pub type GROUP1_R = crate::R<u8, GROUP1_A>;
impl GROUP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GROUP1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GROUP1_A::_0000),
            1 => Val(GROUP1_A::_0001),
            2 => Val(GROUP1_A::_0010),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == GROUP1_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == GROUP1_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == GROUP1_A::_0010
    }
}
#[doc = "Write proxy for field `GROUP1`"]
pub struct GROUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> GROUP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROUP1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FB_ALE"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP1_A::_0000)
    }
    #[doc = "FB_CS1"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP1_A::_0001)
    }
    #[doc = "FB_TS"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP1_A::_0010)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - FlexBus signal group 5 multiplex control"]
    #[inline(always)]
    pub fn group5(&self) -> GROUP5_R {
        GROUP5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - FlexBus signal group 4 multiplex control"]
    #[inline(always)]
    pub fn group4(&self) -> GROUP4_R {
        GROUP4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - FlexBus signal group 3 multiplex control"]
    #[inline(always)]
    pub fn group3(&self) -> GROUP3_R {
        GROUP3_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - FlexBus signal group 2 multiplex control"]
    #[inline(always)]
    pub fn group2(&self) -> GROUP2_R {
        GROUP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - FlexBus signal group 1 multiplex control"]
    #[inline(always)]
    pub fn group1(&self) -> GROUP1_R {
        GROUP1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - FlexBus signal group 5 multiplex control"]
    #[inline(always)]
    pub fn group5(&mut self) -> GROUP5_W {
        GROUP5_W { w: self }
    }
    #[doc = "Bits 16:19 - FlexBus signal group 4 multiplex control"]
    #[inline(always)]
    pub fn group4(&mut self) -> GROUP4_W {
        GROUP4_W { w: self }
    }
    #[doc = "Bits 20:23 - FlexBus signal group 3 multiplex control"]
    #[inline(always)]
    pub fn group3(&mut self) -> GROUP3_W {
        GROUP3_W { w: self }
    }
    #[doc = "Bits 24:27 - FlexBus signal group 2 multiplex control"]
    #[inline(always)]
    pub fn group2(&mut self) -> GROUP2_W {
        GROUP2_W { w: self }
    }
    #[doc = "Bits 28:31 - FlexBus signal group 1 multiplex control"]
    #[inline(always)]
    pub fn group1(&mut self) -> GROUP1_W {
        GROUP1_W { w: self }
    }
}
