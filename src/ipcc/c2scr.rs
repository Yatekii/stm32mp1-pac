#[doc = "Reader of register C2SCR"]
pub type R = crate::R<u32, super::C2SCR>;
#[doc = "Writer for register C2SCR"]
pub type W = crate::W<u32, super::C2SCR>;
#[doc = "Register C2SCR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::C2SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `CH1C`"]
pub type CH1C_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1C`"]
pub struct CH1C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1C_W<'a> {
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
#[doc = "Reader of field `CH2C`"]
pub type CH2C_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2C`"]
pub struct CH2C_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2C_W<'a> {
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
#[doc = "Reader of field `CH1S`"]
pub type CH1S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1S`"]
pub struct CH1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1S_W<'a> {
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
#[doc = "Reader of field `CH2S`"]
pub type CH2S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2S`"]
pub struct CH2S_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2S_W<'a> {
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
    #[doc = "Bit 0 - CH1C"]
    #[inline(always)]
    pub fn ch1c(&self) -> CH1C_R {
        CH1C_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH2C"]
    #[inline(always)]
    pub fn ch2c(&self) -> CH2C_R {
        CH2C_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CH1S"]
    #[inline(always)]
    pub fn ch1s(&self) -> CH1S_R {
        CH1S_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CH2S"]
    #[inline(always)]
    pub fn ch2s(&self) -> CH2S_R {
        CH2S_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH1C"]
    #[inline(always)]
    pub fn ch1c(&mut self) -> CH1C_W {
        CH1C_W { w: self }
    }
    #[doc = "Bit 1 - CH2C"]
    #[inline(always)]
    pub fn ch2c(&mut self) -> CH2C_W {
        CH2C_W { w: self }
    }
    #[doc = "Bit 16 - CH1S"]
    #[inline(always)]
    pub fn ch1s(&mut self) -> CH1S_W {
        CH1S_W { w: self }
    }
    #[doc = "Bit 17 - CH2S"]
    #[inline(always)]
    pub fn ch2s(&mut self) -> CH2S_W {
        CH2S_W { w: self }
    }
}
