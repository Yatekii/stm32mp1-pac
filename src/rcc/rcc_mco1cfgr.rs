#[doc = "Reader of register RCC_MCO1CFGR"]
pub type R = crate::R<u32, super::RCC_MCO1CFGR>;
#[doc = "Writer for register RCC_MCO1CFGR"]
pub type W = crate::W<u32, super::RCC_MCO1CFGR>;
#[doc = "Register RCC_MCO1CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MCO1CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCO1SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCO1SEL_A {
    #[doc = "0: HSI clock selected (hsi_ck) (default\r\n                  after reset)"]
    B_0X0 = 0,
    #[doc = "1: HSE clock selected\r\n                  (hse_ck)"]
    B_0X1 = 1,
    #[doc = "2: CSI clock selected\r\n                  (csi_ck)"]
    B_0X2 = 2,
    #[doc = "3: LSI clock selected\r\n                  (lsi_ck)"]
    B_0X3 = 3,
    #[doc = "4: LSE oscillator clock selected\r\n                  (lse_ck)"]
    B_0X4 = 4,
}
impl From<MCO1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCO1SEL`"]
pub type MCO1SEL_R = crate::R<u8, MCO1SEL_A>;
impl MCO1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCO1SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MCO1SEL_A::B_0X0),
            1 => Val(MCO1SEL_A::B_0X1),
            2 => Val(MCO1SEL_A::B_0X2),
            3 => Val(MCO1SEL_A::B_0X3),
            4 => Val(MCO1SEL_A::B_0X4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCO1SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCO1SEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MCO1SEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MCO1SEL_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MCO1SEL_A::B_0X4
    }
}
#[doc = "Write proxy for field `MCO1SEL`"]
pub struct MCO1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSI clock selected (hsi_ck) (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCO1SEL_A::B_0X0)
    }
    #[doc = "HSE clock selected (hse_ck)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCO1SEL_A::B_0X1)
    }
    #[doc = "CSI clock selected (csi_ck)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MCO1SEL_A::B_0X2)
    }
    #[doc = "LSI clock selected (lsi_ck)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MCO1SEL_A::B_0X3)
    }
    #[doc = "LSE oscillator clock selected (lse_ck)"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(MCO1SEL_A::B_0X4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "MCO1DIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCO1DIV_A {
    #[doc = "0: bypass (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: division by 2"]
    B_0X1 = 1,
    #[doc = "2: division by 3"]
    B_0X2 = 2,
    #[doc = "15: division by 16"]
    B_0XF = 15,
}
impl From<MCO1DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO1DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCO1DIV`"]
pub type MCO1DIV_R = crate::R<u8, MCO1DIV_A>;
impl MCO1DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCO1DIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MCO1DIV_A::B_0X0),
            1 => Val(MCO1DIV_A::B_0X1),
            2 => Val(MCO1DIV_A::B_0X2),
            15 => Val(MCO1DIV_A::B_0XF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCO1DIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCO1DIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MCO1DIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == MCO1DIV_A::B_0XF
    }
}
#[doc = "Write proxy for field `MCO1DIV`"]
pub struct MCO1DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO1DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "bypass (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCO1DIV_A::B_0X0)
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCO1DIV_A::B_0X1)
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MCO1DIV_A::B_0X2)
    }
    #[doc = "division by 16"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(MCO1DIV_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "MCO1ON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCO1ON_A {
    #[doc = "0: The MCO1 output is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: The MCO1 output is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<MCO1ON_A> for bool {
    #[inline(always)]
    fn from(variant: MCO1ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCO1ON`"]
pub type MCO1ON_R = crate::R<bool, MCO1ON_A>;
impl MCO1ON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCO1ON_A {
        match self.bits {
            false => MCO1ON_A::B_0X0,
            true => MCO1ON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCO1ON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCO1ON_A::B_0X1
    }
}
#[doc = "Write proxy for field `MCO1ON`"]
pub struct MCO1ON_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO1ON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The MCO1 output is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCO1ON_A::B_0X0)
    }
    #[doc = "The MCO1 output is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCO1ON_A::B_0X1)
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
    #[doc = "Bits 0:2 - MCO1SEL"]
    #[inline(always)]
    pub fn mco1sel(&self) -> MCO1SEL_R {
        MCO1SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - MCO1DIV"]
    #[inline(always)]
    pub fn mco1div(&self) -> MCO1DIV_R {
        MCO1DIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MCO1ON"]
    #[inline(always)]
    pub fn mco1on(&self) -> MCO1ON_R {
        MCO1ON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCO1SEL"]
    #[inline(always)]
    pub fn mco1sel(&mut self) -> MCO1SEL_W {
        MCO1SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - MCO1DIV"]
    #[inline(always)]
    pub fn mco1div(&mut self) -> MCO1DIV_W {
        MCO1DIV_W { w: self }
    }
    #[doc = "Bit 12 - MCO1ON"]
    #[inline(always)]
    pub fn mco1on(&mut self) -> MCO1ON_W {
        MCO1ON_W { w: self }
    }
}
