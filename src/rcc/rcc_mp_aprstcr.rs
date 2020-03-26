#[doc = "Reader of register RCC_MP_APRSTCR"]
pub type R = crate::R<u32, super::RCC_MP_APRSTCR>;
#[doc = "Writer for register RCC_MP_APRSTCR"]
pub type W = crate::W<u32, super::RCC_MP_APRSTCR>;
#[doc = "Register RCC_MP_APRSTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_APRSTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RDCTLEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDCTLEN_A {
    #[doc = "0: The RDCTL control block is bypassed\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: The RDCTL control block is\r\n                  enabled."]
    B_0X1 = 1,
}
impl From<RDCTLEN_A> for bool {
    #[inline(always)]
    fn from(variant: RDCTLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDCTLEN`"]
pub type RDCTLEN_R = crate::R<bool, RDCTLEN_A>;
impl RDCTLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDCTLEN_A {
        match self.bits {
            false => RDCTLEN_A::B_0X0,
            true => RDCTLEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RDCTLEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RDCTLEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `RDCTLEN`"]
pub struct RDCTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCTLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDCTLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The RDCTL control block is bypassed (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RDCTLEN_A::B_0X0)
    }
    #[doc = "The RDCTL control block is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RDCTLEN_A::B_0X1)
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
#[doc = "RSTTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTTO_A {
    #[doc = "0: The timeout function is disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: The timeout is set to 2 x 2HSIDIV\r\n                  us"]
    B_0X1 = 1,
    #[doc = "127: The timeout is set to 128 x 2HSIDIV\r\n                  us"]
    B_0X7F = 127,
}
impl From<RSTTO_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTTO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSTTO`"]
pub type RSTTO_R = crate::R<u8, RSTTO_A>;
impl RSTTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSTTO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSTTO_A::B_0X0),
            1 => Val(RSTTO_A::B_0X1),
            127 => Val(RSTTO_A::B_0X7F),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RSTTO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RSTTO_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X7F`"]
    #[inline(always)]
    pub fn is_b_0x7f(&self) -> bool {
        *self == RSTTO_A::B_0X7F
    }
}
#[doc = "Write proxy for field `RSTTO`"]
pub struct RSTTO_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSTTO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The timeout function is disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RSTTO_A::B_0X0)
    }
    #[doc = "The timeout is set to 2 x 2HSIDIV us"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RSTTO_A::B_0X1)
    }
    #[doc = "The timeout is set to 128 x 2HSIDIV us"]
    #[inline(always)]
    pub fn b_0x7f(self) -> &'a mut W {
        self.variant(RSTTO_A::B_0X7F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RDCTLEN"]
    #[inline(always)]
    pub fn rdctlen(&self) -> RDCTLEN_R {
        RDCTLEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - RSTTO"]
    #[inline(always)]
    pub fn rstto(&self) -> RSTTO_R {
        RSTTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RDCTLEN"]
    #[inline(always)]
    pub fn rdctlen(&mut self) -> RDCTLEN_W {
        RDCTLEN_W { w: self }
    }
    #[doc = "Bits 8:14 - RSTTO"]
    #[inline(always)]
    pub fn rstto(&mut self) -> RSTTO_W {
        RSTTO_W { w: self }
    }
}
