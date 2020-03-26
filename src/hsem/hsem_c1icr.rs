#[doc = "Writer for register HSEM_C1ICR"]
pub type W = crate::W<u32, super::HSEM_C1ICR>;
#[doc = "Register HSEM_C1ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSEM_C1ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ISC0`"]
pub struct ISC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC0_W<'a> {
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
#[doc = "Write proxy for field `ISC1`"]
pub struct ISC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC1_W<'a> {
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
#[doc = "Write proxy for field `ISC2`"]
pub struct ISC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC2_W<'a> {
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
#[doc = "Write proxy for field `ISC3`"]
pub struct ISC3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC3_W<'a> {
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
#[doc = "Write proxy for field `ISC4`"]
pub struct ISC4_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC4_W<'a> {
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
#[doc = "Write proxy for field `ISC5`"]
pub struct ISC5_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC5_W<'a> {
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
#[doc = "Write proxy for field `ISC6`"]
pub struct ISC6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC6_W<'a> {
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
#[doc = "Write proxy for field `ISC7`"]
pub struct ISC7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC7_W<'a> {
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
#[doc = "Write proxy for field `ISC8`"]
pub struct ISC8_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC8_W<'a> {
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
#[doc = "Write proxy for field `ISC9`"]
pub struct ISC9_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC9_W<'a> {
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
#[doc = "Write proxy for field `ISC10`"]
pub struct ISC10_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC10_W<'a> {
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
#[doc = "Write proxy for field `ISC11`"]
pub struct ISC11_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC11_W<'a> {
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
#[doc = "Write proxy for field `ISC12`"]
pub struct ISC12_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC12_W<'a> {
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
#[doc = "Write proxy for field `ISC13`"]
pub struct ISC13_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC13_W<'a> {
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
#[doc = "Write proxy for field `ISC14`"]
pub struct ISC14_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC14_W<'a> {
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
#[doc = "Write proxy for field `ISC15`"]
pub struct ISC15_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC15_W<'a> {
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
#[doc = "Write proxy for field `ISC16`"]
pub struct ISC16_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC16_W<'a> {
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
#[doc = "Write proxy for field `ISC17`"]
pub struct ISC17_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC17_W<'a> {
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
#[doc = "Write proxy for field `ISC18`"]
pub struct ISC18_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC18_W<'a> {
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
#[doc = "Write proxy for field `ISC19`"]
pub struct ISC19_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC19_W<'a> {
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
#[doc = "Write proxy for field `ISC20`"]
pub struct ISC20_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC20_W<'a> {
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
#[doc = "Write proxy for field `ISC21`"]
pub struct ISC21_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC21_W<'a> {
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
#[doc = "Write proxy for field `ISC22`"]
pub struct ISC22_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC22_W<'a> {
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
#[doc = "Write proxy for field `ISC23`"]
pub struct ISC23_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC23_W<'a> {
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
#[doc = "Write proxy for field `ISC24`"]
pub struct ISC24_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC24_W<'a> {
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
#[doc = "Write proxy for field `ISC25`"]
pub struct ISC25_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC25_W<'a> {
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
#[doc = "Write proxy for field `ISC26`"]
pub struct ISC26_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC26_W<'a> {
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
#[doc = "Write proxy for field `ISC27`"]
pub struct ISC27_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC27_W<'a> {
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
#[doc = "Write proxy for field `ISC28`"]
pub struct ISC28_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC28_W<'a> {
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
#[doc = "Write proxy for field `ISC29`"]
pub struct ISC29_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC29_W<'a> {
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
#[doc = "Write proxy for field `ISC30`"]
pub struct ISC30_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC30_W<'a> {
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
#[doc = "Write proxy for field `ISC31`"]
pub struct ISC31_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC31_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc0(&mut self) -> ISC0_W {
        ISC0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc1(&mut self) -> ISC1_W {
        ISC1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc2(&mut self) -> ISC2_W {
        ISC2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc3(&mut self) -> ISC3_W {
        ISC3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc4(&mut self) -> ISC4_W {
        ISC4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc5(&mut self) -> ISC5_W {
        ISC5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc6(&mut self) -> ISC6_W {
        ISC6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc7(&mut self) -> ISC7_W {
        ISC7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc8(&mut self) -> ISC8_W {
        ISC8_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc9(&mut self) -> ISC9_W {
        ISC9_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc10(&mut self) -> ISC10_W {
        ISC10_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc11(&mut self) -> ISC11_W {
        ISC11_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc12(&mut self) -> ISC12_W {
        ISC12_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc13(&mut self) -> ISC13_W {
        ISC13_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc14(&mut self) -> ISC14_W {
        ISC14_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc15(&mut self) -> ISC15_W {
        ISC15_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc16(&mut self) -> ISC16_W {
        ISC16_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc17(&mut self) -> ISC17_W {
        ISC17_W { w: self }
    }
    #[doc = "Bit 18 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc18(&mut self) -> ISC18_W {
        ISC18_W { w: self }
    }
    #[doc = "Bit 19 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc19(&mut self) -> ISC19_W {
        ISC19_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc20(&mut self) -> ISC20_W {
        ISC20_W { w: self }
    }
    #[doc = "Bit 21 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc21(&mut self) -> ISC21_W {
        ISC21_W { w: self }
    }
    #[doc = "Bit 22 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc22(&mut self) -> ISC22_W {
        ISC22_W { w: self }
    }
    #[doc = "Bit 23 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc23(&mut self) -> ISC23_W {
        ISC23_W { w: self }
    }
    #[doc = "Bit 24 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc24(&mut self) -> ISC24_W {
        ISC24_W { w: self }
    }
    #[doc = "Bit 25 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc25(&mut self) -> ISC25_W {
        ISC25_W { w: self }
    }
    #[doc = "Bit 26 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc26(&mut self) -> ISC26_W {
        ISC26_W { w: self }
    }
    #[doc = "Bit 27 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc27(&mut self) -> ISC27_W {
        ISC27_W { w: self }
    }
    #[doc = "Bit 28 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc28(&mut self) -> ISC28_W {
        ISC28_W { w: self }
    }
    #[doc = "Bit 29 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc29(&mut self) -> ISC29_W {
        ISC29_W { w: self }
    }
    #[doc = "Bit 30 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc30(&mut self) -> ISC30_W {
        ISC30_W { w: self }
    }
    #[doc = "Bit 31 - Interrupt(N) semaphore n clear bit"]
    #[inline(always)]
    pub fn isc31(&mut self) -> ISC31_W {
        ISC31_W { w: self }
    }
}
