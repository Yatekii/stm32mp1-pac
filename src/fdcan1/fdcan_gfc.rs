#[doc = "Reader of register FDCAN_GFC"]
pub type R = crate::R<u32, super::FDCAN_GFC>;
#[doc = "Writer for register FDCAN_GFC"]
pub type W = crate::W<u32, super::FDCAN_GFC>;
#[doc = "Register FDCAN_GFC `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_GFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RRFE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRFE_A {
    #[doc = "0: Filter remote frames with 29-bit\r\n                  standard IDs"]
    B_0X0 = 0,
    #[doc = "1: Reject all remote frames with 29-bit\r\n                  standard IDs"]
    B_0X1 = 1,
}
impl From<RRFE_A> for bool {
    #[inline(always)]
    fn from(variant: RRFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RRFE`"]
pub type RRFE_R = crate::R<bool, RRFE_A>;
impl RRFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRFE_A {
        match self.bits {
            false => RRFE_A::B_0X0,
            true => RRFE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RRFE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RRFE_A::B_0X1
    }
}
#[doc = "Write proxy for field `RRFE`"]
pub struct RRFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Filter remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RRFE_A::B_0X0)
    }
    #[doc = "Reject all remote frames with 29-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RRFE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "RRFS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRFS_A {
    #[doc = "0: Filter remote frames with 11-bit\r\n                  standard IDs"]
    B_0X0 = 0,
    #[doc = "1: Reject all remote frames with 11-bit\r\n                  standard IDs"]
    B_0X1 = 1,
}
impl From<RRFS_A> for bool {
    #[inline(always)]
    fn from(variant: RRFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RRFS`"]
pub type RRFS_R = crate::R<bool, RRFS_A>;
impl RRFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRFS_A {
        match self.bits {
            false => RRFS_A::B_0X0,
            true => RRFS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RRFS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RRFS_A::B_0X1
    }
}
#[doc = "Write proxy for field `RRFS`"]
pub struct RRFS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRFS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Filter remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RRFS_A::B_0X0)
    }
    #[doc = "Reject all remote frames with 11-bit standard IDs"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RRFS_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "ANFE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANFE_AW {
    #[doc = "0: Accept in Rx FIFO 0"]
    B_0X0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    B_0X1 = 1,
    #[doc = "2: Reject"]
    B_0X2 = 2,
    #[doc = "3: Reject"]
    B_0X3 = 3,
}
impl From<ANFE_AW> for u8 {
    #[inline(always)]
    fn from(variant: ANFE_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `ANFE`"]
pub struct ANFE_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANFE_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ANFE_AW::B_0X0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ANFE_AW::B_0X1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ANFE_AW::B_0X2)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ANFE_AW::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "ANFS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANFS_AW {
    #[doc = "0: Accept in Rx FIFO 0"]
    B_0X0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    B_0X1 = 1,
    #[doc = "2: Reject"]
    B_0X2 = 2,
    #[doc = "3: Reject"]
    B_0X3 = 3,
}
impl From<ANFS_AW> for u8 {
    #[inline(always)]
    fn from(variant: ANFS_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `ANFS`"]
pub struct ANFS_W<'a> {
    w: &'a mut W,
}
impl<'a> ANFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANFS_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ANFS_AW::B_0X0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ANFS_AW::B_0X1)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ANFS_AW::B_0X2)
    }
    #[doc = "Reject"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ANFS_AW::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RRFE"]
    #[inline(always)]
    pub fn rrfe(&mut self) -> RRFE_W {
        RRFE_W { w: self }
    }
    #[doc = "Bit 1 - RRFS"]
    #[inline(always)]
    pub fn rrfs(&mut self) -> RRFS_W {
        RRFS_W { w: self }
    }
    #[doc = "Bits 2:3 - ANFE"]
    #[inline(always)]
    pub fn anfe(&mut self) -> ANFE_W {
        ANFE_W { w: self }
    }
    #[doc = "Bits 4:5 - ANFS"]
    #[inline(always)]
    pub fn anfs(&mut self) -> ANFS_W {
        ANFS_W { w: self }
    }
}
