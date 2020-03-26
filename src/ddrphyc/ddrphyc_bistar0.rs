#[doc = "Reader of register DDRPHYC_BISTAR0"]
pub type R = crate::R<u32, super::DDRPHYC_BISTAR0>;
#[doc = "Writer for register DDRPHYC_BISTAR0"]
pub type W = crate::W<u32, super::DDRPHYC_BISTAR0>;
#[doc = "Register DDRPHYC_BISTAR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_BISTAR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BCOL`"]
pub type BCOL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BCOL`"]
pub struct BCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> BCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `BROW`"]
pub type BROW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BROW`"]
pub struct BROW_W<'a> {
    w: &'a mut W,
}
impl<'a> BROW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 12)) | (((value as u32) & 0xffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `BBANK`"]
pub type BBANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BBANK`"]
pub struct BBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BBANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - BCOL"]
    #[inline(always)]
    pub fn bcol(&self) -> BCOL_R {
        BCOL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:27 - BROW"]
    #[inline(always)]
    pub fn brow(&self) -> BROW_R {
        BROW_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - BBANK"]
    #[inline(always)]
    pub fn bbank(&self) -> BBANK_R {
        BBANK_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - BCOL"]
    #[inline(always)]
    pub fn bcol(&mut self) -> BCOL_W {
        BCOL_W { w: self }
    }
    #[doc = "Bits 12:27 - BROW"]
    #[inline(always)]
    pub fn brow(&mut self) -> BROW_W {
        BROW_W { w: self }
    }
    #[doc = "Bits 28:31 - BBANK"]
    #[inline(always)]
    pub fn bbank(&mut self) -> BBANK_W {
        BBANK_W { w: self }
    }
}
