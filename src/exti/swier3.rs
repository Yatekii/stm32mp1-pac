#[doc = "Reader of register SWIER3"]
pub type R = crate::R<u32, super::SWIER3>;
#[doc = "Writer for register SWIER3"]
pub type W = crate::W<u32, super::SWIER3>;
#[doc = "Register SWIER3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SWIER3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SW65`"]
pub type SW65_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW65`"]
pub struct SW65_W<'a> {
    w: &'a mut W,
}
impl<'a> SW65_W<'a> {
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
#[doc = "Reader of field `SW66`"]
pub type SW66_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW66`"]
pub struct SW66_W<'a> {
    w: &'a mut W,
}
impl<'a> SW66_W<'a> {
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
#[doc = "Reader of field `SW68`"]
pub type SW68_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW68`"]
pub struct SW68_W<'a> {
    w: &'a mut W,
}
impl<'a> SW68_W<'a> {
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
#[doc = "Reader of field `SW73`"]
pub type SW73_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW73`"]
pub struct SW73_W<'a> {
    w: &'a mut W,
}
impl<'a> SW73_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SW74`"]
pub type SW74_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SW74`"]
pub struct SW74_W<'a> {
    w: &'a mut W,
}
impl<'a> SW74_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - SW"]
    #[inline(always)]
    pub fn sw65(&self) -> SW65_R {
        SW65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SW"]
    #[inline(always)]
    pub fn sw66(&self) -> SW66_R {
        SW66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SW"]
    #[inline(always)]
    pub fn sw68(&self) -> SW68_R {
        SW68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SW"]
    #[inline(always)]
    pub fn sw73(&self) -> SW73_R {
        SW73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SW"]
    #[inline(always)]
    pub fn sw74(&self) -> SW74_R {
        SW74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SW"]
    #[inline(always)]
    pub fn sw65(&mut self) -> SW65_W {
        SW65_W { w: self }
    }
    #[doc = "Bit 2 - SW"]
    #[inline(always)]
    pub fn sw66(&mut self) -> SW66_W {
        SW66_W { w: self }
    }
    #[doc = "Bit 4 - SW"]
    #[inline(always)]
    pub fn sw68(&mut self) -> SW68_W {
        SW68_W { w: self }
    }
    #[doc = "Bit 9 - SW"]
    #[inline(always)]
    pub fn sw73(&mut self) -> SW73_W {
        SW73_W { w: self }
    }
    #[doc = "Bit 10 - SW"]
    #[inline(always)]
    pub fn sw74(&mut self) -> SW74_W {
        SW74_W { w: self }
    }
}
