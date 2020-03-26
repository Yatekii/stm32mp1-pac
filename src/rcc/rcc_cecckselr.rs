#[doc = "Reader of register RCC_CECCKSELR"]
pub type R = crate::R<u32, super::RCC_CECCKSELR>;
#[doc = "Writer for register RCC_CECCKSELR"]
pub type W = crate::W<u32, super::RCC_CECCKSELR>;
#[doc = "Register RCC_CECCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_CECCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CECSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CECSRC_A {
    #[doc = "0: lse_ck clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: lsi_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: csi_ker_ck/122 clock selected as\r\n                  kernel peripheral clock"]
    B_0X2 = 2,
}
impl From<CECSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CECSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CECSRC`"]
pub type CECSRC_R = crate::R<u8, CECSRC_A>;
impl CECSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CECSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CECSRC_A::B_0X0),
            1 => Val(CECSRC_A::B_0X1),
            2 => Val(CECSRC_A::B_0X2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CECSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CECSRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CECSRC_A::B_0X2
    }
}
#[doc = "Write proxy for field `CECSRC`"]
pub struct CECSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CECSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "lse_ck clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CECSRC_A::B_0X0)
    }
    #[doc = "lsi_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CECSRC_A::B_0X1)
    }
    #[doc = "csi_ker_ck/122 clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CECSRC_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CECSRC"]
    #[inline(always)]
    pub fn cecsrc(&self) -> CECSRC_R {
        CECSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CECSRC"]
    #[inline(always)]
    pub fn cecsrc(&mut self) -> CECSRC_W {
        CECSRC_W { w: self }
    }
}
