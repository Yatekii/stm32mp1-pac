#[doc = "Reader of register RCC_DBGCFGR"]
pub type R = crate::R<u32, super::RCC_DBGCFGR>;
#[doc = "Writer for register RCC_DBGCFGR"]
pub type W = crate::W<u32, super::RCC_DBGCFGR>;
#[doc = "Register RCC_DBGCFGR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RCC_DBGCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "TRACEDIV\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRACEDIV_A {
    #[doc = "0: aclk"]
    B_0X0 = 0,
    #[doc = "1: aclk / 2 (default after\r\n                  reset)"]
    B_0X1 = 1,
    #[doc = "2: aclk / 4"]
    B_0X2 = 2,
    #[doc = "3: aclk / 8"]
    B_0X3 = 3,
}
impl From<TRACEDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACEDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRACEDIV`"]
pub type TRACEDIV_R = crate::R<u8, TRACEDIV_A>;
impl TRACEDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRACEDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRACEDIV_A::B_0X0),
            1 => Val(TRACEDIV_A::B_0X1),
            2 => Val(TRACEDIV_A::B_0X2),
            3 => Val(TRACEDIV_A::B_0X3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TRACEDIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TRACEDIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TRACEDIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TRACEDIV_A::B_0X3
    }
}
#[doc = "Write proxy for field `TRACEDIV`"]
pub struct TRACEDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACEDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACEDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "aclk"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TRACEDIV_A::B_0X0)
    }
    #[doc = "aclk / 2 (default after reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TRACEDIV_A::B_0X1)
    }
    #[doc = "aclk / 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TRACEDIV_A::B_0X2)
    }
    #[doc = "aclk / 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TRACEDIV_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "DBGCKEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGCKEN_A {
    #[doc = "0: The enabling of the clock for the\r\n                  debug function is controlled by cdbgwrupreq\r\n                  signal from DAP. (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: The clock for the debug function is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<DBGCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBGCKEN`"]
pub type DBGCKEN_R = crate::R<bool, DBGCKEN_A>;
impl DBGCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGCKEN_A {
        match self.bits {
            false => DBGCKEN_A::B_0X0,
            true => DBGCKEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBGCKEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBGCKEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBGCKEN`"]
pub struct DBGCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGCKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The enabling of the clock for the debug function is controlled by cdbgwrupreq signal from DAP. (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBGCKEN_A::B_0X0)
    }
    #[doc = "The clock for the debug function is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBGCKEN_A::B_0X1)
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
#[doc = "TRACECKEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECKEN_A {
    #[doc = "0: The clock for the trace function is\r\n                  disabled (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: The clock for the trace function is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<TRACECKEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRACECKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRACECKEN`"]
pub type TRACECKEN_R = crate::R<bool, TRACECKEN_A>;
impl TRACECKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACECKEN_A {
        match self.bits {
            false => TRACECKEN_A::B_0X0,
            true => TRACECKEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TRACECKEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TRACECKEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TRACECKEN`"]
pub struct TRACECKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACECKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The clock for the trace function is disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TRACECKEN_A::B_0X0)
    }
    #[doc = "The clock for the trace function is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TRACECKEN_A::B_0X1)
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
#[doc = "DBGRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGRST_A {
    #[doc = "0: The trace and debug parts are not\r\n                  reset. (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: The trace and debug parts are under\r\n                  reset."]
    B_0X1 = 1,
}
impl From<DBGRST_A> for bool {
    #[inline(always)]
    fn from(variant: DBGRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBGRST`"]
pub type DBGRST_R = crate::R<bool, DBGRST_A>;
impl DBGRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGRST_A {
        match self.bits {
            false => DBGRST_A::B_0X0,
            true => DBGRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBGRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DBGRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DBGRST`"]
pub struct DBGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBGRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The trace and debug parts are not reset. (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBGRST_A::B_0X0)
    }
    #[doc = "The trace and debug parts are under reset."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DBGRST_A::B_0X1)
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
    #[doc = "Bits 0:2 - TRACEDIV"]
    #[inline(always)]
    pub fn tracediv(&self) -> TRACEDIV_R {
        TRACEDIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - DBGCKEN"]
    #[inline(always)]
    pub fn dbgcken(&self) -> DBGCKEN_R {
        DBGCKEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TRACECKEN"]
    #[inline(always)]
    pub fn tracecken(&self) -> TRACECKEN_R {
        TRACECKEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DBGRST"]
    #[inline(always)]
    pub fn dbgrst(&self) -> DBGRST_R {
        DBGRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TRACEDIV"]
    #[inline(always)]
    pub fn tracediv(&mut self) -> TRACEDIV_W {
        TRACEDIV_W { w: self }
    }
    #[doc = "Bit 8 - DBGCKEN"]
    #[inline(always)]
    pub fn dbgcken(&mut self) -> DBGCKEN_W {
        DBGCKEN_W { w: self }
    }
    #[doc = "Bit 9 - TRACECKEN"]
    #[inline(always)]
    pub fn tracecken(&mut self) -> TRACECKEN_W {
        TRACECKEN_W { w: self }
    }
    #[doc = "Bit 12 - DBGRST"]
    #[inline(always)]
    pub fn dbgrst(&mut self) -> DBGRST_W {
        DBGRST_W { w: self }
    }
}
