#[doc = "Reader of register RCC_ADCCKSELR"]
pub type R = crate::R<u32, super::RCC_ADCCKSELR>;
#[doc = "Writer for register RCC_ADCCKSELR"]
pub type W = crate::W<u32, super::RCC_ADCCKSELR>;
#[doc = "Register RCC_ADCCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_ADCCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADCSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCSRC_A {
    #[doc = "0: pll4_q_ck clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: per_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
}
impl From<ADCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCSRC`"]
pub type ADCSRC_R = crate::R<u8, ADCSRC_A>;
impl ADCSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADCSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADCSRC_A::B_0X0),
            1 => Val(ADCSRC_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADCSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADCSRC_A::B_0X1
    }
}
#[doc = "Write proxy for field `ADCSRC`"]
pub struct ADCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll4_q_ck clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADCSRC_A::B_0X0)
    }
    #[doc = "per_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADCSRC_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADCSRC"]
    #[inline(always)]
    pub fn adcsrc(&self) -> ADCSRC_R {
        ADCSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADCSRC"]
    #[inline(always)]
    pub fn adcsrc(&mut self) -> ADCSRC_W {
        ADCSRC_W { w: self }
    }
}
