#[doc = "Reader of register RCC_STGENCKSELR"]
pub type R = crate::R<u32, super::RCC_STGENCKSELR>;
#[doc = "Writer for register RCC_STGENCKSELR"]
pub type W = crate::W<u32, super::RCC_STGENCKSELR>;
#[doc = "Register RCC_STGENCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_STGENCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "STGENSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STGENSRC_A {
    #[doc = "0: hsi_ker_ck clock selected (default\r\n                  after reset)"]
    B_0X0 = 0,
    #[doc = "1: hse_ker_ck clock\r\n                  selected"]
    B_0X1 = 1,
}
impl From<STGENSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: STGENSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STGENSRC`"]
pub type STGENSRC_R = crate::R<u8, STGENSRC_A>;
impl STGENSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STGENSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STGENSRC_A::B_0X0),
            1 => Val(STGENSRC_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STGENSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STGENSRC_A::B_0X1
    }
}
#[doc = "Write proxy for field `STGENSRC`"]
pub struct STGENSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STGENSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "hsi_ker_ck clock selected (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(STGENSRC_A::B_0X0)
    }
    #[doc = "hse_ker_ck clock selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(STGENSRC_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - STGENSRC"]
    #[inline(always)]
    pub fn stgensrc(&self) -> STGENSRC_R {
        STGENSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - STGENSRC"]
    #[inline(always)]
    pub fn stgensrc(&mut self) -> STGENSRC_W {
        STGENSRC_W { w: self }
    }
}
