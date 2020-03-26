#[doc = "Reader of register MDMA_C12IFCR"]
pub type R = crate::R<u32, super::MDMA_C12IFCR>;
#[doc = "Writer for register MDMA_C12IFCR"]
pub type W = crate::W<u32, super::MDMA_C12IFCR>;
#[doc = "Register MDMA_C12IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C12IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF12`"]
pub struct CTEIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF12_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF12`"]
pub struct CCTCIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF12_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF12`"]
pub struct CBRTIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF12_W<'a> {
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
#[doc = "Write proxy for field `CBTIF12`"]
pub struct CBTIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF12_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF12`"]
pub struct CLTCIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF12_W<'a> {
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
    #[doc = "Bit 0 - CTEIF12"]
    #[inline(always)]
    pub fn cteif12(&mut self) -> CTEIF12_W {
        CTEIF12_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF12"]
    #[inline(always)]
    pub fn cctcif12(&mut self) -> CCTCIF12_W {
        CCTCIF12_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF12"]
    #[inline(always)]
    pub fn cbrtif12(&mut self) -> CBRTIF12_W {
        CBRTIF12_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF12"]
    #[inline(always)]
    pub fn cbtif12(&mut self) -> CBTIF12_W {
        CBTIF12_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF12"]
    #[inline(always)]
    pub fn cltcif12(&mut self) -> CLTCIF12_W {
        CLTCIF12_W { w: self }
    }
}
