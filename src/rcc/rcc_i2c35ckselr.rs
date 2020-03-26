#[doc = "Reader of register RCC_I2C35CKSELR"]
pub type R = crate::R<u32, super::RCC_I2C35CKSELR>;
#[doc = "Writer for register RCC_I2C35CKSELR"]
pub type W = crate::W<u32, super::RCC_I2C35CKSELR>;
#[doc = "Register RCC_I2C35CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_I2C35CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I2C35SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C35SRC_A {
    #[doc = "0: pclk1 clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll4_r_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: hsi_ker_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X2 = 2,
    #[doc = "3: csi_ker_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X3 = 3,
}
impl From<I2C35SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C35SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `I2C35SRC`"]
pub type I2C35SRC_R = crate::R<u8, I2C35SRC_A>;
impl I2C35SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, I2C35SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(I2C35SRC_A::B_0X0),
            1 => Val(I2C35SRC_A::B_0X1),
            2 => Val(I2C35SRC_A::B_0X2),
            3 => Val(I2C35SRC_A::B_0X3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == I2C35SRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == I2C35SRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == I2C35SRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == I2C35SRC_A::B_0X3
    }
}
#[doc = "Write proxy for field `I2C35SRC`"]
pub struct I2C35SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C35SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C35SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pclk1 clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(I2C35SRC_A::B_0X0)
    }
    #[doc = "pll4_r_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(I2C35SRC_A::B_0X1)
    }
    #[doc = "hsi_ker_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(I2C35SRC_A::B_0X2)
    }
    #[doc = "csi_ker_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(I2C35SRC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - I2C35SRC"]
    #[inline(always)]
    pub fn i2c35src(&self) -> I2C35SRC_R {
        I2C35SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - I2C35SRC"]
    #[inline(always)]
    pub fn i2c35src(&mut self) -> I2C35SRC_W {
        I2C35SRC_W { w: self }
    }
}
