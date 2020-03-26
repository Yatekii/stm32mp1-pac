#[doc = "Reader of register RCC_LPTIM45CKSELR"]
pub type R = crate::R<u32, super::RCC_LPTIM45CKSELR>;
#[doc = "Writer for register RCC_LPTIM45CKSELR"]
pub type W = crate::W<u32, super::RCC_LPTIM45CKSELR>;
#[doc = "Register RCC_LPTIM45CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_LPTIM45CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPTIM45SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM45SRC_A {
    #[doc = "0: pclk3 clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll4_p_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: pll3_q_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X2 = 2,
    #[doc = "3: lse_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X3 = 3,
    #[doc = "4: lsi_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X4 = 4,
    #[doc = "5: per_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X5 = 5,
}
impl From<LPTIM45SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM45SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPTIM45SRC`"]
pub type LPTIM45SRC_R = crate::R<u8, LPTIM45SRC_A>;
impl LPTIM45SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPTIM45SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPTIM45SRC_A::B_0X0),
            1 => Val(LPTIM45SRC_A::B_0X1),
            2 => Val(LPTIM45SRC_A::B_0X2),
            3 => Val(LPTIM45SRC_A::B_0X3),
            4 => Val(LPTIM45SRC_A::B_0X4),
            5 => Val(LPTIM45SRC_A::B_0X5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM45SRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM45SRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == LPTIM45SRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LPTIM45SRC_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == LPTIM45SRC_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == LPTIM45SRC_A::B_0X5
    }
}
#[doc = "Write proxy for field `LPTIM45SRC`"]
pub struct LPTIM45SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM45SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM45SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pclk3 clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM45SRC_A::B_0X0)
    }
    #[doc = "pll4_p_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM45SRC_A::B_0X1)
    }
    #[doc = "pll3_q_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LPTIM45SRC_A::B_0X2)
    }
    #[doc = "lse_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LPTIM45SRC_A::B_0X3)
    }
    #[doc = "lsi_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(LPTIM45SRC_A::B_0X4)
    }
    #[doc = "per_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(LPTIM45SRC_A::B_0X5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - LPTIM45SRC"]
    #[inline(always)]
    pub fn lptim45src(&self) -> LPTIM45SRC_R {
        LPTIM45SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPTIM45SRC"]
    #[inline(always)]
    pub fn lptim45src(&mut self) -> LPTIM45SRC_W {
        LPTIM45SRC_W { w: self }
    }
}
