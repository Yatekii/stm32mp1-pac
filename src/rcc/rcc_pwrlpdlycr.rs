#[doc = "Reader of register RCC_PWRLPDLYCR"]
pub type R = crate::R<u32, super::RCC_PWRLPDLYCR>;
#[doc = "Writer for register RCC_PWRLPDLYCR"]
pub type W = crate::W<u32, super::RCC_PWRLPDLYCR>;
#[doc = "Register RCC_PWRLPDLYCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_PWRLPDLYCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PWRLP_DLY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PWRLP_DLY_A {
    #[doc = "0: The PWR_LP delay is bypassed\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: A PWR_LP delay of one period of HSI\r\n                  (at 64 MHz) is observed"]
    B_0X1 = 1,
    #[doc = "4194303: A PWR_LP delay of about 65.5\r\n                  milliseconds is observed"]
    B_0X3FFFFF = 4194303,
}
impl From<PWRLP_DLY_A> for u32 {
    #[inline(always)]
    fn from(variant: PWRLP_DLY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWRLP_DLY`"]
pub type PWRLP_DLY_R = crate::R<u32, PWRLP_DLY_A>;
impl PWRLP_DLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PWRLP_DLY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWRLP_DLY_A::B_0X0),
            1 => Val(PWRLP_DLY_A::B_0X1),
            4194303 => Val(PWRLP_DLY_A::B_0X3FFFFF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PWRLP_DLY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PWRLP_DLY_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X3FFFFF`"]
    #[inline(always)]
    pub fn is_b_0x3fffff(&self) -> bool {
        *self == PWRLP_DLY_A::B_0X3FFFFF
    }
}
#[doc = "Write proxy for field `PWRLP_DLY`"]
pub struct PWRLP_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRLP_DLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRLP_DLY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The PWR_LP delay is bypassed (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PWRLP_DLY_A::B_0X0)
    }
    #[doc = "A PWR_LP delay of one period of HSI (at 64 MHz) is observed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PWRLP_DLY_A::B_0X1)
    }
    #[doc = "A PWR_LP delay of about 65.5 milliseconds is observed"]
    #[inline(always)]
    pub fn b_0x3fffff(self) -> &'a mut W {
        self.variant(PWRLP_DLY_A::B_0X3FFFFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u32) & 0x003f_ffff);
        self.w
    }
}
#[doc = "MCTMPSKP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCTMPSKP_A {
    #[doc = "0: The clock restore sequence of the\r\n                  MCU waits for the PWR_LP delay before activating\r\n                  power consuming elements (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: The clock restore sequence of the\r\n                  MCU runs the PWR_LP delay but did not wait for\r\n                  the delay to elapse before activating power\r\n                  consuming elements"]
    B_0X1 = 1,
}
impl From<MCTMPSKP_A> for bool {
    #[inline(always)]
    fn from(variant: MCTMPSKP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCTMPSKP`"]
pub type MCTMPSKP_R = crate::R<bool, MCTMPSKP_A>;
impl MCTMPSKP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCTMPSKP_A {
        match self.bits {
            false => MCTMPSKP_A::B_0X0,
            true => MCTMPSKP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCTMPSKP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCTMPSKP_A::B_0X1
    }
}
#[doc = "Write proxy for field `MCTMPSKP`"]
pub struct MCTMPSKP_W<'a> {
    w: &'a mut W,
}
impl<'a> MCTMPSKP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCTMPSKP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock restore sequence of the MCU waits for the PWR_LP delay before activating power consuming elements (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCTMPSKP_A::B_0X0)
    }
    #[doc = "The clock restore sequence of the MCU runs the PWR_LP delay but did not wait for the delay to elapse before activating power consuming elements"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCTMPSKP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - PWRLP_DLY"]
    #[inline(always)]
    pub fn pwrlp_dly(&self) -> PWRLP_DLY_R {
        PWRLP_DLY_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 24 - MCTMPSKP"]
    #[inline(always)]
    pub fn mctmpskp(&self) -> MCTMPSKP_R {
        MCTMPSKP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - PWRLP_DLY"]
    #[inline(always)]
    pub fn pwrlp_dly(&mut self) -> PWRLP_DLY_W {
        PWRLP_DLY_W { w: self }
    }
    #[doc = "Bit 24 - MCTMPSKP"]
    #[inline(always)]
    pub fn mctmpskp(&mut self) -> MCTMPSKP_W {
        MCTMPSKP_W { w: self }
    }
}
