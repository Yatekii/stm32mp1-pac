#[doc = "Reader of register DDRPHYC_BISTWCR"]
pub type R = crate::R<u16, super::DDRPHYC_BISTWCR>;
#[doc = "Writer for register DDRPHYC_BISTWCR"]
pub type W = crate::W<u16, super::DDRPHYC_BISTWCR>;
#[doc = "Register DDRPHYC_BISTWCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_BISTWCR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BWCNT`"]
pub type BWCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BWCNT`"]
pub struct BWCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BWCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - BWCNT"]
    #[inline(always)]
    pub fn bwcnt(&self) -> BWCNT_R {
        BWCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BWCNT"]
    #[inline(always)]
    pub fn bwcnt(&mut self) -> BWCNT_W {
        BWCNT_W { w: self }
    }
}
