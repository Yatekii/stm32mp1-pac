#[doc = "Reader of register RCC_MSSCKSELR"]
pub type R = crate::R<u32, super::RCC_MSSCKSELR>;
#[doc = "Writer for register RCC_MSSCKSELR"]
pub type W = crate::W<u32, super::RCC_MSSCKSELR>;
#[doc = "Register RCC_MSSCKSELR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_MSSCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "MCUSSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCUSSRC_A {
    #[doc = "0: HSI selected as system clock\r\n                  (hsi_ck) (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: HSE selected as system clock\r\n                  (hse_ck)"]
    B_0X1 = 1,
    #[doc = "2: CSI selected as system clock\r\n                  (csi_ck)"]
    B_0X2 = 2,
    #[doc = "3: PLL3 selected as system clock\r\n                  (pll3_p_ck)"]
    B_0X3 = 3,
}
impl From<MCUSSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: MCUSSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCUSSRC`"]
pub type MCUSSRC_R = crate::R<u8, MCUSSRC_A>;
impl MCUSSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCUSSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MCUSSRC_A::B_0X0),
            1 => Val(MCUSSRC_A::B_0X1),
            2 => Val(MCUSSRC_A::B_0X2),
            3 => Val(MCUSSRC_A::B_0X3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCUSSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCUSSRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MCUSSRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MCUSSRC_A::B_0X3
    }
}
#[doc = "Write proxy for field `MCUSSRC`"]
pub struct MCUSSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUSSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCUSSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSI selected as system clock (hsi_ck) (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCUSSRC_A::B_0X0)
    }
    #[doc = "HSE selected as system clock (hse_ck)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCUSSRC_A::B_0X1)
    }
    #[doc = "CSI selected as system clock (csi_ck)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MCUSSRC_A::B_0X2)
    }
    #[doc = "PLL3 selected as system clock (pll3_p_ck)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MCUSSRC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "MCUSSRCRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCUSSRCRDY_A {
    #[doc = "0: The MCU sub-system switch is not\r\n                  ready or in positions higher than : no clock is\r\n                  generated on its output"]
    B_0X0 = 0,
    #[doc = "1: The MCU sub-system switch is ready:\r\n                  the clock switch is selecting the clock given by\r\n                  MCUSSRC field. (default after\r\n                  reset)"]
    B_0X1 = 1,
}
impl From<MCUSSRCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MCUSSRCRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCUSSRCRDY`"]
pub type MCUSSRCRDY_R = crate::R<bool, MCUSSRCRDY_A>;
impl MCUSSRCRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCUSSRCRDY_A {
        match self.bits {
            false => MCUSSRCRDY_A::B_0X0,
            true => MCUSSRCRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCUSSRCRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCUSSRCRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:2 - MCUSSRC"]
    #[inline(always)]
    pub fn mcussrc(&self) -> MCUSSRC_R {
        MCUSSRC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - MCUSSRCRDY"]
    #[inline(always)]
    pub fn mcussrcrdy(&self) -> MCUSSRCRDY_R {
        MCUSSRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCUSSRC"]
    #[inline(always)]
    pub fn mcussrc(&mut self) -> MCUSSRC_W {
        MCUSSRC_W { w: self }
    }
}
