#[doc = "Reader of register RCC_MC_APB4LPENCLRR"]
pub type R = crate::R<u32, super::RCC_MC_APB4LPENCLRR>;
#[doc = "Writer for register RCC_MC_APB4LPENCLRR"]
pub type W = crate::W<u32, super::RCC_MC_APB4LPENCLRR>;
#[doc = "Register RCC_MC_APB4LPENCLRR `reset()`'s with value 0x0011_0111"]
impl crate::ResetValue for super::RCC_MC_APB4LPENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0011_0111
    }
}
#[doc = "LTDCLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTDCLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<LTDCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LTDCLPEN`"]
pub type LTDCLPEN_R = crate::R<bool, LTDCLPEN_A>;
impl LTDCLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTDCLPEN_A {
        match self.bits {
            false => LTDCLPEN_A::B_0X0,
            true => LTDCLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LTDCLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LTDCLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LTDCLPEN`"]
pub struct LTDCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LTDCLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LTDCLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LTDCLPEN_A::B_0X1)
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
#[doc = "DSILPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSILPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<DSILPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DSILPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSILPEN`"]
pub type DSILPEN_R = crate::R<bool, DSILPEN_A>;
impl DSILPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSILPEN_A {
        match self.bits {
            false => DSILPEN_A::B_0X0,
            true => DSILPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DSILPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DSILPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DSILPEN`"]
pub struct DSILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSILPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DSILPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DSILPEN_A::B_0X1)
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
#[doc = "DDRPERFMLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRPERFMLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the APB clock is disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the APB clock in\r\n                  CSLEEP, reading means that the APB clock is\r\n                  enabled in CSLEEP"]
    B_0X1 = 1,
}
impl From<DDRPERFMLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRPERFMLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRPERFMLPEN`"]
pub type DDRPERFMLPEN_R = crate::R<bool, DDRPERFMLPEN_A>;
impl DDRPERFMLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRPERFMLPEN_A {
        match self.bits {
            false => DDRPERFMLPEN_A::B_0X0,
            true => DDRPERFMLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRPERFMLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRPERFMLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRPERFMLPEN`"]
pub struct DDRPERFMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPERFMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRPERFMLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the APB clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRPERFMLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the APB clock in CSLEEP, reading means that the APB clock is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRPERFMLPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "USBPHYLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPHYLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<USBPHYLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBPHYLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBPHYLPEN`"]
pub type USBPHYLPEN_R = crate::R<bool, USBPHYLPEN_A>;
impl USBPHYLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPHYLPEN_A {
        match self.bits {
            false => USBPHYLPEN_A::B_0X0,
            true => USBPHYLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBPHYLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBPHYLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `USBPHYLPEN`"]
pub struct USBPHYLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPHYLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBPHYLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBPHYLPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "STGENROLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STGENROLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<STGENROLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: STGENROLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STGENROLPEN`"]
pub type STGENROLPEN_R = crate::R<bool, STGENROLPEN_A>;
impl STGENROLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STGENROLPEN_A {
        match self.bits {
            false => STGENROLPEN_A::B_0X0,
            true => STGENROLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STGENROLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STGENROLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `STGENROLPEN`"]
pub struct STGENROLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENROLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STGENROLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(STGENROLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(STGENROLPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "STGENROSTPEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STGENROSTPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSTOP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the clock in CSTOP,\r\n                  reading means that the clock are enabled in\r\n                  CSTOP"]
    B_0X1 = 1,
}
impl From<STGENROSTPEN_A> for bool {
    #[inline(always)]
    fn from(variant: STGENROSTPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STGENROSTPEN`"]
pub type STGENROSTPEN_R = crate::R<bool, STGENROSTPEN_A>;
impl STGENROSTPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STGENROSTPEN_A {
        match self.bits {
            false => STGENROSTPEN_A::B_0X0,
            true => STGENROSTPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STGENROSTPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STGENROSTPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `STGENROSTPEN`"]
pub struct STGENROSTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENROSTPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STGENROSTPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSTOP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(STGENROSTPEN_A::B_0X0)
    }
    #[doc = "Writing disables the clock in CSTOP, reading means that the clock are enabled in CSTOP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(STGENROSTPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LTDCLPEN"]
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DSILPEN"]
    #[inline(always)]
    pub fn dsilpen(&self) -> DSILPEN_R {
        DSILPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMLPEN"]
    #[inline(always)]
    pub fn ddrperfmlpen(&self) -> DDRPERFMLPEN_R {
        DDRPERFMLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USBPHYLPEN"]
    #[inline(always)]
    pub fn usbphylpen(&self) -> USBPHYLPEN_R {
        USBPHYLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENROLPEN"]
    #[inline(always)]
    pub fn stgenrolpen(&self) -> STGENROLPEN_R {
        STGENROLPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - STGENROSTPEN"]
    #[inline(always)]
    pub fn stgenrostpen(&self) -> STGENROSTPEN_R {
        STGENROSTPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCLPEN"]
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W {
        LTDCLPEN_W { w: self }
    }
    #[doc = "Bit 4 - DSILPEN"]
    #[inline(always)]
    pub fn dsilpen(&mut self) -> DSILPEN_W {
        DSILPEN_W { w: self }
    }
    #[doc = "Bit 8 - DDRPERFMLPEN"]
    #[inline(always)]
    pub fn ddrperfmlpen(&mut self) -> DDRPERFMLPEN_W {
        DDRPERFMLPEN_W { w: self }
    }
    #[doc = "Bit 16 - USBPHYLPEN"]
    #[inline(always)]
    pub fn usbphylpen(&mut self) -> USBPHYLPEN_W {
        USBPHYLPEN_W { w: self }
    }
    #[doc = "Bit 20 - STGENROLPEN"]
    #[inline(always)]
    pub fn stgenrolpen(&mut self) -> STGENROLPEN_W {
        STGENROLPEN_W { w: self }
    }
    #[doc = "Bit 21 - STGENROSTPEN"]
    #[inline(always)]
    pub fn stgenrostpen(&mut self) -> STGENROSTPEN_W {
        STGENROSTPEN_W { w: self }
    }
}
