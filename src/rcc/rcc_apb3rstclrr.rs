#[doc = "Reader of register RCC_APB3RSTCLRR"]
pub type R = crate::R<u32, super::RCC_APB3RSTCLRR>;
#[doc = "Writer for register RCC_APB3RSTCLRR"]
pub type W = crate::W<u32, super::RCC_APB3RSTCLRR>;
#[doc = "Register RCC_APB3RSTCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_APB3RSTCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPTIM2RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM2RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<LPTIM2RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM2RST`"]
pub type LPTIM2RST_R = crate::R<bool, LPTIM2RST_A>;
impl LPTIM2RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM2RST_A {
        match self.bits {
            false => LPTIM2RST_A::B_0X0,
            true => LPTIM2RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM2RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM2RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM2RST`"]
pub struct LPTIM2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM2RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM2RST_A::B_0X1)
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
#[doc = "LPTIM3RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM3RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<LPTIM3RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM3RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM3RST`"]
pub type LPTIM3RST_R = crate::R<bool, LPTIM3RST_A>;
impl LPTIM3RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM3RST_A {
        match self.bits {
            false => LPTIM3RST_A::B_0X0,
            true => LPTIM3RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM3RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM3RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM3RST`"]
pub struct LPTIM3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM3RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM3RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM3RST_A::B_0X1)
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
#[doc = "LPTIM4RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM4RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<LPTIM4RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM4RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM4RST`"]
pub type LPTIM4RST_R = crate::R<bool, LPTIM4RST_A>;
impl LPTIM4RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM4RST_A {
        match self.bits {
            false => LPTIM4RST_A::B_0X0,
            true => LPTIM4RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM4RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM4RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM4RST`"]
pub struct LPTIM4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM4RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM4RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM4RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM4RST_A::B_0X1)
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
#[doc = "LPTIM5RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPTIM5RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<LPTIM5RST_A> for bool {
    #[inline(always)]
    fn from(variant: LPTIM5RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPTIM5RST`"]
pub type LPTIM5RST_R = crate::R<bool, LPTIM5RST_A>;
impl LPTIM5RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM5RST_A {
        match self.bits {
            false => LPTIM5RST_A::B_0X0,
            true => LPTIM5RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPTIM5RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPTIM5RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `LPTIM5RST`"]
pub struct LPTIM5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM5RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM5RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LPTIM5RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LPTIM5RST_A::B_0X1)
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
#[doc = "SAI4RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAI4RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SAI4RST_A> for bool {
    #[inline(always)]
    fn from(variant: SAI4RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAI4RST`"]
pub type SAI4RST_R = crate::R<bool, SAI4RST_A>;
impl SAI4RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI4RST_A {
        match self.bits {
            false => SAI4RST_A::B_0X0,
            true => SAI4RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SAI4RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SAI4RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SAI4RST`"]
pub struct SAI4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI4RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI4RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SAI4RST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SAI4RST_A::B_0X1)
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
#[doc = "SYSCFGRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SYSCFGRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSCFGRST`"]
pub type SYSCFGRST_R = crate::R<bool, SYSCFGRST_A>;
impl SYSCFGRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGRST_A {
        match self.bits {
            false => SYSCFGRST_A::B_0X0,
            true => SYSCFGRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SYSCFGRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SYSCFGRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SYSCFGRST`"]
pub struct SYSCFGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SYSCFGRST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SYSCFGRST_A::B_0X1)
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
#[doc = "VREFRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREFRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<VREFRST_A> for bool {
    #[inline(always)]
    fn from(variant: VREFRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VREFRST`"]
pub type VREFRST_R = crate::R<bool, VREFRST_A>;
impl VREFRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFRST_A {
        match self.bits {
            false => VREFRST_A::B_0X0,
            true => VREFRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VREFRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VREFRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `VREFRST`"]
pub struct VREFRST_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREFRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VREFRST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VREFRST_A::B_0X1)
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
#[doc = "TMPSENSRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMPSENSRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<TMPSENSRST_A> for bool {
    #[inline(always)]
    fn from(variant: TMPSENSRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMPSENSRST`"]
pub type TMPSENSRST_R = crate::R<bool, TMPSENSRST_A>;
impl TMPSENSRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMPSENSRST_A {
        match self.bits {
            false => TMPSENSRST_A::B_0X0,
            true => TMPSENSRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TMPSENSRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TMPSENSRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `TMPSENSRST`"]
pub struct TMPSENSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMPSENSRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMPSENSRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TMPSENSRST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TMPSENSRST_A::B_0X1)
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
#[doc = "PMBCTRLRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMBCTRLRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<PMBCTRLRST_A> for bool {
    #[inline(always)]
    fn from(variant: PMBCTRLRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PMBCTRLRST`"]
pub type PMBCTRLRST_R = crate::R<bool, PMBCTRLRST_A>;
impl PMBCTRLRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMBCTRLRST_A {
        match self.bits {
            false => PMBCTRLRST_A::B_0X0,
            true => PMBCTRLRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PMBCTRLRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PMBCTRLRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `PMBCTRLRST`"]
pub struct PMBCTRLRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PMBCTRLRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMBCTRLRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PMBCTRLRST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PMBCTRLRST_A::B_0X1)
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
    #[doc = "Bit 0 - LPTIM2RST"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPTIM3RST"]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPTIM4RST"]
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPTIM5RST"]
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SAI4RST"]
    #[inline(always)]
    pub fn sai4rst(&self) -> SAI4RST_R {
        SAI4RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SYSCFGRST"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VREFRST"]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TMPSENSRST"]
    #[inline(always)]
    pub fn tmpsensrst(&self) -> TMPSENSRST_R {
        TMPSENSRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PMBCTRLRST"]
    #[inline(always)]
    pub fn pmbctrlrst(&self) -> PMBCTRLRST_R {
        PMBCTRLRST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPTIM2RST"]
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W {
        LPTIM2RST_W { w: self }
    }
    #[doc = "Bit 1 - LPTIM3RST"]
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W {
        LPTIM3RST_W { w: self }
    }
    #[doc = "Bit 2 - LPTIM4RST"]
    #[inline(always)]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W {
        LPTIM4RST_W { w: self }
    }
    #[doc = "Bit 3 - LPTIM5RST"]
    #[inline(always)]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W {
        LPTIM5RST_W { w: self }
    }
    #[doc = "Bit 8 - SAI4RST"]
    #[inline(always)]
    pub fn sai4rst(&mut self) -> SAI4RST_W {
        SAI4RST_W { w: self }
    }
    #[doc = "Bit 11 - SYSCFGRST"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W {
        SYSCFGRST_W { w: self }
    }
    #[doc = "Bit 13 - VREFRST"]
    #[inline(always)]
    pub fn vrefrst(&mut self) -> VREFRST_W {
        VREFRST_W { w: self }
    }
    #[doc = "Bit 16 - TMPSENSRST"]
    #[inline(always)]
    pub fn tmpsensrst(&mut self) -> TMPSENSRST_W {
        TMPSENSRST_W { w: self }
    }
    #[doc = "Bit 17 - PMBCTRLRST"]
    #[inline(always)]
    pub fn pmbctrlrst(&mut self) -> PMBCTRLRST_W {
        PMBCTRLRST_W { w: self }
    }
}
