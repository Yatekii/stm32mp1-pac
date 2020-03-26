#[doc = "Reader of register FDCAN_CCCR"]
pub type R = crate::R<u32, super::FDCAN_CCCR>;
#[doc = "Writer for register FDCAN_CCCR"]
pub type W = crate::W<u32, super::FDCAN_CCCR>;
#[doc = "Register FDCAN_CCCR `reset()`'s with value 0x01"]
impl crate::ResetValue for super::FDCAN_CCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "INIT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: Normal Operation"]
    B_0X0 = 0,
    #[doc = "1: Initialization is\r\n                  started"]
    B_0X1 = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INIT`"]
pub type INIT_R = crate::R<bool, INIT_A>;
impl INIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::B_0X0,
            true => INIT_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == INIT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == INIT_A::B_0X1
    }
}
#[doc = "Write proxy for field `INIT`"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(INIT_A::B_0X0)
    }
    #[doc = "Initialization is started"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(INIT_A::B_0X1)
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
#[doc = "CCE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCE_A {
    #[doc = "0: The CPU has no write access to the\r\n                  protected configuration registers"]
    B_0X0 = 0,
    #[doc = "1: The CPU has write access to the\r\n                  protected configuration registers (while\r\n                  CCCR.INIT = 1 )"]
    B_0X1 = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCE`"]
