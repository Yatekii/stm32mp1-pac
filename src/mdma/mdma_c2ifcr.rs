#[doc = "Reader of register MDMA_C2IFCR"]
pub type R = crate::R<u32, super::MDMA_C2IFCR>;
#[doc = "Writer for register MDMA_C2IFCR"]
pub type W = crate::W<u32, super::MDMA_C2IFCR>;
#[doc = "Register MDMA_C2IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C2IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF2`"]
pub struct CTEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF2_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF2`"]
pub struct CCTCIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF2_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF2`"]
pub struct CBRTIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF2_W<'a> {
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
#[doc = "Write proxy for field `CBTIF2`"]
pub struct CBTIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF2_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF2`"]
pub struct CLTCIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF2_W<'a> {
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
    #[doc = "Bit 0 - CTEIF2"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W {
        CTEIF2_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF2"]
    #[inline(always)]
    pub fn cctcif2(&mut self) -> CCTCIF2_W {
        CCTCIF2_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF2"]
    #[inline(always)]
    pub fn cbrtif2(&mut self) -> CBRTIF2_W {
        CBRTIF2_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF2"]
    #[inline(always)]
    pub fn cbtif2(&mut self) -> CBTIF2_W {
        CBTIF2_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF2"]
    #[inline(always)]
    pub fn cltcif2(&mut self) -> CLTCIF2_W {
        CLTCIF2_W { w: self }
    }
}
