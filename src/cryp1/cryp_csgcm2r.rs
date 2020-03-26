#[doc = "Reader of register CRYP_CSGCM2R"]
pub type R = crate::R<u32, super::CRYP_CSGCM2R>;
#[doc = "Writer for register CRYP_CSGCM2R"]
pub type W = crate::W<u32, super::CRYP_CSGCM2R>;
#[doc = "Register CRYP_CSGCM2R `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYP_CSGCM2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRYP_CSGCM2R`"]
pub type CRYP_CSGCM2R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRYP_CSGCM2R`"]
pub struct CRYP_CSGCM2R_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP_CSGCM2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRYP_CSGCM2R"]
    #[inline(always)]
    pub fn cryp_csgcm2r(&self) -> CRYP_CSGCM2R_R {
        CRYP_CSGCM2R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRYP_CSGCM2R"]
    #[inline(always)]
    pub fn cryp_csgcm2r(&mut self) -> CRYP_CSGCM2R_W {
        CRYP_CSGCM2R_W { w: self }
    }
}
