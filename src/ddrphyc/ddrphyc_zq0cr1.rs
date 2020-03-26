#[doc = "Reader of register DDRPHYC_ZQ0CR1"]
pub type R = crate::R<u8, super::DDRPHYC_ZQ0CR1>;
#[doc = "Writer for register DDRPHYC_ZQ0CR1"]
pub type W = crate::W<u8, super::DDRPHYC_ZQ0CR1>;
#[doc = "Register DDRPHYC_ZQ0CR1 `reset()`'s with value 0x7b"]
impl crate::ResetValue for super::DDRPHYC_ZQ0CR1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7b
    }
}
#[doc = "ZPROG\n\nValue on reset: 123"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ZPROG_A {
    #[doc = "1: 120ohm"]
    B_0X1 = 1,
    #[doc = "5: 60ohm"]
    B_0X5 = 5,
    #[doc = "8: 40ohm"]
    B_0X8 = 8,
    #[doc = "11: 40ohm"]
    B_0XB = 11,
    #[doc = "13: 34ohm"]
    B_0XD = 13,
}
impl From<ZPROG_A> for u8 {
    #[inline(always)]
    fn from(variant: ZPROG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ZPROG`"]
pub type ZPROG_R = crate::R<u8, ZPROG_A>;
impl ZPROG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ZPROG_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(ZPROG_A::B_0X1),
            5 => Val(ZPROG_A::B_0X5),
            8 => Val(ZPROG_A::B_0X8),
            11 => Val(ZPROG_A::B_0XB),
            13 => Val(ZPROG_A::B_0XD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ZPROG_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == ZPROG_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == ZPROG_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0XB`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == ZPROG_A::B_0XB
    }
    #[doc = "Checks if the value of the field is `B_0XD`"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == ZPROG_A::B_0XD
    }
}
#[doc = "Write proxy for field `ZPROG`"]
pub struct ZPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> ZPROG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZPROG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "120ohm"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ZPROG_A::B_0X1)
    }
    #[doc = "60ohm"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(ZPROG_A::B_0X5)
    }
    #[doc = "40ohm"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(ZPROG_A::B_0X8)
    }
    #[doc = "40ohm"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(ZPROG_A::B_0XB)
    }
    #[doc = "34ohm"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(ZPROG_A::B_0XD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ZPROG"]
    #[inline(always)]
    pub fn zprog(&self) -> ZPROG_R {
        ZPROG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ZPROG"]
    #[inline(always)]
    pub fn zprog(&mut self) -> ZPROG_W {
        ZPROG_W { w: self }
    }
}
