#[doc = "Reader of register RCC_RCK12SELR"]
pub type R = crate::R<u32, super::RCC_RCK12SELR>;
#[doc = "Writer for register RCC_RCK12SELR"]
pub type W = crate::W<u32, super::RCC_RCK12SELR>;
#[doc = "Register RCC_RCK12SELR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_RCK12SELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "PLL12SRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLL12SRC_A {
    #[doc = "0: HSI selected as PLL clock (hsi_ck)\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: HSE selected as PLL clock\r\n                  (hse_ck)"]
    B_0X1 = 1,
}
impl From<PLL12SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL12SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PLL12SRC`"]
pub type PLL12SRC_R = crate::R<u8, PLL12SRC_A>;
impl PLL12SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PLL12SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PLL12SRC_A::B_0X0),
            1 => Val(PLL12SRC_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL12SRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL12SRC_A::B_0X1
    }
}
#[doc = "Write proxy for field `PLL12SRC`"]
pub struct PLL12SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL12SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL12SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSI selected as PLL clock (hsi_ck) (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLL12SRC_A::B_0X0)
    }
    #[doc = "HSE selected as PLL clock (hse_ck)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLL12SRC_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "PLL12SRCRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL12SRCRDY_A {
    #[doc = "0: The PLL12 switch is not ready or in\r\n                  position : no clock is generated on its\r\n                  output"]
    B_0X0 = 0,
    #[doc = "1: The PLL12 switch is ready: the clock\r\n                  switch is selecting the clock given by PLL12SRC\r\n                  field. (default after reset)"]
    B_0X1 = 1,
}
impl From<PLL12SRCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLL12SRCRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL12SRCRDY`"]
pub type PLL12SRCRDY_R = crate::R<bool, PLL12SRCRDY_A>;
impl PLL12SRCRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL12SRCRDY_A {
        match self.bits {
            false => PLL12SRCRDY_A::B_0X0,
            true => PLL12SRCRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL12SRCRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL12SRCRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:1 - PLL12SRC"]
    #[inline(always)]
    pub fn pll12src(&self) -> PLL12SRC_R {
        PLL12SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - PLL12SRCRDY"]
    #[inline(always)]
    pub fn pll12srcrdy(&self) -> PLL12SRCRDY_R {
        PLL12SRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL12SRC"]
    #[inline(always)]
    pub fn pll12src(&mut self) -> PLL12SRC_W {
        PLL12SRC_W { w: self }
    }
}
