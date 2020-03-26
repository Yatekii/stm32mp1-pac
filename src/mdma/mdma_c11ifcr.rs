#[doc = "Reader of register MDMA_C11IFCR"]
pub type R = crate::R<u32, super::MDMA_C11IFCR>;
#[doc = "Writer for register MDMA_C11IFCR"]
pub type W = crate::W<u32, super::MDMA_C11IFCR>;
#[doc = "Register MDMA_C11IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C11IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF11`"]
pub struct CTEIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF11_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF11`"]
pub struct CCTCIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF11_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF11`"]
pub struct CBRTIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF11_W<'a> {
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
#[doc = "Write proxy for field `CBTIF11`"]
pub struct CBTIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF11_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF11`"]
pub struct CLTCIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF11_W<'a> {
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
    #[doc = "Bit 0 - CTEIF11"]
    #[inline(always)]
    pub fn cteif11(&mut self) -> CTEIF11_W {
        CTEIF11_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF11"]
    #[inline(always)]
    pub fn cctcif11(&mut self) -> CCTCIF11_W {
        CCTCIF11_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF11"]
    #[inline(always)]
    pub fn cbrtif11(&mut self) -> CBRTIF11_W {
        CBRTIF11_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF11"]
    #[inline(always)]
    pub fn cbtif11(&mut self) -> CBTIF11_W {
        CBTIF11_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF11"]
    #[inline(always)]
    pub fn cltcif11(&mut self) -> CLTCIF11_W {
        CLTCIF11_W { w: self }
    }
}
