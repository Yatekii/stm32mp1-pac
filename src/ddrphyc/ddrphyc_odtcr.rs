#[doc = "Reader of register DDRPHYC_ODTCR"]
pub type R = crate::R<u32, super::DDRPHYC_ODTCR>;
#[doc = "Writer for register DDRPHYC_ODTCR"]
pub type W = crate::W<u32, super::DDRPHYC_ODTCR>;
#[doc = "Register DDRPHYC_ODTCR `reset()`'s with value 0x8421_0000"]
impl crate::ResetValue for super::DDRPHYC_ODTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8421_0000
    }
}
#[doc = "Reader of field `RDODT0`"]
pub type RDODT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDODT0`"]
pub struct RDODT0_W<'a> {
    w: &'a mut W,
}
impl<'a> RDODT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RDODT1`"]
pub type RDODT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDODT1`"]
pub struct RDODT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RDODT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `RDODT2`"]
pub type RDODT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDODT2`"]
pub struct RDODT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RDODT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RDODT3`"]
pub type RDODT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDODT3`"]
pub struct RDODT3_W<'a> {
    w: &'a mut W,
}
impl<'a> RDODT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `WRODT0`"]
pub type WRODT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRODT0`"]
pub struct WRODT0_W<'a> {
    w: &'a mut W,
}
impl<'a> WRODT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WRODT1`"]
pub type WRODT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRODT1`"]
pub struct WRODT1_W<'a> {
    w: &'a mut W,
}
impl<'a> WRODT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `WRODT2`"]
pub type WRODT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRODT2`"]
pub struct WRODT2_W<'a> {
    w: &'a mut W,
}
impl<'a> WRODT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `WRODT3`"]
pub type WRODT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRODT3`"]
pub struct WRODT3_W<'a> {
    w: &'a mut W,
}
impl<'a> WRODT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - RDODT0"]
    #[inline(always)]
    pub fn rdodt0(&self) -> RDODT0_R {
        RDODT0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RDODT1"]
    #[inline(always)]
    pub fn rdodt1(&self) -> RDODT1_R {
        RDODT1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RDODT2"]
    #[inline(always)]
    pub fn rdodt2(&self) -> RDODT2_R {
        RDODT2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RDODT3"]
    #[inline(always)]
    pub fn rdodt3(&self) -> RDODT3_R {
        RDODT3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - WRODT0"]
    #[inline(always)]
    pub fn wrodt0(&self) -> WRODT0_R {
        WRODT0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - WRODT1"]
    #[inline(always)]
    pub fn wrodt1(&self) -> WRODT1_R {
        WRODT1_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - WRODT2"]
    #[inline(always)]
    pub fn wrodt2(&self) -> WRODT2_R {
        WRODT2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - WRODT3"]
    #[inline(always)]
    pub fn wrodt3(&self) -> WRODT3_R {
        WRODT3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RDODT0"]
    #[inline(always)]
    pub fn rdodt0(&mut self) -> RDODT0_W {
        RDODT0_W { w: self }
    }
    #[doc = "Bits 4:7 - RDODT1"]
    #[inline(always)]
    pub fn rdodt1(&mut self) -> RDODT1_W {
        RDODT1_W { w: self }
    }
    #[doc = "Bits 8:11 - RDODT2"]
    #[inline(always)]
    pub fn rdodt2(&mut self) -> RDODT2_W {
        RDODT2_W { w: self }
    }
    #[doc = "Bits 12:15 - RDODT3"]
    #[inline(always)]
    pub fn rdodt3(&mut self) -> RDODT3_W {
        RDODT3_W { w: self }
    }
    #[doc = "Bits 16:19 - WRODT0"]
    #[inline(always)]
    pub fn wrodt0(&mut self) -> WRODT0_W {
        WRODT0_W { w: self }
    }
    #[doc = "Bits 20:23 - WRODT1"]
    #[inline(always)]
    pub fn wrodt1(&mut self) -> WRODT1_W {
        WRODT1_W { w: self }
    }
    #[doc = "Bits 24:27 - WRODT2"]
    #[inline(always)]
    pub fn wrodt2(&mut self) -> WRODT2_W {
        WRODT2_W { w: self }
    }
    #[doc = "Bits 28:31 - WRODT3"]
    #[inline(always)]
    pub fn wrodt3(&mut self) -> WRODT3_W {
        WRODT3_W { w: self }
    }
}
