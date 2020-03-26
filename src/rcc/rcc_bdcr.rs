#[doc = "Reader of register RCC_BDCR"]
pub type R = crate::R<u32, super::RCC_BDCR>;
#[doc = "Writer for register RCC_BDCR"]
pub type W = crate::W<u32, super::RCC_BDCR>;
#[doc = "Register RCC_BDCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_BDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LSEON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEON_A {
    #[doc = "0: LSE oscillator OFF (default after\r\n                  backup domain reset)"]
    B_0X0 = 0,
    #[doc = "1: LSE oscillator ON"]
    B_0X1 = 1,
}
impl From<LSEON_A> for bool {
    #[inline(always)]
    fn from(variant: LSEON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSEON`"]
pub type LSEON_R = crate::R<bool, LSEON_A>;
impl LSEON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEON_A {
        match self.bits {
            false => LSEON_A::B_0X0,
            true => LSEON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSEON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSEON_A::B_0X1
    }
}
#[doc = "Write proxy for field `LSEON`"]
pub struct LSEON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSEON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSE oscillator OFF (default after backup domain reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSEON_A::B_0X0)
    }
    #[doc = "LSE oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSEON_A::B_0X1)
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
#[doc = "LSEBYP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSEBYP_A {
    #[doc = "0: LSE oscillator not bypassed (default\r\n                  after backup domain reset)"]
    B_0X0 = 0,
    #[doc = "1: LSE oscillator\r\n                  bypassed"]
    B_0X1 = 1,
}
impl From<LSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSEBYP`"]
pub type LSEBYP_R = crate::R<bool, LSEBYP_A>;
impl LSEBYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEBYP_A {
        match self.bits {
            false => LSEBYP_A::B_0X0,
            true => LSEBYP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSEBYP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSEBYP_A::B_0X1
    }
}
#[doc = "Write proxy for field `LSEBYP`"]
pub struct LSEBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSEBYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSE oscillator not bypassed (default after backup domain reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSEBYP_A::B_0X0)
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSEBYP_A::B_0X1)
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
#[doc = "LSERDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDY_A {
    #[doc = "0: LSE oscillator not ready (default\r\n                  after backup domain reset)"]
    B_0X0 = 0,
    #[doc = "1: LSE oscillator ready"]
    B_0X1 = 1,
}
impl From<LSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSERDY`"]
pub type LSERDY_R = crate::R<bool, LSERDY_A>;
impl LSERDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDY_A {
        match self.bits {
            false => LSERDY_A::B_0X0,
            true => LSERDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSERDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSERDY_A::B_0X1
    }
}
#[doc = "LSEDRV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LSEDRV_A {
    #[doc = "0: Lowest drive (default after backup\r\n                  domain reset)"]
    B_0X0 = 0,
    #[doc = "1: Medium low drive"]
    B_0X1 = 1,
    #[doc = "2: Medium high drive"]
    B_0X2 = 2,
    #[doc = "3: Highest drive"]
    B_0X3 = 3,
}
impl From<LSEDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LSEDRV`"]
pub type LSEDRV_R = crate::R<u8, LSEDRV_A>;
impl LSEDRV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEDRV_A {
        match self.bits {
            0 => LSEDRV_A::B_0X0,
            1 => LSEDRV_A::B_0X1,
            2 => LSEDRV_A::B_0X2,
            3 => LSEDRV_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSEDRV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSEDRV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == LSEDRV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LSEDRV_A::B_0X3
    }
}
#[doc = "Write proxy for field `LSEDRV`"]
pub struct LSEDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> LSEDRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSEDRV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Lowest drive (default after backup domain reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSEDRV_A::B_0X0)
    }
    #[doc = "Medium low drive"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSEDRV_A::B_0X1)
    }
    #[doc = "Medium high drive"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LSEDRV_A::B_0X2)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LSEDRV_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "LSECSSON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSECSSON_A {
    #[doc = "0: Clock Security System on 32 kHz\r\n                  oscillator OFF (default after backup domain\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: Clock Security System on 32 kHz\r\n                  oscillator ON"]
    B_0X1 = 1,
}
impl From<LSECSSON_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSECSSON`"]
pub type LSECSSON_R = crate::R<bool, LSECSSON_A>;
impl LSECSSON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSON_A {
        match self.bits {
            false => LSECSSON_A::B_0X0,
            true => LSECSSON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSECSSON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSECSSON_A::B_0X1
    }
}
#[doc = "Write proxy for field `LSECSSON`"]
pub struct LSECSSON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSECSSON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock Security System on 32 kHz oscillator OFF (default after backup domain reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSECSSON_A::B_0X0)
    }
    #[doc = "Clock Security System on 32 kHz oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSECSSON_A::B_0X1)
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
#[doc = "LSECSSD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSECSSD_A {
    #[doc = "0: No failure detected on 32 kHz\r\n                  oscillator (default after backup domain\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: Failure detected on 32 kHz\r\n                  oscillator"]
    B_0X1 = 1,
}
impl From<LSECSSD_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSECSSD`"]
pub type LSECSSD_R = crate::R<bool, LSECSSD_A>;
impl LSECSSD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSD_A {
        match self.bits {
            false => LSECSSD_A::B_0X0,
            true => LSECSSD_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSECSSD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSECSSD_A::B_0X1
    }
}
#[doc = "RTCSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSRC_A {
    #[doc = "0: No clock (default after backup\r\n                  domain reset)"]
    B_0X0 = 0,
    #[doc = "1: LSE clock used as RTC\r\n                  clock"]
    B_0X1 = 1,
    #[doc = "2: LSI clock used as RTC\r\n                  clock"]
    B_0X2 = 2,
    #[doc = "3: HSE clock divided by RTCDIV value is\r\n                  used as RTC clock"]
    B_0X3 = 3,
}
impl From<RTCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCSRC`"]
pub type RTCSRC_R = crate::R<u8, RTCSRC_A>;
impl RTCSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSRC_A {
        match self.bits {
            0 => RTCSRC_A::B_0X0,
            1 => RTCSRC_A::B_0X1,
            2 => RTCSRC_A::B_0X2,
            3 => RTCSRC_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCSRC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == RTCSRC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == RTCSRC_A::B_0X3
    }
}
#[doc = "Write proxy for field `RTCSRC`"]
pub struct RTCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSRC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No clock (default after backup domain reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RTCSRC_A::B_0X0)
    }
    #[doc = "LSE clock used as RTC clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RTCSRC_A::B_0X1)
    }
    #[doc = "LSI clock used as RTC clock"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(RTCSRC_A::B_0X2)
    }
    #[doc = "HSE clock divided by RTCDIV value is used as RTC clock"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(RTCSRC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "RTCCKEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCKEN_A {
    #[doc = "0: rtc_ck clock is disabled (default\r\n                  after backup domain reset)"]
    B_0X0 = 0,
    #[doc = "1: rtc_ck clock enabled"]
    B_0X1 = 1,
}
impl From<RTCCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTCCKEN`"]
pub type RTCCKEN_R = crate::R<bool, RTCCKEN_A>;
impl RTCCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCKEN_A {
        match self.bits {
            false => RTCCKEN_A::B_0X0,
            true => RTCCKEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTCCKEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTCCKEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `RTCCKEN`"]
pub struct RTCCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "rtc_ck clock is disabled (default after backup domain reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RTCCKEN_A::B_0X0)
    }
    #[doc = "rtc_ck clock enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RTCCKEN_A::B_0X1)
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
#[doc = "VSWRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSWRST_A {
    #[doc = "0: Reset not activated (default after\r\n                  backup domain reset)"]
    B_0X0 = 0,
    #[doc = "1: Resets the entire VSW\r\n                  domain"]
    B_0X1 = 1,
}
impl From<VSWRST_A> for bool {
    #[inline(always)]
    fn from(variant: VSWRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VSWRST`"]
pub type VSWRST_R = crate::R<bool, VSWRST_A>;
impl VSWRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSWRST_A {
        match self.bits {
            false => VSWRST_A::B_0X0,
            true => VSWRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VSWRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VSWRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `VSWRST`"]
pub struct VSWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> VSWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VSWRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset not activated (default after backup domain reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VSWRST_A::B_0X0)
    }
    #[doc = "Resets the entire VSW domain"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VSWRST_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LSEON"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSEBYP"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LSERDY"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - LSEDRV"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - LSECSSON"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LSECSSD"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - RTCSRC"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RTCSRC_R {
        RTCSRC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 20 - RTCCKEN"]
    #[inline(always)]
    pub fn rtccken(&self) -> RTCCKEN_R {
        RTCCKEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 31 - VSWRST"]
    #[inline(always)]
    pub fn vswrst(&self) -> VSWRST_R {
        VSWRST_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSEON"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W {
        LSEON_W { w: self }
    }
    #[doc = "Bit 1 - LSEBYP"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W {
        LSEBYP_W { w: self }
    }
    #[doc = "Bits 4:5 - LSEDRV"]
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LSEDRV_W {
        LSEDRV_W { w: self }
    }
    #[doc = "Bit 8 - LSECSSON"]
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LSECSSON_W {
        LSECSSON_W { w: self }
    }
    #[doc = "Bits 16:17 - RTCSRC"]
    #[inline(always)]
    pub fn rtcsrc(&mut self) -> RTCSRC_W {
        RTCSRC_W { w: self }
    }
    #[doc = "Bit 20 - RTCCKEN"]
    #[inline(always)]
    pub fn rtccken(&mut self) -> RTCCKEN_W {
        RTCCKEN_W { w: self }
    }
    #[doc = "Bit 31 - VSWRST"]
    #[inline(always)]
    pub fn vswrst(&mut self) -> VSWRST_W {
        VSWRST_W { w: self }
    }
}
