#[doc = "Reader of register DFSDM_FLT3FCR"]
pub type R = crate::R<u32, super::DFSDM_FLT3FCR>;
#[doc = "Writer for register DFSDM_FLT3FCR"]
pub type W = crate::W<u32, super::DFSDM_FLT3FCR>;
#[doc = "Register DFSDM_FLT3FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DFSDM_FLT3FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IOSR`"]
pub type IOSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOSR`"]
pub struct IOSR_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `FOSR`"]
pub type FOSR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FOSR`"]
pub struct FOSR_W<'a> {
    w: &'a mut W,
}
impl<'a> FOSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "FORD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FORD_A {
    #[doc = "0: FastSinc filter type"]
    B_0X0 = 0,
    #[doc = "1: Sinc1 filter type"]
    B_0X1 = 1,
    #[doc = "2: Sinc2 filter type"]
    B_0X2 = 2,
    #[doc = "3: Sinc3 filter type"]
    B_0X3 = 3,
    #[doc = "4: Sinc4 filter type"]
    B_0X4 = 4,
    #[doc = "5: Sinc5 filter type"]
    B_0X5 = 5,
}
impl From<FORD_A> for u8 {
    #[inline(always)]
    fn from(variant: FORD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FORD`"]
pub type FORD_R = crate::R<u8, FORD_A>;
impl FORD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FORD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FORD_A::B_0X0),
            1 => Val(FORD_A::B_0X1),
            2 => Val(FORD_A::B_0X2),
            3 => Val(FORD_A::B_0X3),
            4 => Val(FORD_A::B_0X4),
            5 => Val(FORD_A::B_0X5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FORD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FORD_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == FORD_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == FORD_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == FORD_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == FORD_A::B_0X5
    }
}
#[doc = "Write proxy for field `FORD`"]
pub struct FORD_W<'a> {
    w: &'a mut W,
}
impl<'a> FORD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FastSinc filter type"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FORD_A::B_0X0)
    }
    #[doc = "Sinc1 filter type"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FORD_A::B_0X1)
    }
    #[doc = "Sinc2 filter type"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(FORD_A::B_0X2)
    }
    #[doc = "Sinc3 filter type"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(FORD_A::B_0X3)
    }
    #[doc = "Sinc4 filter type"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(FORD_A::B_0X4)
    }
    #[doc = "Sinc5 filter type"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(FORD_A::B_0X5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - IOSR"]
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - FOSR"]
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:31 - FORD"]
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IOSR"]
    #[inline(always)]
    pub fn iosr(&mut self) -> IOSR_W {
        IOSR_W { w: self }
    }
    #[doc = "Bits 16:25 - FOSR"]
    #[inline(always)]
    pub fn fosr(&mut self) -> FOSR_W {
        FOSR_W { w: self }
    }
    #[doc = "Bits 29:31 - FORD"]
    #[inline(always)]
    pub fn ford(&mut self) -> FORD_W {
        FORD_W { w: self }
    }
}
