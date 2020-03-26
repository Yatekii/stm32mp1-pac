#[doc = "Reader of register RCC_FMCCKSELR"]
pub type R = crate::R<u32, super::RCC_FMCCKSELR>;
#[doc = "Writer for register RCC_FMCCKSELR"]
pub type W = crate::W<u32, super::RCC_FMCCKSELR>;
#[doc = "Register RCC_FMCCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_FMCCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FMCSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FMCSRC_A {
    #[doc = "0: aclk clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll3_r_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: pll4_p_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X2 = 2,
    #[doc = "3: per_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X3 = 3,
}
impl From<FMCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: FMCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FMCSRC`"]
pub type FMCSRC_R = crate::R<u8, FMCSRC_A>;
impl FMCSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCSRC_A {
        match self.bits {
            0 => FMCSRC_A::B_0X0,
            1 => FMCSRC_A::B_0X1,
            2 => FMCSRC_A::B_0X2,
            3 => FMCSRC_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMCSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMCSRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == FMCSRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == FMCSRC_A::B_0X3
    }
}
#[doc = "Write proxy for field `FMCSRC`"]
pub struct FMCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "aclk clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMCSRC_A::B_0X0)
    }
    #[doc = "pll3_r_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMCSRC_A::B_0X1)
    }
    #[doc = "pll4_p_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(FMCSRC_A::B_0X2)
    }
    #[doc = "per_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(FMCSRC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FMCSRC"]
    #[inline(always)]
    pub fn fmcsrc(&self) -> FMCSRC_R {
        FMCSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FMCSRC"]
    #[inline(always)]
    pub fn fmcsrc(&mut self) -> FMCSRC_W {
        FMCSRC_W { w: self }
    }
}
