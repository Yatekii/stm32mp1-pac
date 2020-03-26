#[doc = "Reader of register RCC_MP_AHB3LPENCLRR"]
pub type R = crate::R<u32, super::RCC_MP_AHB3LPENCLRR>;
#[doc = "Writer for register RCC_MP_AHB3LPENCLRR"]
pub type W = crate::W<u32, super::RCC_MP_AHB3LPENCLRR>;
#[doc = "Register RCC_MP_AHB3LPENCLRR `reset()`'s with value 0x18f1"]
impl crate::ResetValue for super::RCC_MP_AHB3LPENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x18f1
    }
}
#[doc = "DCMILPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCMILPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<DCMILPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMILPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCMILPEN`"]
pub type DCMILPEN_R = crate::R<bool, DCMILPEN_A>;
impl DCMILPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMILPEN_A {
        match self.bits {
            false => DCMILPEN_A::B_0X0,
            true => DCMILPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DCMILPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DCMILPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DCMILPEN`"]
pub struct DCMILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMILPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCMILPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DCMILPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DCMILPEN_A::B_0X1)
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
#[doc = "CRYP2LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYP2LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<CRYP2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRYP2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRYP2LPEN`"]
pub type CRYP2LPEN_R = crate::R<bool, CRYP2LPEN_A>;
impl CRYP2LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYP2LPEN_A {
        match self.bits {
            false => CRYP2LPEN_A::B_0X0,
            true => CRYP2LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRYP2LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRYP2LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `CRYP2LPEN`"]
pub struct CRYP2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRYP2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRYP2LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRYP2LPEN_A::B_0X1)
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
#[doc = "HASH2LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HASH2LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<HASH2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: HASH2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HASH2LPEN`"]
pub type HASH2LPEN_R = crate::R<bool, HASH2LPEN_A>;
impl HASH2LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HASH2LPEN_A {
        match self.bits {
            false => HASH2LPEN_A::B_0X0,
            true => HASH2LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HASH2LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HASH2LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `HASH2LPEN`"]
pub struct HASH2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HASH2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HASH2LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HASH2LPEN_A::B_0X1)
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
#[doc = "RNG2LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNG2LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<RNG2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNG2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNG2LPEN`"]
pub type RNG2LPEN_R = crate::R<bool, RNG2LPEN_A>;
impl RNG2LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNG2LPEN_A {
        match self.bits {
            false => RNG2LPEN_A::B_0X0,
            true => RNG2LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RNG2LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RNG2LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `RNG2LPEN`"]
pub struct RNG2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNG2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RNG2LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RNG2LPEN_A::B_0X1)
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
#[doc = "CRC2LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC2LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<CRC2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRC2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC2LPEN`"]
pub type CRC2LPEN_R = crate::R<bool, CRC2LPEN_A>;
impl CRC2LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC2LPEN_A {
        match self.bits {
            false => CRC2LPEN_A::B_0X0,
            true => CRC2LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRC2LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRC2LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `CRC2LPEN`"]
pub struct CRC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRC2LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRC2LPEN_A::B_0X1)
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
#[doc = "HSEMLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSEMLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<HSEMLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSEMLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSEMLPEN`"]
pub type HSEMLPEN_R = crate::R<bool, HSEMLPEN_A>;
impl HSEMLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSEMLPEN_A {
        match self.bits {
            false => HSEMLPEN_A::B_0X0,
            true => HSEMLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSEMLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSEMLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSEMLPEN`"]
pub struct HSEMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEMLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSEMLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSEMLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSEMLPEN_A::B_0X1)
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
#[doc = "IPCCLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IPCCLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<IPCCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: IPCCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IPCCLPEN`"]
pub type IPCCLPEN_R = crate::R<bool, IPCCLPEN_A>;
impl IPCCLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPCCLPEN_A {
        match self.bits {
            false => IPCCLPEN_A::B_0X0,
            true => IPCCLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IPCCLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IPCCLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `IPCCLPEN`"]
pub struct IPCCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPCCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IPCCLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IPCCLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IPCCLPEN_A::B_0X1)
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
    #[doc = "Bit 0 - DCMILPEN"]
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP2LPEN"]
    #[inline(always)]
    pub fn cryp2lpen(&self) -> CRYP2LPEN_R {
        CRYP2LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH2LPEN"]
    #[inline(always)]
    pub fn hash2lpen(&self) -> HASH2LPEN_R {
        HASH2LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG2LPEN"]
    #[inline(always)]
    pub fn rng2lpen(&self) -> RNG2LPEN_R {
        RNG2LPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRC2LPEN"]
    #[inline(always)]
    pub fn crc2lpen(&self) -> CRC2LPEN_R {
        CRC2LPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSEMLPEN"]
    #[inline(always)]
    pub fn hsemlpen(&self) -> HSEMLPEN_R {
        HSEMLPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - IPCCLPEN"]
    #[inline(always)]
    pub fn ipcclpen(&self) -> IPCCLPEN_R {
        IPCCLPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCMILPEN"]
    #[inline(always)]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W {
        DCMILPEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYP2LPEN"]
    #[inline(always)]
    pub fn cryp2lpen(&mut self) -> CRYP2LPEN_W {
        CRYP2LPEN_W { w: self }
    }
    #[doc = "Bit 5 - HASH2LPEN"]
    #[inline(always)]
    pub fn hash2lpen(&mut self) -> HASH2LPEN_W {
        HASH2LPEN_W { w: self }
    }
    #[doc = "Bit 6 - RNG2LPEN"]
    #[inline(always)]
    pub fn rng2lpen(&mut self) -> RNG2LPEN_W {
        RNG2LPEN_W { w: self }
    }
    #[doc = "Bit 7 - CRC2LPEN"]
    #[inline(always)]
    pub fn crc2lpen(&mut self) -> CRC2LPEN_W {
        CRC2LPEN_W { w: self }
    }
    #[doc = "Bit 11 - HSEMLPEN"]
    #[inline(always)]
    pub fn hsemlpen(&mut self) -> HSEMLPEN_W {
        HSEMLPEN_W { w: self }
    }
    #[doc = "Bit 12 - IPCCLPEN"]
    #[inline(always)]
    pub fn ipcclpen(&mut self) -> IPCCLPEN_W {
        IPCCLPEN_W { w: self }
    }
}
