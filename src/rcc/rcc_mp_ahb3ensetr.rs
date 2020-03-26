#[doc = "Reader of register RCC_MP_AHB3ENSETR"]
pub type R = crate::R<u32, super::RCC_MP_AHB3ENSETR>;
#[doc = "Writer for register RCC_MP_AHB3ENSETR"]
pub type W = crate::W<u32, super::RCC_MP_AHB3ENSETR>;
#[doc = "Register RCC_MP_AHB3ENSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_AHB3ENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DCMIEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMIEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<DCMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCMIEN`"]
pub type DCMIEN_R = crate::R<bool, DCMIEN_A>;
impl DCMIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMIEN_A {
        match self.bits {
            false => DCMIEN_A::B_0X0,
            true => DCMIEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DCMIEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DCMIEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DCMIEN`"]
pub struct DCMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DCMIEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DCMIEN_A::B_0X1)
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
#[doc = "CRYP2EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYP2EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<CRYP2EN_A> for bool {
    #[inline(always)]
    fn from(variant: CRYP2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYP2EN`"]
pub type CRYP2EN_R = crate::R<bool, CRYP2EN_A>;
impl CRYP2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYP2EN_A {
        match self.bits {
            false => CRYP2EN_A::B_0X0,
            true => CRYP2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRYP2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRYP2EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `CRYP2EN`"]
pub struct CRYP2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYP2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRYP2EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRYP2EN_A::B_0X1)
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
#[doc = "HASH2EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH2EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<HASH2EN_A> for bool {
    #[inline(always)]
    fn from(variant: HASH2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HASH2EN`"]
pub type HASH2EN_R = crate::R<bool, HASH2EN_A>;
impl HASH2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH2EN_A {
        match self.bits {
            false => HASH2EN_A::B_0X0,
            true => HASH2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HASH2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HASH2EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `HASH2EN`"]
pub struct HASH2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HASH2EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HASH2EN_A::B_0X1)
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
#[doc = "RNG2EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG2EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<RNG2EN_A> for bool {
    #[inline(always)]
    fn from(variant: RNG2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNG2EN`"]
pub type RNG2EN_R = crate::R<bool, RNG2EN_A>;
impl RNG2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG2EN_A {
        match self.bits {
            false => RNG2EN_A::B_0X0,
            true => RNG2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RNG2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RNG2EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `RNG2EN`"]
pub struct RNG2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RNG2EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RNG2EN_A::B_0X1)
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
#[doc = "CRC2EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC2EN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<CRC2EN_A> for bool {
    #[inline(always)]
    fn from(variant: CRC2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC2EN`"]
pub type CRC2EN_R = crate::R<bool, CRC2EN_A>;
impl CRC2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC2EN_A {
        match self.bits {
            false => CRC2EN_A::B_0X0,
            true => CRC2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRC2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRC2EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `CRC2EN`"]
pub struct CRC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRC2EN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRC2EN_A::B_0X1)
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
#[doc = "HSEMEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEMEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<HSEMEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSEMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSEMEN`"]
pub type HSEMEN_R = crate::R<bool, HSEMEN_A>;
impl HSEMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEMEN_A {
        match self.bits {
            false => HSEMEN_A::B_0X0,
            true => HSEMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSEMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSEMEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSEMEN`"]
pub struct HSEMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSEMEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSEMEN_A::B_0X1)
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
#[doc = "IPCCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPCCEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<IPCCEN_A> for bool {
    #[inline(always)]
    fn from(variant: IPCCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IPCCEN`"]
pub type IPCCEN_R = crate::R<bool, IPCCEN_A>;
impl IPCCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPCCEN_A {
        match self.bits {
            false => IPCCEN_A::B_0X0,
            true => IPCCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IPCCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IPCCEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `IPCCEN`"]
pub struct IPCCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPCCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IPCCEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IPCCEN_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - DCMIEN"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP2EN"]
    #[inline(always)]
    pub fn cryp2en(&self) -> CRYP2EN_R {
        CRYP2EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH2EN"]
    #[inline(always)]
    pub fn hash2en(&self) -> HASH2EN_R {
        HASH2EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG2EN"]
    #[inline(always)]
    pub fn rng2en(&self) -> RNG2EN_R {
        RNG2EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRC2EN"]
    #[inline(always)]
    pub fn crc2en(&self) -> CRC2EN_R {
        CRC2EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSEMEN"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IPCCEN"]
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMIEN"]
    #[inline(always)]
    pub fn dcmien(&mut self) -> DCMIEN_W {
        DCMIEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYP2EN"]
    #[inline(always)]
    pub fn cryp2en(&mut self) -> CRYP2EN_W {
        CRYP2EN_W { w: self }
    }
    #[doc = "Bit 5 - HASH2EN"]
    #[inline(always)]
    pub fn hash2en(&mut self) -> HASH2EN_W {
        HASH2EN_W { w: self }
    }
    #[doc = "Bit 6 - RNG2EN"]
    #[inline(always)]
    pub fn rng2en(&mut self) -> RNG2EN_W {
        RNG2EN_W { w: self }
    }
    #[doc = "Bit 7 - CRC2EN"]
    #[inline(always)]
    pub fn crc2en(&mut self) -> CRC2EN_W {
        CRC2EN_W { w: self }
    }
    #[doc = "Bit 11 - HSEMEN"]
    #[inline(always)]
    pub fn hsemen(&mut self) -> HSEMEN_W {
        HSEMEN_W { w: self }
    }
    #[doc = "Bit 12 - IPCCEN"]
    #[inline(always)]
    pub fn ipccen(&mut self) -> IPCCEN_W {
        IPCCEN_W { w: self }
    }
}
