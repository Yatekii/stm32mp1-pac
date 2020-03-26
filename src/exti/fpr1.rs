#[doc = "Reader of register FPR1"]
pub type R = crate::R<u32, super::FPR1>;
#[doc = "Writer for register FPR1"]
pub type W = crate::W<u32, super::FPR1>;
#[doc = "Register FPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPIF0`"]
pub type FPIF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF0`"]
pub struct FPIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF0_W<'a> {
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
#[doc = "Reader of field `FPIF1`"]
pub type FPIF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF1`"]
pub struct FPIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF1_W<'a> {
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
#[doc = "Reader of field `FPIF2`"]
pub type FPIF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF2`"]
pub struct FPIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF2_W<'a> {
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
#[doc = "Reader of field `FPIF3`"]
pub type FPIF3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF3`"]
pub struct FPIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF3_W<'a> {
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
#[doc = "Reader of field `FPIF4`"]
pub type FPIF4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF4`"]
pub struct FPIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF4_W<'a> {
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
#[doc = "Reader of field `FPIF5`"]
pub type FPIF5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF5`"]
pub struct FPIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF5_W<'a> {
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
#[doc = "Reader of field `FPIF6`"]
pub type FPIF6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF6`"]
pub struct FPIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF6_W<'a> {
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
#[doc = "Reader of field `FPIF7`"]
pub type FPIF7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF7`"]
pub struct FPIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF7_W<'a> {
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
#[doc = "Reader of field `FPIF8`"]
pub type FPIF8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF8`"]
pub struct FPIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF8_W<'a> {
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
#[doc = "Reader of field `FPIF9`"]
pub type FPIF9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF9`"]
pub struct FPIF9_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF9_W<'a> {
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
#[doc = "Reader of field `FPIF10`"]
pub type FPIF10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF10`"]
pub struct FPIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF10_W<'a> {
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
#[doc = "Reader of field `FPIF11`"]
pub type FPIF11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF11`"]
pub struct FPIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF11_W<'a> {
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
#[doc = "Reader of field `FPIF12`"]
pub type FPIF12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF12`"]
pub struct FPIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF12_W<'a> {
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
#[doc = "Reader of field `FPIF13`"]
pub type FPIF13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF13`"]
pub struct FPIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF13_W<'a> {
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
#[doc = "Reader of field `FPIF14`"]
pub type FPIF14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF14`"]
pub struct FPIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF14_W<'a> {
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
#[doc = "Reader of field `FPIF15`"]
pub type FPIF15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF15`"]
pub struct FPIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF15_W<'a> {
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
#[doc = "Reader of field `FPIF16`"]
pub type FPIF16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPIF16`"]
pub struct FPIF16_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIF16_W<'a> {
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
impl R {
    #[doc = "Bit 0 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif16(&self) -> FPIF16_R {
        FPIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif0(&mut self) -> FPIF0_W {
        FPIF0_W { w: self }
    }
    #[doc = "Bit 1 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif1(&mut self) -> FPIF1_W {
        FPIF1_W { w: self }
    }
    #[doc = "Bit 2 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif2(&mut self) -> FPIF2_W {
        FPIF2_W { w: self }
    }
    #[doc = "Bit 3 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif3(&mut self) -> FPIF3_W {
        FPIF3_W { w: self }
    }
    #[doc = "Bit 4 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif4(&mut self) -> FPIF4_W {
        FPIF4_W { w: self }
    }
    #[doc = "Bit 5 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif5(&mut self) -> FPIF5_W {
        FPIF5_W { w: self }
    }
    #[doc = "Bit 6 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif6(&mut self) -> FPIF6_W {
        FPIF6_W { w: self }
    }
    #[doc = "Bit 7 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif7(&mut self) -> FPIF7_W {
        FPIF7_W { w: self }
    }
    #[doc = "Bit 8 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif8(&mut self) -> FPIF8_W {
        FPIF8_W { w: self }
    }
    #[doc = "Bit 9 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif9(&mut self) -> FPIF9_W {
        FPIF9_W { w: self }
    }
    #[doc = "Bit 10 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif10(&mut self) -> FPIF10_W {
        FPIF10_W { w: self }
    }
    #[doc = "Bit 11 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif11(&mut self) -> FPIF11_W {
        FPIF11_W { w: self }
    }
    #[doc = "Bit 12 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif12(&mut self) -> FPIF12_W {
        FPIF12_W { w: self }
    }
    #[doc = "Bit 13 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif13(&mut self) -> FPIF13_W {
        FPIF13_W { w: self }
    }
    #[doc = "Bit 14 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif14(&mut self) -> FPIF14_W {
        FPIF14_W { w: self }
    }
    #[doc = "Bit 15 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif15(&mut self) -> FPIF15_W {
        FPIF15_W { w: self }
    }
    #[doc = "Bit 16 - configurable event inputs x falling edge pending bit."]
    #[inline(always)]
    pub fn fpif16(&mut self) -> FPIF16_W {
        FPIF16_W { w: self }
    }
}
