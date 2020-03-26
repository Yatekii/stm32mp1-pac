#[doc = "Reader of register RCC_PLL3CFGR1"]
pub type R = crate::R<u32, super::RCC_PLL3CFGR1>;
#[doc = "Writer for register RCC_PLL3CFGR1"]
pub type W = crate::W<u32, super::RCC_PLL3CFGR1>;
#[doc = "Register RCC_PLL3CFGR1 `reset()`'s with value 0x0001_0031"]
impl crate::ResetValue for super::RCC_PLL3CFGR1 {
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
    #[doc = "26: Division ratio is 27"]
    B_0X1A = 26,
    #[doc = "49: Division ratio is 50 (default after\r\n                  reset)"]
    B_0X31 = 49,
    #[doc = "199: Division ratio is 200"]
    B_0XC7 = 199,
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
            26 => Val(DIVN_A::B_0X1A),
            49 => Val(DIVN_A::B_0X31),
            199 => Val(DIVN_A::B_0XC7),
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
    #[doc = "Checks if the value of the field is `B_0X1A`"]
    #[inline(always)]
    pub fn is_b_0x1a(&self) -> bool {
        *self == DIVN_A::B_0X1A
    }
    #[doc = "Checks if the value of the field is `B_0X31`"]
    #[inline(always)]
    pub fn is_b_0x31(&self) -> bool {
        *self == DIVN_A::B_0X31
    }
    #[doc = "Checks if the value of the field is `B_0XC7`"]
    #[inline(always)]
    pub fn is_b_0x_c7(&self) -> bool {
        *self == DIVN_A::B_0XC7
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
    #[doc = "Division ratio is 27"]
    #[inline(always)]
    pub fn b_0x1a(self) -> &'a mut W {
        self.variant(DIVN_A::B_0X1A)
    }
    #[doc = "Division ratio is 50 (default after reset)"]
    #[inline(always)]
    pub fn b_0x31(self) -> &'a mut W {
        self.variant(DIVN_A::B_0X31)
    }
    #[doc = "Division ratio is 200"]
    #[inline(always)]
    pub fn b_0x_c7(self) -> &'a mut W {
        self.variant(DIVN_A::B_0XC7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "DIVM3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVM3_A {
    #[doc = "0: bypass"]
    B_0X0 = 0,
    #[doc = "1: division by 2 (default after\r\n                  reset)"]
    B_0X1 = 1,
    #[doc = "2: division by 3"]
    B_0X2 = 2,
    #[doc = "63: division by 64"]
    B_0X3F = 63,
}
impl From<DIVM3_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVM3`"]
pub type DIVM3_R = crate::R<u8, DIVM3_A>;
impl DIVM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVM3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVM3_A::B_0X0),
            1 => Val(DIVM3_A::B_0X1),
            2 => Val(DIVM3_A::B_0X2),
            63 => Val(DIVM3_A::B_0X3F),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DIVM3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DIVM3_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DIVM3_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3F`"]
    #[inline(always)]
    pub fn is_b_0x3f(&self) -> bool {
        *self == DIVM3_A::B_0X3F
    }
}
#[doc = "Write proxy for field `DIVM3`"]
pub struct DIVM3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVM3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "bypass"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIVM3_A::B_0X0)
    }
    #[doc = "division by 2 (default after reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIVM3_A::B_0X1)
    }
    #[doc = "division by 3"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DIVM3_A::B_0X2)
    }
    #[doc = "division by 64"]
    #[inline(always)]
    pub fn b_0x3f(self) -> &'a mut W {
        self.variant(DIVM3_A::B_0X3F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `IFRGE`"]
pub type IFRGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IFRGE`"]
pub struct IFRGE_W<'a> {
    w: &'a mut W,
}
impl<'a> IFRGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    pub fn divn(&self) -> DIVN_R {
        DIVN_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:21 - DIVM3"]
    #[inline(always)]
    pub fn divm3(&self) -> DIVM3_R {
        DIVM3_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - IFRGE"]
    #[inline(always)]
    pub fn ifrge(&self) -> IFRGE_R {
        IFRGE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - DIVN"]
    #[inline(always)]
    pub fn divn(&mut self) -> DIVN_W {
        DIVN_W { w: self }
    }
    #[doc = "Bits 16:21 - DIVM3"]
    #[inline(always)]
    pub fn divm3(&mut self) -> DIVM3_W {
        DIVM3_W { w: self }
    }
    #[doc = "Bits 24:25 - IFRGE"]
    #[inline(always)]
    pub fn ifrge(&mut self) -> IFRGE_W {
        IFRGE_W { w: self }
    }
}
