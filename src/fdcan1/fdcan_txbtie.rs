#[doc = "Reader of register FDCAN_TXBTIE"]
pub type R = crate::R<u32, super::FDCAN_TXBTIE>;
#[doc = "Writer for register FDCAN_TXBTIE"]
pub type W = crate::W<u32, super::FDCAN_TXBTIE>;
#[doc = "Register FDCAN_TXBTIE `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TXBTIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TIE_A {
    #[doc = "0: Transmission interrupt\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Transmission interrupt\r\n                  enable"]
    B_0X1 = 1,
}
impl From<TIE_A> for u32 {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIE`"]
pub type TIE_R = crate::R<u32, TIE_A>;
impl TIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, TIE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIE_A::B_0X0),
            1 => Val(TIE_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TIE`"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Transmission interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TIE_A::B_0X0)
    }
    #[doc = "Transmission interrupt enable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TIE_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TIE"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
}
