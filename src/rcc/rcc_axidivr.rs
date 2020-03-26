#[doc = "Reader of register RCC_AXIDIVR"]
pub type R = crate::R<u32, super::RCC_AXIDIVR>;
#[doc = "Writer for register RCC_AXIDIVR"]
pub type W = crate::W<u32, super::RCC_AXIDIVR>;
#[doc = "Register RCC_AXIDIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_AXIDIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "AXIDIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AXIDIV_A {
    #[doc = "0: axiss_ck (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: axiss_ck / 2"]
    B_0X1 = 1,
    #[doc = "2: axiss_ck / 3"]
    B_0X2 = 2,
}
impl From<AXIDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: AXIDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AXIDIV`"]
pub type AXIDIV_R = crate::R<u8, AXIDIV_A>;
impl AXIDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AXIDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AXIDIV_A::B_0X0),
            1 => Val(AXIDIV_A::B_0X1),
            2 => Val(AXIDIV_A::B_0X2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AXIDIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AXIDIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AXIDIV_A::B_0X2
    }
}
#[doc = "Write proxy for field `AXIDIV`"]
pub struct AXIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXIDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "axiss_ck (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AXIDIV_A::B_0X0)
    }
    #[doc = "axiss_ck / 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AXIDIV_A::B_0X1)
    }
    #[doc = "axiss_ck / 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(AXIDIV_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "AXIDIVRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXIDIVRDY_A {
    #[doc = "0: The new division factor is not yet\r\n                  taken into account."]
    B_0X0 = 0,
    #[doc = "1: The new division factor is taken\r\n                  into account. (default after reset)"]
    B_0X1 = 1,
}
impl From<AXIDIVRDY_A> for bool {
    #[inline(always)]
    fn from(variant: AXIDIVRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXIDIVRDY`"]
pub type AXIDIVRDY_R = crate::R<bool, AXIDIVRDY_A>;
impl AXIDIVRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXIDIVRDY_A {
        match self.bits {
            false => AXIDIVRDY_A::B_0X0,
            true => AXIDIVRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AXIDIVRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AXIDIVRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:2 - AXIDIV"]
    #[inline(always)]
    pub fn axidiv(&self) -> AXIDIV_R {
        AXIDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - AXIDIVRDY"]
    #[inline(always)]
    pub fn axidivrdy(&self) -> AXIDIVRDY_R {
        AXIDIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - AXIDIV"]
    #[inline(always)]
    pub fn axidiv(&mut self) -> AXIDIV_W {
        AXIDIV_W { w: self }
    }
}
