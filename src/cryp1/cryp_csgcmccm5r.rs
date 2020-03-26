#[doc = "Reader of register CRYP_CSGCMCCM5R"]
pub type R = crate::R<u32, super::CRYP_CSGCMCCM5R>;
#[doc = "Writer for register CRYP_CSGCMCCM5R"]
pub type W = crate::W<u32, super::CRYP_CSGCMCCM5R>;
#[doc = "Register CRYP_CSGCMCCM5R `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYP_CSGCMCCM5R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRYP_CSGCMCCM5R`"]
pub type CRYP_CSGCMCCM5R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRYP_CSGCMCCM5R`"]
pub struct CRYP_CSGCMCCM5R_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP_CSGCMCCM5R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRYP_CSGCMCCM5R"]
    #[inline(always)]
    pub fn cryp_csgcmccm5r(&self) -> CRYP_CSGCMCCM5R_R {
        CRYP_CSGCMCCM5R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRYP_CSGCMCCM5R"]
    #[inline(always)]
    pub fn cryp_csgcmccm5r(&mut self) -> CRYP_CSGCMCCM5R_W {
        CRYP_CSGCMCCM5R_W { w: self }
    }
}
