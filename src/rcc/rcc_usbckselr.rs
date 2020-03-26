#[doc = "Reader of register RCC_USBCKSELR"]
pub type R = crate::R<u32, super::RCC_USBCKSELR>;
#[doc = "Writer for register RCC_USBCKSELR"]
pub type W = crate::W<u32, super::RCC_USBCKSELR>;
#[doc = "Register RCC_USBCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_USBCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USBPHYSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBPHYSRC_A {
    #[doc = "0: hse_ker_ck clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: pll4_r_ck clock selected as kernel\r\n                  peripheral clock"]
    B_0X1 = 1,
    #[doc = "2: hse_ker_ck/2 clock selected as\r\n                  kernel peripheral clock"]
    B_0X2 = 2,
}
impl From<USBPHYSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: USBPHYSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USBPHYSRC`"]
pub type USBPHYSRC_R = crate::R<u8, USBPHYSRC_A>;
impl USBPHYSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, USBPHYSRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(USBPHYSRC_A::B_0X0),
            1 => Val(USBPHYSRC_A::B_0X1),
            2 => Val(USBPHYSRC_A::B_0X2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBPHYSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBPHYSRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == USBPHYSRC_A::B_0X2
    }
}
#[doc = "Write proxy for field `USBPHYSRC`"]
pub struct USBPHYSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPHYSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "hse_ker_ck clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBPHYSRC_A::B_0X0)
    }
    #[doc = "pll4_r_ck clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBPHYSRC_A::B_0X1)
    }
    #[doc = "hse_ker_ck/2 clock selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(USBPHYSRC_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "USBOSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOSRC_A {
    #[doc = "0: pll4_r_ck clock selected as kernel\r\n                  peripheral clock (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: clock provided by the USB PHY\r\n                  (rcc_ck_usbo_48m) selected as kernel peripheral\r\n                  clock"]
    B_0X1 = 1,
}
impl From<USBOSRC_A> for bool {
    #[inline(always)]
    fn from(variant: USBOSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBOSRC`"]
pub type USBOSRC_R = crate::R<bool, USBOSRC_A>;
impl USBOSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBOSRC_A {
        match self.bits {
            false => USBOSRC_A::B_0X0,
            true => USBOSRC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBOSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBOSRC_A::B_0X1
    }
}
#[doc = "Write proxy for field `USBOSRC`"]
pub struct USBOSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBOSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "pll4_r_ck clock selected as kernel peripheral clock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBOSRC_A::B_0X0)
    }
    #[doc = "clock provided by the USB PHY (rcc_ck_usbo_48m) selected as kernel peripheral clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBOSRC_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USBPHYSRC"]
    #[inline(always)]
    pub fn usbphysrc(&self) -> USBPHYSRC_R {
        USBPHYSRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - USBOSRC"]
    #[inline(always)]
    pub fn usbosrc(&self) -> USBOSRC_R {
        USBOSRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USBPHYSRC"]
    #[inline(always)]
    pub fn usbphysrc(&mut self) -> USBPHYSRC_W {
        USBPHYSRC_W { w: self }
    }
    #[doc = "Bit 4 - USBOSRC"]
    #[inline(always)]
    pub fn usbosrc(&mut self) -> USBOSRC_W {
        USBOSRC_W { w: self }
    }
}
