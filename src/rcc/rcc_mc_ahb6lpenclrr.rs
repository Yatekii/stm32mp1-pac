#[doc = "Reader of register RCC_MC_AHB6LPENCLRR"]
pub type R = crate::R<u32, super::RCC_MC_AHB6LPENCLRR>;
#[doc = "Writer for register RCC_MC_AHB6LPENCLRR"]
pub type W = crate::W<u32, super::RCC_MC_AHB6LPENCLRR>;
#[doc = "Register RCC_MC_AHB6LPENCLRR `reset()`'s with value 0x0113_57a1"]
impl crate::ResetValue for super::RCC_MC_AHB6LPENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0113_57a1
    }
}
#[doc = "MDMALPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDMALPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<MDMALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: MDMALPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDMALPEN`"]
pub type MDMALPEN_R = crate::R<bool, MDMALPEN_A>;
impl MDMALPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDMALPEN_A {
        match self.bits {
            false => MDMALPEN_A::B_0X0,
            true => MDMALPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MDMALPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MDMALPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `MDMALPEN`"]
pub struct MDMALPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMALPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDMALPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MDMALPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MDMALPEN_A::B_0X1)
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
#[doc = "GPULPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPULPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<GPULPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPULPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPULPEN`"]
pub type GPULPEN_R = crate::R<bool, GPULPEN_A>;
impl GPULPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPULPEN_A {
        match self.bits {
            false => GPULPEN_A::B_0X0,
            true => GPULPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPULPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPULPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPULPEN`"]
pub struct GPULPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPULPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPULPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPULPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPULPEN_A::B_0X1)
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
#[doc = "ETHCKLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHCKLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that eth_ker_ck clock is disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the eth_ker_ck\r\n                  clock in CSLEEP, reading means that the\r\n                  eth_ker_ck clock is enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<ETHCKLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHCKLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETHCKLPEN`"]
pub type ETHCKLPEN_R = crate::R<bool, ETHCKLPEN_A>;
impl ETHCKLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHCKLPEN_A {
        match self.bits {
            false => ETHCKLPEN_A::B_0X0,
            true => ETHCKLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHCKLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHCKLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETHCKLPEN`"]
pub struct ETHCKLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHCKLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHCKLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that eth_ker_ck clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHCKLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the eth_ker_ck clock in CSLEEP, reading means that the eth_ker_ck clock is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHCKLPEN_A::B_0X1)
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
#[doc = "ETHTXLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHTXLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the transmission clock is disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the transmission\r\n                  clock in CSLEEP, reading means that the\r\n                  transmission clock is enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<ETHTXLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHTXLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETHTXLPEN`"]
pub type ETHTXLPEN_R = crate::R<bool, ETHTXLPEN_A>;
impl ETHTXLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHTXLPEN_A {
        match self.bits {
            false => ETHTXLPEN_A::B_0X0,
            true => ETHTXLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHTXLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHTXLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETHTXLPEN`"]
pub struct ETHTXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHTXLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHTXLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the transmission clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHTXLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the transmission clock in CSLEEP, reading means that the transmission clock is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHTXLPEN_A::B_0X1)
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
#[doc = "ETHRXLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHRXLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the reception clock is disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the reception clock\r\n                  in CSLEEP, reading means that the reception clock\r\n                  is enabled in CSLEEP"]
    B_0X1 = 1,
}
impl From<ETHRXLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHRXLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETHRXLPEN`"]
pub type ETHRXLPEN_R = crate::R<bool, ETHRXLPEN_A>;
impl ETHRXLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHRXLPEN_A {
        match self.bits {
            false => ETHRXLPEN_A::B_0X0,
            true => ETHRXLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHRXLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHRXLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETHRXLPEN`"]
pub struct ETHRXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHRXLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHRXLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the reception clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHRXLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the reception clock in CSLEEP, reading means that the reception clock is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHRXLPEN_A::B_0X1)
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
#[doc = "ETHMACLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHMACLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the bus interface clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the bus interface\r\n                  clocks in CSLEEP, reading means that the bus\r\n                  interface clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<ETHMACLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHMACLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETHMACLPEN`"]
pub type ETHMACLPEN_R = crate::R<bool, ETHMACLPEN_A>;
impl ETHMACLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHMACLPEN_A {
        match self.bits {
            false => ETHMACLPEN_A::B_0X0,
            true => ETHMACLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHMACLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHMACLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETHMACLPEN`"]
pub struct ETHMACLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHMACLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHMACLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the bus interface clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHMACLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the bus interface clocks in CSLEEP, reading means that the bus interface clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHMACLPEN_A::B_0X1)
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
#[doc = "ETHSTPEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHSTPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the ETH RX and TX kernel clocks are gated in\r\n                  CSTOP"]
    B_0X0 = 0,
    #[doc = "1: Writing disabled the ETH RX and TX\r\n                  kernel clocks in CSTOP, reading means that the\r\n                  ETH RX and TX kernel clocks are enabled in\r\n                  CSTOP"]
    B_0X1 = 1,
}
impl From<ETHSTPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHSTPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETHSTPEN`"]
pub type ETHSTPEN_R = crate::R<bool, ETHSTPEN_A>;
impl ETHSTPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHSTPEN_A {
        match self.bits {
            false => ETHSTPEN_A::B_0X0,
            true => ETHSTPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHSTPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHSTPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETHSTPEN`"]
pub struct ETHSTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHSTPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHSTPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the ETH RX and TX kernel clocks are gated in CSTOP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHSTPEN_A::B_0X0)
    }
    #[doc = "Writing disabled the ETH RX and TX kernel clocks in CSTOP, reading means that the ETH RX and TX kernel clocks are enabled in CSTOP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHSTPEN_A::B_0X1)
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
#[doc = "FMCLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<FMCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FMCLPEN`"]
pub type FMCLPEN_R = crate::R<bool, FMCLPEN_A>;
impl FMCLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCLPEN_A {
        match self.bits {
            false => FMCLPEN_A::B_0X0,
            true => FMCLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMCLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMCLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `FMCLPEN`"]
pub struct FMCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMCLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMCLPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "QSPILPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSPILPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<QSPILPEN_A> for bool {
    #[inline(always)]
    fn from(variant: QSPILPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QSPILPEN`"]
pub type QSPILPEN_R = crate::R<bool, QSPILPEN_A>;
impl QSPILPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPILPEN_A {
        match self.bits {
            false => QSPILPEN_A::B_0X0,
            true => QSPILPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == QSPILPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == QSPILPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `QSPILPEN`"]
pub struct QSPILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPILPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(QSPILPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(QSPILPEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "SDMMC1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMC1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SDMMC1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDMMC1LPEN`"]
pub type SDMMC1LPEN_R = crate::R<bool, SDMMC1LPEN_A>;
impl SDMMC1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMC1LPEN_A {
        match self.bits {
            false => SDMMC1LPEN_A::B_0X0,
            true => SDMMC1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SDMMC1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SDMMC1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SDMMC1LPEN`"]
pub struct SDMMC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SDMMC1LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SDMMC1LPEN_A::B_0X1)
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
#[doc = "SDMMC2LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMC2LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SDMMC2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDMMC2LPEN`"]
pub type SDMMC2LPEN_R = crate::R<bool, SDMMC2LPEN_A>;
impl SDMMC2LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMC2LPEN_A {
        match self.bits {
            false => SDMMC2LPEN_A::B_0X0,
            true => SDMMC2LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SDMMC2LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SDMMC2LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SDMMC2LPEN`"]
pub struct SDMMC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SDMMC2LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SDMMC2LPEN_A::B_0X1)
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
#[doc = "CRC1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<CRC1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRC1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC1LPEN`"]
pub type CRC1LPEN_R = crate::R<bool, CRC1LPEN_A>;
impl CRC1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC1LPEN_A {
        match self.bits {
            false => CRC1LPEN_A::B_0X0,
            true => CRC1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRC1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRC1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `CRC1LPEN`"]
pub struct CRC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRC1LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRC1LPEN_A::B_0X1)
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
#[doc = "USBHLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBHLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<USBHLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBHLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBHLPEN`"]
pub type USBHLPEN_R = crate::R<bool, USBHLPEN_A>;
impl USBHLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHLPEN_A {
        match self.bits {
            false => USBHLPEN_A::B_0X0,
            true => USBHLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBHLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBHLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `USBHLPEN`"]
pub struct USBHLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBHLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBHLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBHLPEN_A::B_0X1)
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
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPULPEN"]
    #[inline(always)]
    pub fn gpulpen(&self) -> GPULPEN_R {
        GPULPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ETHCKLPEN"]
    #[inline(always)]
    pub fn ethcklpen(&self) -> ETHCKLPEN_R {
        ETHCKLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ETHTXLPEN"]
    #[inline(always)]
    pub fn ethtxlpen(&self) -> ETHTXLPEN_R {
        ETHTXLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ETHRXLPEN"]
    #[inline(always)]
    pub fn ethrxlpen(&self) -> ETHRXLPEN_R {
        ETHRXLPEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETHMACLPEN"]
    #[inline(always)]
    pub fn ethmaclpen(&self) -> ETHMACLPEN_R {
        ETHMACLPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ETHSTPEN"]
    #[inline(always)]
    pub fn ethstpen(&self) -> ETHSTPEN_R {
        ETHSTPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FMCLPEN"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - QSPILPEN"]
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1LPEN"]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SDMMC2LPEN"]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CRC1LPEN"]
    #[inline(always)]
    pub fn crc1lpen(&self) -> CRC1LPEN_R {
        CRC1LPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - USBHLPEN"]
    #[inline(always)]
    pub fn usbhlpen(&self) -> USBHLPEN_R {
        USBHLPEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMALPEN"]
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W {
        MDMALPEN_W { w: self }
    }
    #[doc = "Bit 5 - GPULPEN"]
    #[inline(always)]
    pub fn gpulpen(&mut self) -> GPULPEN_W {
        GPULPEN_W { w: self }
    }
    #[doc = "Bit 7 - ETHCKLPEN"]
    #[inline(always)]
    pub fn ethcklpen(&mut self) -> ETHCKLPEN_W {
        ETHCKLPEN_W { w: self }
    }
    #[doc = "Bit 8 - ETHTXLPEN"]
    #[inline(always)]
    pub fn ethtxlpen(&mut self) -> ETHTXLPEN_W {
        ETHTXLPEN_W { w: self }
    }
    #[doc = "Bit 9 - ETHRXLPEN"]
    #[inline(always)]
    pub fn ethrxlpen(&mut self) -> ETHRXLPEN_W {
        ETHRXLPEN_W { w: self }
    }
    #[doc = "Bit 10 - ETHMACLPEN"]
    #[inline(always)]
    pub fn ethmaclpen(&mut self) -> ETHMACLPEN_W {
        ETHMACLPEN_W { w: self }
    }
    #[doc = "Bit 11 - ETHSTPEN"]
    #[inline(always)]
    pub fn ethstpen(&mut self) -> ETHSTPEN_W {
        ETHSTPEN_W { w: self }
    }
    #[doc = "Bit 12 - FMCLPEN"]
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W {
        FMCLPEN_W { w: self }
    }
    #[doc = "Bit 14 - QSPILPEN"]
    #[inline(always)]
    pub fn qspilpen(&mut self) -> QSPILPEN_W {
        QSPILPEN_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1LPEN"]
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W {
        SDMMC1LPEN_W { w: self }
    }
    #[doc = "Bit 17 - SDMMC2LPEN"]
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W {
        SDMMC2LPEN_W { w: self }
    }
    #[doc = "Bit 20 - CRC1LPEN"]
    #[inline(always)]
    pub fn crc1lpen(&mut self) -> CRC1LPEN_W {
        CRC1LPEN_W { w: self }
    }
    #[doc = "Bit 24 - USBHLPEN"]
    #[inline(always)]
    pub fn usbhlpen(&mut self) -> USBHLPEN_W {
        USBHLPEN_W { w: self }
    }
}
