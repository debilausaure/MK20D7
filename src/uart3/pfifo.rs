#[doc = "Reader of register PFIFO"]
pub type R = crate::R<u8, super::PFIFO>;
#[doc = "Writer for register PFIFO"]
pub type W = crate::W<u8, super::PFIFO>;
#[doc = "Register PFIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::PFIFO {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receive FIFO. Buffer Depth\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXFIFOSIZE_A {
    #[doc = "0: Receive FIFO/Buffer Depth = 1 Dataword."]
    _000 = 0,
    #[doc = "1: Receive FIFO/Buffer Depth = 4 Datawords."]
    _001 = 1,
    #[doc = "2: Receive FIFO/Buffer Depth = 8 Datawords."]
    _010 = 2,
    #[doc = "3: Receive FIFO/Buffer Depth = 16 Datawords."]
    _011 = 3,
    #[doc = "4: Receive FIFO/Buffer Depth = 32 Datawords."]
    _100 = 4,
    #[doc = "5: Receive FIFO/Buffer Depth = 64 Datawords."]
    _101 = 5,
    #[doc = "6: Receive FIFO/Buffer Depth = 128 Datawords."]
    _110 = 6,
}
impl From<RXFIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RXFIFOSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXFIFOSIZE`"]
pub type RXFIFOSIZE_R = crate::R<u8, RXFIFOSIZE_A>;
impl RXFIFOSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXFIFOSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXFIFOSIZE_A::_000),
            1 => Val(RXFIFOSIZE_A::_001),
            2 => Val(RXFIFOSIZE_A::_010),
            3 => Val(RXFIFOSIZE_A::_011),
            4 => Val(RXFIFOSIZE_A::_100),
            5 => Val(RXFIFOSIZE_A::_101),
            6 => Val(RXFIFOSIZE_A::_110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RXFIFOSIZE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RXFIFOSIZE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RXFIFOSIZE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RXFIFOSIZE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RXFIFOSIZE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RXFIFOSIZE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RXFIFOSIZE_A::_110
    }
}
#[doc = "Receive FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFE_A {
    #[doc = "0: Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    _0 = 0,
    #[doc = "1: Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    _1 = 1,
}
impl From<RXFE_A> for bool {
    #[inline(always)]
    fn from(variant: RXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFE`"]
pub type RXFE_R = crate::R<bool, RXFE_A>;
impl RXFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFE_A {
        match self.bits {
            false => RXFE_A::_0,
            true => RXFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXFE_A::_1
    }
}
#[doc = "Write proxy for field `RXFE`"]
pub struct RXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFE_A::_0)
    }
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Transmit FIFO. Buffer Depth\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXFIFOSIZE_A {
    #[doc = "0: Transmit FIFO/Buffer Depth = 1 Dataword."]
    _000 = 0,
    #[doc = "1: Transmit FIFO/Buffer Depth = 4 Datawords."]
    _001 = 1,
    #[doc = "2: Transmit FIFO/Buffer Depth = 8 Datawords."]
    _010 = 2,
    #[doc = "3: Transmit FIFO/Buffer Depth = 16 Datawords."]
    _011 = 3,
    #[doc = "4: Transmit FIFO/Buffer Depth = 32 Datawords."]
    _100 = 4,
    #[doc = "5: Transmit FIFO/Buffer Depth = 64 Datawords."]
    _101 = 5,
    #[doc = "6: Transmit FIFO/Buffer Depth = 128 Datawords."]
    _110 = 6,
}
impl From<TXFIFOSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: TXFIFOSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXFIFOSIZE`"]
pub type TXFIFOSIZE_R = crate::R<u8, TXFIFOSIZE_A>;
impl TXFIFOSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXFIFOSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXFIFOSIZE_A::_000),
            1 => Val(TXFIFOSIZE_A::_001),
            2 => Val(TXFIFOSIZE_A::_010),
            3 => Val(TXFIFOSIZE_A::_011),
            4 => Val(TXFIFOSIZE_A::_100),
            5 => Val(TXFIFOSIZE_A::_101),
            6 => Val(TXFIFOSIZE_A::_110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == TXFIFOSIZE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == TXFIFOSIZE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == TXFIFOSIZE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == TXFIFOSIZE_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == TXFIFOSIZE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == TXFIFOSIZE_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == TXFIFOSIZE_A::_110
    }
}
#[doc = "Transmit FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFE_A {
    #[doc = "0: Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    _0 = 0,
    #[doc = "1: Transmit FIFO is enabled. Buffer is depth indicted by TXFIFOSIZE."]
    _1 = 1,
}
impl From<TXFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFE`"]
pub type TXFE_R = crate::R<bool, TXFE_A>;
impl TXFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFE_A {
        match self.bits {
            false => TXFE_A::_0,
            true => TXFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXFE_A::_1
    }
}
#[doc = "Write proxy for field `TXFE`"]
pub struct TXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFE_A::_0)
    }
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicted by TXFIFOSIZE."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Receive FIFO. Buffer Depth"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Transmit FIFO. Buffer Depth"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline(always)]
    pub fn rxfe(&mut self) -> RXFE_W {
        RXFE_W { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline(always)]
    pub fn txfe(&mut self) -> TXFE_W {
        TXFE_W { w: self }
    }
}
