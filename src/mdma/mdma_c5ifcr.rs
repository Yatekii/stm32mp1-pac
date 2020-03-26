#[doc = "Reader of register MDMA_C5IFCR"]
pub type R = crate::R<u32, super::MDMA_C5IFCR>;
#[doc = "Writer for register MDMA_C5IFCR"]
pub type W = crate::W<u32, super::MDMA_C5IFCR>;
#[doc = "Register MDMA_C5IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C5IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF5`"]
pub struct CTEIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF5_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF5`"]
pub struct CCTCIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF5_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF5`"]
pub struct CBRTIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF5_W<'a> {
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
#[doc = "Write proxy for field `CBTIF5`"]
pub struct CBTIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF5_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF5`"]
pub struct CLTCIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF5_W<'a> {
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
    #[doc = "Bit 0 - CTEIF5"]
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W {
        CTEIF5_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF5"]
    #[inline(always)]
    pub fn cctcif5(&mut self) -> CCTCIF5_W {
        CCTCIF5_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF5"]
    #[inline(always)]
    pub fn cbrtif5(&mut self) -> CBRTIF5_W {
        CBRTIF5_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF5"]
    #[inline(always)]
    pub fn cbtif5(&mut self) -> CBTIF5_W {
        CBTIF5_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF5"]
    #[inline(always)]
    pub fn cltcif5(&mut self) -> CLTCIF5_W {
        CLTCIF5_W { w: self }
    }
}
