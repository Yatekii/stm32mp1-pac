#[doc = "Reader of register CRYP_CSGCMCCM6R"]
pub type R = crate::R<u32, super::CRYP_CSGCMCCM6R>;
#[doc = "Writer for register CRYP_CSGCMCCM6R"]
pub type W = crate::W<u32, super::CRYP_CSGCMCCM6R>;
#[doc = "Register CRYP_CSGCMCCM6R `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYP_CSGCMCCM6R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRYP_CSGCMCCM6R`"]
pub type CRYP_CSGCMCCM6R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRYP_CSGCMCCM6R`"]
pub struct CRYP_CSGCMCCM6R_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP_CSGCMCCM6R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRYP_CSGCMCCM6R"]
    #[inline(always)]
    pub fn cryp_csgcmccm6r(&self) -> CRYP_CSGCMCCM6R_R {
        CRYP_CSGCMCCM6R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRYP_CSGCMCCM6R"]
    #[inline(always)]
    pub fn cryp_csgcmccm6r(&mut self) -> CRYP_CSGCMCCM6R_W {
        CRYP_CSGCMCCM6R_W { w: self }
    }
}
