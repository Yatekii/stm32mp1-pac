#[doc = "Reader of register RCC_AHB4RSTSETR"]
pub type R = crate::R<u32, super::RCC_AHB4RSTSETR>;
#[doc = "Writer for register RCC_AHB4RSTSETR"]
pub type W = crate::W<u32, super::RCC_AHB4RSTSETR>;
#[doc = "Register RCC_AHB4RSTSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_AHB4RSTSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIOARST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOARST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIOARST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOARST`"]
pub type GPIOARST_R = crate::R<bool, GPIOARST_A>;
impl GPIOARST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOARST_A {
        match self.bits {
            false => GPIOARST_A::B_0X0,
            true => GPIOARST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOARST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOARST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOARST`"]
pub struct GPIOARST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOARST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOARST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOARST_A::B_0X1)
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
#[doc = "GPIOBRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOBRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIOBRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOBRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOBRST`"]
pub type GPIOBRST_R = crate::R<bool, GPIOBRST_A>;
impl GPIOBRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOBRST_A {
        match self.bits {
            false => GPIOBRST_A::B_0X0,
            true => GPIOBRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOBRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOBRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOBRST`"]
pub struct GPIOBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOBRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOBRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOBRST_A::B_0X1)
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
#[doc = "GPIOCRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOCRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIOCRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOCRST`"]
pub type GPIOCRST_R = crate::R<bool, GPIOCRST_A>;
impl GPIOCRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOCRST_A {
        match self.bits {
            false => GPIOCRST_A::B_0X0,
            true => GPIOCRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOCRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOCRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOCRST`"]
pub struct GPIOCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOCRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOCRST_A::B_0X1)
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
#[doc = "GPIODRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIODRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIODRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIODRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIODRST`"]
pub type GPIODRST_R = crate::R<bool, GPIODRST_A>;
impl GPIODRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIODRST_A {
        match self.bits {
            false => GPIODRST_A::B_0X0,
            true => GPIODRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIODRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIODRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIODRST`"]
pub struct GPIODRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIODRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIODRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIODRST_A::B_0X1)
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
#[doc = "GPIOERST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOERST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIOERST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOERST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOERST`"]
pub type GPIOERST_R = crate::R<bool, GPIOERST_A>;
impl GPIOERST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOERST_A {
        match self.bits {
            false => GPIOERST_A::B_0X0,
            true => GPIOERST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOERST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOERST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOERST`"]
pub struct GPIOERST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOERST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOERST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOERST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOERST_A::B_0X1)
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
#[doc = "GPIOFRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOFRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIOFRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOFRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOFRST`"]
pub type GPIOFRST_R = crate::R<bool, GPIOFRST_A>;
impl GPIOFRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOFRST_A {
        match self.bits {
            false => GPIOFRST_A::B_0X0,
            true => GPIOFRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOFRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOFRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOFRST`"]
pub struct GPIOFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOFRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOFRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOFRST_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "GPIOGRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOGRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIOGRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOGRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOGRST`"]
pub type GPIOGRST_R = crate::R<bool, GPIOGRST_A>;
impl GPIOGRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOGRST_A {
        match self.bits {
            false => GPIOGRST_A::B_0X0,
            true => GPIOGRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOGRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOGRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOGRST`"]
pub struct GPIOGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOGRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOGRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOGRST_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "GPIOHRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOHRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIOHRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOHRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOHRST`"]
pub type GPIOHRST_R = crate::R<bool, GPIOHRST_A>;
impl GPIOHRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOHRST_A {
        match self.bits {
            false => GPIOHRST_A::B_0X0,
            true => GPIOHRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOHRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOHRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOHRST`"]
pub struct GPIOHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOHRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOHRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOHRST_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "GPIOIRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOIRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIOIRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOIRST`"]
pub type GPIOIRST_R = crate::R<bool, GPIOIRST_A>;
impl GPIOIRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOIRST_A {
        match self.bits {
            false => GPIOIRST_A::B_0X0,
            true => GPIOIRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOIRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOIRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOIRST`"]
pub struct GPIOIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOIRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOIRST_A::B_0X1)
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
#[doc = "GPIOJRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOJRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIOJRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOJRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOJRST`"]
pub type GPIOJRST_R = crate::R<bool, GPIOJRST_A>;
impl GPIOJRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOJRST_A {
        match self.bits {
            false => GPIOJRST_A::B_0X0,
            true => GPIOJRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOJRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOJRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOJRST`"]
pub struct GPIOJRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOJRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOJRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOJRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOJRST_A::B_0X1)
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
#[doc = "GPIOKRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOKRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIOKRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOKRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOKRST`"]
pub type GPIOKRST_R = crate::R<bool, GPIOKRST_A>;
impl GPIOKRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOKRST_A {
        match self.bits {
            false => GPIOKRST_A::B_0X0,
            true => GPIOKRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOKRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOKRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOKRST`"]
pub struct GPIOKRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOKRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOKRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOKRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOKRST_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - GPIOARST"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIOBRST"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIOCRST"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIODRST"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIOERST"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIOFRST"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIOGRST"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIOHRST"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIOIRST"]
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIOJRST"]
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIOKRST"]
    #[inline(always)]
    pub fn gpiokrst(&self) -> GPIOKRST_R {
        GPIOKRST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOARST"]
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W {
        GPIOARST_W { w: self }
    }
    #[doc = "Bit 1 - GPIOBRST"]
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W {
        GPIOBRST_W { w: self }
    }
    #[doc = "Bit 2 - GPIOCRST"]
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W {
        GPIOCRST_W { w: self }
    }
    #[doc = "Bit 3 - GPIODRST"]
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W {
        GPIODRST_W { w: self }
    }
    #[doc = "Bit 4 - GPIOERST"]
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W {
        GPIOERST_W { w: self }
    }
    #[doc = "Bit 5 - GPIOFRST"]
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W {
        GPIOFRST_W { w: self }
    }
    #[doc = "Bit 6 - GPIOGRST"]
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W {
        GPIOGRST_W { w: self }
    }
    #[doc = "Bit 7 - GPIOHRST"]
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W {
        GPIOHRST_W { w: self }
    }
    #[doc = "Bit 8 - GPIOIRST"]
    #[inline(always)]
    pub fn gpioirst(&mut self) -> GPIOIRST_W {
        GPIOIRST_W { w: self }
    }
    #[doc = "Bit 9 - GPIOJRST"]
    #[inline(always)]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W {
        GPIOJRST_W { w: self }
    }
    #[doc = "Bit 10 - GPIOKRST"]
    #[inline(always)]
    pub fn gpiokrst(&mut self) -> GPIOKRST_W {
        GPIOKRST_W { w: self }
    }
}
