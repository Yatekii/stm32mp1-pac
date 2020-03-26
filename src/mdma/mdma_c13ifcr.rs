#[doc = "Reader of register MDMA_C13IFCR"]
pub type R = crate::R<u32, super::MDMA_C13IFCR>;
#[doc = "Writer for register MDMA_C13IFCR"]
pub type W = crate::W<u32, super::MDMA_C13IFCR>;
#[doc = "Register MDMA_C13IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C13IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF13`"]
pub struct CTEIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF13_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF13`"]
pub struct CCTCIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF13_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF13`"]
pub struct CBRTIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF13_W<'a> {
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
#[doc = "Write proxy for field `CBTIF13`"]
pub struct CBTIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF13_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF13`"]
pub struct CLTCIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF13_W<'a> {
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
    #[doc = "Bit 0 - CTEIF13"]
    #[inline(always)]
    pub fn cteif13(&mut self) -> CTEIF13_W {
        CTEIF13_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF13"]
    #[inline(always)]
    pub fn cctcif13(&mut self) -> CCTCIF13_W {
        CCTCIF13_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF13"]
    #[inline(always)]
    pub fn cbrtif13(&mut self) -> CBRTIF13_W {
        CBRTIF13_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF13"]
    #[inline(always)]
    pub fn cbtif13(&mut self) -> CBTIF13_W {
        CBTIF13_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF13"]
    #[inline(always)]
    pub fn cltcif13(&mut self) -> CLTCIF13_W {
        CLTCIF13_W { w: self }
    }
}
