#[doc = "Reader of register RCC_ETHCKSELR"]
pub type R = crate::R<u32, super::RCC_ETHCKSELR>;
#[doc = "Writer for register RCC_ETHCKSELR"]
pub type W = crate::W<u32, super::RCC_ETHCKSELR>;
#[doc = "Register RCC_ETHCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_ETHCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ETHSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETHSRC_A {
    #[doc = "0: pll4_p_ck clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll3_q_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
}
impl From<ETHSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: ETHSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ETHSRC`"]
pub type ETHSRC_R = crate::R<u8, ETHSRC_A>;
impl ETHSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ETHSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ETHSRC_A::B_0X0),
            1 => Val(ETHSRC_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHSRC_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETHSRC`"]
pub struct ETHSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll4_p_ck clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHSRC_A::B_0X0)
    }
    #[doc = "pll3_q_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHSRC_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "ETHPTPDIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETHPTPDIV_A {
    #[doc = "0: bypass (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: division by 2"]
    B_0X1 = 1,
    #[doc = "2: division by 3"]
    B_0X2 = 2,
    #[doc = "15: division by 16"]
    B_0XF = 15,
}
impl From<ETHPTPDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ETHPTPDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ETHPTPDIV`"]
pub type ETHPTPDIV_R = crate::R<u8, ETHPTPDIV_A>;
impl ETHPTPDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ETHPTPDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ETHPTPDIV_A::B_0X0),
            1 => Val(ETHPTPDIV_A::B_0X1),
            2 => Val(ETHPTPDIV_A::B_0X2),
            15 => Val(ETHPTPDIV_A::B_0XF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHPTPDIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHPTPDIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ETHPTPDIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == ETHPTPDIV_A::B_0XF
    }
}
#[doc = "Write proxy for field `ETHPTPDIV`"]
pub struct ETHPTPDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHPTPDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHPTPDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "bypass (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHPTPDIV_A::B_0X0)
    }
    #[doc = "division by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHPTPDIV_A::B_0X1)
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ETHPTPDIV_A::B_0X2)
    }
    #[doc = "division by 16"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(ETHPTPDIV_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ETHSRC"]
    #[inline(always)]
    pub fn ethsrc(&self) -> ETHSRC_R {
        ETHSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - ETHPTPDIV"]
    #[inline(always)]
    pub fn ethptpdiv(&self) -> ETHPTPDIV_R {
        ETHPTPDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ETHSRC"]
    #[inline(always)]
    pub fn ethsrc(&mut self) -> ETHSRC_W {
        ETHSRC_W { w: self }
    }
    #[doc = "Bits 4:7 - ETHPTPDIV"]
    #[inline(always)]
    pub fn ethptpdiv(&mut self) -> ETHPTPDIV_W {
        ETHPTPDIV_W { w: self }
    }
}
