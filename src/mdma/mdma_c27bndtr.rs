#[doc = "Reader of register MDMA_C27BNDTR"]
pub type R = crate::R<u32, super::MDMA_C27BNDTR>;
#[doc = "Writer for register MDMA_C27BNDTR"]
pub type W = crate::W<u32, super::MDMA_C27BNDTR>;
#[doc = "Register MDMA_C27BNDTR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C27BNDTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BNDT`"]
pub type BNDT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BNDT`"]
pub struct BNDT_W<'a> {
    w: &'a mut W,
}
impl<'a> BNDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
#[doc = "BRSUM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRSUM_A {
    #[doc = "0: At the end of a block transfer, the\r\n                  SAR register will be updated by adding the SUV to\r\n                  the current SAR value (current Source\r\n                  Address)"]
    B_0X0 = 0,
    #[doc = "1: At the end of a block transfer, the\r\n                  SAR register will be updated by subtracting the\r\n                  SUV from the current SAR value (current Source\r\n                  Address)"]
    B_0X1 = 1,
}
impl From<BRSUM_A> for bool {
    #[inline(always)]
    fn from(variant: BRSUM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRSUM`"]
pub type BRSUM_R = crate::R<bool, BRSUM_A>;
impl BRSUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRSUM_A {
        match self.bits {
            false => BRSUM_A::B_0X0,
            true => BRSUM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRSUM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRSUM_A::B_0X1
    }
}
#[doc = "Write proxy for field `BRSUM`"]
pub struct BRSUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRSUM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "At the end of a block transfer, the SAR register will be updated by adding the SUV to the current SAR value (current Source Address)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BRSUM_A::B_0X0)
    }
    #[doc = "At the end of a block transfer, the SAR register will be updated by subtracting the SUV from the current SAR value (current Source Address)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BRSUM_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "BRDUM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRDUM_A {
    #[doc = "0: At the end of a Block transfer, the\r\n                  DAR register will be updated by adding the DUV to\r\n                  the current DAR value (current Destination\r\n                  Address)"]
    B_0X0 = 0,
    #[doc = "1: At the end of a block transfer, the\r\n                  DAR register will be updated by subtracting the\r\n                  DUV from the current DAR value (current\r\n                  Destination Address)"]
    B_0X1 = 1,
}
impl From<BRDUM_A> for bool {
    #[inline(always)]
    fn from(variant: BRDUM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRDUM`"]
pub type BRDUM_R = crate::R<bool, BRDUM_A>;
impl BRDUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRDUM_A {
        match self.bits {
            false => BRDUM_A::B_0X0,
            true => BRDUM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRDUM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRDUM_A::B_0X1
    }
}
#[doc = "Write proxy for field `BRDUM`"]
pub struct BRDUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BRDUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRDUM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "At the end of a Block transfer, the DAR register will be updated by adding the DUV to the current DAR value (current Destination Address)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BRDUM_A::B_0X0)
    }
    #[doc = "At the end of a block transfer, the DAR register will be updated by subtracting the DUV from the current DAR value (current Destination Address)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BRDUM_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `BRC`"]
pub type BRC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BRC`"]
pub struct BRC_W<'a> {
    w: &'a mut W,
}
impl<'a> BRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - BNDT"]
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0x0001_ffff) as u32)
    }
    #[doc = "Bit 18 - BRSUM"]
    #[inline(always)]
    pub fn brsum(&self) -> BRSUM_R {
        BRSUM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - BRDUM"]
    #[inline(always)]
    pub fn brdum(&self) -> BRDUM_R {
        BRDUM_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:31 - BRC"]
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - BNDT"]
    #[inline(always)]
    pub fn bndt(&mut self) -> BNDT_W {
        BNDT_W { w: self }
    }
    #[doc = "Bit 18 - BRSUM"]
    #[inline(always)]
    pub fn brsum(&mut self) -> BRSUM_W {
        BRSUM_W { w: self }
    }
    #[doc = "Bit 19 - BRDUM"]
    #[inline(always)]
    pub fn brdum(&mut self) -> BRDUM_W {
        BRDUM_W { w: self }
    }
    #[doc = "Bits 20:31 - BRC"]
    #[inline(always)]
    pub fn brc(&mut self) -> BRC_W {
        BRC_W { w: self }
    }
}
