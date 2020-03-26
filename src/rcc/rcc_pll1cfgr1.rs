#[doc = "Reader of register RCC_PLL1CFGR1"]
pub type R = crate::R<u32, super::RCC_PLL1CFGR1>;
#[doc = "Writer for register RCC_PLL1CFGR1"]
pub type W = crate::W<u32, super::RCC_PLL1CFGR1>;
#[doc = "Register RCC_PLL1CFGR1 `reset()`'s with value 0x0001_0031"]
impl crate::ResetValue for super::RCC_PLL1CFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0031
    }
}
#[doc = "DIVN\n\nValue on reset: 49"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum DIVN_A {
    #[doc = "24: Division ratio is 25"]
    B_0X18 = 24,
    #[doc = "25: Division ratio is 26"]
    B_0X19 = 25,
    #[doc = "49: Division ratio is 50 (default after\r\n                  reset)"]
    B_0X31 = 49,
    #[doc = "99: Division ratio is 100"]
    B_0X63 = 99,
}
impl From<DIVN_A> for u16 {
    #[inline(always)]
    fn from(variant: DIVN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVN`"]
pub type DIVN_R = crate::R<u16, DIVN_A>;
impl DIVN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, DIVN_A> {
        use crate::Variant::*;
        match self.bits {
            24 => Val(DIVN_A::B_0X18),
            25 => Val(DIVN_A::B_0X19),
            49 => Val(DIVN_A::B_0X31),
            99 => Val(DIVN_A::B_0X63),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X18`"]
    #[inline(always)]
    pub fn is_b_0x18(&self) -> bool {
        *self == DIVN_A::B_0X18
    }
    #[doc = "Checks if the value of the field is `B_0X19`"]
    #[inline(always)]
    pub fn is_b_0x19(&self) -> bool {
        *self == DIVN_A::B_0X19
    }
    #[doc = "Checks if the value of the field is `B_0X31`"]
    #[inline(always)]
    pub fn is_b_0x31(&self) -> bool {
        *self == DIVN_A::B_0X31
    }
    #[doc = "Checks if the value of the field is `B_0X63`"]
    #[inline(always)]
    pub fn is_b_0x63(&self) -> bool {
        *self == DIVN_A::B_0X63
    }
}
#[doc = "Write proxy for field `DIVN`"]
pub struct DIVN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Division ratio is 25"]
    #[inline(always)]
    pub fn b_0x18(self) -> &'a mut W {
        self.variant(DIVN_A::B_0X18)
    }
    #[doc = "Division ratio is 26"]
    #[inline(always)]
    pub fn b_0x19(self) -> &'a mut W {
        self.variant(DIVN_A::B_0X19)
    }
    #[doc = "Division ratio is 50 (default after reset)"]
    #[inline(always)]
    pub fn b_0x31(self) -> &'a mut W {
        self.variant(DIVN_A::B_0X31)
    }
    #[doc = "Division ratio is 100"]
    #[inline(always)]
    pub fn b_0x63(self) -> &'a mut W {
        self.variant(DIVN_A::B_0X63)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "DIVM1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVM1_A {
    #[doc = "0: bypass"]
    B_0X0 = 0,
    #[doc = "1: division by 2 (default after\r\n                  reset)"]
    B_0X1 = 1,
    #[doc = "2: division by 3"]
    B_0X2 = 2,
    #[doc = "63: division by 64"]
    B_0X3F = 63,
}
impl From<DIVM1_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVM1`"]
pub type DIVM1_R = crate::R<u8, DIVM1_A>;
impl DIVM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVM1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVM1_A::B_0X0),
            1 => Val(DIVM1_A::B_0X1),
            2 => Val(DIVM1_A::B_0X2),
            63 => Val(DIVM1_A::B_0X3F),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DIVM1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DIVM1_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DIVM1_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3F`"]
    #[inline(always)]
    pub fn is_b_0x3f(&self) -> bool {
        *self == DIVM1_A::B_0X3F
    }
}
#[doc = "Write proxy for field `DIVM1`"]
pub struct DIVM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVM1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIVM1_A::B_0X0)
    }
    #[doc = "division by 2 (default after reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIVM1_A::B_0X1)
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DIVM1_A::B_0X2)
    }
    #[doc = "division by 64"]
    #[inline(always)]
    pub fn b_0x3f(self) -> &'a mut W {
        self.variant(DIVM1_A::B_0X3F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:21 - DIVM1"]
    #[inline(always)]
    pub fn divm1(&self) -> DIVM1_R {
        DIVM1_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    pub fn divn(&mut self) -> DIVN_W {
        DIVN_W { w: self }
    }
    #[doc = "Bits 16:21 - DIVM1"]
    #[inline(always)]
    pub fn divm1(&mut self) -> DIVM1_W {
        DIVM1_W { w: self }
    }
}
