#[doc = "Reader of register RCC_UART35CKSELR"]
pub type R = crate::R<u32, super::RCC_UART35CKSELR>;
#[doc = "Writer for register RCC_UART35CKSELR"]
pub type W = crate::W<u32, super::RCC_UART35CKSELR>;
#[doc = "Register RCC_UART35CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_UART35CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "UART35SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART35SRC_A {
    #[doc = "0: pclk1 clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll4_q_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: hsi_ker_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X2 = 2,
    #[doc = "3: csi_ker_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X3 = 3,
    #[doc = "4: hse_ker_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X4 = 4,
}
impl From<UART35SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: UART35SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `UART35SRC`"]
pub type UART35SRC_R = crate::R<u8, UART35SRC_A>;
impl UART35SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UART35SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UART35SRC_A::B_0X0),
            1 => Val(UART35SRC_A::B_0X1),
            2 => Val(UART35SRC_A::B_0X2),
            3 => Val(UART35SRC_A::B_0X3),
            4 => Val(UART35SRC_A::B_0X4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UART35SRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UART35SRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == UART35SRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == UART35SRC_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == UART35SRC_A::B_0X4
    }
}
#[doc = "Write proxy for field `UART35SRC`"]
pub struct UART35SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART35SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART35SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pclk1 clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(UART35SRC_A::B_0X0)
    }
    #[doc = "pll4_q_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(UART35SRC_A::B_0X1)
    }
    #[doc = "hsi_ker_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(UART35SRC_A::B_0X2)
    }
    #[doc = "csi_ker_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(UART35SRC_A::B_0X3)
    }
    #[doc = "hse_ker_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(UART35SRC_A::B_0X4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - UART35SRC"]
    #[inline(always)]
    pub fn uart35src(&self) -> UART35SRC_R {
        UART35SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART35SRC"]
    #[inline(always)]
    pub fn uart35src(&mut self) -> UART35SRC_W {
        UART35SRC_W { w: self }
    }
}
