#[doc = "Reader of register RCC_RCK4SELR"]
pub type R = crate::R<u32, super::RCC_RCK4SELR>;
#[doc = "Writer for register RCC_RCK4SELR"]
pub type W = crate::W<u32, super::RCC_RCK4SELR>;
#[doc = "Register RCC_RCK4SELR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_RCK4SELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "PLL4SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL4SRC_A {
    #[doc = "0: HSI selected as PLL clock (hsi_ck)\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: HSE selected as PLL clock\r\n                  (hse_ck)"]
    B_0X1 = 1,
    #[doc = "2: CSI selected as PLL clock\r\n                  (csi_ck)"]
    B_0X2 = 2,
    #[doc = "3: Signal I2S_CKIN used as reference\r\n                  clock"]
    B_0X3 = 3,
}
impl From<PLL4SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL4SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL4SRC`"]
pub type PLL4SRC_R = crate::R<u8, PLL4SRC_A>;
impl PLL4SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL4SRC_A {
        match self.bits {
            0 => PLL4SRC_A::B_0X0,
            1 => PLL4SRC_A::B_0X1,
            2 => PLL4SRC_A::B_0X2,
            3 => PLL4SRC_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL4SRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL4SRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PLL4SRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PLL4SRC_A::B_0X3
    }
}
#[doc = "Write proxy for field `PLL4SRC`"]
pub struct PLL4SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL4SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL4SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HSI selected as PLL clock (hsi_ck) (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLL4SRC_A::B_0X0)
    }
    #[doc = "HSE selected as PLL clock (hse_ck)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLL4SRC_A::B_0X1)
    }
    #[doc = "CSI selected as PLL clock (csi_ck)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PLL4SRC_A::B_0X2)
    }
    #[doc = "Signal I2S_CKIN used as reference clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PLL4SRC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "PLL4SRCRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL4SRCRDY_A {
    #[doc = "0: The PLL4 switch is not ready or in\r\n                  position : no clock is generated on its\r\n                  output"]
    B_0X0 = 0,
    #[doc = "1: The PLL4 switch is ready: the clock\r\n                  switch is selecting the clock given by PLL4SRC\r\n                  field. (default after reset)"]
    B_0X1 = 1,
}
impl From<PLL4SRCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLL4SRCRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL4SRCRDY`"]
pub type PLL4SRCRDY_R = crate::R<bool, PLL4SRCRDY_A>;
impl PLL4SRCRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL4SRCRDY_A {
        match self.bits {
            false => PLL4SRCRDY_A::B_0X0,
            true => PLL4SRCRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL4SRCRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL4SRCRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:1 - PLL4SRC"]
    #[inline(always)]
    pub fn pll4src(&self) -> PLL4SRC_R {
        PLL4SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - PLL4SRCRDY"]
    #[inline(always)]
    pub fn pll4srcrdy(&self) -> PLL4SRCRDY_R {
        PLL4SRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL4SRC"]
    #[inline(always)]
    pub fn pll4src(&mut self) -> PLL4SRC_W {
        PLL4SRC_W { w: self }
    }
}
