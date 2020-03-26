#[doc = "Reader of register RCC_DDRITFCR"]
pub type R = crate::R<u32, super::RCC_DDRITFCR>;
#[doc = "Writer for register RCC_DDRITFCR"]
pub type W = crate::W<u32, super::RCC_DDRITFCR>;
#[doc = "Register RCC_DDRITFCR `reset()`'s with value 0x000f_d02a"]
impl crate::ResetValue for super::RCC_DDRITFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000f_d02a
    }
}
#[doc = "DDRC1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRC1EN_A {
    #[doc = "0: Means that the DDRC peripheral\r\n                  clocks are disabled"]
    B_0X0 = 0,
    #[doc = "1: Means that the DDRC peripheral\r\n                  clocks are enabled"]
    B_0X1 = 1,
}
impl From<DDRC1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRC1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRC1EN`"]
pub type DDRC1EN_R = crate::R<bool, DDRC1EN_A>;
impl DDRC1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRC1EN_A {
        match self.bits {
            false => DDRC1EN_A::B_0X0,
            true => DDRC1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRC1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRC1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRC1EN`"]
pub struct DDRC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRC1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Means that the DDRC peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRC1EN_A::B_0X0)
    }
    #[doc = "Means that the DDRC peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRC1EN_A::B_0X1)
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
#[doc = "DDRC1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRC1LPEN_A {
    #[doc = "0: means that the peripheral clocks are\r\n                  disabled in CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: means that the peripheral clocks are\r\n                  enabled in CSLEEP"]
    B_0X1 = 1,
}
impl From<DDRC1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRC1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRC1LPEN`"]
pub type DDRC1LPEN_R = crate::R<bool, DDRC1LPEN_A>;
impl DDRC1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRC1LPEN_A {
        match self.bits {
            false => DDRC1LPEN_A::B_0X0,
            true => DDRC1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRC1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRC1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRC1LPEN`"]
pub struct DDRC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRC1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRC1LPEN_A::B_0X0)
    }
    #[doc = "means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRC1LPEN_A::B_0X1)
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
#[doc = "DDRC2EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRC2EN_A {
    #[doc = "0: Means that the DDRC peripheral\r\n                  clocks are disabled"]
    B_0X0 = 0,
    #[doc = "1: Means that the DDRC peripheral\r\n                  clocks are enabled"]
    B_0X1 = 1,
}
impl From<DDRC2EN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRC2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRC2EN`"]
pub type DDRC2EN_R = crate::R<bool, DDRC2EN_A>;
impl DDRC2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRC2EN_A {
        match self.bits {
            false => DDRC2EN_A::B_0X0,
            true => DDRC2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRC2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRC2EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRC2EN`"]
pub struct DDRC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRC2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Means that the DDRC peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRC2EN_A::B_0X0)
    }
    #[doc = "Means that the DDRC peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRC2EN_A::B_0X1)
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
#[doc = "DDRC2LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRC2LPEN_A {
    #[doc = "0: means that the peripheral clocks are\r\n                  disabled in CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: means that the peripheral clocks are\r\n                  enabled in CSLEEP"]
    B_0X1 = 1,
}
impl From<DDRC2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRC2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRC2LPEN`"]
pub type DDRC2LPEN_R = crate::R<bool, DDRC2LPEN_A>;
impl DDRC2LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRC2LPEN_A {
        match self.bits {
            false => DDRC2LPEN_A::B_0X0,
            true => DDRC2LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRC2LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRC2LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRC2LPEN`"]
pub struct DDRC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRC2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRC2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRC2LPEN_A::B_0X0)
    }
    #[doc = "means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRC2LPEN_A::B_0X1)
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
#[doc = "DDRPHYCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRPHYCEN_A {
    #[doc = "0: means that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: means that the peripheral clocks are\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<DDRPHYCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRPHYCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRPHYCEN`"]
