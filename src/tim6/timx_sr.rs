#[doc = "Reader of register TIMx_SR"]
pub type R = crate::R<u32, super::TIMX_SR>;
#[doc = "Writer for register TIMx_SR"]
pub type W = crate::W<u32, super::TIMX_SR>;
#[doc = "Register TIMx_SR `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMX_SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UIF`"]
pub type UIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UIF`"]
pub struct UIF_W<'a> {
    w: &'a mut W,
}
impl<'a> UIF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - UIF"]
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UIF"]
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W {
        UIF_W { w: self }
    }
}
