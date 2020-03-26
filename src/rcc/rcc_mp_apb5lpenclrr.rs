#[doc = "Reader of register RCC_MP_APB5LPENCLRR"]
pub type R = crate::R<u32, super::RCC_MP_APB5LPENCLRR>;
#[doc = "Writer for register RCC_MP_APB5LPENCLRR"]
pub type W = crate::W<u32, super::RCC_MP_APB5LPENCLRR>;
#[doc = "Register RCC_MP_APB5LPENCLRR `reset()`'s with value 0x0011_391d"]
impl crate::ResetValue for super::RCC_MP_APB5LPENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0011_391d
    }
}
#[doc = "SPI6LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI6LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SPI6LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI6LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI6LPEN`"]
pub type SPI6LPEN_R = crate::R<bool, SPI6LPEN_A>;
impl SPI6LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI6LPEN_A {
        match self.bits {
            false => SPI6LPEN_A::B_0X0,
            true => SPI6LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI6LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI6LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI6LPEN`"]
pub struct SPI6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI6LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI6LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI6LPEN_A::B_0X1)
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
#[doc = "I2C4LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C4LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<I2C4LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C4LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C4LPEN`"]
pub type I2C4LPEN_R = crate::R<bool, I2C4LPEN_A>;
impl I2C4LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C4LPEN_A {
        match self.bits {
            false => I2C4LPEN_A::B_0X0,
            true => I2C4LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C4LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C4LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `I2C4LPEN`"]
pub struct I2C4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C4LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C4LPEN_A::B_0X1)
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
#[doc = "I2C6LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C6LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<I2C6LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C6LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C6LPEN`"]
pub type I2C6LPEN_R = crate::R<bool, I2C6LPEN_A>;
impl I2C6LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C6LPEN_A {
        match self.bits {
            false => I2C6LPEN_A::B_0X0,
            true => I2C6LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C6LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C6LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `I2C6LPEN`"]
pub struct I2C6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C6LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C6LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C6LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C6LPEN_A::B_0X1)
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
#[doc = "USART1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<USART1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USART1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USART1LPEN`"]
pub type USART1LPEN_R = crate::R<bool, USART1LPEN_A>;
impl USART1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1LPEN_A {
        match self.bits {
            false => USART1LPEN_A::B_0X0,
            true => USART1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `USART1LPEN`"]
pub struct USART1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART1LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART1LPEN_A::B_0X1)
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
#[doc = "RTCAPBLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCAPBLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<RTCAPBLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCAPBLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCAPBLPEN`"]
pub type RTCAPBLPEN_R = crate::R<bool, RTCAPBLPEN_A>;
impl RTCAPBLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCAPBLPEN_A {
        match self.bits {
            false => RTCAPBLPEN_A::B_0X0,
            true => RTCAPBLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCAPBLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCAPBLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `RTCAPBLPEN`"]
pub struct RTCAPBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCAPBLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RTCAPBLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RTCAPBLPEN_A::B_0X1)
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
#[doc = "TZC1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZC1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the clocks are disabled for AXI port 1 in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the AXI port 1\r\n                  peripheral clocks in CSLEEP, reading means that\r\n                  the clocks are enabled for AXI port 1 in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<TZC1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TZC1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TZC1LPEN`"]
pub type TZC1LPEN_R = crate::R<bool, TZC1LPEN_A>;
impl TZC1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZC1LPEN_A {
        match self.bits {
            false => TZC1LPEN_A::B_0X0,
            true => TZC1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TZC1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TZC1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TZC1LPEN`"]
pub struct TZC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TZC1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the clocks are disabled for AXI port 1 in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TZC1LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the AXI port 1 peripheral clocks in CSLEEP, reading means that the clocks are enabled for AXI port 1 in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TZC1LPEN_A::B_0X1)
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
#[doc = "TZC2LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZC2LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the clocks are disabled for AXI port 2 in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the AXI port 2\r\n                  peripheral clocks in CSLEEP, reading means that\r\n                  the clocks are enabled for AXI port 2 in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<TZC2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TZC2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TZC2LPEN`"]
pub type TZC2LPEN_R = crate::R<bool, TZC2LPEN_A>;
impl TZC2LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZC2LPEN_A {
        match self.bits {
            false => TZC2LPEN_A::B_0X0,
            true => TZC2LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TZC2LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TZC2LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TZC2LPEN`"]
pub struct TZC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TZC2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the clocks are disabled for AXI port 2 in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TZC2LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the AXI port 2 peripheral clocks in CSLEEP, reading means that the clocks are enabled for AXI port 2 in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TZC2LPEN_A::B_0X1)
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
#[doc = "TZPCLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZPCLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<TZPCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TZPCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TZPCLPEN`"]
pub type TZPCLPEN_R = crate::R<bool, TZPCLPEN_A>;
impl TZPCLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZPCLPEN_A {
        match self.bits {
            false => TZPCLPEN_A::B_0X0,
            true => TZPCLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TZPCLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TZPCLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TZPCLPEN`"]
pub struct TZPCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZPCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TZPCLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TZPCLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TZPCLPEN_A::B_0X1)
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
#[doc = "IWDG1APBLPEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDG1APBLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the APB clock is disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the APB clock in\r\n                  CSLEEP, reading means that the APB clock is\r\n                  enabled in CSLEEP"]
    B_0X1 = 1,
}
impl From<IWDG1APBLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG1APBLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IWDG1APBLPEN`"]
pub type IWDG1APBLPEN_R = crate::R<bool, IWDG1APBLPEN_A>;
impl IWDG1APBLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDG1APBLPEN_A {
        match self.bits {
            false => IWDG1APBLPEN_A::B_0X0,
            true => IWDG1APBLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IWDG1APBLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IWDG1APBLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `IWDG1APBLPEN`"]
pub struct IWDG1APBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG1APBLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IWDG1APBLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the APB clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IWDG1APBLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the APB clock in CSLEEP, reading means that the APB clock is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IWDG1APBLPEN_A::B_0X1)
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
#[doc = "BSECLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSECLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<BSECLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: BSECLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BSECLPEN`"]
pub type BSECLPEN_R = crate::R<bool, BSECLPEN_A>;
impl BSECLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSECLPEN_A {
        match self.bits {
            false => BSECLPEN_A::B_0X0,
            true => BSECLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BSECLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BSECLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `BSECLPEN`"]
pub struct BSECLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BSECLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BSECLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BSECLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BSECLPEN_A::B_0X1)
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
#[doc = "STGENLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STGENLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<STGENLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: STGENLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STGENLPEN`"]
pub type STGENLPEN_R = crate::R<bool, STGENLPEN_A>;
impl STGENLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STGENLPEN_A {
        match self.bits {
            false => STGENLPEN_A::B_0X0,
            true => STGENLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STGENLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STGENLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `STGENLPEN`"]
pub struct STGENLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STGENLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(STGENLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(STGENLPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "STGENSTPEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STGENSTPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSTOP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSTOP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSTOP"]
    B_0X1 = 1,
}
impl From<STGENSTPEN_A> for bool {
    #[inline(always)]
    fn from(variant: STGENSTPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STGENSTPEN`"]
pub type STGENSTPEN_R = crate::R<bool, STGENSTPEN_A>;
impl STGENSTPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STGENSTPEN_A {
        match self.bits {
            false => STGENSTPEN_A::B_0X0,
            true => STGENSTPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STGENSTPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STGENSTPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `STGENSTPEN`"]
pub struct STGENSTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENSTPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STGENSTPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSTOP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(STGENSTPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSTOP, reading means that the peripheral clocks are enabled in CSTOP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(STGENSTPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI6LPEN"]
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C4LPEN"]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C6LPEN"]
    #[inline(always)]
    pub fn i2c6lpen(&self) -> I2C6LPEN_R {
        I2C6LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1LPEN"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTCAPBLPEN"]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TZC1LPEN"]
    #[inline(always)]
    pub fn tzc1lpen(&self) -> TZC1LPEN_R {
        TZC1LPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TZC2LPEN"]
    #[inline(always)]
    pub fn tzc2lpen(&self) -> TZC2LPEN_R {
        TZC2LPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TZPCLPEN"]
    #[inline(always)]
    pub fn tzpclpen(&self) -> TZPCLPEN_R {
        TZPCLPEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IWDG1APBLPEN"]
    #[inline(always)]
    pub fn iwdg1apblpen(&self) -> IWDG1APBLPEN_R {
        IWDG1APBLPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BSECLPEN"]
    #[inline(always)]
    pub fn bseclpen(&self) -> BSECLPEN_R {
        BSECLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENLPEN"]
    #[inline(always)]
    pub fn stgenlpen(&self) -> STGENLPEN_R {
        STGENLPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - STGENSTPEN"]
    #[inline(always)]
    pub fn stgenstpen(&self) -> STGENSTPEN_R {
        STGENSTPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6LPEN"]
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W {
        SPI6LPEN_W { w: self }
    }
    #[doc = "Bit 2 - I2C4LPEN"]
    #[inline(always)]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W {
        I2C4LPEN_W { w: self }
    }
    #[doc = "Bit 3 - I2C6LPEN"]
    #[inline(always)]
    pub fn i2c6lpen(&mut self) -> I2C6LPEN_W {
        I2C6LPEN_W { w: self }
    }
    #[doc = "Bit 4 - USART1LPEN"]
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W {
        USART1LPEN_W { w: self }
    }
    #[doc = "Bit 8 - RTCAPBLPEN"]
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W {
        RTCAPBLPEN_W { w: self }
    }
    #[doc = "Bit 11 - TZC1LPEN"]
    #[inline(always)]
    pub fn tzc1lpen(&mut self) -> TZC1LPEN_W {
        TZC1LPEN_W { w: self }
    }
    #[doc = "Bit 12 - TZC2LPEN"]
    #[inline(always)]
    pub fn tzc2lpen(&mut self) -> TZC2LPEN_W {
        TZC2LPEN_W { w: self }
    }
    #[doc = "Bit 13 - TZPCLPEN"]
    #[inline(always)]
    pub fn tzpclpen(&mut self) -> TZPCLPEN_W {
        TZPCLPEN_W { w: self }
    }
    #[doc = "Bit 15 - IWDG1APBLPEN"]
    #[inline(always)]
    pub fn iwdg1apblpen(&mut self) -> IWDG1APBLPEN_W {
        IWDG1APBLPEN_W { w: self }
    }
    #[doc = "Bit 16 - BSECLPEN"]
    #[inline(always)]
    pub fn bseclpen(&mut self) -> BSECLPEN_W {
        BSECLPEN_W { w: self }
    }
    #[doc = "Bit 20 - STGENLPEN"]
    #[inline(always)]
    pub fn stgenlpen(&mut self) -> STGENLPEN_W {
        STGENLPEN_W { w: self }
    }
    #[doc = "Bit 21 - STGENSTPEN"]
    #[inline(always)]
    pub fn stgenstpen(&mut self) -> STGENSTPEN_W {
        STGENSTPEN_W { w: self }
    }
}
