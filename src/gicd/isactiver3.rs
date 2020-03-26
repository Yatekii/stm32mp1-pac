#[doc = "Reader of register ISACTIVER3"]
pub type R = crate::R<u32, super::ISACTIVER3>;
#[doc = "Writer for register ISACTIVER3"]
pub type W = crate::W<u32, super::ISACTIVER3>;
#[doc = "Register ISACTIVER3 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISACTIVER3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISACTIVER3`"]
pub type ISACTIVER3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISACTIVER3`"]
pub struct ISACTIVER3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISACTIVER3_W<'a> {
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
    pub fn isactiver3(&self) -> ISACTIVER3_R {
        ISACTIVER3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - interrupt clear-pending"]
    #[inline(always)]
    pub fn isactiver3(&mut self) -> ISACTIVER3_W {
        ISACTIVER3_W { w: self }
    }
}
