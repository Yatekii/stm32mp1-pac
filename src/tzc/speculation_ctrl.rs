#[doc = "Reader of register SPECULATION_CTRL"]
pub type R = crate::R<u32, super::SPECULATION_CTRL>;
#[doc = "Writer for register SPECULATION_CTRL"]
pub type W = crate::W<u32, super::SPECULATION_CTRL>;
#[doc = "Register SPECULATION_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SPECULATION_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READSPEC_DISABLE`"]
pub type READSPEC_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READSPEC_DISABLE`"]
pub struct READSPEC_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> READSPEC_DISABLE_W<'a> {
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
#[doc = "Reader of field `WRITESPEC_DISABLE`"]
pub type WRITESPEC_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITESPEC_DISABLE`"]
pub struct WRITESPEC_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITESPEC_DISABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Read access speculation disable"]
    #[inline(always)]
    pub fn readspec_disable(&self) -> READSPEC_DISABLE_R {
        READSPEC_DISABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write access speculation disable"]
    #[inline(always)]
    pub fn writespec_disable(&self) -> WRITESPEC_DISABLE_R {
        WRITESPEC_DISABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read access speculation disable"]
    #[inline(always)]
    pub fn readspec_disable(&mut self) -> READSPEC_DISABLE_W {
        READSPEC_DISABLE_W { w: self }
    }
    #[doc = "Bit 1 - Write access speculation disable"]
    #[inline(always)]
    pub fn writespec_disable(&mut self) -> WRITESPEC_DISABLE_W {
        WRITESPEC_DISABLE_W { w: self }
    }
}
