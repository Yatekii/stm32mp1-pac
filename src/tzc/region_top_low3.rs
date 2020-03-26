#[doc = "Reader of register REGION_TOP_LOW3"]
pub type R = crate::R<u32, super::REGION_TOP_LOW3>;
#[doc = "Writer for register REGION_TOP_LOW3"]
pub type W = crate::W<u32, super::REGION_TOP_LOW3>;
#[doc = "Register REGION_TOP_LOW3 `reset()`'s with value 0"]
impl crate::ResetValue for super::REGION_TOP_LOW3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOP_ADDRESS_LOW`"]
pub type TOP_ADDRESS_LOW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TOP_ADDRESS_LOW`"]
pub struct TOP_ADDRESS_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_ADDRESS_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - Base address bits\\[31:12\\]
for region"]
    #[inline(always)]
    pub fn top_address_low(&self) -> TOP_ADDRESS_LOW_R {
        TOP_ADDRESS_LOW_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 12:31 - Base address bits\\[31:12\\]
for region"]
    #[inline(always)]
    pub fn top_address_low(&mut self) -> TOP_ADDRESS_LOW_W {
        TOP_ADDRESS_LOW_W { w: self }
    }
}