#[doc = "Reader of register QUADSPI_DLR"]
pub type R = crate::R<u32, super::QUADSPI_DLR>;
#[doc = "Writer for register QUADSPI_DLR"]
pub type W = crate::W<u32, super::QUADSPI_DLR>;
#[doc = "Register QUADSPI_DLR `reset()`'s with value 0"]
impl crate::ResetValue for super::QUADSPI_DLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DL`"]
pub type DL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DL`"]
pub struct DL_W<'a> {
    w: &'a mut W,
}
impl<'a> DL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DL"]
    #[inline(always)]
    pub fn dl(&self) -> DL_R {
        DL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DL"]
    #[inline(always)]
    pub fn dl(&mut self) -> DL_W {
        DL_W { w: self }
    }
}
