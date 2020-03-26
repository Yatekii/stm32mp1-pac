#[doc = "Reader of register RCC_MCUDIVR"]
pub type R = crate::R<u32, super::RCC_MCUDIVR>;
#[doc = "Writer for register RCC_MCUDIVR"]
pub type W = crate::W<u32, super::RCC_MCUDIVR>;
#[doc = "Register RCC_MCUDIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_MCUDIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "MCUDIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCUDIV_A {
    #[doc = "0: mcuss_ck not divided (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: mcuss_ck divided by 2"]
    B_0X1 = 1,
    #[doc = "2: mcuss_ck divided by 4"]
    B_0X2 = 2,
    #[doc = "3: mcuss_ck divided by 8"]
    B_0X3 = 3,
    #[doc = "4: mcuss_ck divided by 16"]
    B_0X4 = 4,
    #[doc = "5: mcuss_ck divided by 32"]
    B_0X5 = 5,
    #[doc = "6: mcuss_ck divided by 64"]
    B_0X6 = 6,
    #[doc = "7: mcuss_ck divided by\r\n                  128"]
    B_0X7 = 7,
    #[doc = "8: mcuss_ck divided by\r\n                  256"]
    B_0X8 = 8,
}
impl From<MCUDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MCUDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCUDIV`"]
pub type MCUDIV_R = crate::R<u8, MCUDIV_A>;
impl MCUDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCUDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MCUDIV_A::B_0X0),
            1 => Val(MCUDIV_A::B_0X1),
            2 => Val(MCUDIV_A::B_0X2),
            3 => Val(MCUDIV_A::B_0X3),
            4 => Val(MCUDIV_A::B_0X4),
            5 => Val(MCUDIV_A::B_0X5),
            6 => Val(MCUDIV_A::B_0X6),
            7 => Val(MCUDIV_A::B_0X7),
            8 => Val(MCUDIV_A::B_0X8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCUDIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCUDIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MCUDIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MCUDIV_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == MCUDIV_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == MCUDIV_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == MCUDIV_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == MCUDIV_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == MCUDIV_A::B_0X8
    }
}
#[doc = "Write proxy for field `MCUDIV`"]
pub struct MCUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCUDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "mcuss_ck not divided (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCUDIV_A::B_0X0)
    }
    #[doc = "mcuss_ck divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCUDIV_A::B_0X1)
    }
    #[doc = "mcuss_ck divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MCUDIV_A::B_0X2)
    }
    #[doc = "mcuss_ck divided by 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(MCUDIV_A::B_0X3)
    }
    #[doc = "mcuss_ck divided by 16"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(MCUDIV_A::B_0X4)
    }
    #[doc = "mcuss_ck divided by 32"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(MCUDIV_A::B_0X5)
    }
    #[doc = "mcuss_ck divided by 64"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(MCUDIV_A::B_0X6)
    }
    #[doc = "mcuss_ck divided by 128"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(MCUDIV_A::B_0X7)
    }
    #[doc = "mcuss_ck divided by 256"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(MCUDIV_A::B_0X8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "MCUDIVRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCUDIVRDY_A {
    #[doc = "0: The new division factor is not yet\r\n                  taken into account."]
    B_0X0 = 0,
    #[doc = "1: The new division factor is taken\r\n                  into account. (default after reset)"]
    B_0X1 = 1,
}
impl From<MCUDIVRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MCUDIVRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCUDIVRDY`"]
pub type MCUDIVRDY_R = crate::R<bool, MCUDIVRDY_A>;
impl MCUDIVRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCUDIVRDY_A {
        match self.bits {
            false => MCUDIVRDY_A::B_0X0,
            true => MCUDIVRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCUDIVRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCUDIVRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:3 - MCUDIV"]
    #[inline(always)]
    pub fn mcudiv(&self) -> MCUDIV_R {
        MCUDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - MCUDIVRDY"]
    #[inline(always)]
    pub fn mcudivrdy(&self) -> MCUDIVRDY_R {
        MCUDIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MCUDIV"]
    #[inline(always)]
    pub fn mcudiv(&mut self) -> MCUDIV_W {
        MCUDIV_W { w: self }
    }
}
