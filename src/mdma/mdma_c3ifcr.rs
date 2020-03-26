#[doc = "Reader of register MDMA_C3IFCR"]
pub type R = crate::R<u32, super::MDMA_C3IFCR>;
#[doc = "Writer for register MDMA_C3IFCR"]
pub type W = crate::W<u32, super::MDMA_C3IFCR>;
#[doc = "Register MDMA_C3IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C3IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF3`"]
pub struct CTEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF3_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `CCTCIF3`"]
pub struct CCTCIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `CBRTIF3`"]
pub struct CBRTIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `CBTIF3`"]
pub struct CBTIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF3_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF3`"]
pub struct CLTCIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF3_W<'a> {
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
impl R {}
impl W {
    #[doc = "Bit 0 - CTEIF3"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W {
        CTEIF3_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF3"]
    #[inline(always)]
    pub fn cctcif3(&mut self) -> CCTCIF3_W {
        CCTCIF3_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF3"]
    #[inline(always)]
    pub fn cbrtif3(&mut self) -> CBRTIF3_W {
        CBRTIF3_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF3"]
    #[inline(always)]
    pub fn cbtif3(&mut self) -> CBTIF3_W {
        CBTIF3_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF3"]
    #[inline(always)]
    pub fn cltcif3(&mut self) -> CLTCIF3_W {
        CLTCIF3_W { w: self }
    }
}
