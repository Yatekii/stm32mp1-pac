#[doc = "Reader of register MDMA_C14IFCR"]
pub type R = crate::R<u32, super::MDMA_C14IFCR>;
#[doc = "Writer for register MDMA_C14IFCR"]
pub type W = crate::W<u32, super::MDMA_C14IFCR>;
#[doc = "Register MDMA_C14IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C14IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF14`"]
pub struct CTEIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF14_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF14`"]
pub struct CCTCIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF14_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF14`"]
pub struct CBRTIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF14_W<'a> {
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
#[doc = "Write proxy for field `CBTIF14`"]
pub struct CBTIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF14_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF14`"]
pub struct CLTCIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF14_W<'a> {
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
    #[doc = "Bit 0 - CTEIF14"]
    #[inline(always)]
    pub fn cteif14(&mut self) -> CTEIF14_W {
        CTEIF14_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF14"]
    #[inline(always)]
    pub fn cctcif14(&mut self) -> CCTCIF14_W {
        CCTCIF14_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF14"]
    #[inline(always)]
    pub fn cbrtif14(&mut self) -> CBRTIF14_W {
        CBRTIF14_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF14"]
    #[inline(always)]
    pub fn cbtif14(&mut self) -> CBTIF14_W {
        CBTIF14_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF14"]
    #[inline(always)]
    pub fn cltcif14(&mut self) -> CLTCIF14_W {
        CLTCIF14_W { w: self }
    }
}
