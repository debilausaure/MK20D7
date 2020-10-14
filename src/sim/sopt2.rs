#[doc = "Reader of register SOPT2"]
pub type R = crate::R<u32, super::SOPT2>;
#[doc = "Writer for register SOPT2"]
pub type W = crate::W<u32, super::SOPT2>;
#[doc = "Register SOPT2 `reset()`'s with value 0x1000"]
impl crate::ResetValue for super::SOPT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000
    }
}
#[doc = "RTC clock out select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCLKOUTSEL_A {
    #[doc = "0: RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0 = 0,
    #[doc = "1: RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    _1 = 1,
}
impl From<RTCCLKOUTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCLKOUTSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCCLKOUTSEL`"]
pub type RTCCLKOUTSEL_R = crate::R<bool, RTCCLKOUTSEL_A>;
impl RTCCLKOUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCLKOUTSEL_A {
        match self.bits {
            false => RTCCLKOUTSEL_A::_0,
            true => RTCCLKOUTSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCCLKOUTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCCLKOUTSEL_A::_1
    }
}
#[doc = "Write proxy for field `RTCCLKOUTSEL`"]
pub struct RTCCLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCLKOUTSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCLKOUTSEL_A::_0)
    }
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCLKOUTSEL_A::_1)
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
#[doc = "CLKOUT select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "0: FlexBus CLKOUT"]
    _000 = 0,
    #[doc = "2: Flash clock"]
    _010 = 2,
    #[doc = "3: LPO clock (1 kHz)"]
    _011 = 3,
    #[doc = "4: IRCLK"]
    _100 = 4,
    #[doc = "5: RTC 32.768kHz"]
    _101 = 5,
    #[doc = "6: ERCLK0"]
    _110 = 6,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKOUTSEL`"]
pub type CLKOUTSEL_R = crate::R<u8, CLKOUTSEL_A>;
impl CLKOUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKOUTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKOUTSEL_A::_000),
            2 => Val(CLKOUTSEL_A::_010),
            3 => Val(CLKOUTSEL_A::_011),
            4 => Val(CLKOUTSEL_A::_100),
            5 => Val(CLKOUTSEL_A::_101),
            6 => Val(CLKOUTSEL_A::_110),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CLKOUTSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CLKOUTSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTSEL_A::_110
    }
}
#[doc = "Write proxy for field `CLKOUTSEL`"]
pub struct CLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FlexBus CLKOUT"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_000)
    }
    #[doc = "Flash clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_011)
    }
    #[doc = "IRCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_100)
    }
    #[doc = "RTC 32.768kHz"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_101)
    }
    #[doc = "ERCLK0"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_110)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "FlexBus security level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FBSL_A {
    #[doc = "0: All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    _00 = 0,
    #[doc = "1: All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    _01 = 1,
    #[doc = "2: Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    _10 = 2,
    #[doc = "3: Off-chip instruction accesses and data accesses are allowed."]
    _11 = 3,
}
impl From<FBSL_A> for u8 {
    #[inline(always)]
    fn from(variant: FBSL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FBSL`"]
pub type FBSL_R = crate::R<u8, FBSL_A>;
impl FBSL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBSL_A {
        match self.bits {
            0 => FBSL_A::_00,
            1 => FBSL_A::_01,
            2 => FBSL_A::_10,
            3 => FBSL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FBSL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FBSL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FBSL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FBSL_A::_11
    }
}
#[doc = "Write proxy for field `FBSL`"]
pub struct FBSL_W<'a> {
    w: &'a mut W,
}
impl<'a> FBSL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FBSL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FBSL_A::_00)
    }
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FBSL_A::_01)
    }
    #[doc = "Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FBSL_A::_10)
    }
    #[doc = "Off-chip instruction accesses and data accesses are allowed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FBSL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "PTD7 pad drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTD7PAD_A {
    #[doc = "0: Single-pad drive strength for PTD7."]
    _0 = 0,
    #[doc = "1: Double pad drive strength for PTD7."]
    _1 = 1,
}
impl From<PTD7PAD_A> for bool {
    #[inline(always)]
    fn from(variant: PTD7PAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PTD7PAD`"]
pub type PTD7PAD_R = crate::R<bool, PTD7PAD_A>;
impl PTD7PAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTD7PAD_A {
        match self.bits {
            false => PTD7PAD_A::_0,
            true => PTD7PAD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTD7PAD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTD7PAD_A::_1
    }
}
#[doc = "Write proxy for field `PTD7PAD`"]
pub struct PTD7PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PTD7PAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTD7PAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single-pad drive strength for PTD7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTD7PAD_A::_0)
    }
    #[doc = "Double pad drive strength for PTD7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTD7PAD_A::_1)
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
#[doc = "Debug trace clock select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECLKSEL_A {
    #[doc = "0: MCGOUTCLK"]
    _0 = 0,
    #[doc = "1: Core/system clock"]
    _1 = 1,
}
impl From<TRACECLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TRACECLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRACECLKSEL`"]
pub type TRACECLKSEL_R = crate::R<bool, TRACECLKSEL_A>;
impl TRACECLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACECLKSEL_A {
        match self.bits {
            false => TRACECLKSEL_A::_0,
            true => TRACECLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRACECLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRACECLKSEL_A::_1
    }
}
#[doc = "Write proxy for field `TRACECLKSEL`"]
pub struct TRACECLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACECLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGOUTCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACECLKSEL_A::_0)
    }
    #[doc = "Core/system clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACECLKSEL_A::_1)
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
#[doc = "PLL/FLL clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLFLLSEL_A {
    #[doc = "0: MCGFLLCLK clock"]
    _0 = 0,
    #[doc = "1: MCGPLLCLK clock"]
    _1 = 1,
}
impl From<PLLFLLSEL_A> for bool {
    #[inline(always)]
    fn from(variant: PLLFLLSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLFLLSEL`"]
