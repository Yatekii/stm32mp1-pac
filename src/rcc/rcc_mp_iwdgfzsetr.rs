#[doc = "Reader of register RCC_MP_IWDGFZSETR"]
pub type R = crate::R<u32, super::RCC_MP_IWDGFZSETR>;
#[doc = "Writer for register RCC_MP_IWDGFZSETR"]
pub type W = crate::W<u32, super::RCC_MP_IWDGFZSETR>;
#[doc = "Register RCC_MP_IWDGFZSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_IWDGFZSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FZ_IWDG1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FZ_IWDG1_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the IWDG1 clock is not frozen (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing freeze the IWDG1 clock,\r\n                  reading means that the IWDG1 clock is\r\n                  frozen"]
    B_0X1 = 1,
}
impl From<FZ_IWDG1_A> for bool {
    #[inline(always)]
    fn from(variant: FZ_IWDG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FZ_IWDG1`"]
pub type FZ_IWDG1_R = crate::R<bool, FZ_IWDG1_A>;
impl FZ_IWDG1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FZ_IWDG1_A {
        match self.bits {
            false => FZ_IWDG1_A::B_0X0,
            true => FZ_IWDG1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FZ_IWDG1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FZ_IWDG1_A::B_0X1
    }
}
#[doc = "Write proxy for field `FZ_IWDG1`"]
pub struct FZ_IWDG1_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_IWDG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FZ_IWDG1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the IWDG1 clock is not frozen (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FZ_IWDG1_A::B_0X0)
    }
    #[doc = "Writing freeze the IWDG1 clock, reading means that the IWDG1 clock is frozen"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FZ_IWDG1_A::B_0X1)
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
#[doc = "FZ_IWDG2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FZ_IWDG2_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the IWDG2 clock is not frozen (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing freeze the IWDG2 clock,\r\n                  reading means that the IWDG2 clock is\r\n                  frozen"]
    B_0X1 = 1,
}
impl From<FZ_IWDG2_A> for bool {
    #[inline(always)]
    fn from(variant: FZ_IWDG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FZ_IWDG2`"]
pub type FZ_IWDG2_R = crate::R<bool, FZ_IWDG2_A>;
impl FZ_IWDG2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FZ_IWDG2_A {
        match self.bits {
            false => FZ_IWDG2_A::B_0X0,
            true => FZ_IWDG2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FZ_IWDG2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FZ_IWDG2_A::B_0X1
    }
}
#[doc = "Write proxy for field `FZ_IWDG2`"]
pub struct FZ_IWDG2_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_IWDG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FZ_IWDG2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the IWDG2 clock is not frozen (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FZ_IWDG2_A::B_0X0)
    }
    #[doc = "Writing freeze the IWDG2 clock, reading means that the IWDG2 clock is frozen"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FZ_IWDG2_A::B_0X1)
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
    #[doc = "Bit 0 - FZ_IWDG1"]
    #[inline(always)]
    pub fn fz_iwdg1(&self) -> FZ_IWDG1_R {
        FZ_IWDG1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FZ_IWDG2"]
    #[inline(always)]
    pub fn fz_iwdg2(&self) -> FZ_IWDG2_R {
        FZ_IWDG2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FZ_IWDG1"]
    #[inline(always)]
    pub fn fz_iwdg1(&mut self) -> FZ_IWDG1_W {
        FZ_IWDG1_W { w: self }
    }
    #[doc = "Bit 1 - FZ_IWDG2"]
    #[inline(always)]
    pub fn fz_iwdg2(&mut self) -> FZ_IWDG2_W {
        FZ_IWDG2_W { w: self }
    }
}
