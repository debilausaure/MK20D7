#[doc = "Reader of register SDID"]
pub type R = crate::R<u32, super::SDID>;
#[doc = "Pincount identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PINID_A {
    #[doc = "5: 64-pin"]
    _0101 = 5,
    #[doc = "6: 80-pin"]
    _0110 = 6,
    #[doc = "7: 81-pin"]
    _0111 = 7,
    #[doc = "8: 100-pin"]
    _1000 = 8,
}
impl From<PINID_A> for u8 {
    #[inline(always)]
    fn from(variant: PINID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PINID`"]
pub type PINID_R = crate::R<u8, PINID_A>;
impl PINID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PINID_A> {
        use crate::Variant::*;
        match self.bits {
            5 => Val(PINID_A::_0101),
            6 => Val(PINID_A::_0110),
            7 => Val(PINID_A::_0111),
            8 => Val(PINID_A::_1000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PINID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PINID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PINID_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PINID_A::_1000
    }
}
#[doc = "Kinetis family identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FAMID_A {
    #[doc = "0: K10"]
    _000 = 0,
    #[doc = "1: K20"]
    _001 = 1,
    #[doc = "2: K30"]
    _010 = 2,
    #[doc = "3: K40"]
    _011 = 3,
    #[doc = "6: K50"]
    _110 = 6,
    #[doc = "7: K51"]
    _111 = 7,
}
impl From<FAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FAMID`"]
pub type FAMID_R = crate::R<u8, FAMID_A>;
impl FAMID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FAMID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FAMID_A::_000),
            1 => Val(FAMID_A::_001),
            2 => Val(FAMID_A::_010),
            3 => Val(FAMID_A::_011),
            6 => Val(FAMID_A::_110),
            7 => Val(FAMID_A::_111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FAMID_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FAMID_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FAMID_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FAMID_A::_011
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FAMID_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FAMID_A::_111
    }
}
#[doc = "Reader of field `REVID`"]
pub type REVID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline(always)]
    pub fn pinid(&self) -> PINID_R {
        PINID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Kinetis family identification"]
    #[inline(always)]
    pub fn famid(&self) -> FAMID_R {
        FAMID_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
