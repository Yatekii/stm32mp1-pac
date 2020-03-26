#[doc = "Reader of register MDMA_C8IFCR"]
pub type R = crate::R<u32, super::MDMA_C8IFCR>;
#[doc = "Writer for register MDMA_C8IFCR"]
pub type W = crate::W<u32, super::MDMA_C8IFCR>;
#[doc = "Register MDMA_C8IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C8IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CTEIF8`"]
pub struct CTEIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF8_W<'a> {
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
#[doc = "Write proxy for field `CCTCIF8`"]
pub struct CCTCIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF8_W<'a> {
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
#[doc = "Write proxy for field `CBRTIF8`"]
pub struct CBRTIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> CBRTIF8_W<'a> {
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
#[doc = "Write proxy for field `CBTIF8`"]
pub struct CBTIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTIF8_W<'a> {
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
#[doc = "Write proxy for field `CLTCIF8`"]
pub struct CLTCIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> CLTCIF8_W<'a> {
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
    #[doc = "Bit 0 - CTEIF8"]
    #[inline(always)]
    pub fn cteif8(&mut self) -> CTEIF8_W {
        CTEIF8_W { w: self }
    }
    #[doc = "Bit 1 - CCTCIF8"]
    #[inline(always)]
    pub fn cctcif8(&mut self) -> CCTCIF8_W {
        CCTCIF8_W { w: self }
    }
    #[doc = "Bit 2 - CBRTIF8"]
    #[inline(always)]
    pub fn cbrtif8(&mut self) -> CBRTIF8_W {
        CBRTIF8_W { w: self }
    }
    #[doc = "Bit 3 - CBTIF8"]
    #[inline(always)]
    pub fn cbtif8(&mut self) -> CBTIF8_W {
        CBTIF8_W { w: self }
    }
    #[doc = "Bit 4 - CLTCIF8"]
    #[inline(always)]
    pub fn cltcif8(&mut self) -> CLTCIF8_W {
        CLTCIF8_W { w: self }
    }
}
