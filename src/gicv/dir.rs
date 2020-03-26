#[doc = "Writer for register DIR"]
pub type W = crate::W<u32, super::DIR>;
#[doc = "Register DIR `reset()`'s with value 0x0102_143b"]
impl crate::ResetValue for super::DIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0102_143b
    }
}
#[doc = "Write proxy for field `IIDR`"]
pub struct IIDR_W<'a> {
    w: &'a mut W,
}
impl<'a> IIDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - IIDR"]
    #[inline(always)]
    pub fn iidr(&mut self) -> IIDR_W {
        IIDR_W { w: self }
    }
}
