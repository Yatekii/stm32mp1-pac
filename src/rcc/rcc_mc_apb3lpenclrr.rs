#[doc = "Reader of register RCC_MC_APB3LPENCLRR"]
pub type R = crate::R<u32, super::RCC_MC_APB3LPENCLRR>;
#[doc = "Writer for register RCC_MC_APB3LPENCLRR"]
pub type W = crate::W<u32, super::RCC_MC_APB3LPENCLRR>;
#[doc = "Register RCC_MC_APB3LPENCLRR `reset()`'s with value 0x0003_290f"]
impl crate::ResetValue for super::RCC_MC_APB3LPENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_290f
    }
}
#[doc = "LPTIM2LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM2LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<LPTIM2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM2LPEN`"]
pub type LPTIM2LPEN_R = crate::R<bool, LPTIM2LPEN_A>;
impl LPTIM2LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM2LPEN_A {
        match self.bits {
            false => LPTIM2LPEN_A::B_0X0,
            true => LPTIM2LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM2LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM2LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM2LPEN`"]
pub struct LPTIM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM2LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM2LPEN_A::B_0X1)
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
#[doc = "LPTIM3LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM3LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<LPTIM3LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM3LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM3LPEN`"]
pub type LPTIM3LPEN_R = crate::R<bool, LPTIM3LPEN_A>;
impl LPTIM3LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM3LPEN_A {
        match self.bits {
            false => LPTIM3LPEN_A::B_0X0,
            true => LPTIM3LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM3LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM3LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM3LPEN`"]
pub struct LPTIM3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM3LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM3LPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "LPTIM4LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM4LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<LPTIM4LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM4LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM4LPEN`"]
pub type LPTIM4LPEN_R = crate::R<bool, LPTIM4LPEN_A>;
impl LPTIM4LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM4LPEN_A {
        match self.bits {
            false => LPTIM4LPEN_A::B_0X0,
            true => LPTIM4LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM4LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM4LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM4LPEN`"]
pub struct LPTIM4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM4LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM4LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM4LPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "LPTIM5LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM5LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<LPTIM5LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM5LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM5LPEN`"]
pub type LPTIM5LPEN_R = crate::R<bool, LPTIM5LPEN_A>;
impl LPTIM5LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM5LPEN_A {
        match self.bits {
            false => LPTIM5LPEN_A::B_0X0,
            true => LPTIM5LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM5LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM5LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM5LPEN`"]
pub struct LPTIM5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM5LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM5LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM5LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM5LPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "SAI4LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI4LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SAI4LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI4LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI4LPEN`"]
pub type SAI4LPEN_R = crate::R<bool, SAI4LPEN_A>;
impl SAI4LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI4LPEN_A {
        match self.bits {
            false => SAI4LPEN_A::B_0X0,
            true => SAI4LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI4LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI4LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI4LPEN`"]
pub struct SAI4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI4LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI4LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI4LPEN_A::B_0X1)
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
#[doc = "SYSCFGLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SYSCFGLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSCFGLPEN`"]
pub type SYSCFGLPEN_R = crate::R<bool, SYSCFGLPEN_A>;
impl SYSCFGLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGLPEN_A {
        match self.bits {
            false => SYSCFGLPEN_A::B_0X0,
            true => SYSCFGLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSCFGLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSCFGLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SYSCFGLPEN`"]
pub struct SYSCFGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SYSCFGLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SYSCFGLPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "VREFLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<VREFLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFLPEN`"]
pub type VREFLPEN_R = crate::R<bool, VREFLPEN_A>;
impl VREFLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFLPEN_A {
        match self.bits {
            false => VREFLPEN_A::B_0X0,
            true => VREFLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VREFLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VREFLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `VREFLPEN`"]
pub struct VREFLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VREFLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VREFLPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "TMPSENSLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMPSENSLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<TMPSENSLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMPSENSLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMPSENSLPEN`"]
pub type TMPSENSLPEN_R = crate::R<bool, TMPSENSLPEN_A>;
impl TMPSENSLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMPSENSLPEN_A {
        match self.bits {
            false => TMPSENSLPEN_A::B_0X0,
            true => TMPSENSLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TMPSENSLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TMPSENSLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TMPSENSLPEN`"]
pub struct TMPSENSLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMPSENSLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMPSENSLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TMPSENSLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TMPSENSLPEN_A::B_0X1)
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
#[doc = "PMBCTRLLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMBCTRLLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<PMBCTRLLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: PMBCTRLLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PMBCTRLLPEN`"]
pub type PMBCTRLLPEN_R = crate::R<bool, PMBCTRLLPEN_A>;
impl PMBCTRLLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMBCTRLLPEN_A {
        match self.bits {
            false => PMBCTRLLPEN_A::B_0X0,
            true => PMBCTRLLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PMBCTRLLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PMBCTRLLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `PMBCTRLLPEN`"]
pub struct PMBCTRLLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMBCTRLLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMBCTRLLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PMBCTRLLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PMBCTRLLPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LPTIM2LPEN"]
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPTIM3LPEN"]
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPTIM4LPEN"]
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPTIM5LPEN"]
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SAI4LPEN"]
    #[inline(always)]
    pub fn sai4lpen(&self) -> SAI4LPEN_R {
        SAI4LPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SYSCFGLPEN"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VREFLPEN"]
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TMPSENSLPEN"]
    #[inline(always)]
    pub fn tmpsenslpen(&self) -> TMPSENSLPEN_R {
        TMPSENSLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PMBCTRLLPEN"]
    #[inline(always)]
    pub fn pmbctrllpen(&self) -> PMBCTRLLPEN_R {
        PMBCTRLLPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2LPEN"]
    #[inline(always)]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W {
        LPTIM2LPEN_W { w: self }
    }
    #[doc = "Bit 1 - LPTIM3LPEN"]
    #[inline(always)]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W {
        LPTIM3LPEN_W { w: self }
    }
    #[doc = "Bit 2 - LPTIM4LPEN"]
    #[inline(always)]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W {
        LPTIM4LPEN_W { w: self }
    }
    #[doc = "Bit 3 - LPTIM5LPEN"]
    #[inline(always)]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W {
        LPTIM5LPEN_W { w: self }
    }
    #[doc = "Bit 8 - SAI4LPEN"]
    #[inline(always)]
    pub fn sai4lpen(&mut self) -> SAI4LPEN_W {
        SAI4LPEN_W { w: self }
    }
    #[doc = "Bit 11 - SYSCFGLPEN"]
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W {
        SYSCFGLPEN_W { w: self }
    }
    #[doc = "Bit 13 - VREFLPEN"]
    #[inline(always)]
    pub fn vreflpen(&mut self) -> VREFLPEN_W {
        VREFLPEN_W { w: self }
    }
    #[doc = "Bit 16 - TMPSENSLPEN"]
    #[inline(always)]
    pub fn tmpsenslpen(&mut self) -> TMPSENSLPEN_W {
        TMPSENSLPEN_W { w: self }
    }
    #[doc = "Bit 17 - PMBCTRLLPEN"]
    #[inline(always)]
    pub fn pmbctrllpen(&mut self) -> PMBCTRLLPEN_W {
        PMBCTRLLPEN_W { w: self }
    }
}
