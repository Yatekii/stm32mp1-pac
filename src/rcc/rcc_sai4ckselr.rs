#[doc = "Reader of register RCC_SAI4CKSELR"]
pub type R = crate::R<u32, super::RCC_SAI4CKSELR>;
#[doc = "Writer for register RCC_SAI4CKSELR"]
pub type W = crate::W<u32, super::RCC_SAI4CKSELR>;
#[doc = "Register RCC_SAI4CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_SAI4CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SAI4SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI4SRC_A {
    #[doc = "0: pll4_q_ck clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll3_q_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: I2S_CKIN clock selected as kernel\r\n                  peripheral clock"]
    B_0X2 = 2,
    #[doc = "3: per_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X3 = 3,
}
impl From<SAI4SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI4SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI4SRC`"]
pub type SAI4SRC_R = crate::R<u8, SAI4SRC_A>;
impl SAI4SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAI4SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAI4SRC_A::B_0X0),
            1 => Val(SAI4SRC_A::B_0X1),
            2 => Val(SAI4SRC_A::B_0X2),
            3 => Val(SAI4SRC_A::B_0X3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI4SRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI4SRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SAI4SRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SAI4SRC_A::B_0X3
    }
}
#[doc = "Write proxy for field `SAI4SRC`"]
pub struct SAI4SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI4SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI4SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll4_q_ck clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI4SRC_A::B_0X0)
    }
    #[doc = "pll3_q_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI4SRC_A::B_0X1)
    }
    #[doc = "I2S_CKIN clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SAI4SRC_A::B_0X2)
    }
    #[doc = "per_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SAI4SRC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SAI4SRC"]
    #[inline(always)]
    pub fn sai4src(&self) -> SAI4SRC_R {
        SAI4SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI4SRC"]
    #[inline(always)]
    pub fn sai4src(&mut self) -> SAI4SRC_W {
        SAI4SRC_W { w: self }
    }
}
