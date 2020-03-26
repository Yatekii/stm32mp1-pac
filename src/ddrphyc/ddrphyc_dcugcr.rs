#[doc = "Reader of register DDRPHYC_DCUGCR"]
pub type R = crate::R<u16, super::DDRPHYC_DCUGCR>;
#[doc = "Writer for register DDRPHYC_DCUGCR"]
pub type W = crate::W<u16, super::DDRPHYC_DCUGCR>;
#[doc = "Register DDRPHYC_DCUGCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_DCUGCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCSW`"]
pub type RCSW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RCSW`"]
pub struct RCSW_W<'a> {
    w: &'a mut W,
}
impl<'a> RCSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RCSW"]
    #[inline(always)]
    pub fn rcsw(&self) -> RCSW_R {
        RCSW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RCSW"]
    #[inline(always)]
    pub fn rcsw(&mut self) -> RCSW_W {
        RCSW_W { w: self }
    }
}
