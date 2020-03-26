#[doc = "Reader of register RCC_MP_TZAHB6ENCLRR"]
pub type R = crate::R<u32, super::RCC_MP_TZAHB6ENCLRR>;
#[doc = "Writer for register RCC_MP_TZAHB6ENCLRR"]
pub type W = crate::W<u32, super::RCC_MP_TZAHB6ENCLRR>;
#[doc = "Register RCC_MP_TZAHB6ENCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_TZAHB6ENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MDMAEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDMAEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<MDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: MDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDMAEN`"]
pub type MDMAEN_R = crate::R<bool, MDMAEN_A>;
impl MDMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDMAEN_A {
        match self.bits {
            false => MDMAEN_A::B_0X0,
            true => MDMAEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MDMAEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MDMAEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `MDMAEN`"]
pub struct MDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MDMAEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MDMAEN_A::B_0X1)
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
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&mut self) -> MDMAEN_W {
        MDMAEN_W { w: self }
    }
}
