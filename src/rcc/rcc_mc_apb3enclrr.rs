#[doc = "Reader of register RCC_MC_APB3ENCLRR"]
pub type R = crate::R<u32, super::RCC_MC_APB3ENCLRR>;
#[doc = "Writer for register RCC_MC_APB3ENCLRR"]
pub type W = crate::W<u32, super::RCC_MC_APB3ENCLRR>;
#[doc = "Register RCC_MC_APB3ENCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MC_APB3ENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPTIM2EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM2EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<LPTIM2EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM2EN`"]
pub type LPTIM2EN_R = crate::R<bool, LPTIM2EN_A>;
impl LPTIM2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM2EN_A {
        match self.bits {
            false => LPTIM2EN_A::B_0X0,
            true => LPTIM2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM2EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM2EN`"]
pub struct LPTIM2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM2EN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM2EN_A::B_0X1)
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
#[doc = "LPTIM3EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM3EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<LPTIM3EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM3EN`"]
pub type LPTIM3EN_R = crate::R<bool, LPTIM3EN_A>;
impl LPTIM3EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM3EN_A {
        match self.bits {
            false => LPTIM3EN_A::B_0X0,
            true => LPTIM3EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM3EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM3EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM3EN`"]
pub struct LPTIM3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM3EN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM3EN_A::B_0X1)
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
#[doc = "LPTIM4EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM4EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<LPTIM4EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM4EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM4EN`"]
pub type LPTIM4EN_R = crate::R<bool, LPTIM4EN_A>;
impl LPTIM4EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM4EN_A {
        match self.bits {
            false => LPTIM4EN_A::B_0X0,
            true => LPTIM4EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM4EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM4EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM4EN`"]
pub struct LPTIM4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM4EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM4EN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM4EN_A::B_0X1)
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
#[doc = "LPTIM5EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM5EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<LPTIM5EN_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM5EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM5EN`"]
pub type LPTIM5EN_R = crate::R<bool, LPTIM5EN_A>;
impl LPTIM5EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM5EN_A {
        match self.bits {
            false => LPTIM5EN_A::B_0X0,
            true => LPTIM5EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM5EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM5EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM5EN`"]
pub struct LPTIM5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM5EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM5EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM5EN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM5EN_A::B_0X1)
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
#[doc = "SAI4EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI4EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<SAI4EN_A> for bool {
    #[inline(always)]
    fn from(variant: SAI4EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI4EN`"]
pub type SAI4EN_R = crate::R<bool, SAI4EN_A>;
impl SAI4EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI4EN_A {
        match self.bits {
            false => SAI4EN_A::B_0X0,
            true => SAI4EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI4EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI4EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI4EN`"]
pub struct SAI4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI4EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI4EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI4EN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI4EN_A::B_0X1)
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
#[doc = "SYSCFGEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<SYSCFGEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSCFGEN`"]
pub type SYSCFGEN_R = crate::R<bool, SYSCFGEN_A>;
impl SYSCFGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGEN_A {
        match self.bits {
            false => SYSCFGEN_A::B_0X0,
            true => SYSCFGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSCFGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSCFGEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SYSCFGEN`"]
pub struct SYSCFGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::B_0X1)
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
#[doc = "VREFEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFEN`"]
pub type VREFEN_R = crate::R<bool, VREFEN_A>;
impl VREFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::B_0X0,
            true => VREFEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VREFEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VREFEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `VREFEN`"]
pub struct VREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VREFEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VREFEN_A::B_0X1)
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
#[doc = "TMPSENSEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMPSENSEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<TMPSENSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TMPSENSEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMPSENSEN`"]
pub type TMPSENSEN_R = crate::R<bool, TMPSENSEN_A>;
impl TMPSENSEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMPSENSEN_A {
        match self.bits {
            false => TMPSENSEN_A::B_0X0,
            true => TMPSENSEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TMPSENSEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TMPSENSEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TMPSENSEN`"]
pub struct TMPSENSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMPSENSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMPSENSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TMPSENSEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TMPSENSEN_A::B_0X1)
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
#[doc = "PMBCTRLEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMBCTRLEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<PMBCTRLEN_A> for bool {
    #[inline(always)]
    fn from(variant: PMBCTRLEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PMBCTRLEN`"]
pub type PMBCTRLEN_R = crate::R<bool, PMBCTRLEN_A>;
impl PMBCTRLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMBCTRLEN_A {
        match self.bits {
            false => PMBCTRLEN_A::B_0X0,
            true => PMBCTRLEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PMBCTRLEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PMBCTRLEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `PMBCTRLEN`"]
pub struct PMBCTRLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMBCTRLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMBCTRLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PMBCTRLEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PMBCTRLEN_A::B_0X1)
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
#[doc = "HDPEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<HDPEN_A> for bool {
    #[inline(always)]
    fn from(variant: HDPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HDPEN`"]
pub type HDPEN_R = crate::R<bool, HDPEN_A>;
impl HDPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDPEN_A {
        match self.bits {
            false => HDPEN_A::B_0X0,
            true => HDPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HDPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HDPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `HDPEN`"]
pub struct HDPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HDPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HDPEN_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPTIM3EN"]
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPTIM4EN"]
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPTIM5EN"]
    #[inline(always)]
    pub fn lptim5en(&self) -> LPTIM5EN_R {
        LPTIM5EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SAI4EN"]
    #[inline(always)]
    pub fn sai4en(&self) -> SAI4EN_R {
        SAI4EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SYSCFGEN"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TMPSENSEN"]
    #[inline(always)]
    pub fn tmpsensen(&self) -> TMPSENSEN_R {
        TMPSENSEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PMBCTRLEN"]
    #[inline(always)]
    pub fn pmbctrlen(&self) -> PMBCTRLEN_R {
        PMBCTRLEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&self) -> HDPEN_R {
        HDPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2EN"]
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W {
        LPTIM2EN_W { w: self }
    }
    #[doc = "Bit 1 - LPTIM3EN"]
    #[inline(always)]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W {
        LPTIM3EN_W { w: self }
    }
    #[doc = "Bit 2 - LPTIM4EN"]
    #[inline(always)]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W {
        LPTIM4EN_W { w: self }
    }
    #[doc = "Bit 3 - LPTIM5EN"]
    #[inline(always)]
    pub fn lptim5en(&mut self) -> LPTIM5EN_W {
        LPTIM5EN_W { w: self }
    }
    #[doc = "Bit 8 - SAI4EN"]
    #[inline(always)]
    pub fn sai4en(&mut self) -> SAI4EN_W {
        SAI4EN_W { w: self }
    }
    #[doc = "Bit 11 - SYSCFGEN"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W {
        SYSCFGEN_W { w: self }
    }
    #[doc = "Bit 13 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VREFEN_W {
        VREFEN_W { w: self }
    }
    #[doc = "Bit 16 - TMPSENSEN"]
    #[inline(always)]
    pub fn tmpsensen(&mut self) -> TMPSENSEN_W {
        TMPSENSEN_W { w: self }
    }
    #[doc = "Bit 17 - PMBCTRLEN"]
    #[inline(always)]
    pub fn pmbctrlen(&mut self) -> PMBCTRLEN_W {
        PMBCTRLEN_W { w: self }
    }
    #[doc = "Bit 20 - HDPEN"]
    #[inline(always)]
    pub fn hdpen(&mut self) -> HDPEN_W {
        HDPEN_W { w: self }
    }
}
