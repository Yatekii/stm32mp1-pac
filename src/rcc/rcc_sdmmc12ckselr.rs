#[doc = "Reader of register RCC_SDMMC12CKSELR"]
pub type R = crate::R<u32, super::RCC_SDMMC12CKSELR>;
#[doc = "Writer for register RCC_SDMMC12CKSELR"]
pub type W = crate::W<u32, super::RCC_SDMMC12CKSELR>;
#[doc = "Register RCC_SDMMC12CKSELR `reset()`'s with value 0x03"]
impl crate::ResetValue for super::RCC_SDMMC12CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "SDMMC12SRC\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDMMC12SRC_A {
    #[doc = "0: hclk6 clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll3_r_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: pll4_p_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X2 = 2,
    #[doc = "3: hsi_ker_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X3 = 3,
}
impl From<SDMMC12SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SDMMC12SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SDMMC12SRC`"]
pub type SDMMC12SRC_R = crate::R<u8, SDMMC12SRC_A>;
impl SDMMC12SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SDMMC12SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SDMMC12SRC_A::B_0X0),
            1 => Val(SDMMC12SRC_A::B_0X1),
            2 => Val(SDMMC12SRC_A::B_0X2),
            3 => Val(SDMMC12SRC_A::B_0X3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SDMMC12SRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SDMMC12SRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SDMMC12SRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SDMMC12SRC_A::B_0X3
    }
}
#[doc = "Write proxy for field `SDMMC12SRC`"]
pub struct SDMMC12SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC12SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC12SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "hclk6 clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SDMMC12SRC_A::B_0X0)
    }
    #[doc = "pll3_r_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SDMMC12SRC_A::B_0X1)
    }
    #[doc = "pll4_p_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SDMMC12SRC_A::B_0X2)
    }
    #[doc = "hsi_ker_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SDMMC12SRC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SDMMC12SRC"]
    #[inline(always)]
    pub fn sdmmc12src(&self) -> SDMMC12SRC_R {
        SDMMC12SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDMMC12SRC"]
    #[inline(always)]
    pub fn sdmmc12src(&mut self) -> SDMMC12SRC_W {
        SDMMC12SRC_W { w: self }
    }
}
