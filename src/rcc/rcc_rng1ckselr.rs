#[doc = "Reader of register RCC_RNG1CKSELR"]
pub type R = crate::R<u32, super::RCC_RNG1CKSELR>;
#[doc = "Writer for register RCC_RNG1CKSELR"]
pub type W = crate::W<u32, super::RCC_RNG1CKSELR>;
#[doc = "Register RCC_RNG1CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_RNG1CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RNG1SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RNG1SRC_A {
    #[doc = "0: csi_ker_ck clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll4_r_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: lse_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X2 = 2,
    #[doc = "3: lsi_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X3 = 3,
}
impl From<RNG1SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: RNG1SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RNG1SRC`"]
pub type RNG1SRC_R = crate::R<u8, RNG1SRC_A>;
impl RNG1SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG1SRC_A {
        match self.bits {
            0 => RNG1SRC_A::B_0X0,
            1 => RNG1SRC_A::B_0X1,
            2 => RNG1SRC_A::B_0X2,
            3 => RNG1SRC_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RNG1SRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RNG1SRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == RNG1SRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == RNG1SRC_A::B_0X3
    }
}
#[doc = "Write proxy for field `RNG1SRC`"]
pub struct RNG1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG1SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG1SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "csi_ker_ck clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RNG1SRC_A::B_0X0)
    }
    #[doc = "pll4_r_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RNG1SRC_A::B_0X1)
    }
    #[doc = "lse_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(RNG1SRC_A::B_0X2)
    }
    #[doc = "lsi_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(RNG1SRC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RNG1SRC"]
    #[inline(always)]
    pub fn rng1src(&self) -> RNG1SRC_R {
        RNG1SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RNG1SRC"]
    #[inline(always)]
    pub fn rng1src(&mut self) -> RNG1SRC_W {
        RNG1SRC_W { w: self }
    }
}
