#[doc = "Reader of register BUILD_CONFIG"]
pub type R = crate::R<u32, super::BUILD_CONFIG>;
#[doc = "Writer for register BUILD_CONFIG"]
pub type W = crate::W<u32, super::BUILD_CONFIG>;
#[doc = "Register BUILD_CONFIG `reset()`'s with value 0x0100_1f08"]
impl crate::ResetValue for super::BUILD_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_1f08
    }
}
#[doc = "Reader of field `NO_OF_REGIONS`"]
pub type NO_OF_REGIONS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NO_OF_REGIONS`"]
pub struct NO_OF_REGIONS_W<'a> {
    w: &'a mut W,
}
impl<'a> NO_OF_REGIONS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ADDRESS_WIDTH`"]
pub type ADDRESS_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRESS_WIDTH`"]
pub struct ADDRESS_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NO_OF_FILTERS`"]
pub type NO_OF_FILTERS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NO_OF_FILTERS`"]
pub struct NO_OF_FILTERS_W<'a> {
    w: &'a mut W,
}
impl<'a> NO_OF_FILTERS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Number fo regions"]
    #[inline(always)]
    pub fn no_of_regions(&self) -> NO_OF_REGIONS_R {
        NO_OF_REGIONS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - ADDRESS WIDTH"]
    #[inline(always)]
    pub fn address_width(&self) -> ADDRESS_WIDTH_R {
        ADDRESS_WIDTH_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - Number of filters"]
    #[inline(always)]
    pub fn no_of_filters(&self) -> NO_OF_FILTERS_R {
        NO_OF_FILTERS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number fo regions"]
    #[inline(always)]
    pub fn no_of_regions(&mut self) -> NO_OF_REGIONS_W {
        NO_OF_REGIONS_W { w: self }
    }
    #[doc = "Bits 8:13 - ADDRESS WIDTH"]
    #[inline(always)]
    pub fn address_width(&mut self) -> ADDRESS_WIDTH_W {
        ADDRESS_WIDTH_W { w: self }
    }
    #[doc = "Bits 24:25 - Number of filters"]
    #[inline(always)]
    pub fn no_of_filters(&mut self) -> NO_OF_FILTERS_W {
        NO_OF_FILTERS_W { w: self }
    }
}
