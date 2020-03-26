#[doc = "Reader of register RCC_SPDIFCKSELR"]
pub type R = crate::R<u32, super::RCC_SPDIFCKSELR>;
#[doc = "Writer for register RCC_SPDIFCKSELR"]
pub type W = crate::W<u32, super::RCC_SPDIFCKSELR>;
#[doc = "Register RCC_SPDIFCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_SPDIFCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SPDIFSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPDIFSRC_A {
    #[doc = "0: pll4_p_ck clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll3_q_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: hsi_ker_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X2 = 2,
}
impl From<SPDIFSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDIFSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPDIFSRC`"]
pub type SPDIFSRC_R = crate::R<u8, SPDIFSRC_A>;
impl SPDIFSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPDIFSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPDIFSRC_A::B_0X0),
            1 => Val(SPDIFSRC_A::B_0X1),
            2 => Val(SPDIFSRC_A::B_0X2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPDIFSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPDIFSRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SPDIFSRC_A::B_0X2
    }
}
#[doc = "Write proxy for field `SPDIFSRC`"]
pub struct SPDIFSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDIFSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll4_p_ck clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPDIFSRC_A::B_0X0)
    }
    #[doc = "pll3_q_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPDIFSRC_A::B_0X1)
    }
    #[doc = "hsi_ker_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SPDIFSRC_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SPDIFSRC"]
    #[inline(always)]
    pub fn spdifsrc(&self) -> SPDIFSRC_R {
        SPDIFSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPDIFSRC"]
    #[inline(always)]
    pub fn spdifsrc(&mut self) -> SPDIFSRC_W {
        SPDIFSRC_W { w: self }
    }
}
