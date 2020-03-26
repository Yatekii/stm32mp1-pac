#[doc = "Reader of register RCC_MPCKSELR"]
pub type R = crate::R<u32, super::RCC_MPCKSELR>;
#[doc = "Writer for register RCC_MPCKSELR"]
pub type W = crate::W<u32, super::RCC_MPCKSELR>;
#[doc = "Register RCC_MPCKSELR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_MPCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "MPUSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPUSRC_A {
    #[doc = "0: HSI selected as system clock\r\n                  (hsi_ck) (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: HSE selected as system clock\r\n                  (hse_ck)"]
    B_0X1 = 1,
    #[doc = "2: PLL1 selected as system clock\r\n                  (pll1_p_ck)"]
    B_0X2 = 2,
    #[doc = "3: PLL1 via MPUDIV is selected as\r\n                  system clock (pll1_p_ck / 2\r\n                  MPUDIV)."]
    B_0X3 = 3,
}
impl From<MPUSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: MPUSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MPUSRC`"]
pub type MPUSRC_R = crate::R<u8, MPUSRC_A>;
impl MPUSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSRC_A {
        match self.bits {
            0 => MPUSRC_A::B_0X0,
            1 => MPUSRC_A::B_0X1,
            2 => MPUSRC_A::B_0X2,
            3 => MPUSRC_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MPUSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MPUSRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MPUSRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MPUSRC_A::B_0X3
    }
}
#[doc = "Write proxy for field `MPUSRC`"]
pub struct MPUSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HSI selected as system clock (hsi_ck) (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MPUSRC_A::B_0X0)
    }
    #[doc = "HSE selected as system clock (hse_ck)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MPUSRC_A::B_0X1)
    }
    #[doc = "PLL1 selected as system clock (pll1_p_ck)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MPUSRC_A::B_0X2)
    }
    #[doc = "PLL1 via MPUDIV is selected as system clock (pll1_p_ck / 2 MPUDIV)."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MPUSRC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "MPUSRCRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUSRCRDY_A {
    #[doc = "0: The MPU switch is not ready: no\r\n                  clock is generated on its output"]
    B_0X0 = 0,
    #[doc = "1: The MPU switch is ready: the clock\r\n                  switch is selecting the clock given by MPUSRC\r\n                  field. (default after reset)"]
    B_0X1 = 1,
}
impl From<MPUSRCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MPUSRCRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUSRCRDY`"]
pub type MPUSRCRDY_R = crate::R<bool, MPUSRCRDY_A>;
impl MPUSRCRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUSRCRDY_A {
        match self.bits {
            false => MPUSRCRDY_A::B_0X0,
            true => MPUSRCRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MPUSRCRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MPUSRCRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:1 - MPUSRC"]
    #[inline(always)]
    pub fn mpusrc(&self) -> MPUSRC_R {
        MPUSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - MPUSRCRDY"]
    #[inline(always)]
    pub fn mpusrcrdy(&self) -> MPUSRCRDY_R {
        MPUSRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MPUSRC"]
    #[inline(always)]
    pub fn mpusrc(&mut self) -> MPUSRC_W {
        MPUSRC_W { w: self }
    }
}
