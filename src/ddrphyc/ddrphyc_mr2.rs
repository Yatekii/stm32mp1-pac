#[doc = "Reader of register DDRPHYC_MR2"]
pub type R = crate::R<u16, super::DDRPHYC_MR2>;
#[doc = "Writer for register DDRPHYC_MR2"]
pub type W = crate::W<u16, super::DDRPHYC_MR2>;
#[doc = "Register DDRPHYC_MR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_MR2 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RLWL`"]
pub type RLWL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RLWL`"]
pub struct RLWL_W<'a> {
    w: &'a mut W,
}
impl<'a> RLWL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - RLWL"]
    #[inline(always)]
    pub fn rlwl(&self) -> RLWL_R {
        RLWL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RLWL"]
    #[inline(always)]
    pub fn rlwl(&mut self) -> RLWL_W {
        RLWL_W { w: self }
    }
}
