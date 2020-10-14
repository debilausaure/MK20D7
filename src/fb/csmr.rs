#[doc = "Reader of register CSMR%s"]
pub type R = crate::R<u32, super::CSMR>;
#[doc = "Writer for register CSMR%s"]
pub type W = crate::W<u32, super::CSMR>;
#[doc = "Register CSMR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CSMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V_A {
    #[doc = "0: Chip select invalid"]
    _0 = 0,
    #[doc = "1: Chip select valid"]
    _1 = 1,
}
impl From<V_A> for bool {
    #[inline(always)]
    fn from(variant: V_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `V`"]
pub type V_R = crate::R<bool, V_A>;
impl V_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> V_A {
        match self.bits {
            false => V_A::_0,
            true => V_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == V_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == V_A::_1
    }
}
#[doc = "Write proxy for field `V`"]
pub struct V_W<'a> {
    w: &'a mut W,
}
impl<'a> V_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: V_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Chip select invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(V_A::_0)
    }
    #[doc = "Chip select valid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(V_A::_1)
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
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_A {
    #[doc = "0: Read and write accesses are allowed"]
    _0 = 0,
    #[doc = "1: Only read accesses are allowed"]
    _1 = 1,
}
impl From<WP_A> for bool {
    #[inline(always)]
    fn from(variant: WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WP`"]
pub type WP_R = crate::R<bool, WP_A>;
impl WP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_A {
        match self.bits {
            false => WP_A::_0,
            true => WP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP_A::_1
    }
}
#[doc = "Write proxy for field `WP`"]
pub struct WP_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read and write accesses are allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP_A::_0)
    }
    #[doc = "Only read accesses are allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP_A::_1)
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
#[doc = "Base address mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BAM_A {
    #[doc = "0: Corresponding address bit is used in chip-select decode"]
    _0 = 0,
    #[doc = "1: Corresponding address bit is a don't care in chip-select decode."]
    _1 = 1,
}
impl From<BAM_A> for u16 {
    #[inline(always)]
    fn from(variant: BAM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BAM`"]
pub type BAM_R = crate::R<u16, BAM_A>;
impl BAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, BAM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BAM_A::_0),
            1 => Val(BAM_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BAM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BAM_A::_1
    }
}
#[doc = "Write proxy for field `BAM`"]
pub struct BAM_W<'a> {
    w: &'a mut W,
}
impl<'a> BAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BAM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Corresponding address bit is used in chip-select decode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BAM_A::_0)
    }
    #[doc = "Corresponding address bit is a don't care in chip-select decode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BAM_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write protect"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Base address mask"]
    #[inline(always)]
    pub fn bam(&self) -> BAM_R {
        BAM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn v(&mut self) -> V_W {
        V_W { w: self }
    }
    #[doc = "Bit 8 - Write protect"]
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W {
        WP_W { w: self }
    }
    #[doc = "Bits 16:31 - Base address mask"]
    #[inline(always)]
    pub fn bam(&mut self) -> BAM_W {
        BAM_W { w: self }
    }
}
