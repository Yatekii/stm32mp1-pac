#[doc = "Reader of register RCC_RCK3SELR"]
pub type R = crate::R<u32, super::RCC_RCK3SELR>;
#[doc = "Writer for register RCC_RCK3SELR"]
pub type W = crate::W<u32, super::RCC_RCK3SELR>;
#[doc = "Register RCC_RCK3SELR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_RCK3SELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "PLL3SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL3SRC_A {
    #[doc = "0: HSI selected as PLL clock (hsi_ck)\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: HSE selected as PLL clock\r\n                  (hse_ck)"]
    B_0X1 = 1,
    #[doc = "2: CSI selected as PLL clock\r\n                  (csi_ck)"]
    B_0X2 = 2,
    #[doc = "3: No clock send to DIVMx divider and\r\n                  PLLs"]
    B_0X3 = 3,
}
impl From<PLL3SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL3SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL3SRC`"]
pub type PLL3SRC_R = crate::R<u8, PLL3SRC_A>;
impl PLL3SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL3SRC_A {
        match self.bits {
            0 => PLL3SRC_A::B_0X0,
            1 => PLL3SRC_A::B_0X1,
            2 => PLL3SRC_A::B_0X2,
            3 => PLL3SRC_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL3SRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL3SRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PLL3SRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PLL3SRC_A::B_0X3
    }
}
#[doc = "Write proxy for field `PLL3SRC`"]
pub struct PLL3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3SRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HSI selected as PLL clock (hsi_ck) (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLL3SRC_A::B_0X0)
    }
    #[doc = "HSE selected as PLL clock (hse_ck)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLL3SRC_A::B_0X1)
    }
    #[doc = "CSI selected as PLL clock (csi_ck)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PLL3SRC_A::B_0X2)
    }
    #[doc = "No clock send to DIVMx divider and PLLs"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PLL3SRC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "PLL3SRCRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL3SRCRDY_A {
    #[doc = "0: The PLL3 switch is not ready or in\r\n                  position : no clock is generated on its\r\n                  output"]
    B_0X0 = 0,
    #[doc = "1: The PLL3 switch is ready: the clock\r\n                  switch is selecting the clock given by PLL3SRC\r\n                  field. (default after reset)"]
    B_0X1 = 1,
}
impl From<PLL3SRCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLL3SRCRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL3SRCRDY`"]
pub type PLL3SRCRDY_R = crate::R<bool, PLL3SRCRDY_A>;
impl PLL3SRCRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL3SRCRDY_A {
        match self.bits {
            false => PLL3SRCRDY_A::B_0X0,
            true => PLL3SRCRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL3SRCRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL3SRCRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:1 - PLL3SRC"]
    #[inline(always)]
    pub fn pll3src(&self) -> PLL3SRC_R {
        PLL3SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - PLL3SRCRDY"]
    #[inline(always)]
    pub fn pll3srcrdy(&self) -> PLL3SRCRDY_R {
        PLL3SRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL3SRC"]
    #[inline(always)]
    pub fn pll3src(&mut self) -> PLL3SRC_W {
        PLL3SRC_W { w: self }
    }
}
