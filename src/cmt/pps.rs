#[doc = "Reader of register PPS"]
pub type R = crate::R<u8, super::PPS>;
#[doc = "Writer for register PPS"]
pub type W = crate::W<u8, super::PPS>;
#[doc = "Register PPS `reset()`'s with value 0"]
impl crate::ResetValue for super::PPS {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Primary Prescaler Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PPSDIV_A {
    #[doc = "0: Bus Clock * 1"]
    _0000 = 0,
    #[doc = "1: Bus Clock * 2"]
    _0001 = 1,
    #[doc = "2: Bus Clock * 3"]
    _0010 = 2,
    #[doc = "3: Bus Clock * 4"]
    _0011 = 3,
    #[doc = "4: Bus Clock * 5"]
    _0100 = 4,
    #[doc = "5: Bus Clock * 6"]
    _0101 = 5,
    #[doc = "6: Bus Clock * 7"]
    _0110 = 6,
    #[doc = "7: Bus Clock * 8"]
    _0111 = 7,
    #[doc = "8: Bus Clock * 9"]
    _1000 = 8,
    #[doc = "9: Bus Clock * 10"]
    _1001 = 9,
    #[doc = "10: Bus Clock * 11"]
    _1010 = 10,
    #[doc = "11: Bus Clock * 12"]
    _1011 = 11,
    #[doc = "12: Bus Clock * 13"]
    _1100 = 12,
    #[doc = "13: Bus Clock * 14"]
    _1101 = 13,
    #[doc = "14: Bus Clock * 15"]
    _1110 = 14,
    #[doc = "15: Bus Clock * 16"]
    _1111 = 15,
}
impl From<PPSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PPSDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PPSDIV`"]
pub type PPSDIV_R = crate::R<u8, PPSDIV_A>;
impl PPSDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPSDIV_A {
        match self.bits {
            0 => PPSDIV_A::_0000,
            1 => PPSDIV_A::_0001,
            2 => PPSDIV_A::_0010,
            3 => PPSDIV_A::_0011,
            4 => PPSDIV_A::_0100,
            5 => PPSDIV_A::_0101,
            6 => PPSDIV_A::_0110,
            7 => PPSDIV_A::_0111,
            8 => PPSDIV_A::_1000,
            9 => PPSDIV_A::_1001,
            10 => PPSDIV_A::_1010,
            11 => PPSDIV_A::_1011,
            12 => PPSDIV_A::_1100,
            13 => PPSDIV_A::_1101,
            14 => PPSDIV_A::_1110,
            15 => PPSDIV_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == PPSDIV_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == PPSDIV_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PPSDIV_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PPSDIV_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PPSDIV_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PPSDIV_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PPSDIV_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PPSDIV_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PPSDIV_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PPSDIV_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == PPSDIV_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == PPSDIV_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == PPSDIV_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == PPSDIV_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == PPSDIV_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == PPSDIV_A::_1111
    }
}
#[doc = "Write proxy for field `PPSDIV`"]
pub struct PPSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPSDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bus Clock * 1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0000)
    }
    #[doc = "Bus Clock * 2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0001)
    }
    #[doc = "Bus Clock * 3"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0010)
    }
    #[doc = "Bus Clock * 4"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0011)
    }
    #[doc = "Bus Clock * 5"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0100)
    }
    #[doc = "Bus Clock * 6"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0101)
    }
    #[doc = "Bus Clock * 7"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0110)
    }
    #[doc = "Bus Clock * 8"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0111)
    }
    #[doc = "Bus Clock * 9"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1000)
    }
    #[doc = "Bus Clock * 10"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1001)
    }
    #[doc = "Bus Clock * 11"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1010)
    }
    #[doc = "Bus Clock * 12"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1011)
    }
    #[doc = "Bus Clock * 13"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1100)
    }
    #[doc = "Bus Clock * 14"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1101)
    }
    #[doc = "Bus Clock * 15"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1110)
    }
    #[doc = "Bus Clock * 16"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Primary Prescaler Divider"]
    #[inline(always)]
    pub fn ppsdiv(&self) -> PPSDIV_R {
        PPSDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Primary Prescaler Divider"]
    #[inline(always)]
    pub fn ppsdiv(&mut self) -> PPSDIV_W {
        PPSDIV_W { w: self }
    }
}
