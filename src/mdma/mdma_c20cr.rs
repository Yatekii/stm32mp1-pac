#[doc = "Reader of register MDMA_C20CR"]
pub type R = crate::R<u32, super::MDMA_C20CR>;
#[doc = "Writer for register MDMA_C20CR"]
pub type W = crate::W<u32, super::MDMA_C20CR>;
#[doc = "Register MDMA_C20CR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C20CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Channel disabled"]
    B_0X0 = 0,
    #[doc = "1: Channel enabled"]
    B_0X1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, EN_A>;
impl EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::B_0X0,
            true => EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EN_A::B_0X0)
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EN_A::B_0X1)
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
#[doc = "TEIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIE_A {
    #[doc = "0: TE interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: TE interrupt enabled"]
    B_0X1 = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIE`"]
pub type TEIE_R = crate::R<bool, TEIE_A>;
impl TEIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::B_0X0,
            true => TEIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TEIE`"]
pub struct TEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TE interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEIE_A::B_0X0)
    }
    #[doc = "TE interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEIE_A::B_0X1)
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
#[doc = "CTCIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIE_A {
    #[doc = "0: TC interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: TC interrupt enabled"]
    B_0X1 = 1,
}
impl From<CTCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIE`"]
pub type CTCIE_R = crate::R<bool, CTCIE_A>;
impl CTCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIE_A {
        match self.bits {
            false => CTCIE_A::B_0X0,
            true => CTCIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `CTCIE`"]
pub struct CTCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CTCIE_A::B_0X0)
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CTCIE_A::B_0X1)
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
#[doc = "BRTIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIE_A {
    #[doc = "0: BT interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: BT interrupt enabled"]
    B_0X1 = 1,
}
impl From<BRTIE_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIE`"]
pub type BRTIE_R = crate::R<bool, BRTIE_A>;
impl BRTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIE_A {
        match self.bits {
            false => BRTIE_A::B_0X0,
            true => BRTIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `BRTIE`"]
pub struct BRTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BT interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BRTIE_A::B_0X0)
    }
    #[doc = "BT interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BRTIE_A::B_0X1)
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
#[doc = "BTIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIE_A {
    #[doc = "0: BT complete interrupt\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: BT complete interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<BTIE_A> for bool {
    #[inline(always)]
    fn from(variant: BTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIE`"]
pub type BTIE_R = crate::R<bool, BTIE_A>;
impl BTIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIE_A {
        match self.bits {
            false => BTIE_A::B_0X0,
            true => BTIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `BTIE`"]
pub struct BTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BTIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BT complete interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BTIE_A::B_0X0)
    }
    #[doc = "BT complete interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BTIE_A::B_0X1)
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
#[doc = "TCIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    #[doc = "0: TC interrupt disabled"]
    B_0X0 = 0,
    #[doc = "1: TC interrupt enabled"]
    B_0X1 = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIE`"]
pub type TCIE_R = crate::R<bool, TCIE_A>;
impl TCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::B_0X0,
            true => TCIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TCIE`"]
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TCIE_A::B_0X0)
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TCIE_A::B_0X1)
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
#[doc = "PL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PL_A {
    #[doc = "0: Low"]
    B_0X0 = 0,
    #[doc = "1: Medium"]
    B_0X1 = 1,
    #[doc = "2: High"]
    B_0X2 = 2,
    #[doc = "3: Very high"]
    B_0X3 = 3,
}
impl From<PL_A> for u8 {
    #[inline(always)]
    fn from(variant: PL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PL`"]
pub type PL_R = crate::R<u8, PL_A>;
impl PL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PL_A {
        match self.bits {
            0 => PL_A::B_0X0,
            1 => PL_A::B_0X1,
            2 => PL_A::B_0X2,
            3 => PL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PL_A::B_0X3
    }
}
#[doc = "Write proxy for field `PL`"]
pub struct PL_W<'a> {
    w: &'a mut W,
}
impl<'a> PL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PL_A::B_0X0)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PL_A::B_0X1)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PL_A::B_0X2)
    }
    #[doc = "Very high"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PL_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "BEX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEX_A {
    #[doc = "0: Little endianess preserved for\r\n                  bytes"]
    B_0X0 = 0,
    #[doc = "1: byte order exchanged in each\r\n                  half-word"]
    B_0X1 = 1,
}
impl From<BEX_A> for bool {
    #[inline(always)]
    fn from(variant: BEX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BEX`"]
pub type BEX_R = crate::R<bool, BEX_A>;
impl BEX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEX_A {
        match self.bits {
            false => BEX_A::B_0X0,
            true => BEX_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BEX_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BEX_A::B_0X1
    }
}
#[doc = "Write proxy for field `BEX`"]
pub struct BEX_W<'a> {
    w: &'a mut W,
}
impl<'a> BEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BEX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Little endianess preserved for bytes"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BEX_A::B_0X0)
    }
    #[doc = "byte order exchanged in each half-word"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BEX_A::B_0X1)
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
#[doc = "HEX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HEX_A {
    #[doc = "0: Little endianess preserved for half\r\n                  words"]
    B_0X0 = 0,
    #[doc = "1: half-word order exchanged in each\r\n                  word"]
    B_0X1 = 1,
}
impl From<HEX_A> for bool {
    #[inline(always)]
    fn from(variant: HEX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HEX`"]
pub type HEX_R = crate::R<bool, HEX_A>;
impl HEX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HEX_A {
        match self.bits {
            false => HEX_A::B_0X0,
            true => HEX_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HEX_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HEX_A::B_0X1
    }
}
#[doc = "Write proxy for field `HEX`"]
pub struct HEX_W<'a> {
    w: &'a mut W,
}
impl<'a> HEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HEX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Little endianess preserved for half words"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HEX_A::B_0X0)
    }
    #[doc = "half-word order exchanged in each word"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HEX_A::B_0X1)
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
#[doc = "WEX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WEX_A {
    #[doc = "0: Little endianess preserved for\r\n                  words"]
    B_0X0 = 0,
    #[doc = "1: word order exchanged in double\r\n                  word"]
    B_0X1 = 1,
}
impl From<WEX_A> for bool {
    #[inline(always)]
    fn from(variant: WEX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WEX`"]
pub type WEX_R = crate::R<bool, WEX_A>;
impl WEX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WEX_A {
        match self.bits {
            false => WEX_A::B_0X0,
            true => WEX_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WEX_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WEX_A::B_0X1
    }
}
#[doc = "Write proxy for field `WEX`"]
pub struct WEX_W<'a> {
    w: &'a mut W,
}
impl<'a> WEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WEX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Little endianess preserved for words"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WEX_A::B_0X0)
    }
    #[doc = "word order exchanged in double word"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WEX_A::B_0X1)
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
#[doc = "Write proxy for field `SWRQ`"]
pub struct SWRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRQ_W<'a> {
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
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CTCIE"]
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BRTIE"]
    #[inline(always)]
    pub fn brtie(&self) -> BRTIE_R {
        BRTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BTIE"]
    #[inline(always)]
    pub fn btie(&self) -> BTIE_R {
        BTIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - PL"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 12 - BEX"]
    #[inline(always)]
    pub fn bex(&self) -> BEX_R {
        BEX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - HEX"]
    #[inline(always)]
    pub fn hex(&self) -> HEX_R {
        HEX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - WEX"]
    #[inline(always)]
    pub fn wex(&self) -> WEX_R {
        WEX_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - TEIE"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W {
        TEIE_W { w: self }
    }
    #[doc = "Bit 2 - CTCIE"]
    #[inline(always)]
    pub fn ctcie(&mut self) -> CTCIE_W {
        CTCIE_W { w: self }
    }
    #[doc = "Bit 3 - BRTIE"]
    #[inline(always)]
    pub fn brtie(&mut self) -> BRTIE_W {
        BRTIE_W { w: self }
    }
    #[doc = "Bit 4 - BTIE"]
    #[inline(always)]
    pub fn btie(&mut self) -> BTIE_W {
        BTIE_W { w: self }
    }
    #[doc = "Bit 5 - TCIE"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    #[doc = "Bits 6:7 - PL"]
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W {
        PL_W { w: self }
    }
    #[doc = "Bit 12 - BEX"]
    #[inline(always)]
    pub fn bex(&mut self) -> BEX_W {
        BEX_W { w: self }
    }
    #[doc = "Bit 13 - HEX"]
    #[inline(always)]
    pub fn hex(&mut self) -> HEX_W {
        HEX_W { w: self }
    }
    #[doc = "Bit 14 - WEX"]
    #[inline(always)]
    pub fn wex(&mut self) -> WEX_W {
        WEX_W { w: self }
    }
    #[doc = "Bit 16 - SWRQ"]
    #[inline(always)]
    pub fn swrq(&mut self) -> SWRQ_W {
        SWRQ_W { w: self }
    }
}
