#[doc = "Reader of register SCANC"]
pub type R = crate::R<u32, super::SCANC>;
#[doc = "Writer for register SCANC"]
pub type W = crate::W<u32, super::SCANC>;
#[doc = "Register SCANC `reset()`'s with value 0"]
impl crate::ResetValue for super::SCANC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Active Mode Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AMPSC_A {
    #[doc = "0: Input Clock Source divided by 1."]
    _000 = 0,
    #[doc = "1: Input Clock Source divided by 2."]
    _001 = 1,
    #[doc = "2: Input Clock Source divided by 4."]
    _010 = 2,
    #[doc = "3: Input Clock Source divided by 8."]
    _011 = 3,
    #[doc = "4: Input Clock Source divided by 16."]
    _100 = 4,
    #[doc = "5: Input Clock Source divided by 32."]
    _101 = 5,
    #[doc = "6: Input Clock Source divided by 64."]
    _110 = 6,
    #[doc = "7: Input Clock Source divided by 128."]
    _111 = 7,
}
impl From<AMPSC_A> for u8 {
    #[inline(always)]
    fn from(variant: AMPSC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AMPSC`"]
pub type AMPSC_R = crate::R<u8, AMPSC_A>;
impl AMPSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPSC_A {
        match self.bits {
            0 => AMPSC_A::_000,
            1 => AMPSC_A::_001,
            2 => AMPSC_A::_010,
            3 => AMPSC_A::_011,
            4 => AMPSC_A::_100,
            5 => AMPSC_A::_101,
            6 => AMPSC_A::_110,
            7 => AMPSC_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == AMPSC_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == AMPSC_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == AMPSC_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == AMPSC_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == AMPSC_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == AMPSC_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == AMPSC_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == AMPSC_A::_111
    }
}
#[doc = "Write proxy for field `AMPSC`"]
pub struct AMPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMPSC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input Clock Source divided by 1."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(AMPSC_A::_000)
    }
    #[doc = "Input Clock Source divided by 2."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(AMPSC_A::_001)
    }
    #[doc = "Input Clock Source divided by 4."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(AMPSC_A::_010)
    }
    #[doc = "Input Clock Source divided by 8."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(AMPSC_A::_011)
    }
    #[doc = "Input Clock Source divided by 16."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(AMPSC_A::_100)
    }
    #[doc = "Input Clock Source divided by 32."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(AMPSC_A::_101)
    }
    #[doc = "Input Clock Source divided by 64."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(AMPSC_A::_110)
    }
    #[doc = "Input Clock Source divided by 128."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(AMPSC_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Active Mode Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AMCLKS_A {
    #[doc = "0: LPOSCCLK"]
    _00 = 0,
    #[doc = "1: MCGIRCLK."]
    _01 = 1,
    #[doc = "2: OSCERCLK."]
    _10 = 2,
    #[doc = "3: Not valid."]
    _11 = 3,
}
impl From<AMCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: AMCLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AMCLKS`"]
pub type AMCLKS_R = crate::R<u8, AMCLKS_A>;
impl AMCLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMCLKS_A {
        match self.bits {
            0 => AMCLKS_A::_00,
            1 => AMCLKS_A::_01,
            2 => AMCLKS_A::_10,
            3 => AMCLKS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == AMCLKS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == AMCLKS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == AMCLKS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == AMCLKS_A::_11
    }
}
#[doc = "Write proxy for field `AMCLKS`"]
pub struct AMCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> AMCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMCLKS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LPOSCCLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(AMCLKS_A::_00)
    }
    #[doc = "MCGIRCLK."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(AMCLKS_A::_01)
    }
    #[doc = "OSCERCLK."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(AMCLKS_A::_10)
    }
    #[doc = "Not valid."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(AMCLKS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Scan Module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Continue Scan."]
    _00000000 = 0,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SMOD`"]
pub type SMOD_R = crate::R<u8, SMOD_A>;
impl SMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SMOD_A::_00000000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000000`"]
    #[inline(always)]
    pub fn is_00000000(&self) -> bool {
        *self == SMOD_A::_00000000
    }
}
#[doc = "Write proxy for field `SMOD`"]
pub struct SMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Continue Scan."]
    #[inline(always)]
    pub fn _00000000(self) -> &'a mut W {
        self.variant(SMOD_A::_00000000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "External OSC Charge Current select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTCHRG_A {
    #[doc = "0: 2 uA charge current."]
    _0000 = 0,
    #[doc = "1: 4 uA charge current."]
    _0001 = 1,
    #[doc = "2: 6 uA charge current."]
    _0010 = 2,
    #[doc = "3: 8 uA charge current."]
    _0011 = 3,
    #[doc = "4: 10 uA charge current."]
    _0100 = 4,
    #[doc = "5: 12 uA charge current."]
    _0101 = 5,
    #[doc = "6: 14 uA charge current."]
    _0110 = 6,
    #[doc = "7: 16 uA charge current."]
    _0111 = 7,
    #[doc = "8: 18 uA charge current."]
    _1000 = 8,
    #[doc = "9: 20 uA charge current."]
    _1001 = 9,
    #[doc = "10: 22 uA charge current."]
    _1010 = 10,
    #[doc = "11: 24 uA charge current."]
    _1011 = 11,
    #[doc = "12: 26 uA charge current."]
    _1100 = 12,
    #[doc = "13: 28 uA charge current."]
    _1101 = 13,
    #[doc = "14: 30 uA charge current."]
    _1110 = 14,
    #[doc = "15: 32 uA charge current."]
    _1111 = 15,
}
impl From<EXTCHRG_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTCHRG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTCHRG`"]
pub type EXTCHRG_R = crate::R<u8, EXTCHRG_A>;
impl EXTCHRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTCHRG_A {
        match self.bits {
            0 => EXTCHRG_A::_0000,
            1 => EXTCHRG_A::_0001,
            2 => EXTCHRG_A::_0010,
            3 => EXTCHRG_A::_0011,
            4 => EXTCHRG_A::_0100,
            5 => EXTCHRG_A::_0101,
            6 => EXTCHRG_A::_0110,
            7 => EXTCHRG_A::_0111,
            8 => EXTCHRG_A::_1000,
            9 => EXTCHRG_A::_1001,
            10 => EXTCHRG_A::_1010,
            11 => EXTCHRG_A::_1011,
            12 => EXTCHRG_A::_1100,
            13 => EXTCHRG_A::_1101,
            14 => EXTCHRG_A::_1110,
            15 => EXTCHRG_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == EXTCHRG_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == EXTCHRG_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == EXTCHRG_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == EXTCHRG_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == EXTCHRG_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == EXTCHRG_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == EXTCHRG_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == EXTCHRG_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == EXTCHRG_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == EXTCHRG_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == EXTCHRG_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == EXTCHRG_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == EXTCHRG_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == EXTCHRG_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == EXTCHRG_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == EXTCHRG_A::_1111
    }
}
#[doc = "Write proxy for field `EXTCHRG`"]
pub struct EXTCHRG_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTCHRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTCHRG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "2 uA charge current."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_0000)
    }
    #[doc = "4 uA charge current."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_0001)
    }
    #[doc = "6 uA charge current."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_0010)
    }
    #[doc = "8 uA charge current."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_0011)
    }
    #[doc = "10 uA charge current."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_0100)
    }
    #[doc = "12 uA charge current."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_0101)
    }
    #[doc = "14 uA charge current."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_0110)
    }
    #[doc = "16 uA charge current."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_0111)
    }
    #[doc = "18 uA charge current."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_1000)
    }
    #[doc = "20 uA charge current."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_1001)
    }
    #[doc = "22 uA charge current."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_1010)
    }
    #[doc = "24 uA charge current."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_1011)
    }
    #[doc = "26 uA charge current."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_1100)
    }
    #[doc = "28 uA charge current."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_1101)
    }
    #[doc = "30 uA charge current."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_1110)
    }
    #[doc = "32 uA charge current."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(EXTCHRG_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Ref OSC Charge Current select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFCHRG_A {
    #[doc = "0: 2 uA charge current."]
    _0000 = 0,
    #[doc = "1: 4 uA charge current."]
    _0001 = 1,
    #[doc = "2: 6 uA charge current."]
    _0010 = 2,
    #[doc = "3: 8 uA charge current."]
    _0011 = 3,
    #[doc = "4: 10 uA charge current."]
    _0100 = 4,
    #[doc = "5: 12 uA charge current."]
    _0101 = 5,
    #[doc = "6: 14 uA charge current."]
    _0110 = 6,
    #[doc = "7: 16 uA charge current."]
    _0111 = 7,
    #[doc = "8: 18 uA charge current."]
    _1000 = 8,
    #[doc = "9: 20 uA charge current."]
    _1001 = 9,
    #[doc = "10: 22 uA charge current."]
    _1010 = 10,
    #[doc = "11: 24 uA charge current."]
    _1011 = 11,
    #[doc = "12: 26 uA charge current."]
    _1100 = 12,
    #[doc = "13: 28 uA charge current."]
    _1101 = 13,
    #[doc = "14: 30 uA charge current."]
    _1110 = 14,
    #[doc = "15: 32 uA charge current."]
    _1111 = 15,
}
impl From<REFCHRG_A> for u8 {
    #[inline(always)]
    fn from(variant: REFCHRG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFCHRG`"]
