#[doc = "Reader of register DFSDM_FLT2CR2"]
pub type R = crate::R<u32, super::DFSDM_FLT2CR2>;
#[doc = "Writer for register DFSDM_FLT2CR2"]
pub type W = crate::W<u32, super::DFSDM_FLT2CR2>;
#[doc = "Register DFSDM_FLT2CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFSDM_FLT2CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "JEOCIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOCIE_A {
    #[doc = "0: Injected end of conversion interrupt\r\n                  is disabled"]
    B_0X0 = 0,
    #[doc = "1: Injected end of conversion interrupt\r\n                  is enabled"]
    B_0X1 = 1,
}
impl From<JEOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JEOCIE`"]
pub type JEOCIE_R = crate::R<bool, JEOCIE_A>;
impl JEOCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOCIE_A {
        match self.bits {
            false => JEOCIE_A::B_0X0,
            true => JEOCIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == JEOCIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == JEOCIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `JEOCIE`"]
pub struct JEOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JEOCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEOCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Injected end of conversion interrupt is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JEOCIE_A::B_0X0)
    }
    #[doc = "Injected end of conversion interrupt is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JEOCIE_A::B_0X1)
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
#[doc = "REOCIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REOCIE_A {
    #[doc = "0: Regular end of conversion interrupt\r\n                  is disabled"]
    B_0X0 = 0,
    #[doc = "1: Regular end of conversion interrupt\r\n                  is enabled"]
    B_0X1 = 1,
}
impl From<REOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: REOCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REOCIE`"]
pub type REOCIE_R = crate::R<bool, REOCIE_A>;
impl REOCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REOCIE_A {
        match self.bits {
            false => REOCIE_A::B_0X0,
            true => REOCIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REOCIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REOCIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `REOCIE`"]
pub struct REOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REOCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REOCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Regular end of conversion interrupt is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(REOCIE_A::B_0X0)
    }
    #[doc = "Regular end of conversion interrupt is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(REOCIE_A::B_0X1)
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
#[doc = "JOVRIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JOVRIE_A {
    #[doc = "0: Injected data overrun interrupt is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Injected data overrun interrupt is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<JOVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: JOVRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JOVRIE`"]
pub type JOVRIE_R = crate::R<bool, JOVRIE_A>;
impl JOVRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JOVRIE_A {
        match self.bits {
            false => JOVRIE_A::B_0X0,
            true => JOVRIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == JOVRIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == JOVRIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `JOVRIE`"]
pub struct JOVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> JOVRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JOVRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Injected data overrun interrupt is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JOVRIE_A::B_0X0)
    }
    #[doc = "Injected data overrun interrupt is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JOVRIE_A::B_0X1)
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
#[doc = "ROVRIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVRIE_A {
    #[doc = "0: Regular data overrun interrupt is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Regular data overrun interrupt is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<ROVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ROVRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROVRIE`"]
pub type ROVRIE_R = crate::R<bool, ROVRIE_A>;
impl ROVRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVRIE_A {
        match self.bits {
            false => ROVRIE_A::B_0X0,
            true => ROVRIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ROVRIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ROVRIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `ROVRIE`"]
pub struct ROVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROVRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Regular data overrun interrupt is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ROVRIE_A::B_0X0)
    }
    #[doc = "Regular data overrun interrupt is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ROVRIE_A::B_0X1)
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
#[doc = "AWDIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDIE_A {
    #[doc = "0: Analog watchdog interrupt is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Analog watchdog interrupt is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<AWDIE_A> for bool {
    #[inline(always)]
    fn from(variant: AWDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWDIE`"]
pub type AWDIE_R = crate::R<bool, AWDIE_A>;
impl AWDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDIE_A {
        match self.bits {
            false => AWDIE_A::B_0X0,
            true => AWDIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWDIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWDIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `AWDIE`"]
pub struct AWDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Analog watchdog interrupt is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AWDIE_A::B_0X0)
    }
    #[doc = "Analog watchdog interrupt is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AWDIE_A::B_0X1)
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
#[doc = "SCDIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCDIE_A {
    #[doc = "0: short-circuit detector interrupt is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: short-circuit detector interrupt is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<SCDIE_A> for bool {
    #[inline(always)]
    fn from(variant: SCDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCDIE`"]
pub type SCDIE_R = crate::R<bool, SCDIE_A>;
impl SCDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCDIE_A {
        match self.bits {
            false => SCDIE_A::B_0X0,
            true => SCDIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SCDIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SCDIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `SCDIE`"]
pub struct SCDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "short-circuit detector interrupt is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SCDIE_A::B_0X0)
    }
    #[doc = "short-circuit detector interrupt is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SCDIE_A::B_0X1)
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
#[doc = "CKABIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKABIE_A {
    #[doc = "0: Detection of channel input clock\r\n                  absence interrupt is disabled"]
    B_0X0 = 0,
    #[doc = "1: Detection of channel input clock\r\n                  absence interrupt is enabled"]
    B_0X1 = 1,
}
impl From<CKABIE_A> for bool {
    #[inline(always)]
    fn from(variant: CKABIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKABIE`"]
pub type CKABIE_R = crate::R<bool, CKABIE_A>;
impl CKABIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKABIE_A {
        match self.bits {
            false => CKABIE_A::B_0X0,
            true => CKABIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CKABIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CKABIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `CKABIE`"]
pub struct CKABIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKABIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKABIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Detection of channel input clock absence interrupt is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKABIE_A::B_0X0)
    }
    #[doc = "Detection of channel input clock absence interrupt is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CKABIE_A::B_0X1)
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
#[doc = "Reader of field `EXCH`"]
pub type EXCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXCH`"]
pub struct EXCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `AWDCH`"]
pub type AWDCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AWDCH`"]
pub struct AWDCH_W<'a> {
    w: &'a mut W,
}
impl<'a> AWDCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - JEOCIE"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - REOCIE"]
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - JOVRIE"]
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ROVRIE"]
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AWDIE"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SCDIE"]
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CKABIE"]
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - EXCH"]
    #[inline(always)]
    pub fn exch(&self) -> EXCH_R {
        EXCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - AWDCH"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - JEOCIE"]
    #[inline(always)]
    pub fn jeocie(&mut self) -> JEOCIE_W {
        JEOCIE_W { w: self }
    }
    #[doc = "Bit 1 - REOCIE"]
    #[inline(always)]
    pub fn reocie(&mut self) -> REOCIE_W {
        REOCIE_W { w: self }
    }
    #[doc = "Bit 2 - JOVRIE"]
    #[inline(always)]
    pub fn jovrie(&mut self) -> JOVRIE_W {
        JOVRIE_W { w: self }
    }
    #[doc = "Bit 3 - ROVRIE"]
    #[inline(always)]
    pub fn rovrie(&mut self) -> ROVRIE_W {
        ROVRIE_W { w: self }
    }
    #[doc = "Bit 4 - AWDIE"]
    #[inline(always)]
    pub fn awdie(&mut self) -> AWDIE_W {
        AWDIE_W { w: self }
    }
    #[doc = "Bit 5 - SCDIE"]
    #[inline(always)]
    pub fn scdie(&mut self) -> SCDIE_W {
        SCDIE_W { w: self }
    }
    #[doc = "Bit 6 - CKABIE"]
    #[inline(always)]
    pub fn ckabie(&mut self) -> CKABIE_W {
        CKABIE_W { w: self }
    }
    #[doc = "Bits 8:15 - EXCH"]
    #[inline(always)]
    pub fn exch(&mut self) -> EXCH_W {
        EXCH_W { w: self }
    }
    #[doc = "Bits 16:23 - AWDCH"]
    #[inline(always)]
    pub fn awdch(&mut self) -> AWDCH_W {
        AWDCH_W { w: self }
    }
}
