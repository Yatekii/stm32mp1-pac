#[doc = "Reader of register DDRPHYC_DCUAR"]
pub type R = crate::R<u16, super::DDRPHYC_DCUAR>;
#[doc = "Writer for register DDRPHYC_DCUAR"]
pub type W = crate::W<u16, super::DDRPHYC_DCUAR>;
#[doc = "Register DDRPHYC_DCUAR `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRPHYC_DCUAR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CWADDR`"]
pub type CWADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CWADDR`"]
pub struct CWADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CWADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u16) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CSADDR`"]
pub type CSADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSADDR`"]
pub struct CSADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u16) & 0x0f) << 4);
        self.w
    }
}
#[doc = "CSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSEL_A {
    #[doc = "0: Command cache"]
    B_0X0 = 0,
    #[doc = "1: Expected data cache"]
    B_0X1 = 1,
    #[doc = "2: Read data cache"]
    B_0X2 = 2,
}
impl From<CSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CSEL`"]
pub type CSEL_R = crate::R<u8, CSEL_A>;
impl CSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CSEL_A::B_0X0),
            1 => Val(CSEL_A::B_0X1),
            2 => Val(CSEL_A::B_0X2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CSEL_A::B_0X2
    }
}
#[doc = "Write proxy for field `CSEL`"]
pub struct CSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Command cache"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CSEL_A::B_0X0)
    }
    #[doc = "Expected data cache"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CSEL_A::B_0X1)
    }
    #[doc = "Read data cache"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CSEL_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u16) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `INCA`"]
pub type INCA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INCA`"]
pub struct INCA_W<'a> {
    w: &'a mut W,
}
impl<'a> INCA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u16) & 0x01) << 10);
        self.w
    }
}
#[doc = "ATYPE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATYPE_A {
    #[doc = "0: Write access"]
    B_0X0 = 0,
    #[doc = "1: Read access"]
    B_0X1 = 1,
}
impl From<ATYPE_A> for bool {
    #[inline(always)]
    fn from(variant: ATYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ATYPE`"]
pub type ATYPE_R = crate::R<bool, ATYPE_A>;
impl ATYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATYPE_A {
        match self.bits {
            false => ATYPE_A::B_0X0,
            true => ATYPE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ATYPE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ATYPE_A::B_0X1
    }
}
#[doc = "Write proxy for field `ATYPE`"]
pub struct ATYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ATYPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write access"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ATYPE_A::B_0X0)
    }
    #[doc = "Read access"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ATYPE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u16) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CWADDR"]
    #[inline(always)]
    pub fn cwaddr(&self) -> CWADDR_R {
        CWADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CSADDR"]
    #[inline(always)]
    pub fn csaddr(&self) -> CSADDR_R {
        CSADDR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - CSEL"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - INCA"]
    #[inline(always)]
    pub fn inca(&self) -> INCA_R {
        INCA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ATYPE"]
    #[inline(always)]
    pub fn atype(&self) -> ATYPE_R {
        ATYPE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CWADDR"]
    #[inline(always)]
    pub fn cwaddr(&mut self) -> CWADDR_W {
        CWADDR_W { w: self }
    }
    #[doc = "Bits 4:7 - CSADDR"]
    #[inline(always)]
    pub fn csaddr(&mut self) -> CSADDR_W {
        CSADDR_W { w: self }
    }
    #[doc = "Bits 8:9 - CSEL"]
    #[inline(always)]
    pub fn csel(&mut self) -> CSEL_W {
        CSEL_W { w: self }
    }
    #[doc = "Bit 10 - INCA"]
    #[inline(always)]
    pub fn inca(&mut self) -> INCA_W {
        INCA_W { w: self }
    }
    #[doc = "Bit 11 - ATYPE"]
    #[inline(always)]
    pub fn atype(&mut self) -> ATYPE_W {
        ATYPE_W { w: self }
    }
}
