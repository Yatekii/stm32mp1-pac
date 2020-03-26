#[doc = "Reader of register DDRPHYC_MR1"]
pub type R = crate::R<u16, super::DDRPHYC_MR1>;
#[doc = "Writer for register DDRPHYC_MR1"]
pub type W = crate::W<u16, super::DDRPHYC_MR1>;
#[doc = "Register DDRPHYC_MR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_MR1 {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BL`"]
pub type BL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BL`"]
pub struct BL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u16) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `BT`"]
pub type BT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BT`"]
pub struct BT_W<'a> {
    w: &'a mut W,
}
impl<'a> BT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u16) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `WC`"]
pub type WC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WC`"]
pub struct WC_W<'a> {
    w: &'a mut W,
}
impl<'a> WC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u16) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `NWR`"]
pub type NWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NWR`"]
pub struct NWR_W<'a> {
    w: &'a mut W,
}
impl<'a> NWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u16) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - BL"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - BT"]
    #[inline(always)]
    pub fn bt(&self) -> BT_R {
        BT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WC"]
    #[inline(always)]
    pub fn wc(&self) -> WC_R {
        WC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - NWR"]
    #[inline(always)]
    pub fn nwr(&self) -> NWR_R {
        NWR_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BL"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W { w: self }
    }
    #[doc = "Bit 3 - BT"]
    #[inline(always)]
    pub fn bt(&mut self) -> BT_W {
        BT_W { w: self }
    }
    #[doc = "Bit 4 - WC"]
    #[inline(always)]
    pub fn wc(&mut self) -> WC_W {
        WC_W { w: self }
    }
    #[doc = "Bits 5:7 - NWR"]
    #[inline(always)]
    pub fn nwr(&mut self) -> NWR_W {
        NWR_W { w: self }
    }
}
