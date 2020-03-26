#[doc = "Reader of register MDMA_C6IFCR"]
pub type R = crate::R<u32, super::MDMA_C6IFCR>;
#[doc = "Writer for register MDMA_C6IFCR"]
pub type W = crate::W<u32, super::MDMA_C6IFCR>;
#[doc = "Register MDMA_C6IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C6IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF6`"]
pub struct CTEIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF6_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF6`"]
pub struct CCTCIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF6_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF6`"]
pub struct CBRTIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF6_W<'a> {
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
#[doc = "Write proxy for field `CBTIF6`"]
pub struct CBTIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF6_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF6`"]
pub struct CLTCIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF6_W<'a> {
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
    #[doc = "Bit 0 - CTEIF6"]
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W {
        CTEIF6_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF6"]
    #[inline(always)]
    pub fn cctcif6(&mut self) -> CCTCIF6_W {
        CCTCIF6_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF6"]
    #[inline(always)]
    pub fn cbrtif6(&mut self) -> CBRTIF6_W {
        CBRTIF6_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF6"]
    #[inline(always)]
    pub fn cbtif6(&mut self) -> CBTIF6_W {
        CBTIF6_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF6"]
    #[inline(always)]
    pub fn cltcif6(&mut self) -> CLTCIF6_W {
        CLTCIF6_W { w: self }
    }
}
