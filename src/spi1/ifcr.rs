#[doc = "Reader of register IFCR"]
pub type R = crate::R<u32, super::IFCR>;
#[doc = "Writer for register IFCR"]
pub type W = crate::W<u32, super::IFCR>;
#[doc = "Register IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EOTC`"]
pub type EOTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOTC`"]
pub struct EOTC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTC_W<'a> {
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
#[doc = "Reader of field `TXTFC`"]
pub type TXTFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXTFC`"]
pub struct TXTFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTFC_W<'a> {
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
#[doc = "Reader of field `UDRC`"]
pub type UDRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UDRC`"]
pub struct UDRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `OVRC`"]
pub type OVRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVRC`"]
pub struct OVRC_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CRCEC`"]
pub type CRCEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCEC`"]
pub struct CRCEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TIFREC`"]
pub type TIFREC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIFREC`"]
pub struct TIFREC_W<'a> {
    w: &'a mut W,
}
impl<'a> TIFREC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `MODFC`"]
pub type MODFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODFC`"]
pub struct MODFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MODFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TSERFC`"]
pub type TSERFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSERFC`"]
pub struct TSERFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSERFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SUSPC`"]
pub type SUSPC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUSPC`"]
pub struct SUSPC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - EOTC"]
    #[inline(always)]
    pub fn eotc(&self) -> EOTC_R {
        EOTC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXTFC"]
    #[inline(always)]
    pub fn txtfc(&self) -> TXTFC_R {
        TXTFC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UDRC"]
    #[inline(always)]
    pub fn udrc(&self) -> UDRC_R {
        UDRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OVRC"]
    #[inline(always)]
    pub fn ovrc(&self) -> OVRC_R {
        OVRC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRCEC"]
    #[inline(always)]
    pub fn crcec(&self) -> CRCEC_R {
        CRCEC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIFREC"]
    #[inline(always)]
    pub fn tifrec(&self) -> TIFREC_R {
        TIFREC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MODFC"]
    #[inline(always)]
    pub fn modfc(&self) -> MODFC_R {
        MODFC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TSERFC"]
    #[inline(always)]
    pub fn tserfc(&self) -> TSERFC_R {
        TSERFC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SUSPC"]
    #[inline(always)]
    pub fn suspc(&self) -> SUSPC_R {
        SUSPC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - EOTC"]
    #[inline(always)]
    pub fn eotc(&mut self) -> EOTC_W {
        EOTC_W { w: self }
    }
    #[doc = "Bit 4 - TXTFC"]
    #[inline(always)]
    pub fn txtfc(&mut self) -> TXTFC_W {
        TXTFC_W { w: self }
    }
    #[doc = "Bit 5 - UDRC"]
    #[inline(always)]
    pub fn udrc(&mut self) -> UDRC_W {
        UDRC_W { w: self }
    }
    #[doc = "Bit 6 - OVRC"]
    #[inline(always)]
    pub fn ovrc(&mut self) -> OVRC_W {
        OVRC_W { w: self }
    }
    #[doc = "Bit 7 - CRCEC"]
    #[inline(always)]
    pub fn crcec(&mut self) -> CRCEC_W {
        CRCEC_W { w: self }
    }
    #[doc = "Bit 8 - TIFREC"]
    #[inline(always)]
    pub fn tifrec(&mut self) -> TIFREC_W {
        TIFREC_W { w: self }
    }
    #[doc = "Bit 9 - MODFC"]
    #[inline(always)]
    pub fn modfc(&mut self) -> MODFC_W {
        MODFC_W { w: self }
    }
    #[doc = "Bit 10 - TSERFC"]
    #[inline(always)]
    pub fn tserfc(&mut self) -> TSERFC_W {
        TSERFC_W { w: self }
    }
    #[doc = "Bit 11 - SUSPC"]
    #[inline(always)]
    pub fn suspc(&mut self) -> SUSPC_W {
        SUSPC_W { w: self }
    }
}
