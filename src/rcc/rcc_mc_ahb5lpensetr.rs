#[doc = "Reader of register RCC_MC_AHB5LPENSETR"]
pub type R = crate::R<u32, super::RCC_MC_AHB5LPENSETR>;
#[doc = "Writer for register RCC_MC_AHB5LPENSETR"]
pub type W = crate::W<u32, super::RCC_MC_AHB5LPENSETR>;
#[doc = "Register RCC_MC_AHB5LPENSETR `reset()`'s with value 0x0171"]
impl crate::ResetValue for super::RCC_MC_AHB5LPENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0171
    }
}
#[doc = "GPIOZLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOZLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<GPIOZLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOZLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPIOZLPEN`"]
pub type GPIOZLPEN_R = crate::R<bool, GPIOZLPEN_A>;
impl GPIOZLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOZLPEN_A {
        match self.bits {
            false => GPIOZLPEN_A::B_0X0,
            true => GPIOZLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPIOZLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPIOZLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPIOZLPEN`"]
pub struct GPIOZLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOZLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIOZLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPIOZLPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPIOZLPEN_A::B_0X1)
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
#[doc = "CRYP1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYP1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<CRYP1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRYP1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYP1LPEN`"]
pub type CRYP1LPEN_R = crate::R<bool, CRYP1LPEN_A>;
impl CRYP1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYP1LPEN_A {
        match self.bits {
            false => CRYP1LPEN_A::B_0X0,
            true => CRYP1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRYP1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRYP1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `CRYP1LPEN`"]
pub struct CRYP1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYP1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRYP1LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRYP1LPEN_A::B_0X1)
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
#[doc = "HASH1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<HASH1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: HASH1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HASH1LPEN`"]
pub type HASH1LPEN_R = crate::R<bool, HASH1LPEN_A>;
impl HASH1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH1LPEN_A {
        match self.bits {
            false => HASH1LPEN_A::B_0X0,
            true => HASH1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HASH1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HASH1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `HASH1LPEN`"]
pub struct HASH1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HASH1LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HASH1LPEN_A::B_0X1)
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
#[doc = "RNG1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<RNG1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNG1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNG1LPEN`"]
pub type RNG1LPEN_R = crate::R<bool, RNG1LPEN_A>;
impl RNG1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG1LPEN_A {
        match self.bits {
            false => RNG1LPEN_A::B_0X0,
            true => RNG1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RNG1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RNG1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `RNG1LPEN`"]
pub struct RNG1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RNG1LPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RNG1LPEN_A::B_0X1)
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
#[doc = "BKPSRAMLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKPSRAMLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the clock is disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral clock\r\n                  in CSLEEP, reading means that the clock is\r\n                  enabled in CSLEEP"]
    B_0X1 = 1,
}
impl From<BKPSRAMLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: BKPSRAMLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKPSRAMLPEN`"]
pub type BKPSRAMLPEN_R = crate::R<bool, BKPSRAMLPEN_A>;
impl BKPSRAMLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKPSRAMLPEN_A {
        match self.bits {
            false => BKPSRAMLPEN_A::B_0X0,
            true => BKPSRAMLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKPSRAMLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKPSRAMLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKPSRAMLPEN`"]
pub struct BKPSRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPSRAMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKPSRAMLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKPSRAMLPEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clock in CSLEEP, reading means that the clock is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKPSRAMLPEN_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - GPIOZLPEN"]
    #[inline(always)]
    pub fn gpiozlpen(&self) -> GPIOZLPEN_R {
        GPIOZLPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP1LPEN"]
    #[inline(always)]
    pub fn cryp1lpen(&self) -> CRYP1LPEN_R {
        CRYP1LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH1LPEN"]
    #[inline(always)]
    pub fn hash1lpen(&self) -> HASH1LPEN_R {
        HASH1LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG1LPEN"]
    #[inline(always)]
    pub fn rng1lpen(&self) -> RNG1LPEN_R {
        RNG1LPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BKPSRAMLPEN"]
    #[inline(always)]
    pub fn bkpsramlpen(&self) -> BKPSRAMLPEN_R {
        BKPSRAMLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZLPEN"]
    #[inline(always)]
    pub fn gpiozlpen(&mut self) -> GPIOZLPEN_W {
        GPIOZLPEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYP1LPEN"]
    #[inline(always)]
    pub fn cryp1lpen(&mut self) -> CRYP1LPEN_W {
        CRYP1LPEN_W { w: self }
    }
    #[doc = "Bit 5 - HASH1LPEN"]
    #[inline(always)]
    pub fn hash1lpen(&mut self) -> HASH1LPEN_W {
        HASH1LPEN_W { w: self }
    }
    #[doc = "Bit 6 - RNG1LPEN"]
    #[inline(always)]
    pub fn rng1lpen(&mut self) -> RNG1LPEN_W {
        RNG1LPEN_W { w: self }
    }
    #[doc = "Bit 8 - BKPSRAMLPEN"]
    #[inline(always)]
    pub fn bkpsramlpen(&mut self) -> BKPSRAMLPEN_W {
        BKPSRAMLPEN_W { w: self }
    }
}