pub type REFCHRG_R = crate::R<u8, REFCHRG_A>;
impl REFCHRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFCHRG_A {
        match self.bits {
            0 => REFCHRG_A::_0000,
            1 => REFCHRG_A::_0001,
            2 => REFCHRG_A::_0010,
            3 => REFCHRG_A::_0011,
            4 => REFCHRG_A::_0100,
            5 => REFCHRG_A::_0101,
            6 => REFCHRG_A::_0110,
            7 => REFCHRG_A::_0111,
            8 => REFCHRG_A::_1000,
            9 => REFCHRG_A::_1001,
            10 => REFCHRG_A::_1010,
            11 => REFCHRG_A::_1011,
            12 => REFCHRG_A::_1100,
            13 => REFCHRG_A::_1101,
            14 => REFCHRG_A::_1110,
            15 => REFCHRG_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == REFCHRG_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == REFCHRG_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == REFCHRG_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == REFCHRG_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == REFCHRG_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == REFCHRG_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == REFCHRG_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == REFCHRG_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == REFCHRG_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == REFCHRG_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == REFCHRG_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == REFCHRG_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == REFCHRG_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == REFCHRG_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == REFCHRG_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == REFCHRG_A::_1111
    }
}
#[doc = "Write proxy for field `REFCHRG`"]
pub struct REFCHRG_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCHRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFCHRG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "2 uA charge current."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(REFCHRG_A::_0000)
    }
    #[doc = "4 uA charge current."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(REFCHRG_A::_0001)
    }
    #[doc = "6 uA charge current."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(REFCHRG_A::_0010)
    }
    #[doc = "8 uA charge current."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(REFCHRG_A::_0011)
    }
    #[doc = "10 uA charge current."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(REFCHRG_A::_0100)
    }
    #[doc = "12 uA charge current."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(REFCHRG_A::_0101)
    }
    #[doc = "14 uA charge current."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(REFCHRG_A::_0110)
    }
    #[doc = "16 uA charge current."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(REFCHRG_A::_0111)
    }
    #[doc = "18 uA charge current."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(REFCHRG_A::_1000)
    }
    #[doc = "20 uA charge current."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(REFCHRG_A::_1001)
    }
    #[doc = "22 uA charge current."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(REFCHRG_A::_1010)
    }
    #[doc = "24 uA charge current."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(REFCHRG_A::_1011)
    }
    #[doc = "26 uA charge current."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(REFCHRG_A::_1100)
    }
    #[doc = "28 uA charge current."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(REFCHRG_A::_1101)
    }
    #[doc = "30 uA charge current."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(REFCHRG_A::_1110)
    }
    #[doc = "32 uA charge current."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(REFCHRG_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Active Mode Prescaler"]
    #[inline(always)]
    pub fn ampsc(&self) -> AMPSC_R {
        AMPSC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Active Mode Clock Source"]
    #[inline(always)]
    pub fn amclks(&self) -> AMCLKS_R {
        AMCLKS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Scan Module"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - External OSC Charge Current select"]
    #[inline(always)]
    pub fn extchrg(&self) -> EXTCHRG_R {
        EXTCHRG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Ref OSC Charge Current select"]
    #[inline(always)]
    pub fn refchrg(&self) -> REFCHRG_R {
        REFCHRG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Active Mode Prescaler"]
    #[inline(always)]
    pub fn ampsc(&mut self) -> AMPSC_W {
        AMPSC_W { w: self }
    }
    #[doc = "Bits 3:4 - Active Mode Clock Source"]
    #[inline(always)]
    pub fn amclks(&mut self) -> AMCLKS_W {
        AMCLKS_W { w: self }
    }
    #[doc = "Bits 8:15 - Scan Module"]
    #[inline(always)]
    pub fn smod(&mut self) -> SMOD_W {
        SMOD_W { w: self }
    }
    #[doc = "Bits 16:19 - External OSC Charge Current select"]
    #[inline(always)]
    pub fn extchrg(&mut self) -> EXTCHRG_W {
        EXTCHRG_W { w: self }
    }
    #[doc = "Bits 24:27 - Ref OSC Charge Current select"]
    #[inline(always)]
    pub fn refchrg(&mut self) -> REFCHRG_W {
        REFCHRG_W { w: self }
    }
}
