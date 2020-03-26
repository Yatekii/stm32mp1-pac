#[doc = "Reader of register MDMA_C0IFCR"]
pub type R = crate::R<u32, super::MDMA_C0IFCR>;
#[doc = "Writer for register MDMA_C0IFCR"]
pub type W = crate::W<u32, super::MDMA_C0IFCR>;
#[doc = "Register MDMA_C0IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C0IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF0`"]
pub struct CTEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF0_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF0`"]
pub struct CCTCIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF0_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF0`"]
pub struct CBRTIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF0_W<'a> {
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
#[doc = "Write proxy for field `CBTIF0`"]
pub struct CBTIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF0_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF0`"]
pub struct CLTCIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF0_W<'a> {
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
    #[doc = "Bit 0 - CTEIF0"]
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF0_W {
        CTEIF0_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF0"]
    #[inline(always)]
    pub fn cctcif0(&mut self) -> CCTCIF0_W {
        CCTCIF0_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF0"]
    #[inline(always)]
    pub fn cbrtif0(&mut self) -> CBRTIF0_W {
        CBRTIF0_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF0"]
    #[inline(always)]
    pub fn cbtif0(&mut self) -> CBTIF0_W {
        CBTIF0_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF0"]
    #[inline(always)]
    pub fn cltcif0(&mut self) -> CLTCIF0_W {
        CLTCIF0_W { w: self }
    }
}
