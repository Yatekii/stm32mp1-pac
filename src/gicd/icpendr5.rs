#[doc = "Reader of register ICPENDR5"]
pub type R = crate::R<u32, super::ICPENDR5>;
#[doc = "Writer for register ICPENDR5"]
pub type W = crate::W<u32, super::ICPENDR5>;
#[doc = "Register ICPENDR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::ICPENDR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICPENDR5`"]
pub type ICPENDR5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICPENDR5`"]
pub struct ICPENDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> ICPENDR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - interrupt clear-pending"]
    #[inline(always)]
    pub fn icpendr5(&self) -> ICPENDR5_R {
        ICPENDR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - interrupt clear-pending"]
    #[inline(always)]
    pub fn icpendr5(&mut self) -> ICPENDR5_W {
        ICPENDR5_W { w: self }
    }
}
