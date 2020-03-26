#[doc = "Reader of register RCC_TIMG2PRER"]
pub type R = crate::R<u32, super::RCC_TIMG2PRER>;
#[doc = "Writer for register RCC_TIMG2PRER"]
pub type W = crate::W<u32, super::RCC_TIMG2PRER>;
#[doc = "Register RCC_TIMG2PRER `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_TIMG2PRER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "TIMG2PRE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMG2PRE_A {
    #[doc = "0: The Timers kernel clock is equal to\r\n                  mlhclk if APB2DIV is corresponding to a division\r\n                  by 1 or 2, else it is equal to 2 x Fpclk2\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: The Timers kernel clock is equal to\r\n                  mlhclk if APB2DIV is corresponding to division by\r\n                  1, 2 or 4, else it is equal to 4 x\r\n                  Fpclk2"]
    B_0X1 = 1,
}
impl From<TIMG2PRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMG2PRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMG2PRE`"]
pub type TIMG2PRE_R = crate::R<bool, TIMG2PRE_A>;
impl TIMG2PRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMG2PRE_A {
        match self.bits {
            false => TIMG2PRE_A::B_0X0,
            true => TIMG2PRE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIMG2PRE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIMG2PRE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIMG2PRE`"]
pub struct TIMG2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMG2PRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMG2PRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Timers kernel clock is equal to mlhclk if APB2DIV is corresponding to a division by 1 or 2, else it is equal to 2 x Fpclk2 (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIMG2PRE_A::B_0X0)
    }
    #[doc = "The Timers kernel clock is equal to mlhclk if APB2DIV is corresponding to division by 1, 2 or 4, else it is equal to 4 x Fpclk2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIMG2PRE_A::B_0X1)
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
#[doc = "TIMG2PRERDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMG2PRERDY_A {
    #[doc = "0: The new division factor is not yet\r\n                  taken into account."]
    B_0X0 = 0,
    #[doc = "1: The new division factor is taken\r\n                  into account. (default after reset)"]
    B_0X1 = 1,
}
impl From<TIMG2PRERDY_A> for bool {
    #[inline(always)]
    fn from(variant: TIMG2PRERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMG2PRERDY`"]
pub type TIMG2PRERDY_R = crate::R<bool, TIMG2PRERDY_A>;
impl TIMG2PRERDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMG2PRERDY_A {
        match self.bits {
            false => TIMG2PRERDY_A::B_0X0,
            true => TIMG2PRERDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIMG2PRERDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIMG2PRERDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TIMG2PRE"]
    #[inline(always)]
    pub fn timg2pre(&self) -> TIMG2PRE_R {
        TIMG2PRE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - TIMG2PRERDY"]
    #[inline(always)]
    pub fn timg2prerdy(&self) -> TIMG2PRERDY_R {
        TIMG2PRERDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIMG2PRE"]
    #[inline(always)]
    pub fn timg2pre(&mut self) -> TIMG2PRE_W {
        TIMG2PRE_W { w: self }
    }
}
