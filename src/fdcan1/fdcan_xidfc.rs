#[doc = "Reader of register FDCAN_XIDFC"]
pub type R = crate::R<u32, super::FDCAN_XIDFC>;
#[doc = "Writer for register FDCAN_XIDFC"]
pub type W = crate::W<u32, super::FDCAN_XIDFC>;
#[doc = "Register FDCAN_XIDFC `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_XIDFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLESA`"]
pub type FLESA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FLESA`"]
pub struct FLESA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLESA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "LSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LSE_A {
    #[doc = "0: No standard Message ID\r\n                  filter"]
    B_0X0 = 0,
}
impl From<LSE_A> for u8 {
    #[inline(always)]
    fn from(variant: LSE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LSE`"]
pub type LSE_R = crate::R<u8, LSE_A>;
impl LSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LSE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LSE_A::B_0X0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSE_A::B_0X0
    }
}
#[doc = "Write proxy for field `LSE`"]
pub struct LSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No standard Message ID filter"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSE_A::B_0X0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - FLESA"]
    #[inline(always)]
    pub fn flesa(&self) -> FLESA_R {
        FLESA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - LSE"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - FLESA"]
    #[inline(always)]
    pub fn flesa(&mut self) -> FLESA_W {
        FLESA_W { w: self }
    }
    #[doc = "Bits 16:23 - LSE"]
    #[inline(always)]
    pub fn lse(&mut self) -> LSE_W {
        LSE_W { w: self }
    }
}
