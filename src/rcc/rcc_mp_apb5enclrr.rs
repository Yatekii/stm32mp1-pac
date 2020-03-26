#[doc = "Reader of register RCC_MP_APB5ENCLRR"]
pub type R = crate::R<u32, super::RCC_MP_APB5ENCLRR>;
#[doc = "Writer for register RCC_MP_APB5ENCLRR"]
pub type W = crate::W<u32, super::RCC_MP_APB5ENCLRR>;
#[doc = "Register RCC_MP_APB5ENCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_APB5ENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SPI6EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI6EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<SPI6EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI6EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI6EN`"]
pub type SPI6EN_R = crate::R<bool, SPI6EN_A>;
impl SPI6EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI6EN_A {
        match self.bits {
            false => SPI6EN_A::B_0X0,
            true => SPI6EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI6EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI6EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI6EN`"]
pub struct SPI6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI6EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI6EN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI6EN_A::B_0X1)
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
#[doc = "I2C4EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C4EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<I2C4EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C4EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C4EN`"]
pub type I2C4EN_R = crate::R<bool, I2C4EN_A>;
impl I2C4EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C4EN_A {
        match self.bits {
            false => I2C4EN_A::B_0X0,
            true => I2C4EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C4EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C4EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `I2C4EN`"]
pub struct I2C4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C4EN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C4EN_A::B_0X1)
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
#[doc = "I2C6EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C6EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<I2C6EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C6EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C6EN`"]
pub type I2C6EN_R = crate::R<bool, I2C6EN_A>;
impl I2C6EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C6EN_A {
        match self.bits {
            false => I2C6EN_A::B_0X0,
            true => I2C6EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C6EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C6EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `I2C6EN`"]
pub struct I2C6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C6EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C6EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C6EN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C6EN_A::B_0X1)
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
#[doc = "USART1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<USART1EN_A> for bool {
    #[inline(always)]
    fn from(variant: USART1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USART1EN`"]
pub type USART1EN_R = crate::R<bool, USART1EN_A>;
impl USART1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1EN_A {
        match self.bits {
            false => USART1EN_A::B_0X0,
            true => USART1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `USART1EN`"]
pub struct USART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART1EN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART1EN_A::B_0X1)
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
#[doc = "RTCAPBEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCAPBEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<RTCAPBEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCAPBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCAPBEN`"]
pub type RTCAPBEN_R = crate::R<bool, RTCAPBEN_A>;
impl RTCAPBEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCAPBEN_A {
        match self.bits {
            false => RTCAPBEN_A::B_0X0,
            true => RTCAPBEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCAPBEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCAPBEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `RTCAPBEN`"]
pub struct RTCAPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCAPBEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RTCAPBEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RTCAPBEN_A::B_0X1)
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
#[doc = "TZC1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZC1EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the clocks are disabled for AXI port\r\n                  1"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the AXI port 1\r\n                  peripheral clocks, reading means that the\r\n                  peripheral clocks are enabled for AXI port\r\n                  1"]
    B_0X1 = 1,
}
impl From<TZC1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TZC1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TZC1EN`"]
pub type TZC1EN_R = crate::R<bool, TZC1EN_A>;
impl TZC1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZC1EN_A {
        match self.bits {
            false => TZC1EN_A::B_0X0,
            true => TZC1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TZC1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TZC1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TZC1EN`"]
pub struct TZC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TZC1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the clocks are disabled for AXI port 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TZC1EN_A::B_0X0)
    }
    #[doc = "Writing disables the AXI port 1 peripheral clocks, reading means that the peripheral clocks are enabled for AXI port 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TZC1EN_A::B_0X1)
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
#[doc = "TZC2EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZC2EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the clocks are disabled for AXI port\r\n                  2"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the AXI port 2\r\n                  peripheral clocks, reading means that the\r\n                  peripheral clocks are enabled for AXI port\r\n                  2"]
    B_0X1 = 1,
}
impl From<TZC2EN_A> for bool {
    #[inline(always)]
    fn from(variant: TZC2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TZC2EN`"]
pub type TZC2EN_R = crate::R<bool, TZC2EN_A>;
impl TZC2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZC2EN_A {
        match self.bits {
            false => TZC2EN_A::B_0X0,
            true => TZC2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TZC2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TZC2EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TZC2EN`"]
pub struct TZC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TZC2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the clocks are disabled for AXI port 2"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TZC2EN_A::B_0X0)
    }
    #[doc = "Writing disables the AXI port 2 peripheral clocks, reading means that the peripheral clocks are enabled for AXI port 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TZC2EN_A::B_0X1)
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
#[doc = "TZPCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZPCEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<TZPCEN_A> for bool {
    #[inline(always)]
    fn from(variant: TZPCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TZPCEN`"]
pub type TZPCEN_R = crate::R<bool, TZPCEN_A>;
impl TZPCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZPCEN_A {
        match self.bits {
            false => TZPCEN_A::B_0X0,
            true => TZPCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TZPCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TZPCEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TZPCEN`"]
pub struct TZPCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZPCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TZPCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TZPCEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TZPCEN_A::B_0X1)
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
#[doc = "IWDG1APBEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDG1APBEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the APB clock is disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the APB clock,\r\n                  reading means that the APB clock is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<IWDG1APBEN_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG1APBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IWDG1APBEN`"]
pub type IWDG1APBEN_R = crate::R<bool, IWDG1APBEN_A>;
impl IWDG1APBEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDG1APBEN_A {
        match self.bits {
            false => IWDG1APBEN_A::B_0X0,
            true => IWDG1APBEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IWDG1APBEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IWDG1APBEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `IWDG1APBEN`"]
pub struct IWDG1APBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG1APBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IWDG1APBEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the APB clock is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IWDG1APBEN_A::B_0X0)
    }
    #[doc = "Writing disables the APB clock, reading means that the APB clock is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IWDG1APBEN_A::B_0X1)
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
#[doc = "BSECEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSECEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<BSECEN_A> for bool {
    #[inline(always)]
    fn from(variant: BSECEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BSECEN`"]
pub type BSECEN_R = crate::R<bool, BSECEN_A>;
impl BSECEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSECEN_A {
        match self.bits {
            false => BSECEN_A::B_0X0,
            true => BSECEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BSECEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BSECEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `BSECEN`"]
pub struct BSECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BSECEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BSECEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BSECEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BSECEN_A::B_0X1)
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
#[doc = "STGENEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STGENEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<STGENEN_A> for bool {
    #[inline(always)]
    fn from(variant: STGENEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STGENEN`"]
pub type STGENEN_R = crate::R<bool, STGENEN_A>;
impl STGENEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STGENEN_A {
        match self.bits {
            false => STGENEN_A::B_0X0,
            true => STGENEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STGENEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STGENEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `STGENEN`"]
pub struct STGENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STGENEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(STGENEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(STGENEN_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - SPI6EN"]
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C4EN"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C6EN"]
    #[inline(always)]
    pub fn i2c6en(&self) -> I2C6EN_R {
        I2C6EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1EN"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTCAPBEN"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TZC1EN"]
    #[inline(always)]
    pub fn tzc1en(&self) -> TZC1EN_R {
        TZC1EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TZC2EN"]
    #[inline(always)]
    pub fn tzc2en(&self) -> TZC2EN_R {
        TZC2EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TZPCEN"]
    #[inline(always)]
    pub fn tzpcen(&self) -> TZPCEN_R {
        TZPCEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IWDG1APBEN"]
    #[inline(always)]
    pub fn iwdg1apben(&self) -> IWDG1APBEN_R {
        IWDG1APBEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BSECEN"]
    #[inline(always)]
    pub fn bsecen(&self) -> BSECEN_R {
        BSECEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENEN"]
    #[inline(always)]
    pub fn stgenen(&self) -> STGENEN_R {
        STGENEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6EN"]
    #[inline(always)]
    pub fn spi6en(&mut self) -> SPI6EN_W {
        SPI6EN_W { w: self }
    }
    #[doc = "Bit 2 - I2C4EN"]
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W {
        I2C4EN_W { w: self }
    }
    #[doc = "Bit 3 - I2C6EN"]
    #[inline(always)]
    pub fn i2c6en(&mut self) -> I2C6EN_W {
        I2C6EN_W { w: self }
    }
    #[doc = "Bit 4 - USART1EN"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W { w: self }
    }
    #[doc = "Bit 8 - RTCAPBEN"]
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W {
        RTCAPBEN_W { w: self }
    }
    #[doc = "Bit 11 - TZC1EN"]
    #[inline(always)]
    pub fn tzc1en(&mut self) -> TZC1EN_W {
        TZC1EN_W { w: self }
    }
    #[doc = "Bit 12 - TZC2EN"]
    #[inline(always)]
    pub fn tzc2en(&mut self) -> TZC2EN_W {
        TZC2EN_W { w: self }
    }
    #[doc = "Bit 13 - TZPCEN"]
    #[inline(always)]
    pub fn tzpcen(&mut self) -> TZPCEN_W {
        TZPCEN_W { w: self }
    }
    #[doc = "Bit 15 - IWDG1APBEN"]
    #[inline(always)]
    pub fn iwdg1apben(&mut self) -> IWDG1APBEN_W {
        IWDG1APBEN_W { w: self }
    }
    #[doc = "Bit 16 - BSECEN"]
    #[inline(always)]
    pub fn bsecen(&mut self) -> BSECEN_W {
        BSECEN_W { w: self }
    }
    #[doc = "Bit 20 - STGENEN"]
    #[inline(always)]
    pub fn stgenen(&mut self) -> STGENEN_W {
        STGENEN_W { w: self }
    }
}
