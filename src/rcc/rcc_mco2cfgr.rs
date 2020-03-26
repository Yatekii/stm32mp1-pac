#[doc = "Reader of register RCC_MCO2CFGR"]
pub type R = crate::R<u32, super::RCC_MCO2CFGR>;
#[doc = "Writer for register RCC_MCO2CFGR"]
pub type W = crate::W<u32, super::RCC_MCO2CFGR>;
#[doc = "Register RCC_MCO2CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MCO2CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCO2SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCO2SEL_A {
    #[doc = "0: MPU clock selected (mpuss_ck)\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: AXI clock selected\r\n                  (axiss_ck)"]
    B_0X1 = 1,
    #[doc = "2: MCU clock selected\r\n                  (mcuss_ck)"]
    B_0X2 = 2,
    #[doc = "3: PLL4 clock selected\r\n                  (pll4_p_ck)"]
    B_0X3 = 3,
    #[doc = "4: HSE clock selected\r\n                  (hse_ck)"]
    B_0X4 = 4,
    #[doc = "5: HSI clock selected\r\n                  (hsi_ck)"]
    B_0X5 = 5,
}
impl From<MCO2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO2SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCO2SEL`"]
pub type MCO2SEL_R = crate::R<u8, MCO2SEL_A>;
impl MCO2SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCO2SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MCO2SEL_A::B_0X0),
            1 => Val(MCO2SEL_A::B_0X1),
            2 => Val(MCO2SEL_A::B_0X2),
            3 => Val(MCO2SEL_A::B_0X3),
            4 => Val(MCO2SEL_A::B_0X4),
            5 => Val(MCO2SEL_A::B_0X5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCO2SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCO2SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MCO2SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MCO2SEL_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MCO2SEL_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == MCO2SEL_A::B_0X5
    }
}
#[doc = "Write proxy for field `MCO2SEL`"]
pub struct MCO2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO2SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MPU clock selected (mpuss_ck) (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCO2SEL_A::B_0X0)
    }
    #[doc = "AXI clock selected (axiss_ck)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCO2SEL_A::B_0X1)
    }
    #[doc = "MCU clock selected (mcuss_ck)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MCO2SEL_A::B_0X2)
    }
    #[doc = "PLL4 clock selected (pll4_p_ck)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MCO2SEL_A::B_0X3)
    }
    #[doc = "HSE clock selected (hse_ck)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(MCO2SEL_A::B_0X4)
    }
    #[doc = "HSI clock selected (hsi_ck)"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(MCO2SEL_A::B_0X5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "MCO2DIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCO2DIV_A {
    #[doc = "0: bypass (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: division by 2"]
    B_0X1 = 1,
    #[doc = "2: division by 3"]
    B_0X2 = 2,
    #[doc = "15: division by 16"]
    B_0XF = 15,
}
impl From<MCO2DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO2DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCO2DIV`"]
pub type MCO2DIV_R = crate::R<u8, MCO2DIV_A>;
impl MCO2DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCO2DIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MCO2DIV_A::B_0X0),
            1 => Val(MCO2DIV_A::B_0X1),
            2 => Val(MCO2DIV_A::B_0X2),
            15 => Val(MCO2DIV_A::B_0XF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCO2DIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCO2DIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MCO2DIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == MCO2DIV_A::B_0XF
    }
}
#[doc = "Write proxy for field `MCO2DIV`"]
pub struct MCO2DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO2DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "bypass (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCO2DIV_A::B_0X0)
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCO2DIV_A::B_0X1)
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MCO2DIV_A::B_0X2)
    }
    #[doc = "division by 16"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(MCO2DIV_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "MCO2ON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCO2ON_A {
    #[doc = "0: The MCO2 output is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: The MCO2 output is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<MCO2ON_A> for bool {
    #[inline(always)]
    fn from(variant: MCO2ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCO2ON`"]
pub type MCO2ON_R = crate::R<bool, MCO2ON_A>;
impl MCO2ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCO2ON_A {
        match self.bits {
            false => MCO2ON_A::B_0X0,
            true => MCO2ON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCO2ON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCO2ON_A::B_0X1
    }
}
#[doc = "Write proxy for field `MCO2ON`"]
pub struct MCO2ON_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO2ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The MCO2 output is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCO2ON_A::B_0X0)
    }
    #[doc = "The MCO2 output is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCO2ON_A::B_0X1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - MCO2SEL"]
    #[inline(always)]
    pub fn mco2sel(&self) -> MCO2SEL_R {
        MCO2SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - MCO2DIV"]
    #[inline(always)]
    pub fn mco2div(&self) -> MCO2DIV_R {
        MCO2DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MCO2ON"]
    #[inline(always)]
    pub fn mco2on(&self) -> MCO2ON_R {
        MCO2ON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCO2SEL"]
    #[inline(always)]
    pub fn mco2sel(&mut self) -> MCO2SEL_W {
        MCO2SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - MCO2DIV"]
    #[inline(always)]
    pub fn mco2div(&mut self) -> MCO2DIV_W {
        MCO2DIV_W { w: self }
    }
    #[doc = "Bit 12 - MCO2ON"]
    #[inline(always)]
    pub fn mco2on(&mut self) -> MCO2ON_W {
        MCO2ON_W { w: self }
    }
}
