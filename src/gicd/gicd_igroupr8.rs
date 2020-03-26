#[doc = "Reader of register GICD_IGROUPR8"]
pub type R = crate::R<u32, super::GICD_IGROUPR8>;
#[doc = "Writer for register GICD_IGROUPR8"]
pub type W = crate::W<u32, super::GICD_IGROUPR8>;
#[doc = "Register GICD_IGROUPR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_IGROUPR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IGROUPR8`"]
pub type IGROUPR8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IGROUPR8`"]
pub struct IGROUPR8_W<'a> {
    w: &'a mut W,
}
impl<'a> IGROUPR8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - group of interrupts"]
    #[inline(always)]
    pub fn igroupr8(&self) -> IGROUPR8_R {
        IGROUPR8_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - group of interrupts"]
    #[inline(always)]
    pub fn igroupr8(&mut self) -> IGROUPR8_W {
        IGROUPR8_W { w: self }
    }
}
