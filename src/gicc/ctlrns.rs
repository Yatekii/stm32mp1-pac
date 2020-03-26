#[doc = "Reader of register CTLRNS"]
pub type R = crate::R<u32, super::CTLRNS>;
#[doc = "Writer for register CTLRNS"]
pub type W = crate::W<u32, super::CTLRNS>;
#[doc = "Register CTLRNS `reset()`'s with value 0"]
impl crate::ResetValue for super::CTLRNS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLEGRP1`"]
pub type ENABLEGRP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLEGRP1`"]
pub struct ENABLEGRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEGRP1_W<'a> {
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
#[doc = "Reader of field `FIQBYPDISGRP1`"]
pub type FIQBYPDISGRP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIQBYPDISGRP1`"]
pub struct FIQBYPDISGRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> FIQBYPDISGRP1_W<'a> {
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
#[doc = "Reader of field `IRQBYPDISGRP1`"]
pub type IRQBYPDISGRP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQBYPDISGRP1`"]
pub struct IRQBYPDISGRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQBYPDISGRP1_W<'a> {
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
#[doc = "Reader of field `EOIMODENS`"]
pub type EOIMODENS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOIMODENS`"]
pub struct EOIMODENS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOIMODENS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enable group1 interrupts"]
    #[inline(always)]
    pub fn enablegrp1(&self) -> ENABLEGRP1_R {
        ENABLEGRP1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - FIQ bypass disable for group 1 interrupts"]
    #[inline(always)]
    pub fn fiqbypdisgrp1(&self) -> FIQBYPDISGRP1_R {
        FIQBYPDISGRP1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IRQ bypass for group 1 interrupts"]
    #[inline(always)]
    pub fn irqbypdisgrp1(&self) -> IRQBYPDISGRP1_R {
        IRQBYPDISGRP1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EOI mode for non- secure accesses"]
    #[inline(always)]
    pub fn eoimodens(&self) -> EOIMODENS_R {
        EOIMODENS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable group1 interrupts"]
    #[inline(always)]
    pub fn enablegrp1(&mut self) -> ENABLEGRP1_W {
        ENABLEGRP1_W { w: self }
    }
    #[doc = "Bit 5 - FIQ bypass disable for group 1 interrupts"]
    #[inline(always)]
    pub fn fiqbypdisgrp1(&mut self) -> FIQBYPDISGRP1_W {
        FIQBYPDISGRP1_W { w: self }
    }
    #[doc = "Bit 6 - IRQ bypass for group 1 interrupts"]
    #[inline(always)]
    pub fn irqbypdisgrp1(&mut self) -> IRQBYPDISGRP1_W {
        IRQBYPDISGRP1_W { w: self }
    }
    #[doc = "Bit 9 - EOI mode for non- secure accesses"]
    #[inline(always)]
    pub fn eoimodens(&mut self) -> EOIMODENS_W {
        EOIMODENS_W { w: self }
    }
}
