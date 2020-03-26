#[doc = "Reader of register RCC_AHB5RSTSETR"]
pub type R = crate::R<u32, super::RCC_AHB5RSTSETR>;
#[doc = "Writer for register RCC_AHB5RSTSETR"]
pub type W = crate::W<u32, super::RCC_AHB5RSTSETR>;
#[doc = "Register RCC_AHB5RSTSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_AHB5RSTSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIOZRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOZRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<GPIOZRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOZRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOZRST`"]
pub type GPIOZRST_R = crate::R<bool, GPIOZRST_A>;
impl GPIOZRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOZRST_A {
        match self.bits {
            false => GPIOZRST_A::B_0X0,
            true => GPIOZRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOZRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOZRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOZRST`"]
pub struct GPIOZRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOZRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOZRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOZRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOZRST_A::B_0X1)
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
#[doc = "CRYP1RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYP1RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<CRYP1RST_A> for bool {
    #[inline(always)]
    fn from(variant: CRYP1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYP1RST`"]
pub type CRYP1RST_R = crate::R<bool, CRYP1RST_A>;
impl CRYP1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYP1RST_A {
        match self.bits {
            false => CRYP1RST_A::B_0X0,
            true => CRYP1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRYP1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRYP1RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `CRYP1RST`"]
pub struct CRYP1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYP1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRYP1RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRYP1RST_A::B_0X1)
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
#[doc = "HASH1RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH1RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<HASH1RST_A> for bool {
    #[inline(always)]
    fn from(variant: HASH1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HASH1RST`"]
pub type HASH1RST_R = crate::R<bool, HASH1RST_A>;
impl HASH1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH1RST_A {
        match self.bits {
            false => HASH1RST_A::B_0X0,
            true => HASH1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HASH1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HASH1RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `HASH1RST`"]
pub struct HASH1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HASH1RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HASH1RST_A::B_0X1)
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
#[doc = "RNG1RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG1RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<RNG1RST_A> for bool {
    #[inline(always)]
    fn from(variant: RNG1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNG1RST`"]
pub type RNG1RST_R = crate::R<bool, RNG1RST_A>;
impl RNG1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG1RST_A {
        match self.bits {
            false => RNG1RST_A::B_0X0,
            true => RNG1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RNG1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RNG1RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `RNG1RST`"]
pub struct RNG1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RNG1RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RNG1RST_A::B_0X1)
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
#[doc = "AXIMCRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXIMCRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<AXIMCRST_A> for bool {
    #[inline(always)]
    fn from(variant: AXIMCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXIMCRST`"]
pub type AXIMCRST_R = crate::R<bool, AXIMCRST_A>;
impl AXIMCRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXIMCRST_A {
        match self.bits {
            false => AXIMCRST_A::B_0X0,
            true => AXIMCRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AXIMCRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AXIMCRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `AXIMCRST`"]
pub struct AXIMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIMCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXIMCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AXIMCRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AXIMCRST_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - GPIOZRST"]
    #[inline(always)]
    pub fn gpiozrst(&self) -> GPIOZRST_R {
        GPIOZRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP1RST"]
    #[inline(always)]
    pub fn cryp1rst(&self) -> CRYP1RST_R {
        CRYP1RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH1RST"]
    #[inline(always)]
    pub fn hash1rst(&self) -> HASH1RST_R {
        HASH1RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG1RST"]
    #[inline(always)]
    pub fn rng1rst(&self) -> RNG1RST_R {
        RNG1RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AXIMCRST"]
    #[inline(always)]
    pub fn aximcrst(&self) -> AXIMCRST_R {
        AXIMCRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZRST"]
    #[inline(always)]
    pub fn gpiozrst(&mut self) -> GPIOZRST_W {
        GPIOZRST_W { w: self }
    }
    #[doc = "Bit 4 - CRYP1RST"]
    #[inline(always)]
    pub fn cryp1rst(&mut self) -> CRYP1RST_W {
        CRYP1RST_W { w: self }
    }
    #[doc = "Bit 5 - HASH1RST"]
    #[inline(always)]
    pub fn hash1rst(&mut self) -> HASH1RST_W {
        HASH1RST_W { w: self }
    }
    #[doc = "Bit 6 - RNG1RST"]
    #[inline(always)]
    pub fn rng1rst(&mut self) -> RNG1RST_W {
        RNG1RST_W { w: self }
    }
    #[doc = "Bit 16 - AXIMCRST"]
    #[inline(always)]
    pub fn aximcrst(&mut self) -> AXIMCRST_W {
        AXIMCRST_W { w: self }
    }
}
