#[doc = "Reader of register DDRPHYC_BISTRR"]
pub type R = crate::R<u32, super::DDRPHYC_BISTRR>;
#[doc = "Writer for register DDRPHYC_BISTRR"]
pub type W = crate::W<u32, super::DDRPHYC_BISTRR>;
#[doc = "Register DDRPHYC_BISTRR `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_BISTRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BINST`"]
pub type BINST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BINST`"]
pub struct BINST_W<'a> {
    w: &'a mut W,
}
impl<'a> BINST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `BMODE`"]
pub type BMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BMODE`"]
pub struct BMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `BINF`"]
pub type BINF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BINF`"]
pub struct BINF_W<'a> {
    w: &'a mut W,
}
impl<'a> BINF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
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
        self.w.bits = (self.w.bits & !(0xff << 5)) | (((value as u32) & 0xff) << 5);
        self.w
    }
}
#[doc = "Reader of field `BSCONF`"]
pub type BSCONF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSCONF`"]
pub struct BSCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> BSCONF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `BDXEN`"]
pub type BDXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BDXEN`"]
pub struct BDXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BDXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `BACEN`"]
pub type BACEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BACEN`"]
pub struct BACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BACEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `BDMEN`"]
pub type BDMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BDMEN`"]
pub struct BDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `BDPAT`"]
pub type BDPAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BDPAT`"]
pub struct BDPAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BDPAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `BDXSEL`"]
pub type BDXSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BDXSEL`"]
pub struct BDXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BDXSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | (((value as u32) & 0x0f) << 19);
        self.w
    }
}
#[doc = "Reader of field `BCKSEL`"]
pub type BCKSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BCKSEL`"]
pub struct BCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BCKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - BINST"]
    #[inline(always)]
    pub fn binst(&self) -> BINST_R {
        BINST_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - BMODE"]
    #[inline(always)]
    pub fn bmode(&self) -> BMODE_R {
        BMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BINF"]
    #[inline(always)]
    pub fn binf(&self) -> BINF_R {
        BINF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:12 - NFAIL"]
    #[inline(always)]
    pub fn nfail(&self) -> NFAIL_R {
        NFAIL_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13 - BSCONF"]
    #[inline(always)]
    pub fn bsconf(&self) -> BSCONF_R {
        BSCONF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - BDXEN"]
    #[inline(always)]
    pub fn bdxen(&self) -> BDXEN_R {
        BDXEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - BACEN"]
    #[inline(always)]
    pub fn bacen(&self) -> BACEN_R {
        BACEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BDMEN"]
    #[inline(always)]
    pub fn bdmen(&self) -> BDMEN_R {
        BDMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - BDPAT"]
    #[inline(always)]
    pub fn bdpat(&self) -> BDPAT_R {
        BDPAT_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:22 - BDXSEL"]
    #[inline(always)]
    pub fn bdxsel(&self) -> BDXSEL_R {
        BDXSEL_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:25 - BCKSEL"]
    #[inline(always)]
    pub fn bcksel(&self) -> BCKSEL_R {
        BCKSEL_R::new(((self.bits >> 23) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BINST"]
    #[inline(always)]
    pub fn binst(&mut self) -> BINST_W {
        BINST_W { w: self }
    }
    #[doc = "Bit 3 - BMODE"]
    #[inline(always)]
    pub fn bmode(&mut self) -> BMODE_W {
        BMODE_W { w: self }
    }
    #[doc = "Bit 4 - BINF"]
    #[inline(always)]
    pub fn binf(&mut self) -> BINF_W {
        BINF_W { w: self }
    }
    #[doc = "Bits 5:12 - NFAIL"]
    #[inline(always)]
    pub fn nfail(&mut self) -> NFAIL_W {
        NFAIL_W { w: self }
    }
    #[doc = "Bit 13 - BSCONF"]
    #[inline(always)]
    pub fn bsconf(&mut self) -> BSCONF_W {
        BSCONF_W { w: self }
    }
    #[doc = "Bit 14 - BDXEN"]
    #[inline(always)]
    pub fn bdxen(&mut self) -> BDXEN_W {
        BDXEN_W { w: self }
    }
    #[doc = "Bit 15 - BACEN"]
    #[inline(always)]
    pub fn bacen(&mut self) -> BACEN_W {
        BACEN_W { w: self }
    }
    #[doc = "Bit 16 - BDMEN"]
    #[inline(always)]
    pub fn bdmen(&mut self) -> BDMEN_W {
        BDMEN_W { w: self }
    }
    #[doc = "Bits 17:18 - BDPAT"]
    #[inline(always)]
    pub fn bdpat(&mut self) -> BDPAT_W {
        BDPAT_W { w: self }
    }
    #[doc = "Bits 19:22 - BDXSEL"]
    #[inline(always)]
    pub fn bdxsel(&mut self) -> BDXSEL_W {
        BDXSEL_W { w: self }
    }
    #[doc = "Bits 23:25 - BCKSEL"]
    #[inline(always)]
    pub fn bcksel(&mut self) -> BCKSEL_W {
        BCKSEL_W { w: self }
    }
}
