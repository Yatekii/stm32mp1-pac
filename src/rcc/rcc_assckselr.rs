#[doc = "Reader of register RCC_ASSCKSELR"]
pub type R = crate::R<u32, super::RCC_ASSCKSELR>;
#[doc = "Writer for register RCC_ASSCKSELR"]
pub type W = crate::W<u32, super::RCC_ASSCKSELR>;
#[doc = "Register RCC_ASSCKSELR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_ASSCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "AXISSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AXISSRC_A {
    #[doc = "0: HSI selected as system clock\r\n                  (hsi_ck) (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: HSE selected as system clock\r\n                  (hse_ck)"]
    B_0X1 = 1,
    #[doc = "2: PLL2 selected as system clock\r\n                  (pll2_p_ck)"]
    B_0X2 = 2,
}
impl From<AXISSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: AXISSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AXISSRC`"]
pub type AXISSRC_R = crate::R<u8, AXISSRC_A>;
impl AXISSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AXISSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AXISSRC_A::B_0X0),
            1 => Val(AXISSRC_A::B_0X1),
            2 => Val(AXISSRC_A::B_0X2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AXISSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AXISSRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AXISSRC_A::B_0X2
    }
}
#[doc = "Write proxy for field `AXISSRC`"]
pub struct AXISSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AXISSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXISSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSI selected as system clock (hsi_ck) (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AXISSRC_A::B_0X0)
    }
    #[doc = "HSE selected as system clock (hse_ck)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AXISSRC_A::B_0X1)
    }
    #[doc = "PLL2 selected as system clock (pll2_p_ck)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(AXISSRC_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "AXISSRCRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXISSRCRDY_A {
    #[doc = "0: The AXI sub-system switch is not\r\n                  ready or in positions higher than : no clock is\r\n                  generated on its output"]
    B_0X0 = 0,
    #[doc = "1: The AXI sub-system switch is ready:\r\n                  the clock switch is selecting the clock given by\r\n                  AXISSRC field. (default after\r\n                  reset)"]
    B_0X1 = 1,
}
impl From<AXISSRCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: AXISSRCRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXISSRCRDY`"]
pub type AXISSRCRDY_R = crate::R<bool, AXISSRCRDY_A>;
impl AXISSRCRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXISSRCRDY_A {
        match self.bits {
            false => AXISSRCRDY_A::B_0X0,
            true => AXISSRCRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AXISSRCRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AXISSRCRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:2 - AXISSRC"]
    #[inline(always)]
    pub fn axissrc(&self) -> AXISSRC_R {
        AXISSRC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - AXISSRCRDY"]
    #[inline(always)]
    pub fn axissrcrdy(&self) -> AXISSRCRDY_R {
        AXISSRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXISSRC"]
    #[inline(always)]
    pub fn axissrc(&mut self) -> AXISSRC_W {
        AXISSRC_W { w: self }
    }
}
