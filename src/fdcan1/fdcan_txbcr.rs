#[doc = "Reader of register FDCAN_TXBCR"]
pub type R = crate::R<u32, super::FDCAN_TXBCR>;
#[doc = "Writer for register FDCAN_TXBCR"]
pub type W = crate::W<u32, super::FDCAN_TXBCR>;
#[doc = "Register FDCAN_TXBCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TXBCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CR_A {
    #[doc = "0: No cancellation\r\n                  pending"]
    B_0X0 = 0,
    #[doc = "1: Cancellation pending"]
    B_0X1 = 1,
}
impl From<CR_A> for u32 {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CR`"]
pub type CR_R = crate::R<u32, CR_A>;
impl CR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CR_A::B_0X0),
            1 => Val(CR_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CR_A::B_0X1
    }
}
#[doc = "Write proxy for field `CR`"]
pub struct CR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No cancellation pending"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CR_A::B_0X0)
    }
    #[doc = "Cancellation pending"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CR_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CR"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CR"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W {
        CR_W { w: self }
    }
}
