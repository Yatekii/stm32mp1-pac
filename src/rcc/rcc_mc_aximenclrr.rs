#[doc = "Reader of register RCC_MC_AXIMENCLRR"]
pub type R = crate::R<u32, super::RCC_MC_AXIMENCLRR>;
#[doc = "Writer for register RCC_MC_AXIMENCLRR"]
pub type W = crate::W<u32, super::RCC_MC_AXIMENCLRR>;
#[doc = "Register RCC_MC_AXIMENCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MC_AXIMENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SYSRAMEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAMEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the memory is not allocated by the\r\n                  MCU"]
    B_0X0 = 0,
    #[doc = "1: Writing deallocates the memory to\r\n                  the MCU, reading means that the memory is\r\n                  allocated to the MCU."]
    B_0X1 = 1,
}
impl From<SYSRAMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSRAMEN`"]
pub type SYSRAMEN_R = crate::R<bool, SYSRAMEN_A>;
impl SYSRAMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAMEN_A {
        match self.bits {
            false => SYSRAMEN_A::B_0X0,
            true => SYSRAMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSRAMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSRAMEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SYSRAMEN`"]
pub struct SYSRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the memory is not allocated by the MCU"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SYSRAMEN_A::B_0X0)
    }
    #[doc = "Writing deallocates the memory to the MCU, reading means that the memory is allocated to the MCU."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SYSRAMEN_A::B_0X1)
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
    #[doc = "Bit 0 - SYSRAMEN"]
    #[inline(always)]
    pub fn sysramen(&self) -> SYSRAMEN_R {
        SYSRAMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSRAMEN"]
    #[inline(always)]
    pub fn sysramen(&mut self) -> SYSRAMEN_W {
        SYSRAMEN_W { w: self }
    }
}
