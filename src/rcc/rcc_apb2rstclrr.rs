#[doc = "Reader of register RCC_APB2RSTCLRR"]
pub type R = crate::R<u32, super::RCC_APB2RSTCLRR>;
#[doc = "Writer for register RCC_APB2RSTCLRR"]
pub type W = crate::W<u32, super::RCC_APB2RSTCLRR>;
#[doc = "Register RCC_APB2RSTCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_APB2RSTCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TIM1RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<TIM1RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM1RST`"]
pub type TIM1RST_R = crate::R<bool, TIM1RST_A>;
impl TIM1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1RST_A {
        match self.bits {
            false => TIM1RST_A::B_0X0,
            true => TIM1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM1RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM1RST`"]
pub struct TIM1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM1RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM1RST_A::B_0X1)
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
#[doc = "TIM8RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM8RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<TIM8RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM8RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM8RST`"]
pub type TIM8RST_R = crate::R<bool, TIM8RST_A>;
impl TIM8RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM8RST_A {
        match self.bits {
            false => TIM8RST_A::B_0X0,
            true => TIM8RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM8RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM8RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM8RST`"]
pub struct TIM8RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM8RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM8RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM8RST_A::B_0X1)
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
#[doc = "TIM15RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM15RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<TIM15RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM15RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM15RST`"]
pub type TIM15RST_R = crate::R<bool, TIM15RST_A>;
impl TIM15RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM15RST_A {
        match self.bits {
            false => TIM15RST_A::B_0X0,
            true => TIM15RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM15RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM15RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM15RST`"]
pub struct TIM15RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM15RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM15RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM15RST_A::B_0X1)
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
#[doc = "TIM16RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM16RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<TIM16RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM16RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM16RST`"]
pub type TIM16RST_R = crate::R<bool, TIM16RST_A>;
impl TIM16RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM16RST_A {
        match self.bits {
            false => TIM16RST_A::B_0X0,
            true => TIM16RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM16RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM16RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM16RST`"]
pub struct TIM16RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM16RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM16RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM16RST_A::B_0X1)
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
#[doc = "TIM17RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM17RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<TIM17RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM17RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIM17RST`"]
pub type TIM17RST_R = crate::R<bool, TIM17RST_A>;
impl TIM17RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM17RST_A {
        match self.bits {
            false => TIM17RST_A::B_0X0,
            true => TIM17RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIM17RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIM17RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIM17RST`"]
pub struct TIM17RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM17RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIM17RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIM17RST_A::B_0X1)
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
#[doc = "SPI1RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SPI1RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI1RST`"]
pub type SPI1RST_R = crate::R<bool, SPI1RST_A>;
impl SPI1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1RST_A {
        match self.bits {
            false => SPI1RST_A::B_0X0,
            true => SPI1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI1RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI1RST`"]
pub struct SPI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI1RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI1RST_A::B_0X1)
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
#[doc = "SPI4RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI4RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SPI4RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI4RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI4RST`"]
pub type SPI4RST_R = crate::R<bool, SPI4RST_A>;
impl SPI4RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI4RST_A {
        match self.bits {
            false => SPI4RST_A::B_0X0,
            true => SPI4RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI4RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI4RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI4RST`"]
pub struct SPI4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI4RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI4RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI4RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI4RST_A::B_0X1)
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
#[doc = "SPI5RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI5RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SPI5RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI5RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI5RST`"]
pub type SPI5RST_R = crate::R<bool, SPI5RST_A>;
impl SPI5RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI5RST_A {
        match self.bits {
            false => SPI5RST_A::B_0X0,
            true => SPI5RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI5RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI5RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI5RST`"]
pub struct SPI5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI5RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI5RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI5RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI5RST_A::B_0X1)
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
#[doc = "USART6RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART6RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<USART6RST_A> for bool {
    #[inline(always)]
    fn from(variant: USART6RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USART6RST`"]
pub type USART6RST_R = crate::R<bool, USART6RST_A>;
impl USART6RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART6RST_A {
        match self.bits {
            false => USART6RST_A::B_0X0,
            true => USART6RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART6RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART6RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `USART6RST`"]
pub struct USART6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART6RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART6RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART6RST_A::B_0X1)
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
#[doc = "SAI1RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI1RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SAI1RST_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI1RST`"]
pub type SAI1RST_R = crate::R<bool, SAI1RST_A>;
impl SAI1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1RST_A {
        match self.bits {
            false => SAI1RST_A::B_0X0,
            true => SAI1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI1RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI1RST`"]
pub struct SAI1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI1RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI1RST_A::B_0X1)
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
#[doc = "SAI2RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI2RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SAI2RST_A> for bool {
    #[inline(always)]
    fn from(variant: SAI2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI2RST`"]
pub type SAI2RST_R = crate::R<bool, SAI2RST_A>;
impl SAI2RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2RST_A {
        match self.bits {
            false => SAI2RST_A::B_0X0,
            true => SAI2RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI2RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI2RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI2RST`"]
pub struct SAI2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI2RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI2RST_A::B_0X1)
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
#[doc = "SAI3RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI3RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SAI3RST_A> for bool {
    #[inline(always)]
    fn from(variant: SAI3RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI3RST`"]
pub type SAI3RST_R = crate::R<bool, SAI3RST_A>;
impl SAI3RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI3RST_A {
        match self.bits {
            false => SAI3RST_A::B_0X0,
            true => SAI3RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI3RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI3RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI3RST`"]
pub struct SAI3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI3RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI3RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI3RST_A::B_0X1)
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
#[doc = "DFSDMRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFSDMRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<DFSDMRST_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDMRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFSDMRST`"]
pub type DFSDMRST_R = crate::R<bool, DFSDMRST_A>;
impl DFSDMRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDMRST_A {
        match self.bits {
            false => DFSDMRST_A::B_0X0,
            true => DFSDMRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DFSDMRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DFSDMRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DFSDMRST`"]
pub struct DFSDMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFSDMRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DFSDMRST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DFSDMRST_A::B_0X1)
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
#[doc = "FDCANRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDCANRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<FDCANRST_A> for bool {
    #[inline(always)]
    fn from(variant: FDCANRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDCANRST`"]
pub type FDCANRST_R = crate::R<bool, FDCANRST_A>;
impl FDCANRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDCANRST_A {
        match self.bits {
            false => FDCANRST_A::B_0X0,
            true => FDCANRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FDCANRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FDCANRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `FDCANRST`"]
pub struct FDCANRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FDCANRST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FDCANRST_A::B_0X1)
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
    #[doc = "Bit 0 - TIM1RST"]
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM8RST"]
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM15RST"]
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM16RST"]
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM17RST"]
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SPI4RST"]
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI5RST"]
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - USART6RST"]
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SAI1RST"]
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SAI2RST"]
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SAI3RST"]
    #[inline(always)]
    pub fn sai3rst(&self) -> SAI3RST_R {
        SAI3RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DFSDMRST"]
    #[inline(always)]
    pub fn dfsdmrst(&self) -> DFSDMRST_R {
        DFSDMRST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FDCANRST"]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1RST"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W {
        TIM1RST_W { w: self }
    }
    #[doc = "Bit 1 - TIM8RST"]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W {
        TIM8RST_W { w: self }
    }
    #[doc = "Bit 2 - TIM15RST"]
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W {
        TIM15RST_W { w: self }
    }
    #[doc = "Bit 3 - TIM16RST"]
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W {
        TIM16RST_W { w: self }
    }
    #[doc = "Bit 4 - TIM17RST"]
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W {
        TIM17RST_W { w: self }
    }
    #[doc = "Bit 8 - SPI1RST"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W {
        SPI1RST_W { w: self }
    }
    #[doc = "Bit 9 - SPI4RST"]
    #[inline(always)]
    pub fn spi4rst(&mut self) -> SPI4RST_W {
        SPI4RST_W { w: self }
    }
    #[doc = "Bit 10 - SPI5RST"]
    #[inline(always)]
    pub fn spi5rst(&mut self) -> SPI5RST_W {
        SPI5RST_W { w: self }
    }
    #[doc = "Bit 13 - USART6RST"]
    #[inline(always)]
    pub fn usart6rst(&mut self) -> USART6RST_W {
        USART6RST_W { w: self }
    }
    #[doc = "Bit 16 - SAI1RST"]
    #[inline(always)]
    pub fn sai1rst(&mut self) -> SAI1RST_W {
        SAI1RST_W { w: self }
    }
    #[doc = "Bit 17 - SAI2RST"]
    #[inline(always)]
    pub fn sai2rst(&mut self) -> SAI2RST_W {
        SAI2RST_W { w: self }
    }
    #[doc = "Bit 18 - SAI3RST"]
    #[inline(always)]
    pub fn sai3rst(&mut self) -> SAI3RST_W {
        SAI3RST_W { w: self }
    }
    #[doc = "Bit 20 - DFSDMRST"]
    #[inline(always)]
    pub fn dfsdmrst(&mut self) -> DFSDMRST_W {
        DFSDMRST_W { w: self }
    }
    #[doc = "Bit 24 - FDCANRST"]
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FDCANRST_W {
        FDCANRST_W { w: self }
    }
}
