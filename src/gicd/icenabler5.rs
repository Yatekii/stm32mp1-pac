#[doc = "Reader of register ICENABLER5"]
pub type R = crate::R<u32, super::ICENABLER5>;
#[doc = "Writer for register ICENABLER5"]
pub type W = crate::W<u32, super::ICENABLER5>;
#[doc = "Register ICENABLER5 `reset()`'s with value 0"]
impl crate::ResetValue for super::ICENABLER5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICENABLER5`"]
pub type ICENABLER5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICENABLER5`"]
pub struct ICENABLER5_W<'a> {
    w: &'a mut W,
}
impl<'a> ICENABLER5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - interrupt clear-enable 0"]
    #[inline(always)]
    pub fn icenabler5(&self) -> ICENABLER5_R {
        ICENABLER5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - interrupt clear-enable 0"]
    #[inline(always)]
    pub fn icenabler5(&mut self) -> ICENABLER5_W {
        ICENABLER5_W { w: self }
    }
}
