#[doc = "Reader of register MDMA_C4IFCR"]
pub type R = crate::R<u32, super::MDMA_C4IFCR>;
#[doc = "Writer for register MDMA_C4IFCR"]
pub type W = crate::W<u32, super::MDMA_C4IFCR>;
#[doc = "Register MDMA_C4IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C4IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF4`"]
pub struct CTEIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF4_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF4`"]
pub struct CCTCIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF4_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF4`"]
pub struct CBRTIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF4_W<'a> {
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
#[doc = "Write proxy for field `CBTIF4`"]
pub struct CBTIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF4_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF4`"]
pub struct CLTCIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF4_W<'a> {
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
    #[doc = "Bit 0 - CTEIF4"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W {
        CTEIF4_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF4"]
    #[inline(always)]
    pub fn cctcif4(&mut self) -> CCTCIF4_W {
        CCTCIF4_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF4"]
    #[inline(always)]
    pub fn cbrtif4(&mut self) -> CBRTIF4_W {
        CBRTIF4_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF4"]
    #[inline(always)]
    pub fn cbtif4(&mut self) -> CBTIF4_W {
        CBTIF4_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF4"]
    #[inline(always)]
    pub fn cltcif4(&mut self) -> CLTCIF4_W {
        CLTCIF4_W { w: self }
    }
}
