#[doc = "Reader of register RCC_APB5RSTSETR"]
pub type R = crate::R<u32, super::RCC_APB5RSTSETR>;
#[doc = "Writer for register RCC_APB5RSTSETR"]
pub type W = crate::W<u32, super::RCC_APB5RSTSETR>;
#[doc = "Register RCC_APB5RSTSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_APB5RSTSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SPI6RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI6RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SPI6RST_A> for bool {
    #[inline(always)]
    fn from(variant: SPI6RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPI6RST`"]
pub type SPI6RST_R = crate::R<bool, SPI6RST_A>;
impl SPI6RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI6RST_A {
        match self.bits {
            false => SPI6RST_A::B_0X0,
            true => SPI6RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI6RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI6RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SPI6RST`"]
pub struct SPI6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI6RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI6RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI6RST_A::B_0X1)
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
#[doc = "I2C4RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C4RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<I2C4RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C4RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C4RST`"]
pub type I2C4RST_R = crate::R<bool, I2C4RST_A>;
impl I2C4RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C4RST_A {
        match self.bits {
            false => I2C4RST_A::B_0X0,
            true => I2C4RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C4RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C4RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `I2C4RST`"]
pub struct I2C4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C4RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C4RST_A::B_0X1)
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
#[doc = "I2C6RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C6RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<I2C6RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2C6RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `I2C6RST`"]
pub type I2C6RST_R = crate::R<bool, I2C6RST_A>;
impl I2C6RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C6RST_A {
        match self.bits {
            false => I2C6RST_A::B_0X0,
            true => I2C6RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C6RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C6RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `I2C6RST`"]
pub struct I2C6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C6RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C6RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C6RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C6RST_A::B_0X1)
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
#[doc = "USART1RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<USART1RST_A> for bool {
    #[inline(always)]
    fn from(variant: USART1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USART1RST`"]
pub type USART1RST_R = crate::R<bool, USART1RST_A>;
impl USART1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1RST_A {
        match self.bits {
            false => USART1RST_A::B_0X0,
            true => USART1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USART1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USART1RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `USART1RST`"]
pub struct USART1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USART1RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USART1RST_A::B_0X1)
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
#[doc = "STGENRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STGENRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<STGENRST_A> for bool {
    #[inline(always)]
    fn from(variant: STGENRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STGENRST`"]
pub type STGENRST_R = crate::R<bool, STGENRST_A>;
impl STGENRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STGENRST_A {
        match self.bits {
            false => STGENRST_A::B_0X0,
            true => STGENRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STGENRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STGENRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `STGENRST`"]
pub struct STGENRST_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STGENRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(STGENRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(STGENRST_A::B_0X1)
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
    #[doc = "Bit 0 - SPI6RST"]
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C4RST"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C6RST"]
    #[inline(always)]
    pub fn i2c6rst(&self) -> I2C6RST_R {
        I2C6RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENRST"]
    #[inline(always)]
    pub fn stgenrst(&self) -> STGENRST_R {
        STGENRST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6RST"]
    #[inline(always)]
    pub fn spi6rst(&mut self) -> SPI6RST_W {
        SPI6RST_W { w: self }
    }
    #[doc = "Bit 2 - I2C4RST"]
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W {
        I2C4RST_W { w: self }
    }
    #[doc = "Bit 3 - I2C6RST"]
    #[inline(always)]
    pub fn i2c6rst(&mut self) -> I2C6RST_W {
        I2C6RST_W { w: self }
    }
    #[doc = "Bit 4 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W {
        USART1RST_W { w: self }
    }
    #[doc = "Bit 20 - STGENRST"]
    #[inline(always)]
    pub fn stgenrst(&mut self) -> STGENRST_W {
        STGENRST_W { w: self }
    }
}
