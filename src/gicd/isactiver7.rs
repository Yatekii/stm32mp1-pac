#[doc = "Reader of register ISACTIVER7"]
pub type R = crate::R<u32, super::ISACTIVER7>;
#[doc = "Writer for register ISACTIVER7"]
pub type W = crate::W<u32, super::ISACTIVER7>;
#[doc = "Register ISACTIVER7 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISACTIVER7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISACTIVER7`"]
pub type ISACTIVER7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISACTIVER7`"]
pub struct ISACTIVER7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISACTIVER7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - interrupt clear-pending"]
    #[inline(always)]
    pub fn isactiver7(&self) -> ISACTIVER7_R {
        ISACTIVER7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - interrupt clear-pending"]
    #[inline(always)]
    pub fn isactiver7(&mut self) -> ISACTIVER7_W {
        ISACTIVER7_W { w: self }
    }
}
