#[doc = "Reader of register RCC_AHB6RSTSETR"]
pub type R = crate::R<u32, super::RCC_AHB6RSTSETR>;
#[doc = "Writer for register RCC_AHB6RSTSETR"]
pub type W = crate::W<u32, super::RCC_AHB6RSTSETR>;
#[doc = "Register RCC_AHB6RSTSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_AHB6RSTSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GPURST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPURST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  on-going"]
    B_0X1 = 1,
}
impl From<GPURST_A> for bool {
    #[inline(always)]
    fn from(variant: GPURST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GPURST`"]
pub type GPURST_R = crate::R<bool, GPURST_A>;
impl GPURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPURST_A {
        match self.bits {
            false => GPURST_A::B_0X0,
            true => GPURST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPURST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPURST_A::B_0X1
    }
}
#[doc = "Write proxy for field `GPURST`"]
pub struct GPURST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPURST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GPURST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is on-going"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GPURST_A::B_0X1)
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
#[doc = "ETHMACRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETHMACRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<ETHMACRST_A> for bool {
    #[inline(always)]
    fn from(variant: ETHMACRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETHMACRST`"]
pub type ETHMACRST_R = crate::R<bool, ETHMACRST_A>;
impl ETHMACRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHMACRST_A {
        match self.bits {
            false => ETHMACRST_A::B_0X0,
            true => ETHMACRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETHMACRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETHMACRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETHMACRST`"]
pub struct ETHMACRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHMACRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETHMACRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETHMACRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETHMACRST_A::B_0X1)
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
#[doc = "FMCRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FMCRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<FMCRST_A> for bool {
    #[inline(always)]
    fn from(variant: FMCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FMCRST`"]
pub type FMCRST_R = crate::R<bool, FMCRST_A>;
impl FMCRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCRST_A {
        match self.bits {
            false => FMCRST_A::B_0X0,
            true => FMCRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMCRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMCRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `FMCRST`"]
pub struct FMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FMCRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FMCRST_A::B_0X1)
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
#[doc = "QSPIRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSPIRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<QSPIRST_A> for bool {
    #[inline(always)]
    fn from(variant: QSPIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `QSPIRST`"]
pub type QSPIRST_R = crate::R<bool, QSPIRST_A>;
impl QSPIRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPIRST_A {
        match self.bits {
            false => QSPIRST_A::B_0X0,
            true => QSPIRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == QSPIRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == QSPIRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `QSPIRST`"]
pub struct QSPIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(QSPIRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(QSPIRST_A::B_0X1)
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
#[doc = "SDMMC1RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMC1RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SDMMC1RST_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDMMC1RST`"]
pub type SDMMC1RST_R = crate::R<bool, SDMMC1RST_A>;
impl SDMMC1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMC1RST_A {
        match self.bits {
            false => SDMMC1RST_A::B_0X0,
            true => SDMMC1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SDMMC1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SDMMC1RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SDMMC1RST`"]
pub struct SDMMC1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SDMMC1RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SDMMC1RST_A::B_0X1)
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
#[doc = "SDMMC2RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMC2RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SDMMC2RST_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDMMC2RST`"]
pub type SDMMC2RST_R = crate::R<bool, SDMMC2RST_A>;
impl SDMMC2RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMC2RST_A {
        match self.bits {
            false => SDMMC2RST_A::B_0X0,
            true => SDMMC2RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SDMMC2RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SDMMC2RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SDMMC2RST`"]
pub struct SDMMC2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SDMMC2RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SDMMC2RST_A::B_0X1)
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
#[doc = "CRC1RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC1RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<CRC1RST_A> for bool {
    #[inline(always)]
    fn from(variant: CRC1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRC1RST`"]
pub type CRC1RST_R = crate::R<bool, CRC1RST_A>;
impl CRC1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC1RST_A {
        match self.bits {
            false => CRC1RST_A::B_0X0,
            true => CRC1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRC1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRC1RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `CRC1RST`"]
pub struct CRC1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CRC1RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CRC1RST_A::B_0X1)
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
#[doc = "USBHRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBHRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<USBHRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBHRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBHRST`"]
pub type USBHRST_R = crate::R<bool, USBHRST_A>;
impl USBHRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBHRST_A {
        match self.bits {
            false => USBHRST_A::B_0X0,
            true => USBHRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBHRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBHRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `USBHRST`"]
pub struct USBHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBHRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBHRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBHRST_A::B_0X1)
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
    #[doc = "Bit 5 - GPURST"]
    #[inline(always)]
    pub fn gpurst(&self) -> GPURST_R {
        GPURST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ETHMACRST"]
    #[inline(always)]
    pub fn ethmacrst(&self) -> ETHMACRST_R {
        ETHMACRST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FMCRST"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FMCRST_R {
        FMCRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - QSPIRST"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC1RST"]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SDMMC2RST"]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CRC1RST"]
    #[inline(always)]
    pub fn crc1rst(&self) -> CRC1RST_R {
        CRC1RST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - USBHRST"]
    #[inline(always)]
    pub fn usbhrst(&self) -> USBHRST_R {
        USBHRST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - GPURST"]
    #[inline(always)]
    pub fn gpurst(&mut self) -> GPURST_W {
        GPURST_W { w: self }
    }
    #[doc = "Bit 10 - ETHMACRST"]
    #[inline(always)]
    pub fn ethmacrst(&mut self) -> ETHMACRST_W {
        ETHMACRST_W { w: self }
    }
    #[doc = "Bit 12 - FMCRST"]
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FMCRST_W {
        FMCRST_W { w: self }
    }
    #[doc = "Bit 14 - QSPIRST"]
    #[inline(always)]
    pub fn qspirst(&mut self) -> QSPIRST_W {
        QSPIRST_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC1RST"]
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W {
        SDMMC1RST_W { w: self }
    }
    #[doc = "Bit 17 - SDMMC2RST"]
    #[inline(always)]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W {
        SDMMC2RST_W { w: self }
    }
    #[doc = "Bit 20 - CRC1RST"]
    #[inline(always)]
    pub fn crc1rst(&mut self) -> CRC1RST_W {
        CRC1RST_W { w: self }
    }
    #[doc = "Bit 24 - USBHRST"]
    #[inline(always)]
    pub fn usbhrst(&mut self) -> USBHRST_W {
        USBHRST_W { w: self }
    }
}
