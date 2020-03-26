#[doc = "Reader of register DDRPHYC_BISTMSKR1"]
pub type R = crate::R<u32, super::DDRPHYC_BISTMSKR1>;
#[doc = "Writer for register DDRPHYC_BISTMSKR1"]
pub type W = crate::W<u32, super::DDRPHYC_BISTMSKR1>;
#[doc = "Register DDRPHYC_BISTMSKR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_BISTMSKR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DQMSK`"]
pub type DQMSK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DQMSK`"]
pub struct DQMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DQMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DMMSK`"]
pub type DMMSK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMMSK`"]
pub struct DMMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> DMMSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `RASMSK`"]
pub type RASMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RASMSK`"]
pub struct RASMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RASMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CASMSK`"]
pub type CASMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CASMSK`"]
pub struct CASMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CASMSK_W<'a> {
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
#[doc = "Reader of field `PARMSK`"]
pub type PARMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARMSK`"]
pub struct PARMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PARMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `TPDMASK`"]
pub type TPDMASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TPDMASK`"]
pub struct TPDMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TPDMASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DQMSK"]
    #[inline(always)]
    pub fn dqmsk(&self) -> DQMSK_R {
        DQMSK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - DMMSK"]
    #[inline(always)]
    pub fn dmmsk(&self) -> DMMSK_R {
        DMMSK_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - RASMSK"]
    #[inline(always)]
    pub fn rasmsk(&self) -> RASMSK_R {
        RASMSK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CASMSK"]
    #[inline(always)]
    pub fn casmsk(&self) -> CASMSK_R {
        CASMSK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 30 - PARMSK"]
    #[inline(always)]
    pub fn parmsk(&self) -> PARMSK_R {
        PARMSK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TPDMASK"]
    #[inline(always)]
    pub fn tpdmask(&self) -> TPDMASK_R {
        TPDMASK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - DQMSK"]
    #[inline(always)]
    pub fn dqmsk(&mut self) -> DQMSK_W {
        DQMSK_W { w: self }
    }
    #[doc = "Bits 16:17 - DMMSK"]
    #[inline(always)]
    pub fn dmmsk(&mut self) -> DMMSK_W {
        DMMSK_W { w: self }
    }
    #[doc = "Bit 18 - RASMSK"]
    #[inline(always)]
    pub fn rasmsk(&mut self) -> RASMSK_W {
        RASMSK_W { w: self }
    }
    #[doc = "Bit 19 - CASMSK"]
    #[inline(always)]
    pub fn casmsk(&mut self) -> CASMSK_W {
        CASMSK_W { w: self }
    }
    #[doc = "Bit 30 - PARMSK"]
    #[inline(always)]
    pub fn parmsk(&mut self) -> PARMSK_W {
        PARMSK_W { w: self }
    }
    #[doc = "Bit 31 - TPDMASK"]
    #[inline(always)]
    pub fn tpdmask(&mut self) -> TPDMASK_W {
        TPDMASK_W { w: self }
    }
}
