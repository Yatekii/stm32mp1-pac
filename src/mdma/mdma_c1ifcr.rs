#[doc = "Reader of register MDMA_C1IFCR"]
pub type R = crate::R<u32, super::MDMA_C1IFCR>;
#[doc = "Writer for register MDMA_C1IFCR"]
pub type W = crate::W<u32, super::MDMA_C1IFCR>;
#[doc = "Register MDMA_C1IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C1IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF1`"]
pub struct CTEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF1_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF1`"]
pub struct CCTCIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF1_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF1`"]
pub struct CBRTIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF1_W<'a> {
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
#[doc = "Write proxy for field `CBTIF1`"]
pub struct CBTIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF1_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF1`"]
pub struct CLTCIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF1_W<'a> {
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
    #[doc = "Bit 0 - CTEIF1"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W {
        CTEIF1_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF1"]
    #[inline(always)]
    pub fn cctcif1(&mut self) -> CCTCIF1_W {
        CCTCIF1_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF1"]
    #[inline(always)]
    pub fn cbrtif1(&mut self) -> CBRTIF1_W {
        CBRTIF1_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF1"]
    #[inline(always)]
    pub fn cbtif1(&mut self) -> CBTIF1_W {
        CBTIF1_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF1"]
    #[inline(always)]
    pub fn cltcif1(&mut self) -> CLTCIF1_W {
        CLTCIF1_W { w: self }
    }
}
