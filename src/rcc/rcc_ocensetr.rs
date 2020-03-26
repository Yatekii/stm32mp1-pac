#[doc = "Reader of register RCC_OCENSETR"]
pub type R = crate::R<u32, super::RCC_OCENSETR>;
#[doc = "Writer for register RCC_OCENSETR"]
pub type W = crate::W<u32, super::RCC_OCENSETR>;
#[doc = "Register RCC_OCENSETR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RCC_OCENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "HSION\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSION_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Set the HSION bit"]
    B_0X1 = 1,
}
impl From<HSION_A> for bool {
    #[inline(always)]
    fn from(variant: HSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSION`"]
pub type HSION_R = crate::R<bool, HSION_A>;
impl HSION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSION_A {
        match self.bits {
            false => HSION_A::B_0X0,
            true => HSION_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSION_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSION_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSION`"]
pub struct HSION_W<'a> {
    w: &'a mut W,
}
impl<'a> HSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSION_A::B_0X0)
    }
    #[doc = "Set the HSION bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSION_A::B_0X1)
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
#[doc = "HSIKERON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIKERON_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Set the HSIKERON bit"]
    B_0X1 = 1,
}
impl From<HSIKERON_A> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSIKERON`"]
pub type HSIKERON_R = crate::R<bool, HSIKERON_A>;
impl HSIKERON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIKERON_A {
        match self.bits {
            false => HSIKERON_A::B_0X0,
            true => HSIKERON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIKERON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIKERON_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSIKERON`"]
