#[doc = "Reader of register CRYP_CSGCMCCM0R"]
pub type R = crate::R<u32, super::CRYP_CSGCMCCM0R>;
#[doc = "Writer for register CRYP_CSGCMCCM0R"]
pub type W = crate::W<u32, super::CRYP_CSGCMCCM0R>;
#[doc = "Register CRYP_CSGCMCCM0R `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYP_CSGCMCCM0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRYP_CSGCMCCM0R`"]
pub type CRYP_CSGCMCCM0R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRYP_CSGCMCCM0R`"]
pub struct CRYP_CSGCMCCM0R_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP_CSGCMCCM0R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRYP_CSGCMCCM0R"]
    #[inline(always)]
    pub fn cryp_csgcmccm0r(&self) -> CRYP_CSGCMCCM0R_R {
        CRYP_CSGCMCCM0R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRYP_CSGCMCCM0R"]
    #[inline(always)]
    pub fn cryp_csgcmccm0r(&mut self) -> CRYP_CSGCMCCM0R_W {
        CRYP_CSGCMCCM0R_W { w: self }
    }
}