pub type CCE_R = crate::R<bool, CCE_A>;
impl CCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::B_0X0,
            true => CCE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CCE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CCE_A::B_0X1
    }
}
#[doc = "Write proxy for field `CCE`"]
pub struct CCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CPU has no write access to the protected configuration registers"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CCE_A::B_0X0)
    }
    #[doc = "The CPU has write access to the protected configuration registers (while CCCR.INIT = 1 )"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CCE_A::B_0X1)
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
#[doc = "ASM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASM_A {
    #[doc = "0: Normal CAN operation"]
    B_0X0 = 0,
    #[doc = "1: Restricted Operation Mode\r\n                  active"]
    B_0X1 = 1,
}
impl From<ASM_A> for bool {
    #[inline(always)]
    fn from(variant: ASM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASM`"]
pub type ASM_R = crate::R<bool, ASM_A>;
impl ASM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASM_A {
        match self.bits {
            false => ASM_A::B_0X0,
            true => ASM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ASM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ASM_A::B_0X1
    }
}
#[doc = "Write proxy for field `ASM`"]
pub struct ASM_W<'a> {
    w: &'a mut W,
}
impl<'a> ASM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal CAN operation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ASM_A::B_0X0)
    }
    #[doc = "Restricted Operation Mode active"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ASM_A::B_0X1)
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
#[doc = "CSA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSA_A {
    #[doc = "0: No clock stop\r\n                  acknowledged"]
    B_0X0 = 0,
    #[doc = "1: FDCAN may be set in power down by\r\n                  stopping APB clock and kernel clock"]
    B_0X1 = 1,
}
impl From<CSA_A> for bool {
    #[inline(always)]
    fn from(variant: CSA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSA`"]
pub type CSA_R = crate::R<bool, CSA_A>;
impl CSA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSA_A {
        match self.bits {
            false => CSA_A::B_0X0,
            true => CSA_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSA_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSA_A::B_0X1
    }
}
#[doc = "Write proxy for field `CSA`"]
pub struct CSA_W<'a> {
    w: &'a mut W,
}
impl<'a> CSA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock stop acknowledged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CSA_A::B_0X0)
    }
    #[doc = "FDCAN may be set in power down by stopping APB clock and kernel clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CSA_A::B_0X1)
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
#[doc = "CSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSR_A {
    #[doc = "0: No clock stop is\r\n                  requested"]
    B_0X0 = 0,
    #[doc = "1: Clock stop requested. When clock\r\n                  stop is requested, first INIT and then CSA will\r\n                  be set"]
    B_0X1 = 1,
}
impl From<CSR_A> for bool {
    #[inline(always)]
    fn from(variant: CSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSR`"]
pub type CSR_R = crate::R<bool, CSR_A>;
impl CSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSR_A {
        match self.bits {
            false => CSR_A::B_0X0,
            true => CSR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSR_A::B_0X1
    }
}
#[doc = "Write proxy for field `CSR`"]
pub struct CSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock stop is requested"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CSR_A::B_0X0)
    }
    #[doc = "Clock stop requested. When clock stop is requested, first INIT and then CSA will be set"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CSR_A::B_0X1)
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
#[doc = "MON\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MON_A {
    #[doc = "0: Bus Monitoring Mode is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Bus Monitoring Mode is\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<MON_A> for bool {
    #[inline(always)]
    fn from(variant: MON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MON`"]
pub type MON_R = crate::R<bool, MON_A>;
impl MON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MON_A {
        match self.bits {
            false => MON_A::B_0X0,
            true => MON_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MON_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MON_A::B_0X1
    }
}
#[doc = "Write proxy for field `MON`"]
pub struct MON_W<'a> {
    w: &'a mut W,
}
impl<'a> MON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bus Monitoring Mode is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MON_A::B_0X0)
    }
    #[doc = "Bus Monitoring Mode is enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MON_A::B_0X1)
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
#[doc = "DAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAR_A {
    #[doc = "0: Automatic retransmission of messages\r\n                  not transmitted successfully\r\n                  enabled"]
    B_0X0 = 0,
    #[doc = "1: Automatic retransmission\r\n                  disabled"]
    B_0X1 = 1,
}
impl From<DAR_A> for bool {
    #[inline(always)]
    fn from(variant: DAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DAR`"]
pub type DAR_R = crate::R<bool, DAR_A>;
impl DAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAR_A {
        match self.bits {
            false => DAR_A::B_0X0,
            true => DAR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DAR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DAR_A::B_0X1
    }
}
#[doc = "Write proxy for field `DAR`"]
pub struct DAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Automatic retransmission of messages not transmitted successfully enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DAR_A::B_0X0)
    }
    #[doc = "Automatic retransmission disabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DAR_A::B_0X1)
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
#[doc = "TEST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEST_A {
    #[doc = "0: Normal operation, register TEST\r\n                  holds reset values"]
    B_0X0 = 0,
    #[doc = "1: Test Mode, write access to register\r\n                  TEST enabled"]
    B_0X1 = 1,
}
impl From<TEST_A> for bool {
    #[inline(always)]
    fn from(variant: TEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEST`"]
pub type TEST_R = crate::R<bool, TEST_A>;
impl TEST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEST_A {
        match self.bits {
            false => TEST_A::B_0X0,
            true => TEST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEST_A::B_0X1
    }
}
#[doc = "Write proxy for field `TEST`"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation, register TEST holds reset values"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEST_A::B_0X0)
    }
    #[doc = "Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEST_A::B_0X1)
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
#[doc = "FDOE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDOE_A {
    #[doc = "0: FD operation disabled"]
    B_0X0 = 0,
    #[doc = "1: FD operation enabled"]
    B_0X1 = 1,
}
impl From<FDOE_A> for bool {
    #[inline(always)]
    fn from(variant: FDOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDOE`"]
pub type FDOE_R = crate::R<bool, FDOE_A>;
impl FDOE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDOE_A {
        match self.bits {
            false => FDOE_A::B_0X0,
            true => FDOE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FDOE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FDOE_A::B_0X1
    }
}
#[doc = "Write proxy for field `FDOE`"]
pub struct FDOE_W<'a> {
    w: &'a mut W,
}
impl<'a> FDOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDOE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FD operation disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FDOE_A::B_0X0)
    }
    #[doc = "FD operation enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FDOE_A::B_0X1)
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
#[doc = "BRSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRSE_A {
    #[doc = "0: Bit rate switching for transmissions\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Bit rate switching for transmissions\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<BRSE_A> for bool {
    #[inline(always)]
    fn from(variant: BRSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRSE`"]
pub type BRSE_R = crate::R<bool, BRSE_A>;
impl BRSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRSE_A {
        match self.bits {
            false => BRSE_A::B_0X0,
            true => BRSE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRSE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRSE_A::B_0X1
    }
}
#[doc = "Write proxy for field `BRSE`"]
pub struct BRSE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bit rate switching for transmissions disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BRSE_A::B_0X0)
    }
    #[doc = "Bit rate switching for transmissions enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BRSE_A::B_0X1)
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
#[doc = "PXHD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PXHD_A {
    #[doc = "0: Protocol exception handling\r\n                  enabled"]
    B_0X0 = 0,
    #[doc = "1: Protocol exception handling\r\n                  disabled"]
    B_0X1 = 1,
}
impl From<PXHD_A> for bool {
    #[inline(always)]
    fn from(variant: PXHD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PXHD`"]
pub type PXHD_R = crate::R<bool, PXHD_A>;
impl PXHD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PXHD_A {
        match self.bits {
            false => PXHD_A::B_0X0,
            true => PXHD_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PXHD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PXHD_A::B_0X1
    }
}
#[doc = "Write proxy for field `PXHD`"]
pub struct PXHD_W<'a> {
    w: &'a mut W,
}
impl<'a> PXHD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PXHD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protocol exception handling enabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PXHD_A::B_0X0)
    }
    #[doc = "Protocol exception handling disabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PXHD_A::B_0X1)
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
#[doc = "EFBI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFBI_A {
    #[doc = "0: Edge filtering\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Two consecutive dominant tq required\r\n                  to detect an edge for hard\r\n                  synchronization"]
    B_0X1 = 1,
}
impl From<EFBI_A> for bool {
    #[inline(always)]
    fn from(variant: EFBI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EFBI`"]
pub type EFBI_R = crate::R<bool, EFBI_A>;
impl EFBI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFBI_A {
        match self.bits {
            false => EFBI_A::B_0X0,
            true => EFBI_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EFBI_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EFBI_A::B_0X1
    }
}
#[doc = "Write proxy for field `EFBI`"]
pub struct EFBI_W<'a> {
    w: &'a mut W,
}
impl<'a> EFBI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EFBI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Edge filtering disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EFBI_A::B_0X0)
    }
    #[doc = "Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EFBI_A::B_0X1)
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
#[doc = "TXP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXP_A {
    #[doc = "0: disabled"]
    B_0X0 = 0,
    #[doc = "1: enabled"]
    B_0X1 = 1,
}
impl From<TXP_A> for bool {
    #[inline(always)]
    fn from(variant: TXP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXP`"]
pub type TXP_R = crate::R<bool, TXP_A>;
impl TXP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXP_A {
        match self.bits {
            false => TXP_A::B_0X0,
            true => TXP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXP_A::B_0X1
    }
}
#[doc = "Write proxy for field `TXP`"]
pub struct TXP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TXP_A::B_0X0)
    }
    #[doc = "enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TXP_A::B_0X1)
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
#[doc = "NISO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NISO_A {
    #[doc = "0: CAN FD frame format according to\r\n                  ISO11898-1"]
    B_0X0 = 0,
    #[doc = "1: CAN FD frame format according to\r\n                  Bosch CAN FD Specification V1.0"]
    B_0X1 = 1,
}
impl From<NISO_A> for bool {
    #[inline(always)]
    fn from(variant: NISO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NISO`"]
pub type NISO_R = crate::R<bool, NISO_A>;
impl NISO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NISO_A {
        match self.bits {
            false => NISO_A::B_0X0,
            true => NISO_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NISO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NISO_A::B_0X1
    }
}
#[doc = "Write proxy for field `NISO`"]
pub struct NISO_W<'a> {
    w: &'a mut W,
}
impl<'a> NISO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NISO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAN FD frame format according to ISO11898-1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(NISO_A::B_0X0)
    }
    #[doc = "CAN FD frame format according to Bosch CAN FD Specification V1.0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(NISO_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - INIT"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CCE"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ASM"]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CSA"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSR"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MON"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DAR"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TEST"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FDOE"]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BRSE"]
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PXHD"]
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EFBI"]
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TXP"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - NISO"]
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INIT"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 1 - CCE"]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W { w: self }
    }
    #[doc = "Bit 2 - ASM"]
    #[inline(always)]
    pub fn asm(&mut self) -> ASM_W {
        ASM_W { w: self }
    }
    #[doc = "Bit 3 - CSA"]
    #[inline(always)]
    pub fn csa(&mut self) -> CSA_W {
        CSA_W { w: self }
    }
    #[doc = "Bit 4 - CSR"]
    #[inline(always)]
    pub fn csr(&mut self) -> CSR_W {
        CSR_W { w: self }
    }
    #[doc = "Bit 5 - MON"]
    #[inline(always)]
    pub fn mon(&mut self) -> MON_W {
        MON_W { w: self }
    }
    #[doc = "Bit 6 - DAR"]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W {
        DAR_W { w: self }
    }
    #[doc = "Bit 7 - TEST"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    #[doc = "Bit 8 - FDOE"]
    #[inline(always)]
    pub fn fdoe(&mut self) -> FDOE_W {
        FDOE_W { w: self }
    }
    #[doc = "Bit 9 - BRSE"]
    #[inline(always)]
    pub fn brse(&mut self) -> BRSE_W {
        BRSE_W { w: self }
    }
    #[doc = "Bit 12 - PXHD"]
    #[inline(always)]
    pub fn pxhd(&mut self) -> PXHD_W {
        PXHD_W { w: self }
    }
    #[doc = "Bit 13 - EFBI"]
    #[inline(always)]
    pub fn efbi(&mut self) -> EFBI_W {
        EFBI_W { w: self }
    }
    #[doc = "Bit 14 - TXP"]
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W {
        TXP_W { w: self }
    }
    #[doc = "Bit 15 - NISO"]
    #[inline(always)]
    pub fn niso(&mut self) -> NISO_W {
        NISO_W { w: self }
    }
}
