#[doc = "Reader of register ISENABLER7"]
pub type R = crate::R<u32, super::ISENABLER7>;
#[doc = "Writer for register ISENABLER7"]
pub type W = crate::W<u32, super::ISENABLER7>;
#[doc = "Register ISENABLER7 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISENABLER7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISENABLER7`"]
pub type ISENABLER7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISENABLER7`"]
pub struct ISENABLER7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENABLER7_W<'a> {
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
    pub fn isenabler7(&self) -> ISENABLER7_R {
        ISENABLER7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - interrupt set-enable"]
    #[inline(always)]
    pub fn isenabler7(&mut self) -> ISENABLER7_W {
        ISENABLER7_W { w: self }
    }
}