pub struct HSIKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIKERON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIKERON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSIKERON_A::B_0X0)
    }
    #[doc = "Set the HSIKERON bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSIKERON_A::B_0X1)
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
#[doc = "CSION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSION_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Set the CSION bit"]
    B_0X1 = 1,
}
impl From<CSION_A> for bool {
    #[inline(always)]
    fn from(variant: CSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSION`"]
pub type CSION_R = crate::R<bool, CSION_A>;
impl CSION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSION_A {
        match self.bits {
            false => CSION_A::B_0X0,
            true => CSION_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSION_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSION_A::B_0X1
    }
}
#[doc = "Write proxy for field `CSION`"]
pub struct CSION_W<'a> {
    w: &'a mut W,
}
impl<'a> CSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CSION_A::B_0X0)
    }
    #[doc = "Set the CSION bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CSION_A::B_0X1)
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
#[doc = "CSIKERON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIKERON_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Set the CSIKERON bit"]
    B_0X1 = 1,
}
impl From<CSIKERON_A> for bool {
    #[inline(always)]
    fn from(variant: CSIKERON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSIKERON`"]
pub type CSIKERON_R = crate::R<bool, CSIKERON_A>;
impl CSIKERON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSIKERON_A {
        match self.bits {
            false => CSIKERON_A::B_0X0,
            true => CSIKERON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSIKERON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSIKERON_A::B_0X1
    }
}
#[doc = "Write proxy for field `CSIKERON`"]
pub struct CSIKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIKERON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSIKERON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CSIKERON_A::B_0X0)
    }
    #[doc = "Set the CSIKERON bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CSIKERON_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "DIGBYP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGBYP_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Set DIGBYP bit (digital\r\n                  bypass)"]
    B_0X1 = 1,
}
impl From<DIGBYP_A> for bool {
    #[inline(always)]
    fn from(variant: DIGBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIGBYP`"]
pub type DIGBYP_R = crate::R<bool, DIGBYP_A>;
impl DIGBYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIGBYP_A {
        match self.bits {
            false => DIGBYP_A::B_0X0,
            true => DIGBYP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DIGBYP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DIGBYP_A::B_0X1
    }
}
#[doc = "Write proxy for field `DIGBYP`"]
pub struct DIGBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIGBYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DIGBYP_A::B_0X0)
    }
    #[doc = "Set DIGBYP bit (digital bypass)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DIGBYP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "HSEON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEON_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Set HSEON bit"]
    B_0X1 = 1,
}
impl From<HSEON_A> for bool {
    #[inline(always)]
    fn from(variant: HSEON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSEON`"]
pub type HSEON_R = crate::R<bool, HSEON_A>;
impl HSEON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEON_A {
        match self.bits {
            false => HSEON_A::B_0X0,
            true => HSEON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSEON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSEON_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSEON`"]
pub struct HSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSEON_A::B_0X0)
    }
    #[doc = "Set HSEON bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSEON_A::B_0X1)
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
#[doc = "HSEKERON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEKERON_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Set the HSEKERON bit"]
    B_0X1 = 1,
}
impl From<HSEKERON_A> for bool {
    #[inline(always)]
    fn from(variant: HSEKERON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSEKERON`"]
pub type HSEKERON_R = crate::R<bool, HSEKERON_A>;
impl HSEKERON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEKERON_A {
        match self.bits {
            false => HSEKERON_A::B_0X0,
            true => HSEKERON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSEKERON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSEKERON_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSEKERON`"]
pub struct HSEKERON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEKERON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEKERON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSEKERON_A::B_0X0)
    }
    #[doc = "Set the HSEKERON bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSEKERON_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "HSEBYP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEBYP_A {
    #[doc = "0: No effect"]
    B_0X0 = 0,
    #[doc = "1: Set the HSEBYP bit"]
    B_0X1 = 1,
}
impl From<HSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSEBYP`"]
pub type HSEBYP_R = crate::R<bool, HSEBYP_A>;
impl HSEBYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEBYP_A {
        match self.bits {
            false => HSEBYP_A::B_0X0,
            true => HSEBYP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSEBYP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSEBYP_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSEBYP`"]
pub struct HSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEBYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSEBYP_A::B_0X0)
    }
    #[doc = "Set the HSEBYP bit"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSEBYP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "HSECSSON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSECSSON_A {
    #[doc = "0: Reading means that the Clock\r\n                  Security System on HSE is OFF (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the Clock Security\r\n                  System on HSE, reading means that the Clock\r\n                  Security System on HSE is ON"]
    B_0X1 = 1,
}
impl From<HSECSSON_A> for bool {
    #[inline(always)]
    fn from(variant: HSECSSON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSECSSON`"]
pub type HSECSSON_R = crate::R<bool, HSECSSON_A>;
impl HSECSSON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSECSSON_A {
        match self.bits {
            false => HSECSSON_A::B_0X0,
            true => HSECSSON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSECSSON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSECSSON_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSECSSON`"]
pub struct HSECSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSECSSON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSECSSON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reading means that the Clock Security System on HSE is OFF (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSECSSON_A::B_0X0)
    }
    #[doc = "Writing enables the Clock Security System on HSE, reading means that the Clock Security System on HSE is ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSECSSON_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - HSION"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HSIKERON"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSION"]
    #[inline(always)]
    pub fn csion(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CSIKERON"]
    #[inline(always)]
    pub fn csikeron(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DIGBYP"]
    #[inline(always)]
    pub fn digbyp(&self) -> DIGBYP_R {
        DIGBYP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HSEON"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HSEKERON"]
    #[inline(always)]
    pub fn hsekeron(&self) -> HSEKERON_R {
        HSEKERON_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSEBYP"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSECSSON"]
    #[inline(always)]
    pub fn hsecsson(&self) -> HSECSSON_R {
        HSECSSON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSION"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HSION_W {
        HSION_W { w: self }
    }
    #[doc = "Bit 1 - HSIKERON"]
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HSIKERON_W {
        HSIKERON_W { w: self }
    }
    #[doc = "Bit 4 - CSION"]
    #[inline(always)]
    pub fn csion(&mut self) -> CSION_W {
        CSION_W { w: self }
    }
    #[doc = "Bit 5 - CSIKERON"]
    #[inline(always)]
    pub fn csikeron(&mut self) -> CSIKERON_W {
        CSIKERON_W { w: self }
    }
    #[doc = "Bit 7 - DIGBYP"]
    #[inline(always)]
    pub fn digbyp(&mut self) -> DIGBYP_W {
        DIGBYP_W { w: self }
    }
    #[doc = "Bit 8 - HSEON"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HSEON_W {
        HSEON_W { w: self }
    }
    #[doc = "Bit 9 - HSEKERON"]
    #[inline(always)]
    pub fn hsekeron(&mut self) -> HSEKERON_W {
        HSEKERON_W { w: self }
    }
    #[doc = "Bit 10 - HSEBYP"]
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HSEBYP_W {
        HSEBYP_W { w: self }
    }
    #[doc = "Bit 11 - HSECSSON"]
    #[inline(always)]
    pub fn hsecsson(&mut self) -> HSECSSON_W {
        HSECSSON_W { w: self }
    }
}
