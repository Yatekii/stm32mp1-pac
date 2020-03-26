#[doc = "Reader of register MDMA_C15TBR"]
pub type R = crate::R<u32, super::MDMA_C15TBR>;
#[doc = "Writer for register MDMA_C15TBR"]
pub type W = crate::W<u32, super::MDMA_C15TBR>;
#[doc = "Register MDMA_C15TBR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C15TBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSEL`"]
pub type TSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSEL`"]
pub struct TSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "SBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBUS_A {
    #[doc = "0: The system/AXI bus is used as source\r\n                  (read operation) on channel x."]
    B_0X0 = 0,
    #[doc = "1: The AHB bus/TCM is used as source\r\n                  (read operation) on channel x."]
    B_0X1 = 1,
}
impl From<SBUS_A> for bool {
    #[inline(always)]
    fn from(variant: SBUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBUS`"]
pub type SBUS_R = crate::R<bool, SBUS_A>;
impl SBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBUS_A {
        match self.bits {
            false => SBUS_A::B_0X0,
            true => SBUS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SBUS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SBUS_A::B_0X1
    }
}
#[doc = "Write proxy for field `SBUS`"]
pub struct SBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The system/AXI bus is used as source (read operation) on channel x."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SBUS_A::B_0X0)
    }
    #[doc = "The AHB bus/TCM is used as source (read operation) on channel x."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SBUS_A::B_0X1)
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
#[doc = "DBUS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBUS_A {
    #[doc = "0: The system/AXI bus is used as\r\n                  destination (write operation) on channel\r\n                  x."]
    B_0X0 = 0,
    #[doc = "1: The AHB bus/TCM is used as\r\n                  destination (write operation) on channel\r\n                  x."]
    B_0X1 = 1,
}
impl From<DBUS_A> for bool {
    #[inline(always)]
    fn from(variant: DBUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBUS`"]
pub type DBUS_R = crate::R<bool, DBUS_A>;
impl DBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBUS_A {
        match self.bits {
            false => DBUS_A::B_0X0,
            true => DBUS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBUS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBUS_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBUS`"]
pub struct DBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The system/AXI bus is used as destination (write operation) on channel x."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBUS_A::B_0X0)
    }
    #[doc = "The AHB bus/TCM is used as destination (write operation) on channel x."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBUS_A::B_0X1)
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
impl R {
    #[doc = "Bits 0:5 - TSEL"]
    #[inline(always)]
    pub fn tsel(&self) -> TSEL_R {
        TSEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SBUS"]
    #[inline(always)]
    pub fn sbus(&self) -> SBUS_R {
        SBUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DBUS"]
    #[inline(always)]
    pub fn dbus(&self) -> DBUS_R {
        DBUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - TSEL"]
    #[inline(always)]
    pub fn tsel(&mut self) -> TSEL_W {
        TSEL_W { w: self }
    }
    #[doc = "Bit 16 - SBUS"]
    #[inline(always)]
    pub fn sbus(&mut self) -> SBUS_W {
        SBUS_W { w: self }
    }
    #[doc = "Bit 17 - DBUS"]
    #[inline(always)]
    pub fn dbus(&mut self) -> DBUS_W {
        DBUS_W { w: self }
    }
}
