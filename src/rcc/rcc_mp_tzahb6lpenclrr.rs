#[doc = "Reader of register RCC_MP_TZAHB6LPENCLRR"]
pub type R = crate::R<u32, super::RCC_MP_TZAHB6LPENCLRR>;
#[doc = "Writer for register RCC_MP_TZAHB6LPENCLRR"]
pub type W = crate::W<u32, super::RCC_MP_TZAHB6LPENCLRR>;
#[doc = "Register RCC_MP_TZAHB6LPENCLRR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RCC_MP_TZAHB6LPENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "MDMALPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDMALPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<MDMALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: MDMALPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDMALPEN`"]
pub type MDMALPEN_R = crate::R<bool, MDMALPEN_A>;
impl MDMALPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDMALPEN_A {
        match self.bits {
            false => MDMALPEN_A::B_0X0,
            true => MDMALPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MDMALPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MDMALPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `MDMALPEN`"]
pub struct MDMALPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMALPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDMALPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MDMALPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MDMALPEN_A::B_0X1)
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
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W {
        MDMALPEN_W { w: self }
    }
}
