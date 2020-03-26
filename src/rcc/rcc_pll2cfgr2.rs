#[doc = "Reader of register RCC_PLL2CFGR2"]
pub type R = crate::R<u32, super::RCC_PLL2CFGR2>;
#[doc = "Writer for register RCC_PLL2CFGR2"]
pub type W = crate::W<u32, super::RCC_PLL2CFGR2>;
#[doc = "Register RCC_PLL2CFGR2 `reset()`'s with value 0x0001_0101"]
impl crate::ResetValue for super::RCC_PLL2CFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0101
    }
}
#[doc = "DIVP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVP_A {
    #[doc = "0: pll2_p_ck = fout2_ck"]
    B_0X0 = 0,
    #[doc = "1: pll2_p_ck = fout2_ck / 2 (default\r\n                  after reset)"]
    B_0X1 = 1,
    #[doc = "2: pll2_p_ck = fout2_ck /\r\n                  3"]
    B_0X2 = 2,
    #[doc = "127: pll2_p_ck = fout2_ck /\r\n                  128"]
    B_0X7F = 127,
}
impl From<DIVP_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVP`"]
pub type DIVP_R = crate::R<u8, DIVP_A>;
impl DIVP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVP_A::B_0X0),
            1 => Val(DIVP_A::B_0X1),
            2 => Val(DIVP_A::B_0X2),
            127 => Val(DIVP_A::B_0X7F),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DIVP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DIVP_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DIVP_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X7F`"]
    #[inline(always)]
    pub fn is_b_0x7f(&self) -> bool {
        *self == DIVP_A::B_0X7F
    }
}
#[doc = "Write proxy for field `DIVP`"]
pub struct DIVP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll2_p_ck = fout2_ck"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIVP_A::B_0X0)
    }
    #[doc = "pll2_p_ck = fout2_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIVP_A::B_0X1)
    }
    #[doc = "pll2_p_ck = fout2_ck / 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DIVP_A::B_0X2)
    }
    #[doc = "pll2_p_ck = fout2_ck / 128"]
    #[inline(always)]
    pub fn b_0x7f(self) -> &'a mut W {
        self.variant(DIVP_A::B_0X7F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "DIVQ\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVQ_A {
    #[doc = "0: pll2_q_ck = fout2_ck"]
    B_0X0 = 0,
    #[doc = "1: pll2_q_ck = fout2_ck / 2 (default\r\n                  after reset)"]
    B_0X1 = 1,
    #[doc = "2: pll2_q_ck = fout2_ck /\r\n                  3"]
    B_0X2 = 2,
    #[doc = "127: pll2_q_ck = fout2_ck /\r\n                  128"]
    B_0X7F = 127,
}
impl From<DIVQ_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVQ`"]
pub type DIVQ_R = crate::R<u8, DIVQ_A>;
impl DIVQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVQ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVQ_A::B_0X0),
            1 => Val(DIVQ_A::B_0X1),
            2 => Val(DIVQ_A::B_0X2),
            127 => Val(DIVQ_A::B_0X7F),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DIVQ_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DIVQ_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DIVQ_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X7F`"]
    #[inline(always)]
    pub fn is_b_0x7f(&self) -> bool {
        *self == DIVQ_A::B_0X7F
    }
}
#[doc = "Write proxy for field `DIVQ`"]
pub struct DIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll2_q_ck = fout2_ck"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIVQ_A::B_0X0)
    }
    #[doc = "pll2_q_ck = fout2_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIVQ_A::B_0X1)
    }
    #[doc = "pll2_q_ck = fout2_ck / 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DIVQ_A::B_0X2)
    }
    #[doc = "pll2_q_ck = fout2_ck / 128"]
    #[inline(always)]
    pub fn b_0x7f(self) -> &'a mut W {
        self.variant(DIVQ_A::B_0X7F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "DIVR\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVR_A {
    #[doc = "0: pll2_r_ck = fout2_ck"]
    B_0X0 = 0,
    #[doc = "1: pll2_r_ck = fout2_ck / 2 (default\r\n                  after reset)"]
    B_0X1 = 1,
    #[doc = "2: pll2_r_ck = fout2_ck /\r\n                  3"]
    B_0X2 = 2,
    #[doc = "127: pll2_r_ck = fout2_ck /\r\n                  128"]
    B_0X7F = 127,
}
impl From<DIVR_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVR`"]
pub type DIVR_R = crate::R<u8, DIVR_A>;
impl DIVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVR_A::B_0X0),
            1 => Val(DIVR_A::B_0X1),
            2 => Val(DIVR_A::B_0X2),
            127 => Val(DIVR_A::B_0X7F),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DIVR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DIVR_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DIVR_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X7F`"]
    #[inline(always)]
    pub fn is_b_0x7f(&self) -> bool {
        *self == DIVR_A::B_0X7F
    }
}
#[doc = "Write proxy for field `DIVR`"]
pub struct DIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll2_r_ck = fout2_ck"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIVR_A::B_0X0)
    }
    #[doc = "pll2_r_ck = fout2_ck / 2 (default after reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIVR_A::B_0X1)
    }
    #[doc = "pll2_r_ck = fout2_ck / 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DIVR_A::B_0X2)
    }
    #[doc = "pll2_r_ck = fout2_ck / 128"]
    #[inline(always)]
    pub fn b_0x7f(self) -> &'a mut W {
        self.variant(DIVR_A::B_0X7F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - DIVP"]
    #[inline(always)]
    pub fn divp(&self) -> DIVP_R {
        DIVP_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - DIVQ"]
    #[inline(always)]
    pub fn divq(&self) -> DIVQ_R {
        DIVQ_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - DIVR"]
    #[inline(always)]
    pub fn divr(&self) -> DIVR_R {
        DIVR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DIVP"]
    #[inline(always)]
    pub fn divp(&mut self) -> DIVP_W {
        DIVP_W { w: self }
    }
    #[doc = "Bits 8:14 - DIVQ"]
    #[inline(always)]
    pub fn divq(&mut self) -> DIVQ_W {
        DIVQ_W { w: self }
    }
    #[doc = "Bits 16:22 - DIVR"]
    #[inline(always)]
    pub fn divr(&mut self) -> DIVR_W {
        DIVR_W { w: self }
    }
}
