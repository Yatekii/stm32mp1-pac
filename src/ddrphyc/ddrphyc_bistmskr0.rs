#[doc = "Reader of register DDRPHYC_BISTMSKR0"]
pub type R = crate::R<u32, super::DDRPHYC_BISTMSKR0>;
#[doc = "Writer for register DDRPHYC_BISTMSKR0"]
pub type W = crate::W<u32, super::DDRPHYC_BISTMSKR0>;
#[doc = "Register DDRPHYC_BISTMSKR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_BISTMSKR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AMSK`"]
pub type AMSK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AMSK`"]
pub struct AMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> AMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `BAMSK`"]
pub type BAMSK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BAMSK`"]
pub struct BAMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> BAMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `WEMSK`"]
pub type WEMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WEMSK`"]
pub struct WEMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> WEMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `CKEMSK`"]
pub type CKEMSK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKEMSK`"]
pub struct CKEMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `CSMSK`"]
pub type CSMSK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSMSK`"]
pub struct CSMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CSMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `ODTMSK`"]
pub type ODTMSK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ODTMSK`"]
pub struct ODTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ODTMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - AMSK"]
    #[inline(always)]
    pub fn amsk(&self) -> AMSK_R {
        AMSK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - BAMSK"]
    #[inline(always)]
    pub fn bamsk(&self) -> BAMSK_R {
        BAMSK_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - WEMSK"]
    #[inline(always)]
    pub fn wemsk(&self) -> WEMSK_R {
        WEMSK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - CKEMSK"]
    #[inline(always)]
    pub fn ckemsk(&self) -> CKEMSK_R {
        CKEMSK_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CSMSK"]
    #[inline(always)]
    pub fn csmsk(&self) -> CSMSK_R {
        CSMSK_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - ODTMSK"]
    #[inline(always)]
    pub fn odtmsk(&self) -> ODTMSK_R {
        ODTMSK_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - AMSK"]
    #[inline(always)]
    pub fn amsk(&mut self) -> AMSK_W {
        AMSK_W { w: self }
    }
    #[doc = "Bits 16:18 - BAMSK"]
    #[inline(always)]
    pub fn bamsk(&mut self) -> BAMSK_W {
        BAMSK_W { w: self }
    }
    #[doc = "Bit 19 - WEMSK"]
    #[inline(always)]
    pub fn wemsk(&mut self) -> WEMSK_W {
        WEMSK_W { w: self }
    }
    #[doc = "Bits 20:23 - CKEMSK"]
    #[inline(always)]
    pub fn ckemsk(&mut self) -> CKEMSK_W {
        CKEMSK_W { w: self }
    }
    #[doc = "Bits 24:27 - CSMSK"]
    #[inline(always)]
    pub fn csmsk(&mut self) -> CSMSK_W {
        CSMSK_W { w: self }
    }
    #[doc = "Bits 28:31 - ODTMSK"]
    #[inline(always)]
    pub fn odtmsk(&mut self) -> ODTMSK_W {
        ODTMSK_W { w: self }
    }
}
