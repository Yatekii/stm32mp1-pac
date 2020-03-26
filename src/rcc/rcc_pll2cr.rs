#[doc = "Reader of register RCC_PLL2CR"]
pub type R = crate::R<u32, super::RCC_PLL2CR>;
#[doc = "Writer for register RCC_PLL2CR"]
pub type W = crate::W<u32, super::RCC_PLL2CR>;
#[doc = "Register RCC_PLL2CR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_PLL2CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PLLON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLON_A {
    #[doc = "0: PLL2 OFF (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: PLL2 ON, and ref2_ck is provided to\r\n                  the PLL2"]
    B_0X1 = 1,
}
impl From<PLLON_A> for bool {
    #[inline(always)]
    fn from(variant: PLLON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLON`"]
pub type PLLON_R = crate::R<bool, PLLON_A>;
impl PLLON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLON_A {
        match self.bits {
            false => PLLON_A::B_0X0,
            true => PLLON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLLON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLLON_A::B_0X1
    }
}
#[doc = "Write proxy for field `PLLON`"]
pub struct PLLON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL2 OFF (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLLON_A::B_0X0)
    }
    #[doc = "PLL2 ON, and ref2_ck is provided to the PLL2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLLON_A::B_0X1)
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
#[doc = "PLL2RDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL2RDY_A {
    #[doc = "0: PLL2 unlocked (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: PLL2 locked"]
    B_0X1 = 1,
}
impl From<PLL2RDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL2RDY`"]
pub type PLL2RDY_R = crate::R<bool, PLL2RDY_A>;
impl PLL2RDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL2RDY_A {
        match self.bits {
            false => PLL2RDY_A::B_0X0,
            true => PLL2RDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL2RDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL2RDY_A::B_0X1
    }
}
#[doc = "SSCG_CTRL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSCG_CTRL_A {
    #[doc = "0: Clock Spreading Generator disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Clock Spreading Generator\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<SSCG_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: SSCG_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SSCG_CTRL`"]
pub type SSCG_CTRL_R = crate::R<bool, SSCG_CTRL_A>;
impl SSCG_CTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSCG_CTRL_A {
        match self.bits {
            false => SSCG_CTRL_A::B_0X0,
            true => SSCG_CTRL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SSCG_CTRL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SSCG_CTRL_A::B_0X1
    }
}
#[doc = "Write proxy for field `SSCG_CTRL`"]
pub struct SSCG_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SSCG_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSCG_CTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock Spreading Generator disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SSCG_CTRL_A::B_0X0)
    }
    #[doc = "Clock Spreading Generator enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SSCG_CTRL_A::B_0X1)
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
#[doc = "DIVPEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVPEN_A {
    #[doc = "0: pll2_p_ck output is disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: pll2_p_ck output is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<DIVPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DIVPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIVPEN`"]
pub type DIVPEN_R = crate::R<bool, DIVPEN_A>;
impl DIVPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVPEN_A {
        match self.bits {
            false => DIVPEN_A::B_0X0,
            true => DIVPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DIVPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DIVPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DIVPEN`"]
pub struct DIVPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "pll2_p_ck output is disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIVPEN_A::B_0X0)
    }
    #[doc = "pll2_p_ck output is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIVPEN_A::B_0X1)
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
#[doc = "DIVQEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVQEN_A {
    #[doc = "0: pll2_q_ck output is disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: pll2_q_ck output is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<DIVQEN_A> for bool {
    #[inline(always)]
    fn from(variant: DIVQEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIVQEN`"]
pub type DIVQEN_R = crate::R<bool, DIVQEN_A>;
impl DIVQEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVQEN_A {
        match self.bits {
            false => DIVQEN_A::B_0X0,
            true => DIVQEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DIVQEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DIVQEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DIVQEN`"]
pub struct DIVQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVQEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "pll2_q_ck output is disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIVQEN_A::B_0X0)
    }
    #[doc = "pll2_q_ck output is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIVQEN_A::B_0X1)
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
#[doc = "DIVREN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVREN_A {
    #[doc = "0: pll2_r_ck output is disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: pll2_r_ck output is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<DIVREN_A> for bool {
    #[inline(always)]
    fn from(variant: DIVREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIVREN`"]
pub type DIVREN_R = crate::R<bool, DIVREN_A>;
impl DIVREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVREN_A {
        match self.bits {
            false => DIVREN_A::B_0X0,
            true => DIVREN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DIVREN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DIVREN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DIVREN`"]
pub struct DIVREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "pll2_r_ck output is disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIVREN_A::B_0X0)
    }
    #[doc = "pll2_r_ck output is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIVREN_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - PLLON"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLL2RDY"]
    #[inline(always)]
    pub fn pll2rdy(&self) -> PLL2RDY_R {
        PLL2RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSCG_CTRL"]
    #[inline(always)]
    pub fn sscg_ctrl(&self) -> SSCG_CTRL_R {
        SSCG_CTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DIVPEN"]
    #[inline(always)]
    pub fn divpen(&self) -> DIVPEN_R {
        DIVPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DIVQEN"]
    #[inline(always)]
    pub fn divqen(&self) -> DIVQEN_R {
        DIVQEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DIVREN"]
    #[inline(always)]
    pub fn divren(&self) -> DIVREN_R {
        DIVREN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLLON"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PLLON_W {
        PLLON_W { w: self }
    }
    #[doc = "Bit 2 - SSCG_CTRL"]
    #[inline(always)]
    pub fn sscg_ctrl(&mut self) -> SSCG_CTRL_W {
        SSCG_CTRL_W { w: self }
    }
    #[doc = "Bit 4 - DIVPEN"]
    #[inline(always)]
    pub fn divpen(&mut self) -> DIVPEN_W {
        DIVPEN_W { w: self }
    }
    #[doc = "Bit 5 - DIVQEN"]
    #[inline(always)]
    pub fn divqen(&mut self) -> DIVQEN_W {
        DIVQEN_W { w: self }
    }
    #[doc = "Bit 6 - DIVREN"]
    #[inline(always)]
    pub fn divren(&mut self) -> DIVREN_W {
        DIVREN_W { w: self }
    }
}
