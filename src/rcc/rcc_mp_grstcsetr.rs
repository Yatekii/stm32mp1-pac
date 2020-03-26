#[doc = "Reader of register RCC_MP_GRSTCSETR"]
pub type R = crate::R<u32, super::RCC_MP_GRSTCSETR>;
#[doc = "Writer for register RCC_MP_GRSTCSETR"]
pub type W = crate::W<u32, super::RCC_MP_GRSTCSETR>;
#[doc = "Register RCC_MP_GRSTCSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_GRSTCSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MPSYSRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPSYSRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing generate a system reset, see\r\n                  Figure2."]
    B_0X1 = 1,
}
impl From<MPSYSRST_A> for bool {
    #[inline(always)]
    fn from(variant: MPSYSRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPSYSRST`"]
pub type MPSYSRST_R = crate::R<bool, MPSYSRST_A>;
impl MPSYSRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPSYSRST_A {
        match self.bits {
            false => MPSYSRST_A::B_0X0,
            true => MPSYSRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MPSYSRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MPSYSRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `MPSYSRST`"]
pub struct MPSYSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MPSYSRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPSYSRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MPSYSRST_A::B_0X0)
    }
    #[doc = "Writing generate a system reset, see Figure2."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MPSYSRST_A::B_0X1)
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
#[doc = "MCURST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCURST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing generate a reset of the MCU\r\n                  core, reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<MCURST_A> for bool {
    #[inline(always)]
    fn from(variant: MCURST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCURST`"]
pub type MCURST_R = crate::R<bool, MCURST_A>;
impl MCURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCURST_A {
        match self.bits {
            false => MCURST_A::B_0X0,
            true => MCURST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCURST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCURST_A::B_0X1
    }
}
#[doc = "Write proxy for field `MCURST`"]
pub struct MCURST_W<'a> {
    w: &'a mut W,
}
impl<'a> MCURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCURST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCURST_A::B_0X0)
    }
    #[doc = "Writing generate a reset of the MCU core, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCURST_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - MPSYSRST"]
    #[inline(always)]
    pub fn mpsysrst(&self) -> MPSYSRST_R {
        MPSYSRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCURST"]
    #[inline(always)]
    pub fn mcurst(&self) -> MCURST_R {
        MCURST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPSYSRST"]
    #[inline(always)]
    pub fn mpsysrst(&mut self) -> MPSYSRST_W {
        MPSYSRST_W { w: self }
    }
    #[doc = "Bit 1 - MCURST"]
    #[inline(always)]
    pub fn mcurst(&mut self) -> MCURST_W {
        MCURST_W { w: self }
    }
}
