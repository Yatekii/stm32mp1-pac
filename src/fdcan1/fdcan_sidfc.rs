#[doc = "Reader of register FDCAN_SIDFC"]
pub type R = crate::R<u32, super::FDCAN_SIDFC>;
#[doc = "Writer for register FDCAN_SIDFC"]
pub type W = crate::W<u32, super::FDCAN_SIDFC>;
#[doc = "Register FDCAN_SIDFC `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_SIDFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLSSA`"]
pub type FLSSA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FLSSA`"]
pub struct FLSSA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLSSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "LSS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LSS_A {
    #[doc = "0: No standard Message ID\r\n                  filter"]
    B_0X0 = 0,
}
impl From<LSS_A> for u8 {
    #[inline(always)]
    fn from(variant: LSS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LSS`"]
pub type LSS_R = crate::R<u8, LSS_A>;
impl LSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LSS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LSS_A::B_0X0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSS_A::B_0X0
    }
}
#[doc = "Write proxy for field `LSS`"]
pub struct LSS_W<'a> {
    w: &'a mut W,
}
impl<'a> LSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No standard Message ID filter"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSS_A::B_0X0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - FLSSA"]
    #[inline(always)]
    pub fn flssa(&self) -> FLSSA_R {
        FLSSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - LSS"]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - FLSSA"]
    #[inline(always)]
    pub fn flssa(&mut self) -> FLSSA_W {
        FLSSA_W { w: self }
    }
    #[doc = "Bits 16:23 - LSS"]
    #[inline(always)]
    pub fn lss(&mut self) -> LSS_W {
        LSS_W { w: self }
    }
}
