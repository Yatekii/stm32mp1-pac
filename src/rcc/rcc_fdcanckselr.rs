#[doc = "Reader of register RCC_FDCANCKSELR"]
pub type R = crate::R<u32, super::RCC_FDCANCKSELR>;
#[doc = "Writer for register RCC_FDCANCKSELR"]
pub type W = crate::W<u32, super::RCC_FDCANCKSELR>;
#[doc = "Register RCC_FDCANCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_FDCANCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FDCANSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FDCANSRC_A {
    #[doc = "0: hse_ker_ck clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll3_q_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: pll4_q_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X2 = 2,
}
impl From<FDCANSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: FDCANSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FDCANSRC`"]
pub type FDCANSRC_R = crate::R<u8, FDCANSRC_A>;
impl FDCANSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FDCANSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FDCANSRC_A::B_0X0),
            1 => Val(FDCANSRC_A::B_0X1),
            2 => Val(FDCANSRC_A::B_0X2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FDCANSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FDCANSRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == FDCANSRC_A::B_0X2
    }
}
#[doc = "Write proxy for field `FDCANSRC`"]
pub struct FDCANSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "hse_ker_ck clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FDCANSRC_A::B_0X0)
    }
    #[doc = "pll3_q_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FDCANSRC_A::B_0X1)
    }
    #[doc = "pll4_q_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(FDCANSRC_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FDCANSRC"]
    #[inline(always)]
    pub fn fdcansrc(&self) -> FDCANSRC_R {
        FDCANSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FDCANSRC"]
    #[inline(always)]
    pub fn fdcansrc(&mut self) -> FDCANSRC_W {
        FDCANSRC_W { w: self }
    }
}
