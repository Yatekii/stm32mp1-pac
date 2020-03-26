#[doc = "Reader of register FDCAN_TXBAR"]
pub type R = crate::R<u32, super::FDCAN_TXBAR>;
#[doc = "Writer for register FDCAN_TXBAR"]
pub type W = crate::W<u32, super::FDCAN_TXBAR>;
#[doc = "Register FDCAN_TXBAR `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TXBAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum AR_A {
    #[doc = "0: No transmission request\r\n                  added"]
    B_0X0 = 0,
    #[doc = "1: Transmission requested\r\n                  added."]
    B_0X1 = 1,
}
impl From<AR_A> for u32 {
    #[inline(always)]
    fn from(variant: AR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AR`"]
pub type AR_R = crate::R<u32, AR_A>;
impl AR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, AR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AR_A::B_0X0),
            1 => Val(AR_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AR_A::B_0X1
    }
}
#[doc = "Write proxy for field `AR`"]
pub struct AR_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No transmission request added"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AR_A::B_0X0)
    }
    #[doc = "Transmission requested added."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AR_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AR"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AR"]
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W {
        AR_W { w: self }
    }
}
