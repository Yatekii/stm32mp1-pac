#[doc = "Reader of register DDRPHYC_DCUTPR"]
pub type R = crate::R<u32, super::DDRPHYC_DCUTPR>;
#[doc = "Writer for register DDRPHYC_DCUTPR"]
pub type W = crate::W<u32, super::DDRPHYC_DCUTPR>;
#[doc = "Register DDRPHYC_DCUTPR `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_DCUTPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TDCUTO`"]
pub type TDCUTO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDCUTO`"]
pub struct TDCUTO_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCUTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TDCUT1`"]
pub type TDCUT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDCUT1`"]
pub struct TDCUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCUT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TDCUT2`"]
pub type TDCUT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDCUT2`"]
pub struct TDCUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCUT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TDCUT3`"]
pub type TDCUT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDCUT3`"]
pub struct TDCUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCUT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TDCUTO"]
    #[inline(always)]
    pub fn tdcuto(&self) -> TDCUTO_R {
        TDCUTO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TDCUT1"]
    #[inline(always)]
    pub fn tdcut1(&self) -> TDCUT1_R {
        TDCUT1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TDCUT2"]
    #[inline(always)]
    pub fn tdcut2(&self) -> TDCUT2_R {
        TDCUT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TDCUT3"]
    #[inline(always)]
    pub fn tdcut3(&self) -> TDCUT3_R {
        TDCUT3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TDCUTO"]
    #[inline(always)]
    pub fn tdcuto(&mut self) -> TDCUTO_W {
        TDCUTO_W { w: self }
    }
    #[doc = "Bits 8:15 - TDCUT1"]
    #[inline(always)]
    pub fn tdcut1(&mut self) -> TDCUT1_W {
        TDCUT1_W { w: self }
    }
    #[doc = "Bits 16:23 - TDCUT2"]
    #[inline(always)]
    pub fn tdcut2(&mut self) -> TDCUT2_W {
        TDCUT2_W { w: self }
    }
    #[doc = "Bits 24:31 - TDCUT3"]
    #[inline(always)]
    pub fn tdcut3(&mut self) -> TDCUT3_W {
        TDCUT3_W { w: self }
    }
}