pub type PLLFLLSEL_R = crate::R<bool, PLLFLLSEL_A>;
impl PLLFLLSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLFLLSEL_A {
        match self.bits {
            false => PLLFLLSEL_A::_0,
            true => PLLFLLSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PLLFLLSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PLLFLLSEL_A::_1
    }
}
#[doc = "Write proxy for field `PLLFLLSEL`"]
pub struct PLLFLLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLFLLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLFLLSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCGFLLCLK clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_0)
    }
    #[doc = "MCGPLLCLK clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "USB clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSRC_A {
    #[doc = "0: External bypass clock (USB_CLKIN)."]
    _0 = 0,
    #[doc = "1: MCGPLLCLK/MCGFLLCLK clock divided by the USB fractional divider. See the SIM_CLKDIV2\\[USBFRAC, USBDIV\\]
descriptions."]
    _1 = 1,
}
impl From<USBSRC_A> for bool {
    #[inline(always)]
    fn from(variant: USBSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBSRC`"]
pub type USBSRC_R = crate::R<bool, USBSRC_A>;
impl USBSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSRC_A {
        match self.bits {
            false => USBSRC_A::_0,
            true => USBSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBSRC_A::_1
    }
}
#[doc = "Write proxy for field `USBSRC`"]
pub struct USBSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSRC_A::_0)
    }
    #[doc = "MCGPLLCLK/MCGFLLCLK clock divided by the USB fractional divider. See the SIM_CLKDIV2\\[USBFRAC, USBDIV\\]
descriptions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkoutsel(&self) -> RTCCLKOUTSEL_R {
        RTCCLKOUTSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - FlexBus security level"]
    #[inline(always)]
    pub fn fbsl(&self) -> FBSL_R {
        FBSL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - PTD7 pad drive strength"]
    #[inline(always)]
    pub fn ptd7pad(&self) -> PTD7PAD_R {
        PTD7PAD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    pub fn traceclksel(&self) -> TRACECLKSEL_R {
        TRACECLKSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PLL/FLL clock select"]
    #[inline(always)]
    pub fn pllfllsel(&self) -> PLLFLLSEL_R {
        PLLFLLSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    pub fn usbsrc(&self) -> USBSRC_R {
        USBSRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkoutsel(&mut self) -> RTCCLKOUTSEL_W {
        RTCCLKOUTSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
        CLKOUTSEL_W { w: self }
    }
    #[doc = "Bits 8:9 - FlexBus security level"]
    #[inline(always)]
    pub fn fbsl(&mut self) -> FBSL_W {
        FBSL_W { w: self }
    }
    #[doc = "Bit 11 - PTD7 pad drive strength"]
    #[inline(always)]
    pub fn ptd7pad(&mut self) -> PTD7PAD_W {
        PTD7PAD_W { w: self }
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    pub fn traceclksel(&mut self) -> TRACECLKSEL_W {
        TRACECLKSEL_W { w: self }
    }
    #[doc = "Bit 16 - PLL/FLL clock select"]
    #[inline(always)]
    pub fn pllfllsel(&mut self) -> PLLFLLSEL_W {
        PLLFLLSEL_W { w: self }
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    pub fn usbsrc(&mut self) -> USBSRC_W {
        USBSRC_W { w: self }
    }
}
