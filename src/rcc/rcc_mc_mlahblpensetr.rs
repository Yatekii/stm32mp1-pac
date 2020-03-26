#[doc = "Reader of register RCC_MC_MLAHBLPENSETR"]
pub type R = crate::R<u32, super::RCC_MC_MLAHBLPENSETR>;
#[doc = "Writer for register RCC_MC_MLAHBLPENSETR"]
pub type W = crate::W<u32, super::RCC_MC_MLAHBLPENSETR>;
#[doc = "Register RCC_MC_MLAHBLPENSETR `reset()`'s with value 0x17"]
impl crate::ResetValue for super::RCC_MC_MLAHBLPENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x17
    }
}
#[doc = "SRAM1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the memory interface clock is disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the memory interface\r\n                  clock in CSLEEP, reading means that the memory\r\n                  interface is enabled in CSLEEP"]
    B_0X1 = 1,
}
impl From<SRAM1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM1LPEN`"]
pub type SRAM1LPEN_R = crate::R<bool, SRAM1LPEN_A>;
impl SRAM1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM1LPEN_A {
        match self.bits {
            false => SRAM1LPEN_A::B_0X0,
            true => SRAM1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SRAM1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SRAM1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SRAM1LPEN`"]
pub struct SRAM1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the memory interface clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SRAM1LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the memory interface clock in CSLEEP, reading means that the memory interface is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SRAM1LPEN_A::B_0X1)
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
#[doc = "SRAM2LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM2LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the memory interface clock is disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the memory interface\r\n                  clock in CSLEEP, reading means that the memory\r\n                  interface is enabled in CSLEEP."]
    B_0X1 = 1,
}
impl From<SRAM2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM2LPEN`"]
pub type SRAM2LPEN_R = crate::R<bool, SRAM2LPEN_A>;
impl SRAM2LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM2LPEN_A {
        match self.bits {
            false => SRAM2LPEN_A::B_0X0,
            true => SRAM2LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SRAM2LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SRAM2LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SRAM2LPEN`"]
pub struct SRAM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the memory interface clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SRAM2LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the memory interface clock in CSLEEP, reading means that the memory interface is enabled in CSLEEP."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SRAM2LPEN_A::B_0X1)
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
#[doc = "SRAM3LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM3LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the memory interface clock is disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the memory interface\r\n                  clock in CSLEEP, reading means that the memory\r\n                  interface is enabled in CSLEEP"]
    B_0X1 = 1,
}
impl From<SRAM3LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM3LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAM3LPEN`"]
pub type SRAM3LPEN_R = crate::R<bool, SRAM3LPEN_A>;
impl SRAM3LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM3LPEN_A {
        match self.bits {
            false => SRAM3LPEN_A::B_0X0,
            true => SRAM3LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SRAM3LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SRAM3LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SRAM3LPEN`"]
pub struct SRAM3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM3LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the memory interface clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SRAM3LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the memory interface clock in CSLEEP, reading means that the memory interface is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SRAM3LPEN_A::B_0X1)
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
#[doc = "RETRAMLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RETRAMLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the memory interface clock is disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the memory interface\r\n                  clock in CSLEEP, reading means that the memory\r\n                  interface is enabled in CSLEEP"]
    B_0X1 = 1,
}
impl From<RETRAMLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RETRAMLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RETRAMLPEN`"]
pub type RETRAMLPEN_R = crate::R<bool, RETRAMLPEN_A>;
impl RETRAMLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RETRAMLPEN_A {
        match self.bits {
            false => RETRAMLPEN_A::B_0X0,
            true => RETRAMLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RETRAMLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RETRAMLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `RETRAMLPEN`"]
pub struct RETRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRAMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RETRAMLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the memory interface clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RETRAMLPEN_A::B_0X0)
    }
    #[doc = "Writing enables the memory interface clock in CSLEEP, reading means that the memory interface is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RETRAMLPEN_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - SRAM1LPEN"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM2LPEN"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> SRAM2LPEN_R {
        SRAM2LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRAM3LPEN"]
    #[inline(always)]
    pub fn sram3lpen(&self) -> SRAM3LPEN_R {
        SRAM3LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RETRAMLPEN"]
    #[inline(always)]
    pub fn retramlpen(&self) -> RETRAMLPEN_R {
        RETRAMLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM1LPEN"]
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W {
        SRAM1LPEN_W { w: self }
    }
    #[doc = "Bit 1 - SRAM2LPEN"]
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> SRAM2LPEN_W {
        SRAM2LPEN_W { w: self }
    }
    #[doc = "Bit 2 - SRAM3LPEN"]
    #[inline(always)]
    pub fn sram3lpen(&mut self) -> SRAM3LPEN_W {
        SRAM3LPEN_W { w: self }
    }
    #[doc = "Bit 4 - RETRAMLPEN"]
    #[inline(always)]
    pub fn retramlpen(&mut self) -> RETRAMLPEN_W {
        RETRAMLPEN_W { w: self }
    }
}
