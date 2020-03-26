#[doc = "Reader of register RCC_MP_APB4ENSETR"]
pub type R = crate::R<u32, super::RCC_MP_APB4ENSETR>;
#[doc = "Writer for register RCC_MP_APB4ENSETR"]
pub type W = crate::W<u32, super::RCC_MP_APB4ENSETR>;
#[doc = "Register RCC_MP_APB4ENSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_APB4ENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LTDCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTDCEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<LTDCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LTDCEN`"]
pub type LTDCEN_R = crate::R<bool, LTDCEN_A>;
impl LTDCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTDCEN_A {
        match self.bits {
            false => LTDCEN_A::B_0X0,
            true => LTDCEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LTDCEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LTDCEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `LTDCEN`"]
pub struct LTDCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LTDCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LTDCEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LTDCEN_A::B_0X1)
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
#[doc = "DSIEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSIEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<DSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DSIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSIEN`"]
pub type DSIEN_R = crate::R<bool, DSIEN_A>;
impl DSIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSIEN_A {
        match self.bits {
            false => DSIEN_A::B_0X0,
            true => DSIEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DSIEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DSIEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DSIEN`"]
pub struct DSIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DSIEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DSIEN_A::B_0X1)
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
#[doc = "DDRPERFMEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRPERFMEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the APB clock is disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the APB clock,\r\n                  reading means that the APB clock is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<DDRPERFMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DDRPERFMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRPERFMEN`"]
pub type DDRPERFMEN_R = crate::R<bool, DDRPERFMEN_A>;
impl DDRPERFMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRPERFMEN_A {
        match self.bits {
            false => DDRPERFMEN_A::B_0X0,
            true => DDRPERFMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRPERFMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRPERFMEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRPERFMEN`"]
pub struct DDRPERFMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPERFMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRPERFMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the APB clock is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRPERFMEN_A::B_0X0)
    }
    #[doc = "Writing enables the APB clock, reading means that the APB clock is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRPERFMEN_A::B_0X1)
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
#[doc = "IWDG2APBEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDG2APBEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the APB clock is disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the APB clock,\r\n                  reading means that the APB clock is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<IWDG2APBEN_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG2APBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IWDG2APBEN`"]
pub type IWDG2APBEN_R = crate::R<bool, IWDG2APBEN_A>;
impl IWDG2APBEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDG2APBEN_A {
        match self.bits {
            false => IWDG2APBEN_A::B_0X0,
            true => IWDG2APBEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IWDG2APBEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IWDG2APBEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `IWDG2APBEN`"]
pub struct IWDG2APBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG2APBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IWDG2APBEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the APB clock is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IWDG2APBEN_A::B_0X0)
    }
    #[doc = "Writing enables the APB clock, reading means that the APB clock is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IWDG2APBEN_A::B_0X1)
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
#[doc = "USBPHYEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPHYEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<USBPHYEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBPHYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBPHYEN`"]
pub type USBPHYEN_R = crate::R<bool, USBPHYEN_A>;
impl USBPHYEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPHYEN_A {
        match self.bits {
            false => USBPHYEN_A::B_0X0,
            true => USBPHYEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBPHYEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBPHYEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `USBPHYEN`"]
pub struct USBPHYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPHYEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBPHYEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBPHYEN_A::B_0X1)
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
#[doc = "STGENROEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STGENROEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Writing enables the peripheral\r\n                  clocks, reading means that the peripheral clocks\r\n                  are enabled"]
    B_0X1 = 1,
}
impl From<STGENROEN_A> for bool {
    #[inline(always)]
    fn from(variant: STGENROEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STGENROEN`"]
pub type STGENROEN_R = crate::R<bool, STGENROEN_A>;
impl STGENROEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STGENROEN_A {
        match self.bits {
            false => STGENROEN_A::B_0X0,
            true => STGENROEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == STGENROEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == STGENROEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `STGENROEN`"]
pub struct STGENROEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENROEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STGENROEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(STGENROEN_A::B_0X0)
    }
    #[doc = "Writing enables the peripheral clocks, reading means that the peripheral clocks are enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(STGENROEN_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - LTDCEN"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DSIEN"]
    #[inline(always)]
    pub fn dsien(&self) -> DSIEN_R {
        DSIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMEN"]
    #[inline(always)]
    pub fn ddrperfmen(&self) -> DDRPERFMEN_R {
        DDRPERFMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IWDG2APBEN"]
    #[inline(always)]
    pub fn iwdg2apben(&self) -> IWDG2APBEN_R {
        IWDG2APBEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USBPHYEN"]
    #[inline(always)]
    pub fn usbphyen(&self) -> USBPHYEN_R {
        USBPHYEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENROEN"]
    #[inline(always)]
    pub fn stgenroen(&self) -> STGENROEN_R {
        STGENROEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCEN"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W {
        LTDCEN_W { w: self }
    }
    #[doc = "Bit 4 - DSIEN"]
    #[inline(always)]
    pub fn dsien(&mut self) -> DSIEN_W {
        DSIEN_W { w: self }
    }
    #[doc = "Bit 8 - DDRPERFMEN"]
    #[inline(always)]
    pub fn ddrperfmen(&mut self) -> DDRPERFMEN_W {
        DDRPERFMEN_W { w: self }
    }
    #[doc = "Bit 15 - IWDG2APBEN"]
    #[inline(always)]
    pub fn iwdg2apben(&mut self) -> IWDG2APBEN_W {
        IWDG2APBEN_W { w: self }
    }
    #[doc = "Bit 16 - USBPHYEN"]
    #[inline(always)]
    pub fn usbphyen(&mut self) -> USBPHYEN_W {
        USBPHYEN_W { w: self }
    }
    #[doc = "Bit 20 - STGENROEN"]
    #[inline(always)]
    pub fn stgenroen(&mut self) -> STGENROEN_W {
        STGENROEN_W { w: self }
    }
}
