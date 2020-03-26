#[doc = "Reader of register MDMA_C10IFCR"]
pub type R = crate::R<u32, super::MDMA_C10IFCR>;
#[doc = "Writer for register MDMA_C10IFCR"]
pub type W = crate::W<u32, super::MDMA_C10IFCR>;
#[doc = "Register MDMA_C10IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C10IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF10`"]
pub struct CTEIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF10_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF10`"]
pub struct CCTCIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF10_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF10`"]
pub struct CBRTIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF10_W<'a> {
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
#[doc = "Write proxy for field `CBTIF10`"]
pub struct CBTIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF10_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF10`"]
pub struct CLTCIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF10_W<'a> {
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
    #[doc = "Bit 0 - CTEIF10"]
    #[inline(always)]
    pub fn cteif10(&mut self) -> CTEIF10_W {
        CTEIF10_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF10"]
    #[inline(always)]
    pub fn cctcif10(&mut self) -> CCTCIF10_W {
        CCTCIF10_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF10"]
    #[inline(always)]
    pub fn cbrtif10(&mut self) -> CBRTIF10_W {
        CBRTIF10_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF10"]
    #[inline(always)]
    pub fn cbtif10(&mut self) -> CBTIF10_W {
        CBTIF10_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF10"]
    #[inline(always)]
    pub fn cltcif10(&mut self) -> CLTCIF10_W {
        CLTCIF10_W { w: self }
    }
}
