#[doc = "Reader of register DDRPHYC_BISTAR2"]
pub type R = crate::R<u32, super::DDRPHYC_BISTAR2>;
#[doc = "Writer for register DDRPHYC_BISTAR2"]
pub type W = crate::W<u32, super::DDRPHYC_BISTAR2>;
#[doc = "Register DDRPHYC_BISTAR2 `reset()`'s with value 0x7fff_ffff"]
impl crate::ResetValue for super::DDRPHYC_BISTAR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fff_ffff
    }
}
#[doc = "Reader of field `BMCOL`"]
pub type BMCOL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BMCOL`"]
pub struct BMCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> BMCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `BMROW`"]
pub type BMROW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BMROW`"]
pub struct BMROW_W<'a> {
    w: &'a mut W,
}
impl<'a> BMROW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 12)) | (((value as u32) & 0xffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `BMBANK`"]
pub type BMBANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BMBANK`"]
pub struct BMBANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BMBANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - BMCOL"]
    #[inline(always)]
    pub fn bmcol(&self) -> BMCOL_R {
        BMCOL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:27 - BMROW"]
    #[inline(always)]
    pub fn bmrow(&self) -> BMROW_R {
        BMROW_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - BMBANK"]
    #[inline(always)]
    pub fn bmbank(&self) -> BMBANK_R {
        BMBANK_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - BMCOL"]
    #[inline(always)]
    pub fn bmcol(&mut self) -> BMCOL_W {
        BMCOL_W { w: self }
    }
    #[doc = "Bits 12:27 - BMROW"]
    #[inline(always)]
    pub fn bmrow(&mut self) -> BMROW_W {
        BMROW_W { w: self }
    }
    #[doc = "Bits 28:31 - BMBANK"]
    #[inline(always)]
    pub fn bmbank(&mut self) -> BMBANK_W {
        BMBANK_W { w: self }
    }
}
