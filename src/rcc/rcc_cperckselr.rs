#[doc = "Reader of register RCC_CPERCKSELR"]
pub type R = crate::R<u32, super::RCC_CPERCKSELR>;
#[doc = "Writer for register RCC_CPERCKSELR"]
pub type W = crate::W<u32, super::RCC_CPERCKSELR>;
#[doc = "Register RCC_CPERCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_CPERCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CKPERSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKPERSRC_A {
    #[doc = "0: hsi_ker_ck clock selected (default\r\n                  after reset)"]
    B_0X0 = 0,
    #[doc = "1: csi_ker_ck clock\r\n                  selected"]
    B_0X1 = 1,
    #[doc = "2: hse_ker_ck clock\r\n                  selected"]
    B_0X2 = 2,
    #[doc = "3: Clock disabled"]
    B_0X3 = 3,
}
impl From<CKPERSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CKPERSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CKPERSRC`"]
pub type CKPERSRC_R = crate::R<u8, CKPERSRC_A>;
impl CKPERSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPERSRC_A {
        match self.bits {
            0 => CKPERSRC_A::B_0X0,
            1 => CKPERSRC_A::B_0X1,
            2 => CKPERSRC_A::B_0X2,
            3 => CKPERSRC_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CKPERSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CKPERSRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CKPERSRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CKPERSRC_A::B_0X3
    }
}
#[doc = "Write proxy for field `CKPERSRC`"]
pub struct CKPERSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPERSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKPERSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "hsi_ker_ck clock selected (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKPERSRC_A::B_0X0)
    }
    #[doc = "csi_ker_ck clock selected"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CKPERSRC_A::B_0X1)
    }
    #[doc = "hse_ker_ck clock selected"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(CKPERSRC_A::B_0X2)
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(CKPERSRC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CKPERSRC"]
    #[inline(always)]
    pub fn ckpersrc(&self) -> CKPERSRC_R {
        CKPERSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CKPERSRC"]
    #[inline(always)]
    pub fn ckpersrc(&mut self) -> CKPERSRC_W {
        CKPERSRC_W { w: self }
    }
}
