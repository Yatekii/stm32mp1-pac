#[doc = "Reader of register RCC_MP_APB2LPENSETR"]
pub type R = crate::R<u32, super::RCC_MP_APB2LPENSETR>;
#[doc = "Writer for register RCC_MP_APB2LPENSETR"]
pub type W = crate::W<u32, super::RCC_MP_APB2LPENSETR>;
#[doc = "Register RCC_MP_APB2LPENSETR `reset()`'s with value 0x0137_271f"]
impl crate::ResetValue for super::RCC_MP_APB2LPENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0137_271f
    }
}
#[doc = "TIM1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<TIM1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM1LPEN`"]
pub type TIM1LPEN_R = crate::R<bool, TIM1LPEN_A>;
impl TIM1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1LPEN_A {
        match self.bits {
            false => TIM1LPEN_A::B_0X0,
            true => TIM1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM1LPEN`"]
pub struct TIM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::B_0X1)
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
#[doc = "TIM8LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM8LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<TIM8LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM8LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM8LPEN`"]
pub type TIM8LPEN_R = crate::R<bool, TIM8LPEN_A>;
impl TIM8LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM8LPEN_A {
        match self.bits {
            false => TIM8LPEN_A::B_0X0,
            true => TIM8LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM8LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM8LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM8LPEN`"]
pub struct TIM8LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM8LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM8LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM8LPEN_A::B_0X1)
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
#[doc = "TIM15LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM15LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<TIM15LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM15LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM15LPEN`"]
pub type TIM15LPEN_R = crate::R<bool, TIM15LPEN_A>;
impl TIM15LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM15LPEN_A {
        match self.bits {
            false => TIM15LPEN_A::B_0X0,
            true => TIM15LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM15LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM15LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM15LPEN`"]
pub struct TIM15LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM15LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM15LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM15LPEN_A::B_0X1)
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
#[doc = "TIM16LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM16LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<TIM16LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM16LPEN`"]
pub type TIM16LPEN_R = crate::R<bool, TIM16LPEN_A>;
impl TIM16LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM16LPEN_A {
        match self.bits {
            false => TIM16LPEN_A::B_0X0,
            true => TIM16LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM16LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM16LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM16LPEN`"]
pub struct TIM16LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM16LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM16LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM16LPEN_A::B_0X1)
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
#[doc = "TIM17LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM17LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<TIM17LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM17LPEN`"]
pub type TIM17LPEN_R = crate::R<bool, TIM17LPEN_A>;
impl TIM17LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM17LPEN_A {
        match self.bits {
            false => TIM17LPEN_A::B_0X0,
            true => TIM17LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM17LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM17LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM17LPEN`"]
pub struct TIM17LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM17LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM17LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM17LPEN_A::B_0X1)
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
#[doc = "SPI1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SPI1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1LPEN`"]
pub type SPI1LPEN_R = crate::R<bool, SPI1LPEN_A>;
impl SPI1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1LPEN_A {
        match self.bits {
            false => SPI1LPEN_A::B_0X0,
            true => SPI1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI1LPEN`"]
pub struct SPI1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI1LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI1LPEN_A::B_0X1)
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
#[doc = "SPI4LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI4LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SPI4LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI4LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI4LPEN`"]
pub type SPI4LPEN_R = crate::R<bool, SPI4LPEN_A>;
impl SPI4LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI4LPEN_A {
        match self.bits {
            false => SPI4LPEN_A::B_0X0,
            true => SPI4LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI4LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI4LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI4LPEN`"]
pub struct SPI4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI4LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI4LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI4LPEN_A::B_0X1)
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
#[doc = "SPI5LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI5LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SPI5LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI5LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI5LPEN`"]
pub type SPI5LPEN_R = crate::R<bool, SPI5LPEN_A>;
impl SPI5LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI5LPEN_A {
        match self.bits {
            false => SPI5LPEN_A::B_0X0,
            true => SPI5LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI5LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI5LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI5LPEN`"]
pub struct SPI5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI5LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI5LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI5LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI5LPEN_A::B_0X1)
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
#[doc = "USART6LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART6LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<USART6LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USART6LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USART6LPEN`"]
pub type USART6LPEN_R = crate::R<bool, USART6LPEN_A>;
impl USART6LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART6LPEN_A {
        match self.bits {
            false => USART6LPEN_A::B_0X0,
            true => USART6LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART6LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART6LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `USART6LPEN`"]
pub struct USART6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART6LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART6LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART6LPEN_A::B_0X1)
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
#[doc = "SAI1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SAI1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI1LPEN`"]
pub type SAI1LPEN_R = crate::R<bool, SAI1LPEN_A>;
impl SAI1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1LPEN_A {
        match self.bits {
            false => SAI1LPEN_A::B_0X0,
            true => SAI1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI1LPEN`"]
pub struct SAI1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI1LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI1LPEN_A::B_0X1)
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
#[doc = "SAI2LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SAI2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI2LPEN`"]
pub type SAI2LPEN_R = crate::R<bool, SAI2LPEN_A>;
impl SAI2LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2LPEN_A {
        match self.bits {
            false => SAI2LPEN_A::B_0X0,
            true => SAI2LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI2LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI2LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI2LPEN`"]
pub struct SAI2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI2LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI2LPEN_A::B_0X1)
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
#[doc = "SAI3LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SAI3LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI3LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI3LPEN`"]
pub type SAI3LPEN_R = crate::R<bool, SAI3LPEN_A>;
impl SAI3LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI3LPEN_A {
        match self.bits {
            false => SAI3LPEN_A::B_0X0,
            true => SAI3LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI3LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI3LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI3LPEN`"]
pub struct SAI3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI3LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI3LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI3LPEN_A::B_0X1)
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
#[doc = "DFSDMLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFSDMLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<DFSDMLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDMLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFSDMLPEN`"]
pub type DFSDMLPEN_R = crate::R<bool, DFSDMLPEN_A>;
impl DFSDMLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDMLPEN_A {
        match self.bits {
            false => DFSDMLPEN_A::B_0X0,
            true => DFSDMLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DFSDMLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DFSDMLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DFSDMLPEN`"]
pub struct DFSDMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFSDMLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DFSDMLPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DFSDMLPEN_A::B_0X1)
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
#[doc = "ADFSDMLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADFSDMLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<ADFSDMLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADFSDMLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADFSDMLPEN`"]
pub type ADFSDMLPEN_R = crate::R<bool, ADFSDMLPEN_A>;
impl ADFSDMLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADFSDMLPEN_A {
        match self.bits {
            false => ADFSDMLPEN_A::B_0X0,
            true => ADFSDMLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADFSDMLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADFSDMLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ADFSDMLPEN`"]
pub struct ADFSDMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADFSDMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADFSDMLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADFSDMLPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADFSDMLPEN_A::B_0X1)
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
#[doc = "FDCANLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDCANLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<FDCANLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FDCANLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDCANLPEN`"]
pub type FDCANLPEN_R = crate::R<bool, FDCANLPEN_A>;
impl FDCANLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDCANLPEN_A {
        match self.bits {
            false => FDCANLPEN_A::B_0X0,
            true => FDCANLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FDCANLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FDCANLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `FDCANLPEN`"]
pub struct FDCANLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FDCANLPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FDCANLPEN_A::B_0X1)
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
    #[doc = "Bit 0 - TIM1LPEN"]
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM8LPEN"]
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15LPEN"]
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16LPEN"]
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17LPEN"]
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI1LPEN"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI4LPEN"]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI5LPEN"]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - USART6LPEN"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SAI1LPEN"]
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAI2LPEN"]
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SAI3LPEN"]
    #[inline(always)]
    pub fn sai3lpen(&self) -> SAI3LPEN_R {
        SAI3LPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DFSDMLPEN"]
    #[inline(always)]
    pub fn dfsdmlpen(&self) -> DFSDMLPEN_R {
        DFSDMLPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADFSDMLPEN"]
    #[inline(always)]
    pub fn adfsdmlpen(&self) -> ADFSDMLPEN_R {
        ADFSDMLPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FDCANLPEN"]
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FDCANLPEN_R {
        FDCANLPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1LPEN"]
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W {
        TIM1LPEN_W { w: self }
    }
    #[doc = "Bit 1 - TIM8LPEN"]
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W {
        TIM8LPEN_W { w: self }
    }
    #[doc = "Bit 2 - TIM15LPEN"]
    #[inline(always)]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W {
        TIM15LPEN_W { w: self }
    }
    #[doc = "Bit 3 - TIM16LPEN"]
    #[inline(always)]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W {
        TIM16LPEN_W { w: self }
    }
    #[doc = "Bit 4 - TIM17LPEN"]
    #[inline(always)]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W {
        TIM17LPEN_W { w: self }
    }
    #[doc = "Bit 8 - SPI1LPEN"]
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W {
        SPI1LPEN_W { w: self }
    }
    #[doc = "Bit 9 - SPI4LPEN"]
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W {
        SPI4LPEN_W { w: self }
    }
    #[doc = "Bit 10 - SPI5LPEN"]
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W {
        SPI5LPEN_W { w: self }
    }
    #[doc = "Bit 13 - USART6LPEN"]
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W {
        USART6LPEN_W { w: self }
    }
    #[doc = "Bit 16 - SAI1LPEN"]
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W {
        SAI1LPEN_W { w: self }
    }
    #[doc = "Bit 17 - SAI2LPEN"]
    #[inline(always)]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W {
        SAI2LPEN_W { w: self }
    }
    #[doc = "Bit 18 - SAI3LPEN"]
    #[inline(always)]
    pub fn sai3lpen(&mut self) -> SAI3LPEN_W {
        SAI3LPEN_W { w: self }
    }
    #[doc = "Bit 20 - DFSDMLPEN"]
    #[inline(always)]
    pub fn dfsdmlpen(&mut self) -> DFSDMLPEN_W {
        DFSDMLPEN_W { w: self }
    }
    #[doc = "Bit 21 - ADFSDMLPEN"]
    #[inline(always)]
    pub fn adfsdmlpen(&mut self) -> ADFSDMLPEN_W {
        ADFSDMLPEN_W { w: self }
    }
    #[doc = "Bit 24 - FDCANLPEN"]
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FDCANLPEN_W {
        FDCANLPEN_W { w: self }
    }
}
