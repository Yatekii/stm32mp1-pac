#[doc = "Reader of register ISPENDR8"]
pub type R = crate::R<u32, super::ISPENDR8>;
#[doc = "Writer for register ISPENDR8"]
pub type W = crate::W<u32, super::ISPENDR8>;
#[doc = "Register ISPENDR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISPENDR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISPENDR8`"]
pub type ISPENDR8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISPENDR8`"]
pub struct ISPENDR8_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPENDR8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - interrupt set-pending"]
    #[inline(always)]
    pub fn ispendr8(&self) -> ISPENDR8_R {
        ISPENDR8_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - interrupt set-pending"]
    #[inline(always)]
    pub fn ispendr8(&mut self) -> ISPENDR8_W {
        ISPENDR8_W { w: self }
    }
}
