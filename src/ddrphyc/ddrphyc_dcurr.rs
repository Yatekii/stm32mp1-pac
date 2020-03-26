#[doc = "Reader of register DDRPHYC_DCURR"]
pub type R = crate::R<u32, super::DDRPHYC_DCURR>;
#[doc = "Writer for register DDRPHYC_DCURR"]
pub type W = crate::W<u32, super::DDRPHYC_DCURR>;
#[doc = "Register DDRPHYC_DCURR `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_DCURR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DINST`"]
pub type DINST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DINST`"]
pub struct DINST_W<'a> {
    w: &'a mut W,
}
impl<'a> DINST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `SADDR`"]
pub type SADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SADDR`"]
pub struct SADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `EADDR`"]
pub type EADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EADDR`"]
pub struct EADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> EADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NFAIL`"]
pub type NFAIL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFAIL`"]
pub struct NFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> NFAIL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
#[doc = "Reader of field `SONF`"]
pub type SONF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SONF`"]
pub struct SONF_W<'a> {
    w: &'a mut W,
}
impl<'a> SONF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SCOF`"]
pub type SCOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCOF`"]
pub struct SCOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCOF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `RCEN`"]
pub type RCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCEN`"]
pub struct RCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `XCEN`"]
pub type XCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCEN`"]
pub struct XCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XCEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DINST"]
    #[inline(always)]
    pub fn dinst(&self) -> DINST_R {
        DINST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SADDR"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EADDR"]
    #[inline(always)]
    pub fn eaddr(&self) -> EADDR_R {
        EADDR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:19 - NFAIL"]
    #[inline(always)]
    pub fn nfail(&self) -> NFAIL_R {
        NFAIL_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 20 - SONF"]
    #[inline(always)]
    pub fn sonf(&self) -> SONF_R {
        SONF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SCOF"]
    #[inline(always)]
    pub fn scof(&self) -> SCOF_R {
        SCOF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - RCEN"]
    #[inline(always)]
    pub fn rcen(&self) -> RCEN_R {
        RCEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - XCEN"]
    #[inline(always)]
    pub fn xcen(&self) -> XCEN_R {
        XCEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DINST"]
    #[inline(always)]
    pub fn dinst(&mut self) -> DINST_W {
        DINST_W { w: self }
    }
    #[doc = "Bits 4:7 - SADDR"]
    #[inline(always)]
    pub fn saddr(&mut self) -> SADDR_W {
        SADDR_W { w: self }
    }
    #[doc = "Bits 8:11 - EADDR"]
    #[inline(always)]
    pub fn eaddr(&mut self) -> EADDR_W {
        EADDR_W { w: self }
    }
    #[doc = "Bits 12:19 - NFAIL"]
    #[inline(always)]
    pub fn nfail(&mut self) -> NFAIL_W {
        NFAIL_W { w: self }
    }
    #[doc = "Bit 20 - SONF"]
    #[inline(always)]
    pub fn sonf(&mut self) -> SONF_W {
        SONF_W { w: self }
    }
    #[doc = "Bit 21 - SCOF"]
    #[inline(always)]
    pub fn scof(&mut self) -> SCOF_W {
        SCOF_W { w: self }
    }
    #[doc = "Bit 22 - RCEN"]
    #[inline(always)]
    pub fn rcen(&mut self) -> RCEN_W {
        RCEN_W { w: self }
    }
    #[doc = "Bit 23 - XCEN"]
    #[inline(always)]
    pub fn xcen(&mut self) -> XCEN_W {
        XCEN_W { w: self }
    }
}
