#[doc = "Reader of register RCC_TZAHB6RSTSETR"]
pub type R = crate::R<u32, super::RCC_TZAHB6RSTSETR>;
#[doc = "Writer for register RCC_TZAHB6RSTSETR"]
pub type W = crate::W<u32, super::RCC_TZAHB6RSTSETR>;
#[doc = "Register RCC_TZAHB6RSTSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_TZAHB6RSTSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MDMARST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDMARST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<MDMARST_A> for bool {
    #[inline(always)]
    fn from(variant: MDMARST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDMARST`"]
pub type MDMARST_R = crate::R<bool, MDMARST_A>;
impl MDMARST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDMARST_A {
        match self.bits {
            false => MDMARST_A::B_0X0,
            true => MDMARST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MDMARST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MDMARST_A::B_0X1
    }
}
#[doc = "Write proxy for field `MDMARST`"]
pub struct MDMARST_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMARST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDMARST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MDMARST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MDMARST_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - MDMARST"]
    #[inline(always)]
    pub fn mdmarst(&self) -> MDMARST_R {
        MDMARST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMARST"]
    #[inline(always)]
    pub fn mdmarst(&mut self) -> MDMARST_W {
        MDMARST_W { w: self }
    }
}
