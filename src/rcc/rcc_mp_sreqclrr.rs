#[doc = "Reader of register RCC_MP_SREQCLRR"]
pub type R = crate::R<u32, super::RCC_MP_SREQCLRR>;
#[doc = "Writer for register RCC_MP_SREQCLRR"]
pub type W = crate::W<u32, super::RCC_MP_SREQCLRR>;
#[doc = "Register RCC_MP_SREQCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_SREQCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "STPREQ_P0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPREQ_P0_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the MPU processor number 0 does not allow\r\n                  the MPU domain to go to CSTOP"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the STPREQ_P0 bit,\r\n                  reading means that the MPU processor number 0\r\n                  allows the MPU domain to go to\r\n                  CSTOP"]
    B_0X1 = 1,
}
impl From<STPREQ_P0_A> for bool {
    #[inline(always)]
    fn from(variant: STPREQ_P0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STPREQ_P0`"]
pub type STPREQ_P0_R = crate::R<bool, STPREQ_P0_A>;
impl STPREQ_P0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPREQ_P0_A {
        match self.bits {
            false => STPREQ_P0_A::B_0X0,
            true => STPREQ_P0_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STPREQ_P0_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STPREQ_P0_A::B_0X1
    }
}
#[doc = "Write proxy for field `STPREQ_P0`"]
pub struct STPREQ_P0_W<'a> {
    w: &'a mut W,
}
impl<'a> STPREQ_P0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPREQ_P0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the MPU processor number 0 does not allow the MPU domain to go to CSTOP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(STPREQ_P0_A::B_0X0)
    }
    #[doc = "Writing clears the STPREQ_P0 bit, reading means that the MPU processor number 0 allows the MPU domain to go to CSTOP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(STPREQ_P0_A::B_0X1)
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
#[doc = "STPREQ_P1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPREQ_P1_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the MPU processor number 1 does not allow\r\n                  the MPU domain to go to CSTOP"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the STPREQ_P1 bit,\r\n                  reading means that the MPU processor number 1\r\n                  allows the MPU domain to go to\r\n                  CSTOP"]
    B_0X1 = 1,
}
impl From<STPREQ_P1_A> for bool {
    #[inline(always)]
    fn from(variant: STPREQ_P1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STPREQ_P1`"]
pub type STPREQ_P1_R = crate::R<bool, STPREQ_P1_A>;
impl STPREQ_P1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPREQ_P1_A {
        match self.bits {
            false => STPREQ_P1_A::B_0X0,
            true => STPREQ_P1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STPREQ_P1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STPREQ_P1_A::B_0X1
    }
}
#[doc = "Write proxy for field `STPREQ_P1`"]
pub struct STPREQ_P1_W<'a> {
    w: &'a mut W,
}
impl<'a> STPREQ_P1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPREQ_P1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the MPU processor number 1 does not allow the MPU domain to go to CSTOP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(STPREQ_P1_A::B_0X0)
    }
    #[doc = "Writing clears the STPREQ_P1 bit, reading means that the MPU processor number 1 allows the MPU domain to go to CSTOP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(STPREQ_P1_A::B_0X1)
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
    #[doc = "Bit 0 - STPREQ_P0"]
    #[inline(always)]
    pub fn stpreq_p0(&self) -> STPREQ_P0_R {
        STPREQ_P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - STPREQ_P1"]
    #[inline(always)]
    pub fn stpreq_p1(&self) -> STPREQ_P1_R {
        STPREQ_P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STPREQ_P0"]
    #[inline(always)]
    pub fn stpreq_p0(&mut self) -> STPREQ_P0_W {
        STPREQ_P0_W { w: self }
    }
    #[doc = "Bit 1 - STPREQ_P1"]
    #[inline(always)]
    pub fn stpreq_p1(&mut self) -> STPREQ_P1_W {
        STPREQ_P1_W { w: self }
    }
}
