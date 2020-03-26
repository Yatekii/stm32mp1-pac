#[doc = "Reader of register C2MR"]
pub type R = crate::R<u32, super::C2MR>;
#[doc = "Writer for register C2MR"]
pub type W = crate::W<u32, super::C2MR>;
#[doc = "Register C2MR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::C2MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `CH1OM`"]
pub type CH1OM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1OM`"]
pub struct CH1OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OM_W<'a> {
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
#[doc = "Reader of field `CH2OM`"]
pub type CH2OM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2OM`"]
pub struct CH2OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2OM_W<'a> {
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
#[doc = "Reader of field `CH1FM`"]
pub type CH1FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1FM`"]
pub struct CH1FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1FM_W<'a> {
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
#[doc = "Reader of field `CH2FM`"]
pub type CH2FM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2FM`"]
pub struct CH2FM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2FM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CH1OM"]
    #[inline(always)]
    pub fn ch1om(&self) -> CH1OM_R {
        CH1OM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH2OM"]
    #[inline(always)]
    pub fn ch2om(&self) -> CH2OM_R {
        CH2OM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CH1FM"]
    #[inline(always)]
    pub fn ch1fm(&self) -> CH1FM_R {
        CH1FM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CH2FM"]
    #[inline(always)]
    pub fn ch2fm(&self) -> CH2FM_R {
        CH2FM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH1OM"]
    #[inline(always)]
    pub fn ch1om(&mut self) -> CH1OM_W {
        CH1OM_W { w: self }
    }
    #[doc = "Bit 1 - CH2OM"]
    #[inline(always)]
    pub fn ch2om(&mut self) -> CH2OM_W {
        CH2OM_W { w: self }
    }
    #[doc = "Bit 16 - CH1FM"]
    #[inline(always)]
    pub fn ch1fm(&mut self) -> CH1FM_W {
        CH1FM_W { w: self }
    }
    #[doc = "Bit 17 - CH2FM"]
    #[inline(always)]
    pub fn ch2fm(&mut self) -> CH2FM_W {
        CH2FM_W { w: self }
    }
}
