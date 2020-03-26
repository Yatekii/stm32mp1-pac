#[doc = "Reader of register RCC_MC_AXIMLPENCLRR"]
pub type R = crate::R<u32, super::RCC_MC_AXIMLPENCLRR>;
#[doc = "Writer for register RCC_MC_AXIMLPENCLRR"]
pub type W = crate::W<u32, super::RCC_MC_AXIMLPENCLRR>;
#[doc = "Register RCC_MC_AXIMLPENCLRR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RCC_MC_AXIMLPENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "SYSRAMLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRAMLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the memory interface is disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the memory\r\n                  interface in CSLEEP, reading means that the\r\n                  memory interface is enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SYSRAMLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRAMLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSRAMLPEN`"]
pub type SYSRAMLPEN_R = crate::R<bool, SYSRAMLPEN_A>;
impl SYSRAMLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRAMLPEN_A {
        match self.bits {
            false => SYSRAMLPEN_A::B_0X0,
            true => SYSRAMLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSRAMLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSRAMLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SYSRAMLPEN`"]
pub struct SYSRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRAMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRAMLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the memory interface is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SYSRAMLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the memory interface in CSLEEP, reading means that the memory interface is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SYSRAMLPEN_A::B_0X1)
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
    #[doc = "Bit 0 - SYSRAMLPEN"]
    #[inline(always)]
    pub fn sysramlpen(&self) -> SYSRAMLPEN_R {
        SYSRAMLPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSRAMLPEN"]
    #[inline(always)]
    pub fn sysramlpen(&mut self) -> SYSRAMLPEN_W {
        SYSRAMLPEN_W { w: self }
    }
}
