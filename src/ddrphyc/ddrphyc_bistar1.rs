#[doc = "Reader of register DDRPHYC_BISTAR1"]
pub type R = crate::R<u16, super::DDRPHYC_BISTAR1>;
#[doc = "Writer for register DDRPHYC_BISTAR1"]
pub type W = crate::W<u16, super::DDRPHYC_BISTAR1>;
#[doc = "Register DDRPHYC_BISTAR1 `reset()`'s with value 0x0c"]
impl crate::ResetValue for super::DDRPHYC_BISTAR1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c
    }
}
#[doc = "Reader of field `BRANK`"]
pub type BRANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRANK`"]
pub struct BRANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BRANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u16) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `BMRANK`"]
pub type BMRANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BMRANK`"]
pub struct BMRANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BMRANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u16) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `BAINC`"]
pub type BAINC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BAINC`"]
pub struct BAINC_W<'a> {
    w: &'a mut W,
}
impl<'a> BAINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u16) & 0x0fff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - BRANK"]
    #[inline(always)]
    pub fn brank(&self) -> BRANK_R {
        BRANK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - BMRANK"]
    #[inline(always)]
    pub fn bmrank(&self) -> BMRANK_R {
        BMRANK_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:15 - BAINC"]
    #[inline(always)]
    pub fn bainc(&self) -> BAINC_R {
        BAINC_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - BRANK"]
    #[inline(always)]
    pub fn brank(&mut self) -> BRANK_W {
        BRANK_W { w: self }
    }
    #[doc = "Bits 2:3 - BMRANK"]
    #[inline(always)]
    pub fn bmrank(&mut self) -> BMRANK_W {
        BMRANK_W { w: self }
    }
    #[doc = "Bits 4:15 - BAINC"]
    #[inline(always)]
    pub fn bainc(&mut self) -> BAINC_W {
        BAINC_W { w: self }
    }
}
