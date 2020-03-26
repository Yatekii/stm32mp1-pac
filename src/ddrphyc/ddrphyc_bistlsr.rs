#[doc = "Reader of register DDRPHYC_BISTLSR"]
pub type R = crate::R<u32, super::DDRPHYC_BISTLSR>;
#[doc = "Writer for register DDRPHYC_BISTLSR"]
pub type W = crate::W<u32, super::DDRPHYC_BISTLSR>;
#[doc = "Register DDRPHYC_BISTLSR `reset()`'s with value 0x1234_abcd"]
impl crate::ResetValue for super::DDRPHYC_BISTLSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1234_abcd
    }
}
#[doc = "Reader of field `BISTLSR`"]
pub type BISTLSR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BISTLSR`"]
pub struct BISTLSR_W<'a> {
    w: &'a mut W,
}
impl<'a> BISTLSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - BISTLSR"]
    #[inline(always)]
    pub fn bistlsr(&self) -> BISTLSR_R {
        BISTLSR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - BISTLSR"]
    #[inline(always)]
    pub fn bistlsr(&mut self) -> BISTLSR_W {
        BISTLSR_W { w: self }
    }
}
