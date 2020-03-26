#[doc = "Reader of register DDRPHYC_DCULR"]
pub type R = crate::R<u32, super::DDRPHYC_DCULR>;
#[doc = "Writer for register DDRPHYC_DCULR"]
pub type W = crate::W<u32, super::DDRPHYC_DCULR>;
#[doc = "Register DDRPHYC_DCULR `reset()`'s with value 0xf000_0000"]
impl crate::ResetValue for super::DDRPHYC_DCULR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xf000_0000
    }
}
#[doc = "Reader of field `LSADDR`"]
pub type LSADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSADDR`"]
pub struct LSADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `LEADDR`"]
pub type LEADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LEADDR`"]
pub struct LEADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `LCNT`"]
pub type LCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LCNT`"]
pub struct LCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `LFINF`"]
pub type LFINF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFINF`"]
pub struct LFINF_W<'a> {
    w: &'a mut W,
}
impl<'a> LFINF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `IDA`"]
pub type IDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDA`"]
pub struct IDA_W<'a> {
    w: &'a mut W,
}
impl<'a> IDA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `XLEADDR`"]
pub type XLEADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XLEADDR`"]
pub struct XLEADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> XLEADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - LSADDR"]
    #[inline(always)]
    pub fn lsaddr(&self) -> LSADDR_R {
        LSADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - LEADDR"]
    #[inline(always)]
    pub fn leaddr(&self) -> LEADDR_R {
        LEADDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - LCNT"]
    #[inline(always)]
    pub fn lcnt(&self) -> LCNT_R {
        LCNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - LFINF"]
    #[inline(always)]
    pub fn lfinf(&self) -> LFINF_R {
        LFINF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - IDA"]
    #[inline(always)]
    pub fn ida(&self) -> IDA_R {
        IDA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - XLEADDR"]
    #[inline(always)]
    pub fn xleaddr(&self) -> XLEADDR_R {
        XLEADDR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LSADDR"]
    #[inline(always)]
    pub fn lsaddr(&mut self) -> LSADDR_W {
        LSADDR_W { w: self }
    }
    #[doc = "Bits 4:7 - LEADDR"]
    #[inline(always)]
    pub fn leaddr(&mut self) -> LEADDR_W {
        LEADDR_W { w: self }
    }
    #[doc = "Bits 8:15 - LCNT"]
    #[inline(always)]
    pub fn lcnt(&mut self) -> LCNT_W {
        LCNT_W { w: self }
    }
    #[doc = "Bit 16 - LFINF"]
    #[inline(always)]
    pub fn lfinf(&mut self) -> LFINF_W {
        LFINF_W { w: self }
    }
    #[doc = "Bit 17 - IDA"]
    #[inline(always)]
    pub fn ida(&mut self) -> IDA_W {
        IDA_W { w: self }
    }
    #[doc = "Bits 28:31 - XLEADDR"]
    #[inline(always)]
    pub fn xleaddr(&mut self) -> XLEADDR_W {
        XLEADDR_W { w: self }
    }
}