pub type DDRPHYCEN_R = crate::R<bool, DDRPHYCEN_A>;
impl DDRPHYCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRPHYCEN_A {
        match self.bits {
            false => DDRPHYCEN_A::B_0X0,
            true => DDRPHYCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRPHYCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRPHYCEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRPHYCEN`"]
pub struct DDRPHYCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRPHYCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRPHYCEN_A::B_0X0)
    }
    #[doc = "means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRPHYCEN_A::B_0X1)
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
#[doc = "DDRPHYCLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRPHYCLPEN_A {
    #[doc = "0: means that the peripheral clocks are\r\n                  disabled in CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: means that the peripheral clocks are\r\n                  enabled in CSLEEP"]
    B_0X1 = 1,
}
impl From<DDRPHYCLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRPHYCLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRPHYCLPEN`"]
pub type DDRPHYCLPEN_R = crate::R<bool, DDRPHYCLPEN_A>;
impl DDRPHYCLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRPHYCLPEN_A {
        match self.bits {
            false => DDRPHYCLPEN_A::B_0X0,
            true => DDRPHYCLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRPHYCLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRPHYCLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRPHYCLPEN`"]
pub struct DDRPHYCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRPHYCLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRPHYCLPEN_A::B_0X0)
    }
    #[doc = "means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRPHYCLPEN_A::B_0X1)
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
#[doc = "DDRCAPBEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRCAPBEN_A {
    #[doc = "0: means that the APB clock is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: means that the APB clock is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<DDRCAPBEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRCAPBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRCAPBEN`"]
pub type DDRCAPBEN_R = crate::R<bool, DDRCAPBEN_A>;
impl DDRCAPBEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRCAPBEN_A {
        match self.bits {
            false => DDRCAPBEN_A::B_0X0,
            true => DDRCAPBEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRCAPBEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRCAPBEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRCAPBEN`"]
pub struct DDRCAPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAPBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRCAPBEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "means that the APB clock is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRCAPBEN_A::B_0X0)
    }
    #[doc = "means that the APB clock is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRCAPBEN_A::B_0X1)
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
#[doc = "DDRCAPBLPEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRCAPBLPEN_A {
    #[doc = "0: means that the APB clock is disabled\r\n                  in CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: means that the APB clock is enabled\r\n                  in CSLEEP"]
    B_0X1 = 1,
}
impl From<DDRCAPBLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRCAPBLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRCAPBLPEN`"]
pub type DDRCAPBLPEN_R = crate::R<bool, DDRCAPBLPEN_A>;
impl DDRCAPBLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRCAPBLPEN_A {
        match self.bits {
            false => DDRCAPBLPEN_A::B_0X0,
            true => DDRCAPBLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRCAPBLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRCAPBLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRCAPBLPEN`"]
pub struct DDRCAPBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAPBLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRCAPBLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "means that the APB clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRCAPBLPEN_A::B_0X0)
    }
    #[doc = "means that the APB clock is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRCAPBLPEN_A::B_0X1)
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
#[doc = "AXIDCGEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXIDCGEN_A {
    #[doc = "0: means that the dynamic clock gating\r\n                  of AXIDCG\\[2:1\\]
is disabled during MPU\r\n                  CRUN,"]
    B_0X0 = 0,
    #[doc = "1: means that the dynamic clock gating\r\n                  of AXIDCG{2:1\\]
is enabled during MPU\r\n                  CRUN"]
    B_0X1 = 1,
}
impl From<AXIDCGEN_A> for bool {
    #[inline(always)]
    fn from(variant: AXIDCGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXIDCGEN`"]
pub type AXIDCGEN_R = crate::R<bool, AXIDCGEN_A>;
impl AXIDCGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXIDCGEN_A {
        match self.bits {
            false => AXIDCGEN_A::B_0X0,
            true => AXIDCGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AXIDCGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AXIDCGEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `AXIDCGEN`"]
pub struct AXIDCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIDCGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AXIDCGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "means that the dynamic clock gating of AXIDCG\\[2:1\\]
is disabled during MPU CRUN,"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AXIDCGEN_A::B_0X0)
    }
    #[doc = "means that the dynamic clock gating of AXIDCG{2:1\\]
is enabled during MPU CRUN"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AXIDCGEN_A::B_0X1)
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
#[doc = "DDRPHYCAPBEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRPHYCAPBEN_A {
    #[doc = "0: means that the APB clock is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: means that the APB clock is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<DDRPHYCAPBEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRPHYCAPBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRPHYCAPBEN`"]
pub type DDRPHYCAPBEN_R = crate::R<bool, DDRPHYCAPBEN_A>;
impl DDRPHYCAPBEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRPHYCAPBEN_A {
        match self.bits {
            false => DDRPHYCAPBEN_A::B_0X0,
            true => DDRPHYCAPBEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRPHYCAPBEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRPHYCAPBEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRPHYCAPBEN`"]
pub struct DDRPHYCAPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCAPBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRPHYCAPBEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "means that the APB clock is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRPHYCAPBEN_A::B_0X0)
    }
    #[doc = "means that the APB clock is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRPHYCAPBEN_A::B_0X1)
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
#[doc = "DDRPHYCAPBLPEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRPHYCAPBLPEN_A {
    #[doc = "0: means that the APB clock is disabled\r\n                  in CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: means that the APB clock is enabled\r\n                  in CSLEEP"]
    B_0X1 = 1,
}
impl From<DDRPHYCAPBLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRPHYCAPBLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRPHYCAPBLPEN`"]
pub type DDRPHYCAPBLPEN_R = crate::R<bool, DDRPHYCAPBLPEN_A>;
impl DDRPHYCAPBLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRPHYCAPBLPEN_A {
        match self.bits {
            false => DDRPHYCAPBLPEN_A::B_0X0,
            true => DDRPHYCAPBLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRPHYCAPBLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRPHYCAPBLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRPHYCAPBLPEN`"]
pub struct DDRPHYCAPBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCAPBLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRPHYCAPBLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "means that the APB clock is disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRPHYCAPBLPEN_A::B_0X0)
    }
    #[doc = "means that the APB clock is enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRPHYCAPBLPEN_A::B_0X1)
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
#[doc = "KERDCG_DLY\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KERDCG_DLY_A {
    #[doc = "0: 1 period of ddrc_ker_ck between\r\n                  cactive_ddrc falling edge and the gating of\r\n                  ddrc_ker_ckg."]
    B_0X0 = 0,
    #[doc = "1: 3 periods of ddrc_ker_ck between\r\n                  cactive_ddrc falling edge and the gating of\r\n                  ddrc_ker_ckg."]
    B_0X1 = 1,
    #[doc = "7: 15 periods of ddrc_ker_ck between\r\n                  cactive_ddrc falling edge and the gating of\r\n                  ddrc_ker_ckg."]
    B_0X7 = 7,
}
impl From<KERDCG_DLY_A> for u8 {
    #[inline(always)]
    fn from(variant: KERDCG_DLY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KERDCG_DLY`"]
pub type KERDCG_DLY_R = crate::R<u8, KERDCG_DLY_A>;
impl KERDCG_DLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KERDCG_DLY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(KERDCG_DLY_A::B_0X0),
            1 => Val(KERDCG_DLY_A::B_0X1),
            7 => Val(KERDCG_DLY_A::B_0X7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == KERDCG_DLY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == KERDCG_DLY_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == KERDCG_DLY_A::B_0X7
    }
}
#[doc = "Write proxy for field `KERDCG_DLY`"]
pub struct KERDCG_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> KERDCG_DLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KERDCG_DLY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 period of ddrc_ker_ck between cactive_ddrc falling edge and the gating of ddrc_ker_ckg."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(KERDCG_DLY_A::B_0X0)
    }
    #[doc = "3 periods of ddrc_ker_ck between cactive_ddrc falling edge and the gating of ddrc_ker_ckg."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(KERDCG_DLY_A::B_0X1)
    }
    #[doc = "15 periods of ddrc_ker_ck between cactive_ddrc falling edge and the gating of ddrc_ker_ckg."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(KERDCG_DLY_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "DDRCAPBRST\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRCAPBRST_A {
    #[doc = "0: does not reset the DDRC APB\r\n                  interface"]
    B_0X0 = 0,
    #[doc = "1: resets the DDRC APB\r\n                  interface"]
    B_0X1 = 1,
}
impl From<DDRCAPBRST_A> for bool {
    #[inline(always)]
    fn from(variant: DDRCAPBRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRCAPBRST`"]
pub type DDRCAPBRST_R = crate::R<bool, DDRCAPBRST_A>;
impl DDRCAPBRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRCAPBRST_A {
        match self.bits {
            false => DDRCAPBRST_A::B_0X0,
            true => DDRCAPBRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRCAPBRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRCAPBRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRCAPBRST`"]
pub struct DDRCAPBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAPBRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRCAPBRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "does not reset the DDRC APB interface"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRCAPBRST_A::B_0X0)
    }
    #[doc = "resets the DDRC APB interface"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRCAPBRST_A::B_0X1)
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
#[doc = "DDRCAXIRST\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRCAXIRST_A {
    #[doc = "0: does not reset the DDRC AXI\r\n                  interface"]
    B_0X0 = 0,
    #[doc = "1: resets the DDRC AXI\r\n                  interface"]
    B_0X1 = 1,
}
impl From<DDRCAXIRST_A> for bool {
    #[inline(always)]
    fn from(variant: DDRCAXIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRCAXIRST`"]
pub type DDRCAXIRST_R = crate::R<bool, DDRCAXIRST_A>;
impl DDRCAXIRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRCAXIRST_A {
        match self.bits {
            false => DDRCAXIRST_A::B_0X0,
            true => DDRCAXIRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRCAXIRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRCAXIRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRCAXIRST`"]
pub struct DDRCAXIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCAXIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRCAXIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "does not reset the DDRC AXI interface"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRCAXIRST_A::B_0X0)
    }
    #[doc = "resets the DDRC AXI interface"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRCAXIRST_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "DDRCORERST\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRCORERST_A {
    #[doc = "0: does not reset the DDRC\r\n                  core"]
    B_0X0 = 0,
    #[doc = "1: resets the DDRC core"]
    B_0X1 = 1,
}
impl From<DDRCORERST_A> for bool {
    #[inline(always)]
    fn from(variant: DDRCORERST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRCORERST`"]
pub type DDRCORERST_R = crate::R<bool, DDRCORERST_A>;
impl DDRCORERST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRCORERST_A {
        match self.bits {
            false => DDRCORERST_A::B_0X0,
            true => DDRCORERST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRCORERST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRCORERST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRCORERST`"]
pub struct DDRCORERST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCORERST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRCORERST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "does not reset the DDRC core"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRCORERST_A::B_0X0)
    }
    #[doc = "resets the DDRC core"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRCORERST_A::B_0X1)
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
#[doc = "DPHYAPBRST\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPHYAPBRST_A {
    #[doc = "0: does not reset the DDRPHYC APB\r\n                  interface"]
    B_0X0 = 0,
    #[doc = "1: resets the DDRPHYC APB\r\n                  interface"]
    B_0X1 = 1,
}
impl From<DPHYAPBRST_A> for bool {
    #[inline(always)]
    fn from(variant: DPHYAPBRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPHYAPBRST`"]
pub type DPHYAPBRST_R = crate::R<bool, DPHYAPBRST_A>;
impl DPHYAPBRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPHYAPBRST_A {
        match self.bits {
            false => DPHYAPBRST_A::B_0X0,
            true => DPHYAPBRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DPHYAPBRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DPHYAPBRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DPHYAPBRST`"]
pub struct DPHYAPBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DPHYAPBRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPHYAPBRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "does not reset the DDRPHYC APB interface"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DPHYAPBRST_A::B_0X0)
    }
    #[doc = "resets the DDRPHYC APB interface"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DPHYAPBRST_A::B_0X1)
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
#[doc = "DPHYRST\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPHYRST_A {
    #[doc = "0: does not reset the\r\n                  DDRPHYC"]
    B_0X0 = 0,
    #[doc = "1: resets the DDRPHYC"]
    B_0X1 = 1,
}
impl From<DPHYRST_A> for bool {
    #[inline(always)]
    fn from(variant: DPHYRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPHYRST`"]
pub type DPHYRST_R = crate::R<bool, DPHYRST_A>;
impl DPHYRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPHYRST_A {
        match self.bits {
            false => DPHYRST_A::B_0X0,
            true => DPHYRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DPHYRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DPHYRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DPHYRST`"]
pub struct DPHYRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DPHYRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPHYRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "does not reset the DDRPHYC"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DPHYRST_A::B_0X0)
    }
    #[doc = "resets the DDRPHYC"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DPHYRST_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "DPHYCTLRST\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPHYCTLRST_A {
    #[doc = "0: does not reset the DDRPHYC\r\n                  Control"]
    B_0X0 = 0,
    #[doc = "1: resets the DDRPHYC\r\n                  Control"]
    B_0X1 = 1,
}
impl From<DPHYCTLRST_A> for bool {
    #[inline(always)]
    fn from(variant: DPHYCTLRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPHYCTLRST`"]
pub type DPHYCTLRST_R = crate::R<bool, DPHYCTLRST_A>;
impl DPHYCTLRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPHYCTLRST_A {
        match self.bits {
            false => DPHYCTLRST_A::B_0X0,
            true => DPHYCTLRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DPHYCTLRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DPHYCTLRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DPHYCTLRST`"]
pub struct DPHYCTLRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DPHYCTLRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPHYCTLRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "does not reset the DDRPHYC Control"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DPHYCTLRST_A::B_0X0)
    }
    #[doc = "resets the DDRPHYC Control"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DPHYCTLRST_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "DDRCKMOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DDRCKMOD_A {
    #[doc = "0: Normal mode: the gating of the\r\n                  dphy_ker_ck clock depends on the DDRPHYCEN,\r\n                  DDRPHYCLPEN and MPU mode. The gating of the\r\n                  ddrc_ker_ckg clock depends on the DDRCxEN,\r\n                  DDRCxLPEN and MPU mode. This mode must be\r\n                  selected during DDRC and DDRPHYC initialization\r\n                  phase, and if the application is using the\r\n                  software self-refresh (SSR)."]
    B_0X0 = 0,
    #[doc = "1: Automatic Self-Refresh mode (ASR1):\r\n                  the clock ddrc_ker_ckg is gated automatically\r\n                  according to cactive_ddrc signal. The gating of\r\n                  the dphy_ker_ck clock depends on the DDRPHYCEN,\r\n                  DDRPHYCLPEN and MPU mode."]
    B_0X1 = 1,
    #[doc = "2: Hardware Self-Refresh mode (HSR1):\r\n                  the gating of the ddrc_ker_ckg clock is\r\n                  controlled by the AXI-Low-Power interface\r\n                  connected to the DDRC. The gating of the\r\n                  dphy_ker_ck clock depends on the DDRPHYCEN,\r\n                  DDRPHYCLPEN and MPU mode."]
    B_0X2 = 2,
    #[doc = "5: Full Automatic Self-Refresh mode\r\n                  (ASR2): the clocks ddrc_ker_ckg and dphy_ker_ck\r\n                  are gated automatically according to cactive_ddrc\r\n                  signal."]
    B_0X5 = 5,
    #[doc = "6: Full Hardware Self-Refresh mode\r\n                  (HSR2): the gating of ddrc_ker_ckg and\r\n                  dphy_ker_ck clocks are controlled by the\r\n                  AXI-Low-Power interface connected to the\r\n                  DDRC."]
    B_0X6 = 6,
}
impl From<DDRCKMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: DDRCKMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DDRCKMOD`"]
pub type DDRCKMOD_R = crate::R<u8, DDRCKMOD_A>;
impl DDRCKMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DDRCKMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DDRCKMOD_A::B_0X0),
            1 => Val(DDRCKMOD_A::B_0X1),
            2 => Val(DDRCKMOD_A::B_0X2),
            5 => Val(DDRCKMOD_A::B_0X5),
            6 => Val(DDRCKMOD_A::B_0X6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRCKMOD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRCKMOD_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DDRCKMOD_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == DDRCKMOD_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == DDRCKMOD_A::B_0X6
    }
}
#[doc = "Write proxy for field `DDRCKMOD`"]
pub struct DDRCKMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCKMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRCKMOD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal mode: the gating of the dphy_ker_ck clock depends on the DDRPHYCEN, DDRPHYCLPEN and MPU mode. The gating of the ddrc_ker_ckg clock depends on the DDRCxEN, DDRCxLPEN and MPU mode. This mode must be selected during DDRC and DDRPHYC initialization phase, and if the application is using the software self-refresh (SSR)."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRCKMOD_A::B_0X0)
    }
    #[doc = "Automatic Self-Refresh mode (ASR1): the clock ddrc_ker_ckg is gated automatically according to cactive_ddrc signal. The gating of the dphy_ker_ck clock depends on the DDRPHYCEN, DDRPHYCLPEN and MPU mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRCKMOD_A::B_0X1)
    }
    #[doc = "Hardware Self-Refresh mode (HSR1): the gating of the ddrc_ker_ckg clock is controlled by the AXI-Low-Power interface connected to the DDRC. The gating of the dphy_ker_ck clock depends on the DDRPHYCEN, DDRPHYCLPEN and MPU mode."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DDRCKMOD_A::B_0X2)
    }
    #[doc = "Full Automatic Self-Refresh mode (ASR2): the clocks ddrc_ker_ckg and dphy_ker_ck are gated automatically according to cactive_ddrc signal."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(DDRCKMOD_A::B_0X5)
    }
    #[doc = "Full Hardware Self-Refresh mode (HSR2): the gating of ddrc_ker_ckg and dphy_ker_ck clocks are controlled by the AXI-Low-Power interface connected to the DDRC."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(DDRCKMOD_A::B_0X6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "GSKPMOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSKPMOD_A {
    #[doc = "0: The GSKP block is controlled by the\r\n                  GSKPCTRL bit."]
    B_0X0 = 0,
    #[doc = "1: The GSKP block is controlled\r\n                  automatically by the DFI."]
    B_0X1 = 1,
}
impl From<GSKPMOD_A> for bool {
    #[inline(always)]
    fn from(variant: GSKPMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GSKPMOD`"]
pub type GSKPMOD_R = crate::R<bool, GSKPMOD_A>;
impl GSKPMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSKPMOD_A {
        match self.bits {
            false => GSKPMOD_A::B_0X0,
            true => GSKPMOD_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GSKPMOD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GSKPMOD_A::B_0X1
    }
}
#[doc = "Write proxy for field `GSKPMOD`"]
pub struct GSKPMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GSKPMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSKPMOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The GSKP block is controlled by the GSKPCTRL bit."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GSKPMOD_A::B_0X0)
    }
    #[doc = "The GSKP block is controlled automatically by the DFI."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GSKPMOD_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "GSKPCTRL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GSKPCTRL_A {
    #[doc = "0: The GSKP block is providing the\r\n                  clock phy_out_ck (provided by the\r\n                  DDRPHYC)"]
    B_0X0 = 0,
    #[doc = "1: The GSKP block is providing the\r\n                  clock dphy_ker_ck (provided by the\r\n                  RCC)"]
    B_0X1 = 1,
}
impl From<GSKPCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: GSKPCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GSKPCTRL`"]
pub type GSKPCTRL_R = crate::R<bool, GSKPCTRL_A>;
impl GSKPCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSKPCTRL_A {
        match self.bits {
            false => GSKPCTRL_A::B_0X0,
            true => GSKPCTRL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GSKPCTRL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GSKPCTRL_A::B_0X1
    }
}
#[doc = "Write proxy for field `GSKPCTRL`"]
pub struct GSKPCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> GSKPCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSKPCTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The GSKP block is providing the clock phy_out_ck (provided by the DDRPHYC)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GSKPCTRL_A::B_0X0)
    }
    #[doc = "The GSKP block is providing the clock dphy_ker_ck (provided by the RCC)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GSKPCTRL_A::B_0X1)
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
#[doc = "DFILP_WIDTH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DFILP_WIDTH_A {
    #[doc = "0: Bypass, delay disabled"]
    B_0X0 = 0,
    #[doc = "1: Forces a delay of 160 x Tdphy_ker_ck\r\n                  to be used when Fdphy_ker_ck is between 120 and\r\n                  160 MHz."]
    B_0X1 = 1,
    #[doc = "2: Forces a delay of 224 x Tdphy_ker_ck\r\n                  to be used when Fdphy_ker_ck is between 160 and\r\n                  220 MHz."]
    B_0X2 = 2,
    #[doc = "3: Forces a delay of 320 x Tdphy_ker_ck\r\n                  to be used when Fdphy_ker_ck is between 220 and\r\n                  320 MHz."]
    B_0X3 = 3,
    #[doc = "4: Forces a delay of 416 x Tdphy_ker_ck\r\n                  to be used when Fdphy_ker_ck is between 320 and\r\n                  410 MHz."]
    B_0X4 = 4,
}
impl From<DFILP_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DFILP_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DFILP_WIDTH`"]
pub type DFILP_WIDTH_R = crate::R<u8, DFILP_WIDTH_A>;
impl DFILP_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DFILP_WIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DFILP_WIDTH_A::B_0X0),
            1 => Val(DFILP_WIDTH_A::B_0X1),
            2 => Val(DFILP_WIDTH_A::B_0X2),
            3 => Val(DFILP_WIDTH_A::B_0X3),
            4 => Val(DFILP_WIDTH_A::B_0X4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DFILP_WIDTH_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DFILP_WIDTH_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DFILP_WIDTH_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == DFILP_WIDTH_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == DFILP_WIDTH_A::B_0X4
    }
}
#[doc = "Write proxy for field `DFILP_WIDTH`"]
pub struct DFILP_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DFILP_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFILP_WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Bypass, delay disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DFILP_WIDTH_A::B_0X0)
    }
    #[doc = "Forces a delay of 160 x Tdphy_ker_ck to be used when Fdphy_ker_ck is between 120 and 160 MHz."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DFILP_WIDTH_A::B_0X1)
    }
    #[doc = "Forces a delay of 224 x Tdphy_ker_ck to be used when Fdphy_ker_ck is between 160 and 220 MHz."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DFILP_WIDTH_A::B_0X2)
    }
    #[doc = "Forces a delay of 320 x Tdphy_ker_ck to be used when Fdphy_ker_ck is between 220 and 320 MHz."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(DFILP_WIDTH_A::B_0X3)
    }
    #[doc = "Forces a delay of 416 x Tdphy_ker_ck to be used when Fdphy_ker_ck is between 320 and 410 MHz."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(DFILP_WIDTH_A::B_0X4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "GSKP_DUR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GSKP_DUR_A {
    #[doc = "0: Sets a delay of 32 x\r\n                  Tdphy_ker_ck"]
    B_0X0 = 0,
    #[doc = "1: Sets a delay of 2 x 32 x\r\n                  Tdphy_ker_ck"]
    B_0X1 = 1,
    #[doc = "2: Sets a delay of 3 x 32 x\r\n                  Tdphy_ker_ck"]
    B_0X2 = 2,
    #[doc = "15: Sets a delay of 16 x 32 x\r\n                  Tdphy_ker_ck"]
    B_0XF = 15,
}
impl From<GSKP_DUR_A> for u8 {
    #[inline(always)]
    fn from(variant: GSKP_DUR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GSKP_DUR`"]
pub type GSKP_DUR_R = crate::R<u8, GSKP_DUR_A>;
impl GSKP_DUR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GSKP_DUR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GSKP_DUR_A::B_0X0),
            1 => Val(GSKP_DUR_A::B_0X1),
            2 => Val(GSKP_DUR_A::B_0X2),
            15 => Val(GSKP_DUR_A::B_0XF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GSKP_DUR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GSKP_DUR_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == GSKP_DUR_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == GSKP_DUR_A::B_0XF
    }
}
#[doc = "Write proxy for field `GSKP_DUR`"]
pub struct GSKP_DUR_W<'a> {
    w: &'a mut W,
}
impl<'a> GSKP_DUR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GSKP_DUR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sets a delay of 32 x Tdphy_ker_ck"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(GSKP_DUR_A::B_0X0)
    }
    #[doc = "Sets a delay of 2 x 32 x Tdphy_ker_ck"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(GSKP_DUR_A::B_0X1)
    }
    #[doc = "Sets a delay of 3 x 32 x Tdphy_ker_ck"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(GSKP_DUR_A::B_0X2)
    }
    #[doc = "Sets a delay of 16 x 32 x Tdphy_ker_ck"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(GSKP_DUR_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DDRC1EN"]
    #[inline(always)]
    pub fn ddrc1en(&self) -> DDRC1EN_R {
        DDRC1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DDRC1LPEN"]
    #[inline(always)]
    pub fn ddrc1lpen(&self) -> DDRC1LPEN_R {
        DDRC1LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DDRC2EN"]
    #[inline(always)]
    pub fn ddrc2en(&self) -> DDRC2EN_R {
        DDRC2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DDRC2LPEN"]
    #[inline(always)]
    pub fn ddrc2lpen(&self) -> DDRC2LPEN_R {
        DDRC2LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DDRPHYCEN"]
    #[inline(always)]
    pub fn ddrphycen(&self) -> DDRPHYCEN_R {
        DDRPHYCEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DDRPHYCLPEN"]
    #[inline(always)]
    pub fn ddrphyclpen(&self) -> DDRPHYCLPEN_R {
        DDRPHYCLPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DDRCAPBEN"]
    #[inline(always)]
    pub fn ddrcapben(&self) -> DDRCAPBEN_R {
        DDRCAPBEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DDRCAPBLPEN"]
    #[inline(always)]
    pub fn ddrcapblpen(&self) -> DDRCAPBLPEN_R {
        DDRCAPBLPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AXIDCGEN"]
    #[inline(always)]
    pub fn axidcgen(&self) -> AXIDCGEN_R {
        AXIDCGEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DDRPHYCAPBEN"]
    #[inline(always)]
    pub fn ddrphycapben(&self) -> DDRPHYCAPBEN_R {
        DDRPHYCAPBEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DDRPHYCAPBLPEN"]
    #[inline(always)]
    pub fn ddrphycapblpen(&self) -> DDRPHYCAPBLPEN_R {
        DDRPHYCAPBLPEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - KERDCG_DLY"]
    #[inline(always)]
    pub fn kerdcg_dly(&self) -> KERDCG_DLY_R {
        KERDCG_DLY_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 14 - DDRCAPBRST"]
    #[inline(always)]
    pub fn ddrcapbrst(&self) -> DDRCAPBRST_R {
        DDRCAPBRST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DDRCAXIRST"]
    #[inline(always)]
    pub fn ddrcaxirst(&self) -> DDRCAXIRST_R {
        DDRCAXIRST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DDRCORERST"]
    #[inline(always)]
    pub fn ddrcorerst(&self) -> DDRCORERST_R {
        DDRCORERST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DPHYAPBRST"]
    #[inline(always)]
    pub fn dphyapbrst(&self) -> DPHYAPBRST_R {
        DPHYAPBRST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DPHYRST"]
    #[inline(always)]
    pub fn dphyrst(&self) -> DPHYRST_R {
        DPHYRST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DPHYCTLRST"]
    #[inline(always)]
    pub fn dphyctlrst(&self) -> DPHYCTLRST_R {
        DPHYCTLRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - DDRCKMOD"]
    #[inline(always)]
    pub fn ddrckmod(&self) -> DDRCKMOD_R {
        DDRCKMOD_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - GSKPMOD"]
    #[inline(always)]
    pub fn gskpmod(&self) -> GSKPMOD_R {
        GSKPMOD_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GSKPCTRL"]
    #[inline(always)]
    pub fn gskpctrl(&self) -> GSKPCTRL_R {
        GSKPCTRL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:27 - DFILP_WIDTH"]
    #[inline(always)]
    pub fn dfilp_width(&self) -> DFILP_WIDTH_R {
        DFILP_WIDTH_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 28:31 - GSKP_DUR"]
    #[inline(always)]
    pub fn gskp_dur(&self) -> GSKP_DUR_R {
        GSKP_DUR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DDRC1EN"]
    #[inline(always)]
    pub fn ddrc1en(&mut self) -> DDRC1EN_W {
        DDRC1EN_W { w: self }
    }
    #[doc = "Bit 1 - DDRC1LPEN"]
    #[inline(always)]
    pub fn ddrc1lpen(&mut self) -> DDRC1LPEN_W {
        DDRC1LPEN_W { w: self }
    }
    #[doc = "Bit 2 - DDRC2EN"]
    #[inline(always)]
    pub fn ddrc2en(&mut self) -> DDRC2EN_W {
        DDRC2EN_W { w: self }
    }
    #[doc = "Bit 3 - DDRC2LPEN"]
    #[inline(always)]
    pub fn ddrc2lpen(&mut self) -> DDRC2LPEN_W {
        DDRC2LPEN_W { w: self }
    }
    #[doc = "Bit 4 - DDRPHYCEN"]
    #[inline(always)]
    pub fn ddrphycen(&mut self) -> DDRPHYCEN_W {
        DDRPHYCEN_W { w: self }
    }
    #[doc = "Bit 5 - DDRPHYCLPEN"]
    #[inline(always)]
    pub fn ddrphyclpen(&mut self) -> DDRPHYCLPEN_W {
        DDRPHYCLPEN_W { w: self }
    }
    #[doc = "Bit 6 - DDRCAPBEN"]
    #[inline(always)]
    pub fn ddrcapben(&mut self) -> DDRCAPBEN_W {
        DDRCAPBEN_W { w: self }
    }
    #[doc = "Bit 7 - DDRCAPBLPEN"]
    #[inline(always)]
    pub fn ddrcapblpen(&mut self) -> DDRCAPBLPEN_W {
        DDRCAPBLPEN_W { w: self }
    }
    #[doc = "Bit 8 - AXIDCGEN"]
    #[inline(always)]
    pub fn axidcgen(&mut self) -> AXIDCGEN_W {
        AXIDCGEN_W { w: self }
    }
    #[doc = "Bit 9 - DDRPHYCAPBEN"]
    #[inline(always)]
    pub fn ddrphycapben(&mut self) -> DDRPHYCAPBEN_W {
        DDRPHYCAPBEN_W { w: self }
    }
    #[doc = "Bit 10 - DDRPHYCAPBLPEN"]
    #[inline(always)]
    pub fn ddrphycapblpen(&mut self) -> DDRPHYCAPBLPEN_W {
        DDRPHYCAPBLPEN_W { w: self }
    }
    #[doc = "Bits 11:13 - KERDCG_DLY"]
    #[inline(always)]
    pub fn kerdcg_dly(&mut self) -> KERDCG_DLY_W {
        KERDCG_DLY_W { w: self }
    }
    #[doc = "Bit 14 - DDRCAPBRST"]
    #[inline(always)]
    pub fn ddrcapbrst(&mut self) -> DDRCAPBRST_W {
        DDRCAPBRST_W { w: self }
    }
    #[doc = "Bit 15 - DDRCAXIRST"]
    #[inline(always)]
    pub fn ddrcaxirst(&mut self) -> DDRCAXIRST_W {
        DDRCAXIRST_W { w: self }
    }
    #[doc = "Bit 16 - DDRCORERST"]
    #[inline(always)]
    pub fn ddrcorerst(&mut self) -> DDRCORERST_W {
        DDRCORERST_W { w: self }
    }
    #[doc = "Bit 17 - DPHYAPBRST"]
    #[inline(always)]
    pub fn dphyapbrst(&mut self) -> DPHYAPBRST_W {
        DPHYAPBRST_W { w: self }
    }
    #[doc = "Bit 18 - DPHYRST"]
    #[inline(always)]
    pub fn dphyrst(&mut self) -> DPHYRST_W {
        DPHYRST_W { w: self }
    }
    #[doc = "Bit 19 - DPHYCTLRST"]
    #[inline(always)]
    pub fn dphyctlrst(&mut self) -> DPHYCTLRST_W {
        DPHYCTLRST_W { w: self }
    }
    #[doc = "Bits 20:22 - DDRCKMOD"]
    #[inline(always)]
    pub fn ddrckmod(&mut self) -> DDRCKMOD_W {
        DDRCKMOD_W { w: self }
    }
    #[doc = "Bit 23 - GSKPMOD"]
    #[inline(always)]
    pub fn gskpmod(&mut self) -> GSKPMOD_W {
        GSKPMOD_W { w: self }
    }
    #[doc = "Bit 24 - GSKPCTRL"]
    #[inline(always)]
    pub fn gskpctrl(&mut self) -> GSKPCTRL_W {
        GSKPCTRL_W { w: self }
    }
    #[doc = "Bits 25:27 - DFILP_WIDTH"]
    #[inline(always)]
    pub fn dfilp_width(&mut self) -> DFILP_WIDTH_W {
        DFILP_WIDTH_W { w: self }
    }
    #[doc = "Bits 28:31 - GSKP_DUR"]
    #[inline(always)]
    pub fn gskp_dur(&mut self) -> GSKP_DUR_W {
        GSKP_DUR_W { w: self }
    }
}
