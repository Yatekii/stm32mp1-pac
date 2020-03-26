#[doc = "Reader of register RCC_MP_APB2ENSETR"]
pub type R = crate::R<u32, super::RCC_MP_APB2ENSETR>;
#[doc = "Writer for register RCC_MP_APB2ENSETR"]
pub type W = crate::W<u32, super::RCC_MP_APB2ENSETR>;
#[doc = "Register RCC_MP_APB2ENSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_APB2ENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TIM1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<TIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM1EN`"]
pub type TIM1EN_R = crate::R<bool, TIM1EN_A>;
impl TIM1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1EN_A {
        match self.bits {
            false => TIM1EN_A::B_0X0,
            true => TIM1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM1EN`"]
pub struct TIM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM1EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM1EN_A::B_0X1)
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
#[doc = "TIM8EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM8EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<TIM8EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM8EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM8EN`"]
pub type TIM8EN_R = crate::R<bool, TIM8EN_A>;
impl TIM8EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM8EN_A {
        match self.bits {
            false => TIM8EN_A::B_0X0,
            true => TIM8EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM8EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM8EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM8EN`"]
pub struct TIM8EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM8EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM8EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM8EN_A::B_0X1)
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
#[doc = "TIM15EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM15EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<TIM15EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM15EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM15EN`"]
pub type TIM15EN_R = crate::R<bool, TIM15EN_A>;
impl TIM15EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM15EN_A {
        match self.bits {
            false => TIM15EN_A::B_0X0,
            true => TIM15EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM15EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM15EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM15EN`"]
pub struct TIM15EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM15EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM15EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM15EN_A::B_0X1)
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
#[doc = "TIM16EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM16EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<TIM16EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM16EN`"]
pub type TIM16EN_R = crate::R<bool, TIM16EN_A>;
impl TIM16EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM16EN_A {
        match self.bits {
            false => TIM16EN_A::B_0X0,
            true => TIM16EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM16EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM16EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM16EN`"]
pub struct TIM16EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM16EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM16EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM16EN_A::B_0X1)
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
#[doc = "TIM17EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM17EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<TIM17EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM17EN`"]
pub type TIM17EN_R = crate::R<bool, TIM17EN_A>;
impl TIM17EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM17EN_A {
        match self.bits {
            false => TIM17EN_A::B_0X0,
            true => TIM17EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM17EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM17EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM17EN`"]
pub struct TIM17EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM17EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM17EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM17EN_A::B_0X1)
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
#[doc = "SPI1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<SPI1EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1EN`"]
pub type SPI1EN_R = crate::R<bool, SPI1EN_A>;
impl SPI1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1EN_A {
        match self.bits {
            false => SPI1EN_A::B_0X0,
            true => SPI1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI1EN`"]
pub struct SPI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI1EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI1EN_A::B_0X1)
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
#[doc = "SPI4EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI4EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<SPI4EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI4EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI4EN`"]
pub type SPI4EN_R = crate::R<bool, SPI4EN_A>;
impl SPI4EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI4EN_A {
        match self.bits {
            false => SPI4EN_A::B_0X0,
            true => SPI4EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI4EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI4EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI4EN`"]
pub struct SPI4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI4EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI4EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI4EN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "SPI5EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI5EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<SPI5EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI5EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI5EN`"]
pub type SPI5EN_R = crate::R<bool, SPI5EN_A>;
impl SPI5EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI5EN_A {
        match self.bits {
            false => SPI5EN_A::B_0X0,
            true => SPI5EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI5EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI5EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI5EN`"]
pub struct SPI5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI5EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI5EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI5EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI5EN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "USART6EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART6EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<USART6EN_A> for bool {
    #[inline(always)]
    fn from(variant: USART6EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USART6EN`"]
pub type USART6EN_R = crate::R<bool, USART6EN_A>;
impl USART6EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART6EN_A {
        match self.bits {
            false => USART6EN_A::B_0X0,
            true => USART6EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART6EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART6EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `USART6EN`"]
pub struct USART6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART6EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART6EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART6EN_A::B_0X1)
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
#[doc = "SAI1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<SAI1EN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI1EN`"]
pub type SAI1EN_R = crate::R<bool, SAI1EN_A>;
impl SAI1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1EN_A {
        match self.bits {
            false => SAI1EN_A::B_0X0,
            true => SAI1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI1EN`"]
pub struct SAI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI1EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI1EN_A::B_0X1)
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
#[doc = "SAI2EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<SAI2EN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI2EN`"]
pub type SAI2EN_R = crate::R<bool, SAI2EN_A>;
impl SAI2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2EN_A {
        match self.bits {
            false => SAI2EN_A::B_0X0,
            true => SAI2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI2EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI2EN`"]
pub struct SAI2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI2EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI2EN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "SAI3EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<SAI3EN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI3EN`"]
pub type SAI3EN_R = crate::R<bool, SAI3EN_A>;
impl SAI3EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI3EN_A {
        match self.bits {
            false => SAI3EN_A::B_0X0,
            true => SAI3EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI3EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI3EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI3EN`"]
pub struct SAI3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI3EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI3EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI3EN_A::B_0X1)
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
#[doc = "DFSDMEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFSDMEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<DFSDMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFSDMEN`"]
pub type DFSDMEN_R = crate::R<bool, DFSDMEN_A>;
impl DFSDMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDMEN_A {
        match self.bits {
            false => DFSDMEN_A::B_0X0,
            true => DFSDMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DFSDMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DFSDMEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DFSDMEN`"]
pub struct DFSDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFSDMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DFSDMEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DFSDMEN_A::B_0X1)
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
#[doc = "ADFSDMEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADFSDMEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<ADFSDMEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADFSDMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADFSDMEN`"]
pub type ADFSDMEN_R = crate::R<bool, ADFSDMEN_A>;
impl ADFSDMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADFSDMEN_A {
        match self.bits {
            false => ADFSDMEN_A::B_0X0,
            true => ADFSDMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADFSDMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADFSDMEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ADFSDMEN`"]
pub struct ADFSDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFSDMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADFSDMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADFSDMEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADFSDMEN_A::B_0X1)
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
#[doc = "FDCANEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDCANEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<FDCANEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDCANEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDCANEN`"]
pub type FDCANEN_R = crate::R<bool, FDCANEN_A>;
impl FDCANEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDCANEN_A {
        match self.bits {
            false => FDCANEN_A::B_0X0,
            true => FDCANEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FDCANEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FDCANEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `FDCANEN`"]
pub struct FDCANEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FDCANEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FDCANEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TIM1EN"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM8EN"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15EN"]
    #[inline(always)]
    pub fn tim15en(&self) -> TIM15EN_R {
        TIM15EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16EN"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17EN"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI4EN"]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI5EN"]
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - USART6EN"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SAI1EN"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAI2EN"]
    #[inline(always)]
    pub fn sai2en(&self) -> SAI2EN_R {
        SAI2EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SAI3EN"]
    #[inline(always)]
    pub fn sai3en(&self) -> SAI3EN_R {
        SAI3EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADFSDMEN"]
    #[inline(always)]
    pub fn adfsdmen(&self) -> ADFSDMEN_R {
        ADFSDMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FDCANEN"]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1EN"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W {
        TIM1EN_W { w: self }
    }
    #[doc = "Bit 1 - TIM8EN"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W {
        TIM8EN_W { w: self }
    }
    #[doc = "Bit 2 - TIM15EN"]
    #[inline(always)]
    pub fn tim15en(&mut self) -> TIM15EN_W {
        TIM15EN_W { w: self }
    }
    #[doc = "Bit 3 - TIM16EN"]
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W {
        TIM16EN_W { w: self }
    }
    #[doc = "Bit 4 - TIM17EN"]
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W {
        TIM17EN_W { w: self }
    }
    #[doc = "Bit 8 - SPI1EN"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W { w: self }
    }
    #[doc = "Bit 9 - SPI4EN"]
    #[inline(always)]
    pub fn spi4en(&mut self) -> SPI4EN_W {
        SPI4EN_W { w: self }
    }
    #[doc = "Bit 10 - SPI5EN"]
    #[inline(always)]
    pub fn spi5en(&mut self) -> SPI5EN_W {
        SPI5EN_W { w: self }
    }
    #[doc = "Bit 13 - USART6EN"]
    #[inline(always)]
    pub fn usart6en(&mut self) -> USART6EN_W {
        USART6EN_W { w: self }
    }
    #[doc = "Bit 16 - SAI1EN"]
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W {
        SAI1EN_W { w: self }
    }
    #[doc = "Bit 17 - SAI2EN"]
    #[inline(always)]
    pub fn sai2en(&mut self) -> SAI2EN_W {
        SAI2EN_W { w: self }
    }
    #[doc = "Bit 18 - SAI3EN"]
    #[inline(always)]
    pub fn sai3en(&mut self) -> SAI3EN_W {
        SAI3EN_W { w: self }
    }
    #[doc = "Bit 20 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W {
        DFSDMEN_W { w: self }
    }
    #[doc = "Bit 21 - ADFSDMEN"]
    #[inline(always)]
    pub fn adfsdmen(&mut self) -> ADFSDMEN_W {
        ADFSDMEN_W { w: self }
    }
    #[doc = "Bit 24 - FDCANEN"]
    #[inline(always)]
    pub fn fdcanen(&mut self) -> FDCANEN_W {
        FDCANEN_W { w: self }
    }
}
