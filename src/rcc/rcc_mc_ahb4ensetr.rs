#[doc = "Reader of register RCC_MC_AHB4ENSETR"]
pub type R = crate::R<u32, super::RCC_MC_AHB4ENSETR>;
#[doc = "Writer for register RCC_MC_AHB4ENSETR"]
pub type W = crate::W<u32, super::RCC_MC_AHB4ENSETR>;
#[doc = "Register RCC_MC_AHB4ENSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MC_AHB4ENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPIOAEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOAEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOAEN`"]
pub type GPIOAEN_R = crate::R<bool, GPIOAEN_A>;
impl GPIOAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOAEN_A {
        match self.bits {
            false => GPIOAEN_A::B_0X0,
            true => GPIOAEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOAEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOAEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOAEN`"]
pub struct GPIOAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOAEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOAEN_A::B_0X1)
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
#[doc = "GPIOBEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOBEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIOBEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOBEN`"]
pub type GPIOBEN_R = crate::R<bool, GPIOBEN_A>;
impl GPIOBEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOBEN_A {
        match self.bits {
            false => GPIOBEN_A::B_0X0,
            true => GPIOBEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOBEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOBEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOBEN`"]
pub struct GPIOBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOBEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOBEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOBEN_A::B_0X1)
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
#[doc = "GPIOCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOCEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIOCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOCEN`"]
pub type GPIOCEN_R = crate::R<bool, GPIOCEN_A>;
impl GPIOCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOCEN_A {
        match self.bits {
            false => GPIOCEN_A::B_0X0,
            true => GPIOCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOCEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOCEN`"]
pub struct GPIOCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOCEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOCEN_A::B_0X1)
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
#[doc = "GPIODEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIODEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIODEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIODEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIODEN`"]
pub type GPIODEN_R = crate::R<bool, GPIODEN_A>;
impl GPIODEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIODEN_A {
        match self.bits {
            false => GPIODEN_A::B_0X0,
            true => GPIODEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIODEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIODEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIODEN`"]
pub struct GPIODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIODEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIODEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIODEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIODEN_A::B_0X1)
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
#[doc = "GPIOEEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOEEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIOEEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOEEN`"]
pub type GPIOEEN_R = crate::R<bool, GPIOEEN_A>;
impl GPIOEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOEEN_A {
        match self.bits {
            false => GPIOEEN_A::B_0X0,
            true => GPIOEEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOEEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOEEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOEEN`"]
pub struct GPIOEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOEEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOEEN_A::B_0X1)
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
#[doc = "GPIOFEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOFEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIOFEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOFEN`"]
pub type GPIOFEN_R = crate::R<bool, GPIOFEN_A>;
impl GPIOFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOFEN_A {
        match self.bits {
            false => GPIOFEN_A::B_0X0,
            true => GPIOFEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOFEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOFEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOFEN`"]
pub struct GPIOFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOFEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOFEN_A::B_0X1)
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
#[doc = "GPIOGEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOGEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIOGEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOGEN`"]
pub type GPIOGEN_R = crate::R<bool, GPIOGEN_A>;
impl GPIOGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOGEN_A {
        match self.bits {
            false => GPIOGEN_A::B_0X0,
            true => GPIOGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOGEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOGEN`"]
pub struct GPIOGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOGEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOGEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "GPIOHEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOHEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIOHEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOHEN`"]
pub type GPIOHEN_R = crate::R<bool, GPIOHEN_A>;
impl GPIOHEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOHEN_A {
        match self.bits {
            false => GPIOHEN_A::B_0X0,
            true => GPIOHEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOHEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOHEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOHEN`"]
pub struct GPIOHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOHEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOHEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOHEN_A::B_0X1)
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
#[doc = "GPIOIEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOIEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIOIEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOIEN`"]
pub type GPIOIEN_R = crate::R<bool, GPIOIEN_A>;
impl GPIOIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOIEN_A {
        match self.bits {
            false => GPIOIEN_A::B_0X0,
            true => GPIOIEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOIEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOIEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOIEN`"]
pub struct GPIOIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOIEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOIEN_A::B_0X1)
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
#[doc = "GPIOJEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOJEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIOJEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOJEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOJEN`"]
pub type GPIOJEN_R = crate::R<bool, GPIOJEN_A>;
impl GPIOJEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOJEN_A {
        match self.bits {
            false => GPIOJEN_A::B_0X0,
            true => GPIOJEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOJEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOJEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOJEN`"]
pub struct GPIOJEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOJEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOJEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOJEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOJEN_A::B_0X1)
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
#[doc = "GPIOKEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOKEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIOKEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOKEN`"]
pub type GPIOKEN_R = crate::R<bool, GPIOKEN_A>;
impl GPIOKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOKEN_A {
        match self.bits {
            false => GPIOKEN_A::B_0X0,
            true => GPIOKEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOKEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOKEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOKEN`"]
pub struct GPIOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOKEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOKEN_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - GPIOAEN"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIOBEN"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIOCEN"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIODEN"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIOEEN"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIOFEN"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIOGEN"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIOHEN"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIOIEN"]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPIOJEN"]
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPIOKEN"]
    #[inline(always)]
    pub fn gpioken(&self) -> GPIOKEN_R {
        GPIOKEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOAEN"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W {
        GPIOAEN_W { w: self }
    }
    #[doc = "Bit 1 - GPIOBEN"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W {
        GPIOBEN_W { w: self }
    }
    #[doc = "Bit 2 - GPIOCEN"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W {
        GPIOCEN_W { w: self }
    }
    #[doc = "Bit 3 - GPIODEN"]
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W {
        GPIODEN_W { w: self }
    }
    #[doc = "Bit 4 - GPIOEEN"]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W {
        GPIOEEN_W { w: self }
    }
    #[doc = "Bit 5 - GPIOFEN"]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W {
        GPIOFEN_W { w: self }
    }
    #[doc = "Bit 6 - GPIOGEN"]
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W {
        GPIOGEN_W { w: self }
    }
    #[doc = "Bit 7 - GPIOHEN"]
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W {
        GPIOHEN_W { w: self }
    }
    #[doc = "Bit 8 - GPIOIEN"]
    #[inline(always)]
    pub fn gpioien(&mut self) -> GPIOIEN_W {
        GPIOIEN_W { w: self }
    }
    #[doc = "Bit 9 - GPIOJEN"]
    #[inline(always)]
    pub fn gpiojen(&mut self) -> GPIOJEN_W {
        GPIOJEN_W { w: self }
    }
    #[doc = "Bit 10 - GPIOKEN"]
    #[inline(always)]
    pub fn gpioken(&mut self) -> GPIOKEN_W {
        GPIOKEN_W { w: self }
    }
}
