#[doc = "Reader of register RCC_TZCR"]
pub type R = crate::R<u32, super::RCC_TZCR>;
#[doc = "Writer for register RCC_TZCR"]
pub type W = crate::W<u32, super::RCC_TZCR>;
#[doc = "Register RCC_TZCR `reset()`'s with value 0x03"]
impl crate::ResetValue for super::RCC_TZCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "TZEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TZEN_A {
    #[doc = "0: No protection"]
    B_0X0 = 0,
    #[doc = "1: TrustZone enabled (default after\r\n                  reset)."]
    B_0X1 = 1,
}
impl From<TZEN_A> for bool {
    #[inline(always)]
    fn from(variant: TZEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TZEN`"]
pub type TZEN_R = crate::R<bool, TZEN_A>;
impl TZEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TZEN_A {
        match self.bits {
            false => TZEN_A::B_0X0,
            true => TZEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TZEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TZEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TZEN`"]
pub struct TZEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TZEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No protection"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TZEN_A::B_0X0)
    }
    #[doc = "TrustZone enabled (default after reset)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TZEN_A::B_0X1)
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
#[doc = "MCKPROT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKPROT_A {
    #[doc = "0: The registers controlling mcuss_ck\r\n                  clock are not protected"]
    B_0X0 = 0,
    #[doc = "1: The registers controlling mcuss_ck\r\n                  clock are protected, (default after\r\n                  reset)."]
    B_0X1 = 1,
}
impl From<MCKPROT_A> for bool {
    #[inline(always)]
    fn from(variant: MCKPROT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCKPROT`"]
pub type MCKPROT_R = crate::R<bool, MCKPROT_A>;
impl MCKPROT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKPROT_A {
        match self.bits {
            false => MCKPROT_A::B_0X0,
            true => MCKPROT_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCKPROT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCKPROT_A::B_0X1
    }
}
#[doc = "Write proxy for field `MCKPROT`"]
pub struct MCKPROT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKPROT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCKPROT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The registers controlling mcuss_ck clock are not protected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCKPROT_A::B_0X0)
    }
    #[doc = "The registers controlling mcuss_ck clock are protected, (default after reset)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCKPROT_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - TZEN"]
    #[inline(always)]
    pub fn tzen(&self) -> TZEN_R {
        TZEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCKPROT"]
    #[inline(always)]
    pub fn mckprot(&self) -> MCKPROT_R {
        MCKPROT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZEN"]
    #[inline(always)]
    pub fn tzen(&mut self) -> TZEN_W {
        TZEN_W { w: self }
    }
    #[doc = "Bit 1 - MCKPROT"]
    #[inline(always)]
    pub fn mckprot(&mut self) -> MCKPROT_W {
        MCKPROT_W { w: self }
    }
}
