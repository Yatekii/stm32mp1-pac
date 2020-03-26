#[doc = "Reader of register HSEM_C1IER"]
pub type R = crate::R<u32, super::HSEM_C1IER>;
#[doc = "Writer for register HSEM_C1IER"]
pub type W = crate::W<u32, super::HSEM_C1IER>;
#[doc = "Register HSEM_C1IER `reset()`'s with value 0"]
impl crate::ResetValue for super::HSEM_C1IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISE0`"]
pub type ISE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE0`"]
pub struct ISE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE0_W<'a> {
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
#[doc = "Reader of field `ISE1`"]
pub type ISE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE1`"]
pub struct ISE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE1_W<'a> {
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
#[doc = "Reader of field `ISE2`"]
pub type ISE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE2`"]
pub struct ISE2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE2_W<'a> {
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
#[doc = "Reader of field `ISE3`"]
pub type ISE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE3`"]
pub struct ISE3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ISE4`"]
pub type ISE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE4`"]
pub struct ISE4_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE4_W<'a> {
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
#[doc = "Reader of field `ISE5`"]
pub type ISE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE5`"]
pub struct ISE5_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ISE6`"]
pub type ISE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE6`"]
pub struct ISE6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ISE7`"]
pub type ISE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE7`"]
pub struct ISE7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ISE8`"]
pub type ISE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE8`"]
pub struct ISE8_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ISE9`"]
pub type ISE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE9`"]
pub struct ISE9_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE9_W<'a> {
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
#[doc = "Reader of field `ISE10`"]
pub type ISE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE10`"]
pub struct ISE10_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE10_W<'a> {
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
#[doc = "Reader of field `ISE11`"]
pub type ISE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE11`"]
pub struct ISE11_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ISE12`"]
pub type ISE12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE12`"]
pub struct ISE12_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ISE13`"]
pub type ISE13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE13`"]
pub struct ISE13_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ISE14`"]
pub type ISE14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE14`"]
pub struct ISE14_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ISE15`"]
pub type ISE15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE15`"]
pub struct ISE15_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `ISE16`"]
pub type ISE16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE16`"]
pub struct ISE16_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE16_W<'a> {
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
#[doc = "Reader of field `ISE17`"]
pub type ISE17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE17`"]
pub struct ISE17_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE17_W<'a> {
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
#[doc = "Reader of field `ISE18`"]
pub type ISE18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE18`"]
pub struct ISE18_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE18_W<'a> {
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
#[doc = "Reader of field `ISE19`"]
pub type ISE19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE19`"]
pub struct ISE19_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE19_W<'a> {
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
#[doc = "Reader of field `ISE20`"]
pub type ISE20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE20`"]
pub struct ISE20_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `ISE21`"]
pub type ISE21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE21`"]
pub struct ISE21_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ISE22`"]
pub type ISE22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE22`"]
pub struct ISE22_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ISE23`"]
pub type ISE23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE23`"]
pub struct ISE23_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ISE24`"]
pub type ISE24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE24`"]
pub struct ISE24_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `ISE25`"]
pub type ISE25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE25`"]
pub struct ISE25_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `ISE26`"]
pub type ISE26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE26`"]
pub struct ISE26_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `ISE27`"]
pub type ISE27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE27`"]
pub struct ISE27_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `ISE28`"]
pub type ISE28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE28`"]
pub struct ISE28_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ISE29`"]
pub type ISE29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE29`"]
pub struct ISE29_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ISE30`"]
pub type ISE30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE30`"]
pub struct ISE30_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE30_W<'a> {
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
#[doc = "Reader of field `ISE31`"]
pub type ISE31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISE31`"]
pub struct ISE31_W<'a> {
    w: &'a mut W,
}
impl<'a> ISE31_W<'a> {
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
    #[doc = "Bit 0 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise0(&self) -> ISE0_R {
        ISE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise1(&self) -> ISE1_R {
        ISE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise2(&self) -> ISE2_R {
        ISE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise3(&self) -> ISE3_R {
        ISE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise4(&self) -> ISE4_R {
        ISE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise5(&self) -> ISE5_R {
        ISE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise6(&self) -> ISE6_R {
        ISE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise7(&self) -> ISE7_R {
        ISE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise8(&self) -> ISE8_R {
        ISE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise9(&self) -> ISE9_R {
        ISE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise10(&self) -> ISE10_R {
        ISE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise11(&self) -> ISE11_R {
        ISE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise12(&self) -> ISE12_R {
        ISE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise13(&self) -> ISE13_R {
        ISE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise14(&self) -> ISE14_R {
        ISE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise15(&self) -> ISE15_R {
        ISE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise16(&self) -> ISE16_R {
        ISE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise17(&self) -> ISE17_R {
        ISE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise18(&self) -> ISE18_R {
        ISE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise19(&self) -> ISE19_R {
        ISE19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise20(&self) -> ISE20_R {
        ISE20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise21(&self) -> ISE21_R {
        ISE21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise22(&self) -> ISE22_R {
        ISE22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise23(&self) -> ISE23_R {
        ISE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise24(&self) -> ISE24_R {
        ISE24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise25(&self) -> ISE25_R {
        ISE25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise26(&self) -> ISE26_R {
        ISE26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise27(&self) -> ISE27_R {
        ISE27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise28(&self) -> ISE28_R {
        ISE28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise29(&self) -> ISE29_R {
        ISE29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise30(&self) -> ISE30_R {
        ISE30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Interrupt(N) semaphore n enable bit."]
    #[inline(always)]
    pub fn ise31(&self) -> ISE31_R {
        ISE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise0(&mut self) -> ISE0_W {
        ISE0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise1(&mut self) -> ISE1_W {
        ISE1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise2(&mut self) -> ISE2_W {
        ISE2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise3(&mut self) -> ISE3_W {
        ISE3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise4(&mut self) -> ISE4_W {
        ISE4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise5(&mut self) -> ISE5_W {
        ISE5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise6(&mut self) -> ISE6_W {
        ISE6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise7(&mut self) -> ISE7_W {
        ISE7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise8(&mut self) -> ISE8_W {
        ISE8_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise9(&mut self) -> ISE9_W {
        ISE9_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise10(&mut self) -> ISE10_W {
        ISE10_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise11(&mut self) -> ISE11_W {
        ISE11_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise12(&mut self) -> ISE12_W {
        ISE12_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise13(&mut self) -> ISE13_W {
        ISE13_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise14(&mut self) -> ISE14_W {
        ISE14_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise15(&mut self) -> ISE15_W {
        ISE15_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise16(&mut self) -> ISE16_W {
        ISE16_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise17(&mut self) -> ISE17_W {
        ISE17_W { w: self }
    }
    #[doc = "Bit 18 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise18(&mut self) -> ISE18_W {
        ISE18_W { w: self }
    }
    #[doc = "Bit 19 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise19(&mut self) -> ISE19_W {
        ISE19_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise20(&mut self) -> ISE20_W {
        ISE20_W { w: self }
    }
    #[doc = "Bit 21 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise21(&mut self) -> ISE21_W {
        ISE21_W { w: self }
    }
    #[doc = "Bit 22 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise22(&mut self) -> ISE22_W {
        ISE22_W { w: self }
    }
    #[doc = "Bit 23 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise23(&mut self) -> ISE23_W {
        ISE23_W { w: self }
    }
    #[doc = "Bit 24 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise24(&mut self) -> ISE24_W {
        ISE24_W { w: self }
    }
    #[doc = "Bit 25 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise25(&mut self) -> ISE25_W {
        ISE25_W { w: self }
    }
    #[doc = "Bit 26 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise26(&mut self) -> ISE26_W {
        ISE26_W { w: self }
    }
    #[doc = "Bit 27 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise27(&mut self) -> ISE27_W {
        ISE27_W { w: self }
    }
    #[doc = "Bit 28 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise28(&mut self) -> ISE28_W {
        ISE28_W { w: self }
    }
    #[doc = "Bit 29 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise29(&mut self) -> ISE29_W {
        ISE29_W { w: self }
    }
    #[doc = "Bit 30 - Interrupt semaphore n enable bit"]
    #[inline(always)]
    pub fn ise30(&mut self) -> ISE30_W {
        ISE30_W { w: self }
    }
    #[doc = "Bit 31 - Interrupt(N) semaphore n enable bit."]
    #[inline(always)]
    pub fn ise31(&mut self) -> ISE31_W {
        ISE31_W { w: self }
    }
}
