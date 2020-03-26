#[doc = "Reader of register ISENABLER0"]
pub type R = crate::R<u32, super::ISENABLER0>;
#[doc = "Writer for register ISENABLER0"]
pub type W = crate::W<u32, super::ISENABLER0>;
#[doc = "Register ISENABLER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISENABLER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISENABLER0`"]
pub type ISENABLER0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISENABLER0`"]
pub struct ISENABLER0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENABLER0_W<'a> {
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
    pub fn isenabler0(&self) -> ISENABLER0_R {
        ISENABLER0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - interrupt set-enable"]
    #[inline(always)]
    pub fn isenabler0(&mut self) -> ISENABLER0_W {
        ISENABLER0_W { w: self }
    }
}
