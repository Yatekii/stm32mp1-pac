#[doc = "Reader of register FDCAN_RXF0S"]
pub type R = crate::R<u32, super::FDCAN_RXF0S>;
#[doc = "Writer for register FDCAN_RXF0S"]
pub type W = crate::W<u32, super::FDCAN_RXF0S>;
#[doc = "Register FDCAN_RXF0S `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_RXF0S {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `F0FL`"]
pub type F0FL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F0FL`"]
pub struct F0FL_W<'a> {
    w: &'a mut W,
}
impl<'a> F0FL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `F0GI`"]
pub type F0GI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F0GI`"]
pub struct F0GI_W<'a> {
    w: &'a mut W,
}
impl<'a> F0GI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `F0PI`"]
pub type F0PI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F0PI`"]
pub struct F0PI_W<'a> {
    w: &'a mut W,
}
impl<'a> F0PI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "F0F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F0F_A {
    #[doc = "0: Rx FIFO 0 not full"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 0 full"]
    B_0X1 = 1,
}
impl From<F0F_A> for bool {
    #[inline(always)]
    fn from(variant: F0F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `F0F`"]
pub type F0F_R = crate::R<bool, F0F_A>;
impl F0F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F0F_A {
        match self.bits {
            false => F0F_A::B_0X0,
            true => F0F_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == F0F_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == F0F_A::B_0X1
    }
}
#[doc = "Write proxy for field `F0F`"]
pub struct F0F_W<'a> {
    w: &'a mut W,
}
impl<'a> F0F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: F0F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO 0 not full"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(F0F_A::B_0X0)
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(F0F_A::B_0X1)
    }
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
#[doc = "RF0L\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0L_A {
    #[doc = "0: No Rx FIFO 0 message\r\n                  lost"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 0 message lost, also set\r\n                  after write attempt to Rx FIFO 0 of size\r\n                  zero"]
    B_0X1 = 1,
}
impl From<RF0L_A> for bool {
    #[inline(always)]
    fn from(variant: RF0L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF0L`"]
pub type RF0L_R = crate::R<bool, RF0L_A>;
impl RF0L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF0L_A {
        match self.bits {
            false => RF0L_A::B_0X0,
            true => RF0L_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0L_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0L_A::B_0X1
    }
}
#[doc = "Write proxy for field `RF0L`"]
pub struct RF0L_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF0L_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0L_A::B_0X0)
    }
    #[doc = "Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0L_A::B_0X1)
    }
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
impl R {
    #[doc = "Bits 0:6 - F0FL"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - F0GI"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - F0PI"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - F0F"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RF0L"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - F0FL"]
    #[inline(always)]
    pub fn f0fl(&mut self) -> F0FL_W {
        F0FL_W { w: self }
    }
    #[doc = "Bits 8:13 - F0GI"]
    #[inline(always)]
    pub fn f0gi(&mut self) -> F0GI_W {
        F0GI_W { w: self }
    }
    #[doc = "Bits 16:21 - F0PI"]
    #[inline(always)]
    pub fn f0pi(&mut self) -> F0PI_W {
        F0PI_W { w: self }
    }
    #[doc = "Bit 24 - F0F"]
    #[inline(always)]
    pub fn f0f(&mut self) -> F0F_W {
        F0F_W { w: self }
    }
    #[doc = "Bit 25 - RF0L"]
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W {
        RF0L_W { w: self }
    }
}
