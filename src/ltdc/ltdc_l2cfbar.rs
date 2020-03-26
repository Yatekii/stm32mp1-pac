#[doc = "Reader of register LTDC_L2CFBAR"]
pub type R = crate::R<u32, super::LTDC_L2CFBAR>;
#[doc = "Writer for register LTDC_L2CFBAR"]
pub type W = crate::W<u32, super::LTDC_L2CFBAR>;
#[doc = "Register LTDC_L2CFBAR `reset()`'s with value 0"]
impl crate::ResetValue for super::LTDC_L2CFBAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFBADD`"]
pub type CFBADD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CFBADD`"]
pub struct CFBADD_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CFBADD"]
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CFBADD"]
    #[inline(always)]
    pub fn cfbadd(&mut self) -> CFBADD_W {
        CFBADD_W { w: self }
    }
}
