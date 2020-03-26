#[doc = "Reader of register RCC_MC_AHB6ENSETR"]
pub type R = crate::R<u32, super::RCC_MC_AHB6ENSETR>;
#[doc = "Writer for register RCC_MC_AHB6ENSETR"]
pub type W = crate::W<u32, super::RCC_MC_AHB6ENSETR>;
#[doc = "Register RCC_MC_AHB6ENSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MC_AHB6ENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MDMAEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDMAEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<MDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: MDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MDMAEN`"]
pub type MDMAEN_R = crate::R<bool, MDMAEN_A>;
impl MDMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDMAEN_A {
        match self.bits {
            false => MDMAEN_A::B_0X0,
            true => MDMAEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MDMAEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MDMAEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `MDMAEN`"]
pub struct MDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MDMAEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MDMAEN_A::B_0X1)
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
#[doc = "GPUEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPUEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<GPUEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPUEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPUEN`"]
pub type GPUEN_R = crate::R<bool, GPUEN_A>;
impl GPUEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPUEN_A {
        match self.bits {
            false => GPUEN_A::B_0X0,
            true => GPUEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPUEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPUEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPUEN`"]
pub struct GPUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPUEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPUEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPUEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPUEN_A::B_0X1)
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
#[doc = "ETHCKEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHCKEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that eth_ker_ck clock is disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the eth_ker_ck\r\n                  clock, reading means that the eth_ker_ck clock is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<ETHCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETHCKEN`"]
pub type ETHCKEN_R = crate::R<bool, ETHCKEN_A>;
impl ETHCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHCKEN_A {
        match self.bits {
            false => ETHCKEN_A::B_0X0,
            true => ETHCKEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHCKEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHCKEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETHCKEN`"]
pub struct ETHCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHCKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that eth_ker_ck clock is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHCKEN_A::B_0X0)
    }
    #[doc = "Writing enables the eth_ker_ck clock, reading means that the eth_ker_ck clock is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHCKEN_A::B_0X1)
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
#[doc = "ETHTXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHTXEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the transmission clock is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the transmission\r\n                  clock, reading means that the transmission clock\r\n                  is enabled"]
    B_0X1 = 1,
}
impl From<ETHTXEN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHTXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETHTXEN`"]
pub type ETHTXEN_R = crate::R<bool, ETHTXEN_A>;
impl ETHTXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHTXEN_A {
        match self.bits {
            false => ETHTXEN_A::B_0X0,
            true => ETHTXEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHTXEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHTXEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETHTXEN`"]
pub struct ETHTXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHTXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHTXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the transmission clock is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHTXEN_A::B_0X0)
    }
    #[doc = "Writing enables the transmission clock, reading means that the transmission clock is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHTXEN_A::B_0X1)
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
#[doc = "ETHRXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHRXEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the reception clock is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the reception clock,\r\n                  reading means that the reception clock is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<ETHRXEN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHRXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETHRXEN`"]
pub type ETHRXEN_R = crate::R<bool, ETHRXEN_A>;
impl ETHRXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHRXEN_A {
        match self.bits {
            false => ETHRXEN_A::B_0X0,
            true => ETHRXEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHRXEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHRXEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETHRXEN`"]
pub struct ETHRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHRXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHRXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the reception clock is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHRXEN_A::B_0X0)
    }
    #[doc = "Writing enables the reception clock, reading means that the reception clock is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHRXEN_A::B_0X1)
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
#[doc = "ETHMACEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHMACEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the bus interface clock is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the bus interface\r\n                  clock, reading means that the bus interface clock\r\n                  is enabled"]
    B_0X1 = 1,
}
impl From<ETHMACEN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHMACEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETHMACEN`"]
pub type ETHMACEN_R = crate::R<bool, ETHMACEN_A>;
impl ETHMACEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHMACEN_A {
        match self.bits {
            false => ETHMACEN_A::B_0X0,
            true => ETHMACEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHMACEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHMACEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETHMACEN`"]
pub struct ETHMACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHMACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHMACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the bus interface clock is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHMACEN_A::B_0X0)
    }
    #[doc = "Writing enables the bus interface clock, reading means that the bus interface clock is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHMACEN_A::B_0X1)
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
#[doc = "FMCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<FMCEN_A> for bool {
    #[inline(always)]
    fn from(variant: FMCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FMCEN`"]
pub type FMCEN_R = crate::R<bool, FMCEN_A>;
impl FMCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCEN_A {
        match self.bits {
            false => FMCEN_A::B_0X0,
            true => FMCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMCEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `FMCEN`"]
pub struct FMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMCEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMCEN_A::B_0X1)
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
#[doc = "QSPIEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSPIEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<QSPIEN_A> for bool {
    #[inline(always)]
    fn from(variant: QSPIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QSPIEN`"]
pub type QSPIEN_R = crate::R<bool, QSPIEN_A>;
impl QSPIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPIEN_A {
        match self.bits {
            false => QSPIEN_A::B_0X0,
            true => QSPIEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == QSPIEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == QSPIEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `QSPIEN`"]
pub struct QSPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(QSPIEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(QSPIEN_A::B_0X1)
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
#[doc = "SDMMC1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMC1EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<SDMMC1EN_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDMMC1EN`"]
pub type SDMMC1EN_R = crate::R<bool, SDMMC1EN_A>;
impl SDMMC1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMC1EN_A {
        match self.bits {
            false => SDMMC1EN_A::B_0X0,
            true => SDMMC1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SDMMC1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SDMMC1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SDMMC1EN`"]
pub struct SDMMC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SDMMC1EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SDMMC1EN_A::B_0X1)
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
#[doc = "SDMMC2EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMC2EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<SDMMC2EN_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDMMC2EN`"]
pub type SDMMC2EN_R = crate::R<bool, SDMMC2EN_A>;
impl SDMMC2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMC2EN_A {
        match self.bits {
            false => SDMMC2EN_A::B_0X0,
            true => SDMMC2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SDMMC2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SDMMC2EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SDMMC2EN`"]
pub struct SDMMC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SDMMC2EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SDMMC2EN_A::B_0X1)
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
#[doc = "CRC1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC1EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<CRC1EN_A> for bool {
    #[inline(always)]
    fn from(variant: CRC1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC1EN`"]
pub type CRC1EN_R = crate::R<bool, CRC1EN_A>;
impl CRC1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC1EN_A {
        match self.bits {
            false => CRC1EN_A::B_0X0,
            true => CRC1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRC1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRC1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `CRC1EN`"]
pub struct CRC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRC1EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRC1EN_A::B_0X1)
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
#[doc = "USBHEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBHEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<USBHEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBHEN`"]
pub type USBHEN_R = crate::R<bool, USBHEN_A>;
impl USBHEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHEN_A {
        match self.bits {
            false => USBHEN_A::B_0X0,
            true => USBHEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBHEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBHEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `USBHEN`"]
pub struct USBHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBHEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBHEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBHEN_A::B_0X1)
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
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPUEN"]
    #[inline(always)]
    pub fn gpuen(&self) -> GPUEN_R {
        GPUEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ETHCKEN"]
    #[inline(always)]
    pub fn ethcken(&self) -> ETHCKEN_R {
        ETHCKEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ETHTXEN"]
    #[inline(always)]
    pub fn ethtxen(&self) -> ETHTXEN_R {
        ETHTXEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ETHRXEN"]
    #[inline(always)]
    pub fn ethrxen(&self) -> ETHRXEN_R {
        ETHRXEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETHMACEN"]
    #[inline(always)]
    pub fn ethmacen(&self) -> ETHMACEN_R {
        ETHMACEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - QSPIEN"]
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1EN"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SDMMC2EN"]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CRC1EN"]
    #[inline(always)]
    pub fn crc1en(&self) -> CRC1EN_R {
        CRC1EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - USBHEN"]
    #[inline(always)]
    pub fn usbhen(&self) -> USBHEN_R {
        USBHEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMAEN"]
    #[inline(always)]
    pub fn mdmaen(&mut self) -> MDMAEN_W {
        MDMAEN_W { w: self }
    }
    #[doc = "Bit 5 - GPUEN"]
    #[inline(always)]
    pub fn gpuen(&mut self) -> GPUEN_W {
        GPUEN_W { w: self }
    }
    #[doc = "Bit 7 - ETHCKEN"]
    #[inline(always)]
    pub fn ethcken(&mut self) -> ETHCKEN_W {
        ETHCKEN_W { w: self }
    }
    #[doc = "Bit 8 - ETHTXEN"]
    #[inline(always)]
    pub fn ethtxen(&mut self) -> ETHTXEN_W {
        ETHTXEN_W { w: self }
    }
    #[doc = "Bit 9 - ETHRXEN"]
    #[inline(always)]
    pub fn ethrxen(&mut self) -> ETHRXEN_W {
        ETHRXEN_W { w: self }
    }
    #[doc = "Bit 10 - ETHMACEN"]
    #[inline(always)]
    pub fn ethmacen(&mut self) -> ETHMACEN_W {
        ETHMACEN_W { w: self }
    }
    #[doc = "Bit 12 - FMCEN"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FMCEN_W {
        FMCEN_W { w: self }
    }
    #[doc = "Bit 14 - QSPIEN"]
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W {
        QSPIEN_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1EN"]
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W {
        SDMMC1EN_W { w: self }
    }
    #[doc = "Bit 17 - SDMMC2EN"]
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W {
        SDMMC2EN_W { w: self }
    }
    #[doc = "Bit 20 - CRC1EN"]
    #[inline(always)]
    pub fn crc1en(&mut self) -> CRC1EN_W {
        CRC1EN_W { w: self }
    }
    #[doc = "Bit 24 - USBHEN"]
    #[inline(always)]
    pub fn usbhen(&mut self) -> USBHEN_W {
        USBHEN_W { w: self }
    }
}
