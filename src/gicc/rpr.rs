#[doc = "Writer for register RPR"]
pub type W = crate::W<u32, super::RPR>;
#[doc = "Register RPR `reset()`'s with value 0xff"]
impl crate::ResetValue for super::RPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Write proxy for field `PRIORITY`"]
pub struct PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
impl W {
    #[doc = "Bits 3:7 - current running priority on the CPU interface"]
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W {
        PRIORITY_W { w: self }
    }
}
