#[doc = "Reader of register MDMA_C19IFCR"]
pub type R = crate::R<u32, super::MDMA_C19IFCR>;
#[doc = "Writer for register MDMA_C19IFCR"]
pub type W = crate::W<u32, super::MDMA_C19IFCR>;
#[doc = "Register MDMA_C19IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C19IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF15`"]
pub struct CTEIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF15_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF15`"]
pub struct CCTCIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF15_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF15`"]
pub struct CBRTIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF15_W<'a> {
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
#[doc = "Write proxy for field `CBTIF15`"]
pub struct CBTIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF15_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF15`"]
pub struct CLTCIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF15_W<'a> {
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
    #[doc = "Bit 0 - CTEIF15"]
    #[inline(always)]
    pub fn cteif15(&mut self) -> CTEIF15_W {
        CTEIF15_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF15"]
    #[inline(always)]
    pub fn cctcif15(&mut self) -> CCTCIF15_W {
        CCTCIF15_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF15"]
    #[inline(always)]
    pub fn cbrtif15(&mut self) -> CBRTIF15_W {
        CBRTIF15_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF15"]
    #[inline(always)]
    pub fn cbtif15(&mut self) -> CBTIF15_W {
        CBTIF15_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF15"]
    #[inline(always)]
    pub fn cltcif15(&mut self) -> CLTCIF15_W {
        CLTCIF15_W { w: self }
    }
}
