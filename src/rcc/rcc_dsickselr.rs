#[doc = "Reader of register RCC_DSICKSELR"]
pub type R = crate::R<u32, super::RCC_DSICKSELR>;
#[doc = "Writer for register RCC_DSICKSELR"]
pub type W = crate::W<u32, super::RCC_DSICKSELR>;
#[doc = "Register RCC_DSICKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_DSICKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DSISRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSISRC_A {
    #[doc = "0: DSI clock from PHY is selected as\r\n                  DSI byte lane clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll4_p_ck clock selected as DSI byte\r\n                  lane clock"]
    B_0X1 = 1,
}
impl From<DSISRC_A> for bool {
    #[inline(always)]
    fn from(variant: DSISRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSISRC`"]
pub type DSISRC_R = crate::R<bool, DSISRC_A>;
impl DSISRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSISRC_A {
        match self.bits {
            false => DSISRC_A::B_0X0,
            true => DSISRC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DSISRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DSISRC_A::B_0X1
    }
}
#[doc = "Write proxy for field `DSISRC`"]
pub struct DSISRC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSISRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSISRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DSI clock from PHY is selected as DSI byte lane clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DSISRC_A::B_0X0)
    }
    #[doc = "pll4_p_ck clock selected as DSI byte lane clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DSISRC_A::B_0X1)
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
    #[doc = "Bit 0 - DSISRC"]
    #[inline(always)]
    pub fn dsisrc(&self) -> DSISRC_R {
        DSISRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSISRC"]
    #[inline(always)]
    pub fn dsisrc(&mut self) -> DSISRC_W {
        DSISRC_W { w: self }
    }
}
