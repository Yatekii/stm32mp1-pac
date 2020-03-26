#[doc = "Reader of register DFSDM_FLT2ICR"]
pub type R = crate::R<u32, super::DFSDM_FLT2ICR>;
#[doc = "Writer for register DFSDM_FLT2ICR"]
pub type W = crate::W<u32, super::DFSDM_FLT2ICR>;
#[doc = "Register DFSDM_FLT2ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::DFSDM_FLT2ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CLRJOVRF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRJOVRF_A {
    #[doc = "0: Writing 0 has no\r\n                  effect"]
    B_0X0 = 0,
    #[doc = "1: Writing 1 clears the JOVRF bit in\r\n                  the DFSDM_FLTxISR register"]
    B_0X1 = 1,
}
impl From<CLRJOVRF_A> for bool {
    #[inline(always)]
    fn from(variant: CLRJOVRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLRJOVRF`"]
pub type CLRJOVRF_R = crate::R<bool, CLRJOVRF_A>;
impl CLRJOVRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRJOVRF_A {
        match self.bits {
            false => CLRJOVRF_A::B_0X0,
            true => CLRJOVRF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CLRJOVRF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CLRJOVRF_A::B_0X1
    }
}
#[doc = "Write proxy for field `CLRJOVRF`"]
pub struct CLRJOVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRJOVRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRJOVRF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CLRJOVRF_A::B_0X0)
    }
    #[doc = "Writing 1 clears the JOVRF bit in the DFSDM_FLTxISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CLRJOVRF_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "CLRROVRF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRROVRF_A {
    #[doc = "0: Writing 0 has no\r\n                  effect"]
    B_0X0 = 0,
    #[doc = "1: Writing 1 clears the ROVRF bit in\r\n                  the DFSDM_FLTxISR register"]
    B_0X1 = 1,
}
impl From<CLRROVRF_A> for bool {
    #[inline(always)]
    fn from(variant: CLRROVRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLRROVRF`"]
pub type CLRROVRF_R = crate::R<bool, CLRROVRF_A>;
impl CLRROVRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRROVRF_A {
        match self.bits {
            false => CLRROVRF_A::B_0X0,
            true => CLRROVRF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CLRROVRF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CLRROVRF_A::B_0X1
    }
}
#[doc = "Write proxy for field `CLRROVRF`"]
pub struct CLRROVRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRROVRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRROVRF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CLRROVRF_A::B_0X0)
    }
    #[doc = "Writing 1 clears the ROVRF bit in the DFSDM_FLTxISR register"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CLRROVRF_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CLRCKABF`"]
pub type CLRCKABF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLRCKABF`"]
pub struct CLRCKABF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRCKABF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CLRSCDF`"]
pub type CLRSCDF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLRSCDF`"]
pub struct CLRSCDF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSCDF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - CLRJOVRF"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CLRROVRF"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - CLRCKABF"]
    #[inline(always)]
    pub fn clrckabf(&self) -> CLRCKABF_R {
        CLRCKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CLRSCDF"]
    #[inline(always)]
    pub fn clrscdf(&self) -> CLRSCDF_R {
        CLRSCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - CLRJOVRF"]
    #[inline(always)]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W {
        CLRJOVRF_W { w: self }
    }
    #[doc = "Bit 3 - CLRROVRF"]
    #[inline(always)]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W {
        CLRROVRF_W { w: self }
    }
    #[doc = "Bits 16:23 - CLRCKABF"]
    #[inline(always)]
    pub fn clrckabf(&mut self) -> CLRCKABF_W {
        CLRCKABF_W { w: self }
    }
    #[doc = "Bits 24:31 - CLRSCDF"]
    #[inline(always)]
    pub fn clrscdf(&mut self) -> CLRSCDF_W {
        CLRSCDF_W { w: self }
    }
}
