#[doc = "Reader of register ISENABLER5"]
pub type R = crate::R<u32, super::ISENABLER5>;
#[doc = "Writer for register ISENABLER5"]
pub type W = crate::W<u32, super::ISENABLER5>;
#[doc = "Register ISENABLER5 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISENABLER5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISENABLER5`"]
pub type ISENABLER5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISENABLER5`"]
pub struct ISENABLER5_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENABLER5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - interrupt set-enable"]
    #[inline(always)]
    pub fn isenabler5(&self) -> ISENABLER5_R {
        ISENABLER5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - interrupt set-enable"]
    #[inline(always)]
    pub fn isenabler5(&mut self) -> ISENABLER5_W {
        ISENABLER5_W { w: self }
    }
}
