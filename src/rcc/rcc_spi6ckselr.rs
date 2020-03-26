#[doc = "Reader of register RCC_SPI6CKSELR"]
pub type R = crate::R<u32, super::RCC_SPI6CKSELR>;
#[doc = "Writer for register RCC_SPI6CKSELR"]
pub type W = crate::W<u32, super::RCC_SPI6CKSELR>;
#[doc = "Register RCC_SPI6CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_SPI6CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SPI6SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI6SRC_A {
    #[doc = "0: pclk5 clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll4_q_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: hsi_ker_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X2 = 2,
    #[doc = "3: csi_ker_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X3 = 3,
    #[doc = "4: hse_ker_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X4 = 4,
    #[doc = "5: pll3_q_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X5 = 5,
}
impl From<SPI6SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI6SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPI6SRC`"]
pub type SPI6SRC_R = crate::R<u8, SPI6SRC_A>;
impl SPI6SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPI6SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPI6SRC_A::B_0X0),
            1 => Val(SPI6SRC_A::B_0X1),
            2 => Val(SPI6SRC_A::B_0X2),
            3 => Val(SPI6SRC_A::B_0X3),
            4 => Val(SPI6SRC_A::B_0X4),
            5 => Val(SPI6SRC_A::B_0X5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPI6SRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPI6SRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SPI6SRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SPI6SRC_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == SPI6SRC_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == SPI6SRC_A::B_0X5
    }
}
#[doc = "Write proxy for field `SPI6SRC`"]
pub struct SPI6SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI6SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pclk5 clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPI6SRC_A::B_0X0)
    }
    #[doc = "pll4_q_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPI6SRC_A::B_0X1)
    }
    #[doc = "hsi_ker_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SPI6SRC_A::B_0X2)
    }
    #[doc = "csi_ker_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SPI6SRC_A::B_0X3)
    }
    #[doc = "hse_ker_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(SPI6SRC_A::B_0X4)
    }
    #[doc = "pll3_q_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(SPI6SRC_A::B_0X5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI6SRC"]
    #[inline(always)]
    pub fn spi6src(&self) -> SPI6SRC_R {
        SPI6SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI6SRC"]
    #[inline(always)]
    pub fn spi6src(&mut self) -> SPI6SRC_W {
        SPI6SRC_W { w: self }
    }
}
