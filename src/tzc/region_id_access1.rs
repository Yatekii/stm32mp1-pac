#[doc = "Reader of register REGION_ID_ACCESS1"]
pub type R = crate::R<u32, super::REGION_ID_ACCESS1>;
#[doc = "Writer for register REGION_ID_ACCESS1"]
pub type W = crate::W<u32, super::REGION_ID_ACCESS1>;
#[doc = "Register REGION_ID_ACCESS1 `reset()`'s with value 0"]
impl crate::ResetValue for super::REGION_ID_ACCESS1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NSAID_RD_EN`"]
pub type NSAID_RD_EN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NSAID_RD_EN`"]
pub struct NSAID_RD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NSAID_RD_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `NSAID_WR_EN`"]
pub type NSAID_WR_EN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NSAID_WR_EN`"]
pub struct NSAID_WR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NSAID_WR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Region enable for each filter"]
    #[inline(always)]
    pub fn nsaid_rd_en(&self) -> NSAID_RD_EN_R {
        NSAID_RD_EN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Secure global write enable"]
    #[inline(always)]
    pub fn nsaid_wr_en(&self) -> NSAID_WR_EN_R {
        NSAID_WR_EN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Region enable for each filter"]
    #[inline(always)]
    pub fn nsaid_rd_en(&mut self) -> NSAID_RD_EN_W {
        NSAID_RD_EN_W { w: self }
    }
    #[doc = "Bits 16:31 - Secure global write enable"]
    #[inline(always)]
    pub fn nsaid_wr_en(&mut self) -> NSAID_WR_EN_W {
        NSAID_WR_EN_W { w: self }
    }
}
