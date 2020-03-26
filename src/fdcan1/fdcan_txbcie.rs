#[doc = "Reader of register FDCAN_TXBCIE"]
pub type R = crate::R<u32, super::FDCAN_TXBCIE>;
#[doc = "Writer for register FDCAN_TXBCIE"]
pub type W = crate::W<u32, super::FDCAN_TXBCIE>;
#[doc = "Register FDCAN_TXBCIE `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TXBCIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CFIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CFIE_A {
    #[doc = "0: Cancellation finished interrupt\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Cancellation finished interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<CFIE_A> for u32 {
    #[inline(always)]
    fn from(variant: CFIE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFIE`"]
pub type CFIE_R = crate::R<u32, CFIE_A>;
impl CFIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CFIE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CFIE_A::B_0X0),
            1 => Val(CFIE_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CFIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CFIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `CFIE`"]
pub struct CFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFIE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Cancellation finished interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CFIE_A::B_0X0)
    }
    #[doc = "Cancellation finished interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CFIE_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CFIE"]
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CFIE"]
    #[inline(always)]
    pub fn cfie(&mut self) -> CFIE_W {
        CFIE_W { w: self }
    }
}
