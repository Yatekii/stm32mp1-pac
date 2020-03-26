#[doc = "Reader of register RCC_MPCKDIVR"]
pub type R = crate::R<u32, super::RCC_MPCKDIVR>;
#[doc = "Writer for register RCC_MPCKDIVR"]
pub type W = crate::W<u32, super::RCC_MPCKDIVR>;
#[doc = "Register RCC_MPCKDIVR `reset()`'s with value 0x8000_0001"]
impl crate::ResetValue for super::RCC_MPCKDIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0001
    }
}
#[doc = "MPUDIV\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MPUDIV_A {
    #[doc = "0: The MPUDIV is disabled; i.e. no\r\n                  clock generated"]
    B_0X0 = 0,
    #[doc = "1: The mpuss_ck is equal to pll1_p_ck\r\n                  divided by 2 (default after reset)"]
    B_0X1 = 1,
    #[doc = "2: The mpuss_ck is equal to pll1_p_ck\r\n                  divided by 4"]
    B_0X2 = 2,
    #[doc = "3: The mpuss_ck is equal to pll1_p_ck\r\n                  divided by 8"]
    B_0X3 = 3,
}
impl From<MPUDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MPUDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MPUDIV`"]
pub type MPUDIV_R = crate::R<u8, MPUDIV_A>;
impl MPUDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MPUDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MPUDIV_A::B_0X0),
            1 => Val(MPUDIV_A::B_0X1),
            2 => Val(MPUDIV_A::B_0X2),
            3 => Val(MPUDIV_A::B_0X3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MPUDIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MPUDIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MPUDIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MPUDIV_A::B_0X3
    }
}
#[doc = "Write proxy for field `MPUDIV`"]
pub struct MPUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MPUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPUDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The MPUDIV is disabled; i.e. no clock generated"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MPUDIV_A::B_0X0)
    }
    #[doc = "The mpuss_ck is equal to pll1_p_ck divided by 2 (default after reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MPUDIV_A::B_0X1)
    }
    #[doc = "The mpuss_ck is equal to pll1_p_ck divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MPUDIV_A::B_0X2)
    }
    #[doc = "The mpuss_ck is equal to pll1_p_ck divided by 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MPUDIV_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "MPUDIVRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUDIVRDY_A {
    #[doc = "0: The new division factor is not yet\r\n                  taken into account."]
    B_0X0 = 0,
    #[doc = "1: The new division factor is taken\r\n                  into account. (default after reset)"]
    B_0X1 = 1,
}
impl From<MPUDIVRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MPUDIVRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUDIVRDY`"]
pub type MPUDIVRDY_R = crate::R<bool, MPUDIVRDY_A>;
impl MPUDIVRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUDIVRDY_A {
        match self.bits {
            false => MPUDIVRDY_A::B_0X0,
            true => MPUDIVRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MPUDIVRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MPUDIVRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:2 - MPUDIV"]
    #[inline(always)]
    pub fn mpudiv(&self) -> MPUDIV_R {
        MPUDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - MPUDIVRDY"]
    #[inline(always)]
    pub fn mpudivrdy(&self) -> MPUDIVRDY_R {
        MPUDIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MPUDIV"]
    #[inline(always)]
    pub fn mpudiv(&mut self) -> MPUDIV_W {
        MPUDIV_W { w: self }
    }
}
