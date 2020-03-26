#[doc = "Reader of register RCC_LPTIM23CKSELR"]
pub type R = crate::R<u32, super::RCC_LPTIM23CKSELR>;
#[doc = "Writer for register RCC_LPTIM23CKSELR"]
pub type W = crate::W<u32, super::RCC_LPTIM23CKSELR>;
#[doc = "Register RCC_LPTIM23CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_LPTIM23CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPTIM23SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM23SRC_A {
    #[doc = "0: pclk3 clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll4_q_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: per_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X2 = 2,
    #[doc = "3: lse_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X3 = 3,
    #[doc = "4: lsi_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X4 = 4,
}
impl From<LPTIM23SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM23SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPTIM23SRC`"]
pub type LPTIM23SRC_R = crate::R<u8, LPTIM23SRC_A>;
impl LPTIM23SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPTIM23SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPTIM23SRC_A::B_0X0),
            1 => Val(LPTIM23SRC_A::B_0X1),
            2 => Val(LPTIM23SRC_A::B_0X2),
            3 => Val(LPTIM23SRC_A::B_0X3),
            4 => Val(LPTIM23SRC_A::B_0X4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM23SRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM23SRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == LPTIM23SRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LPTIM23SRC_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == LPTIM23SRC_A::B_0X4
    }
}
#[doc = "Write proxy for field `LPTIM23SRC`"]
pub struct LPTIM23SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM23SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM23SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pclk3 clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM23SRC_A::B_0X0)
    }
    #[doc = "pll4_q_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM23SRC_A::B_0X1)
    }
    #[doc = "per_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LPTIM23SRC_A::B_0X2)
    }
    #[doc = "lse_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LPTIM23SRC_A::B_0X3)
    }
    #[doc = "lsi_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(LPTIM23SRC_A::B_0X4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - LPTIM23SRC"]
    #[inline(always)]
    pub fn lptim23src(&self) -> LPTIM23SRC_R {
        LPTIM23SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPTIM23SRC"]
    #[inline(always)]
    pub fn lptim23src(&mut self) -> LPTIM23SRC_W {
        LPTIM23SRC_W { w: self }
    }
}