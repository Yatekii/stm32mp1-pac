#[doc = "Reader of register RCC_MP_AHB5ENSETR"]
pub type R = crate::R<u32, super::RCC_MP_AHB5ENSETR>;
#[doc = "Writer for register RCC_MP_AHB5ENSETR"]
pub type W = crate::W<u32, super::RCC_MP_AHB5ENSETR>;
#[doc = "Register RCC_MP_AHB5ENSETR `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::RCC_MP_AHB5ENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "GPIOZEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOZEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPIOZEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOZEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOZEN`"]
pub type GPIOZEN_R = crate::R<bool, GPIOZEN_A>;
impl GPIOZEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOZEN_A {
        match self.bits {
            false => GPIOZEN_A::B_0X0,
            true => GPIOZEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOZEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOZEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOZEN`"]
pub struct GPIOZEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOZEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOZEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOZEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOZEN_A::B_0X1)
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
#[doc = "CRYP1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYP1EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<CRYP1EN_A> for bool {
    #[inline(always)]
    fn from(variant: CRYP1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYP1EN`"]
pub type CRYP1EN_R = crate::R<bool, CRYP1EN_A>;
impl CRYP1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYP1EN_A {
        match self.bits {
            false => CRYP1EN_A::B_0X0,
            true => CRYP1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRYP1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRYP1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `CRYP1EN`"]
pub struct CRYP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYP1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRYP1EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRYP1EN_A::B_0X1)
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
#[doc = "HASH1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH1EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<HASH1EN_A> for bool {
    #[inline(always)]
    fn from(variant: HASH1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HASH1EN`"]
pub type HASH1EN_R = crate::R<bool, HASH1EN_A>;
impl HASH1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH1EN_A {
        match self.bits {
            false => HASH1EN_A::B_0X0,
            true => HASH1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HASH1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HASH1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `HASH1EN`"]
pub struct HASH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HASH1EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HASH1EN_A::B_0X1)
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
#[doc = "RNG1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG1EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<RNG1EN_A> for bool {
    #[inline(always)]
    fn from(variant: RNG1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNG1EN`"]
pub type RNG1EN_R = crate::R<bool, RNG1EN_A>;
impl RNG1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG1EN_A {
        match self.bits {
            false => RNG1EN_A::B_0X0,
            true => RNG1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RNG1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RNG1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `RNG1EN`"]
pub struct RNG1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RNG1EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RNG1EN_A::B_0X1)
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
#[doc = "BKPSRAMEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKPSRAMEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<BKPSRAMEN_A> for bool {
    #[inline(always)]
    fn from(variant: BKPSRAMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKPSRAMEN`"]
pub type BKPSRAMEN_R = crate::R<bool, BKPSRAMEN_A>;
impl BKPSRAMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKPSRAMEN_A {
        match self.bits {
            false => BKPSRAMEN_A::B_0X0,
            true => BKPSRAMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKPSRAMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKPSRAMEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKPSRAMEN`"]
pub struct BKPSRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPSRAMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKPSRAMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKPSRAMEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKPSRAMEN_A::B_0X1)
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
#[doc = "AXIMCEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXIMCEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<AXIMCEN_A> for bool {
    #[inline(always)]
    fn from(variant: AXIMCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXIMCEN`"]
pub type AXIMCEN_R = crate::R<bool, AXIMCEN_A>;
impl AXIMCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXIMCEN_A {
        match self.bits {
            false => AXIMCEN_A::B_0X0,
            true => AXIMCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AXIMCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AXIMCEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `AXIMCEN`"]
pub struct AXIMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIMCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXIMCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AXIMCEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AXIMCEN_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - GPIOZEN"]
    #[inline(always)]
    pub fn gpiozen(&self) -> GPIOZEN_R {
        GPIOZEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP1EN"]
    #[inline(always)]
    pub fn cryp1en(&self) -> CRYP1EN_R {
        CRYP1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH1EN"]
    #[inline(always)]
    pub fn hash1en(&self) -> HASH1EN_R {
        HASH1EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG1EN"]
    #[inline(always)]
    pub fn rng1en(&self) -> RNG1EN_R {
        RNG1EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BKPSRAMEN"]
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AXIMCEN"]
    #[inline(always)]
    pub fn aximcen(&self) -> AXIMCEN_R {
        AXIMCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZEN"]
    #[inline(always)]
    pub fn gpiozen(&mut self) -> GPIOZEN_W {
        GPIOZEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYP1EN"]
    #[inline(always)]
    pub fn cryp1en(&mut self) -> CRYP1EN_W {
        CRYP1EN_W { w: self }
    }
    #[doc = "Bit 5 - HASH1EN"]
    #[inline(always)]
    pub fn hash1en(&mut self) -> HASH1EN_W {
        HASH1EN_W { w: self }
    }
    #[doc = "Bit 6 - RNG1EN"]
    #[inline(always)]
    pub fn rng1en(&mut self) -> RNG1EN_W {
        RNG1EN_W { w: self }
    }
    #[doc = "Bit 8 - BKPSRAMEN"]
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W {
        BKPSRAMEN_W { w: self }
    }
    #[doc = "Bit 16 - AXIMCEN"]
    #[inline(always)]
    pub fn aximcen(&mut self) -> AXIMCEN_W {
        AXIMCEN_W { w: self }
    }
}
