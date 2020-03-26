#[doc = "Reader of register RCC_RTCDIVR"]
pub type R = crate::R<u32, super::RCC_RTCDIVR>;
#[doc = "Writer for register RCC_RTCDIVR"]
pub type W = crate::W<u32, super::RCC_RTCDIVR>;
#[doc = "Register RCC_RTCDIVR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_RTCDIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RTCDIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCDIV_A {
    #[doc = "0: HSE (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: HSE/2"]
    B_0X1 = 1,
    #[doc = "2: HSE/3"]
    B_0X2 = 2,
    #[doc = "3: HSE/4"]
    B_0X3 = 3,
    #[doc = "62: HSE/63"]
    B_0X3E = 62,
    #[doc = "63: HSE/64"]
    B_0X3F = 63,
}
impl From<RTCDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCDIV`"]
pub type RTCDIV_R = crate::R<u8, RTCDIV_A>;
impl RTCDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RTCDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RTCDIV_A::B_0X0),
            1 => Val(RTCDIV_A::B_0X1),
            2 => Val(RTCDIV_A::B_0X2),
            3 => Val(RTCDIV_A::B_0X3),
            62 => Val(RTCDIV_A::B_0X3E),
            63 => Val(RTCDIV_A::B_0X3F),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCDIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCDIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == RTCDIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == RTCDIV_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X3E`"]
    #[inline(always)]
    pub fn is_b_0x3e(&self) -> bool {
        *self == RTCDIV_A::B_0X3E
    }
    #[doc = "Checks if the value of the field is `B_0X3F`"]
    #[inline(always)]
    pub fn is_b_0x3f(&self) -> bool {
        *self == RTCDIV_A::B_0X3F
    }
}
#[doc = "Write proxy for field `RTCDIV`"]
pub struct RTCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSE (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RTCDIV_A::B_0X0)
    }
    #[doc = "HSE/2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RTCDIV_A::B_0X1)
    }
    #[doc = "HSE/3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(RTCDIV_A::B_0X2)
    }
    #[doc = "HSE/4"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(RTCDIV_A::B_0X3)
    }
    #[doc = "HSE/63"]
    #[inline(always)]
    pub fn b_0x3e(self) -> &'a mut W {
        self.variant(RTCDIV_A::B_0X3E)
    }
    #[doc = "HSE/64"]
    #[inline(always)]
    pub fn b_0x3f(self) -> &'a mut W {
        self.variant(RTCDIV_A::B_0X3F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - RTCDIV"]
    #[inline(always)]
    pub fn rtcdiv(&self) -> RTCDIV_R {
        RTCDIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RTCDIV"]
    #[inline(always)]
    pub fn rtcdiv(&mut self) -> RTCDIV_W {
        RTCDIV_W { w: self }
    }
}
