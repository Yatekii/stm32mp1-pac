#[doc = "Reader of register FDCAN_PSR"]
pub type R = crate::R<u32, super::FDCAN_PSR>;
#[doc = "Writer for register FDCAN_PSR"]
pub type W = crate::W<u32, super::FDCAN_PSR>;
#[doc = "Register FDCAN_PSR `reset()`'s with value 0x0707"]
impl crate::ResetValue for super::FDCAN_PSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0707
    }
}
#[doc = "LEC\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEC_A {
    #[doc = "0: No Error: No error occurred since\r\n                  LEC has been reset by successful reception or\r\n                  transmission."]
    B_0X0 = 0,
    #[doc = "1: Stuff Error: More than 5 equal bits\r\n                  in a sequence have occurred in a part of a\r\n                  received message where this is not\r\n                  allowed."]
    B_0X1 = 1,
    #[doc = "2: Form Error: A fixed format part of a\r\n                  received frame has the wrong\r\n                  format."]
    B_0X2 = 2,
    #[doc = "3: AckError: The message transmitted by\r\n                  the FDCAN was not acknowledged by another\r\n                  node."]
    B_0X3 = 3,
    #[doc = "4: Bit1Error: During the transmission\r\n                  of a message (with the exception of the\r\n                  arbitration field), the device wanted to send a\r\n                  recessive level (bit of logical value 1 ), but\r\n                  the monitored bus value was\r\n                  dominant."]
    B_0X4 = 4,
    #[doc = "5: Bit0Error: During the transmission\r\n                  of a message (or acknowledge bit, or active error\r\n                  flag, or overload flag), the device wanted to\r\n                  send a dominant level (data or identifier bit\r\n                  logical value 0 ), but the monitored bus value\r\n                  was recessive. During Bus_Off recovery this\r\n                  status is set each time a sequence of 11\r\n                  recessive bits has been monitored. This enables\r\n                  the CPU to monitor the proceeding of the Bus_Off\r\n                  recovery sequence (indicating the bus is not\r\n                  stuck at dominant or continuously\r\n                  disturbed)."]
    B_0X5 = 5,
    #[doc = "6: CRCError: The CRC check sum of a\r\n                  received message was incorrect. The CRC of an\r\n                  incoming message does not match with the CRC\r\n                  calculated from the received data."]
    B_0X6 = 6,
    #[doc = "7: NoChange: Any read access to the\r\n                  Protocol Status Register re-initializes the LEC\r\n                  to 7 . When the LEC shows the value 7 , no CAN\r\n                  bus event was detected since the last CPU read\r\n                  access to the Protocol Status\r\n                  Register."]
    B_0X7 = 7,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEC`"]
pub type LEC_R = crate::R<u8, LEC_A>;
impl LEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::B_0X0,
            1 => LEC_A::B_0X1,
            2 => LEC_A::B_0X2,
            3 => LEC_A::B_0X3,
            4 => LEC_A::B_0X4,
            5 => LEC_A::B_0X5,
            6 => LEC_A::B_0X6,
            7 => LEC_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LEC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LEC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == LEC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == LEC_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == LEC_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == LEC_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == LEC_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == LEC_A::B_0X7
    }
}
#[doc = "Write proxy for field `LEC`"]
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No Error: No error occurred since LEC has been reset by successful reception or transmission."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LEC_A::B_0X0)
    }
    #[doc = "Stuff Error: More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LEC_A::B_0X1)
    }
    #[doc = "Form Error: A fixed format part of a received frame has the wrong format."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(LEC_A::B_0X2)
    }
    #[doc = "AckError: The message transmitted by the FDCAN was not acknowledged by another node."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(LEC_A::B_0X3)
    }
    #[doc = "Bit1Error: During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1 ), but the monitored bus value was dominant."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(LEC_A::B_0X4)
    }
    #[doc = "Bit0Error: During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0 ), but the monitored bus value was recessive. During Bus_Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus_Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(LEC_A::B_0X5)
    }
    #[doc = "CRCError: The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(LEC_A::B_0X6)
    }
    #[doc = "NoChange: Any read access to the Protocol Status Register re-initializes the LEC to 7 . When the LEC shows the value 7 , no CAN bus event was detected since the last CPU read access to the Protocol Status Register."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(LEC_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "ACT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACT_AW {
    #[doc = "0: Synchronizing: node is synchronizing\r\n                  on CAN communication"]
    B_0X0 = 0,
    #[doc = "1: Idle: node is neither receiver nor\r\n                  transmitter"]
    B_0X1 = 1,
    #[doc = "2: Receiver: node is operating as\r\n                  receiver"]
    B_0X2 = 2,
    #[doc = "3: Transmitter: node is operating as\r\n                  transmitter"]
    B_0X3 = 3,
}
impl From<ACT_AW> for u8 {
    #[inline(always)]
    fn from(variant: ACT_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `ACT`"]
pub struct ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACT_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Synchronizing: node is synchronizing on CAN communication"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ACT_AW::B_0X0)
    }
    #[doc = "Idle: node is neither receiver nor transmitter"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ACT_AW::B_0X1)
    }
    #[doc = "Receiver: node is operating as receiver"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ACT_AW::B_0X2)
    }
    #[doc = "Transmitter: node is operating as transmitter"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ACT_AW::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_A {
    #[doc = "0: The FDCAN is in the Error_Active\r\n                  state. It normally takes part in bus\r\n                  communication and sends an active error flag when\r\n                  an error has been detected"]
    B_0X0 = 0,
    #[doc = "1: The FDCAN is in the Error_Passive\r\n                  state"]
    B_0X1 = 1,
}
impl From<EP_A> for bool {
    #[inline(always)]
    fn from(variant: EP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP`"]
pub type EP_R = crate::R<bool, EP_A>;
impl EP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP_A {
        match self.bits {
            false => EP_A::B_0X0,
            true => EP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EP_A::B_0X1
    }
}
#[doc = "Write proxy for field `EP`"]
pub struct EP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The FDCAN is in the Error_Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EP_A::B_0X0)
    }
    #[doc = "The FDCAN is in the Error_Passive state"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EP_A::B_0X1)
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
#[doc = "EW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_A {
    #[doc = "0: Both error counters are below the\r\n                  Error_Warning limit of 96"]
    B_0X0 = 0,
    #[doc = "1: At least one of error counter has\r\n                  reached the Error_Warning limit of\r\n                  96"]
    B_0X1 = 1,
}
impl From<EW_A> for bool {
    #[inline(always)]
    fn from(variant: EW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EW`"]
pub type EW_R = crate::R<bool, EW_A>;
impl EW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EW_A {
        match self.bits {
            false => EW_A::B_0X0,
            true => EW_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EW_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EW_A::B_0X1
    }
}
#[doc = "Write proxy for field `EW`"]
pub struct EW_W<'a> {
    w: &'a mut W,
}
impl<'a> EW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Both error counters are below the Error_Warning limit of 96"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EW_A::B_0X0)
    }
    #[doc = "At least one of error counter has reached the Error_Warning limit of 96"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EW_A::B_0X1)
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
#[doc = "BO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BO_A {
    #[doc = "0: The FDCAN is not\r\n                  Bus_Off"]
    B_0X0 = 0,
    #[doc = "1: The FDCAN is in Bus_Off\r\n                  state"]
    B_0X1 = 1,
}
impl From<BO_A> for bool {
    #[inline(always)]
    fn from(variant: BO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BO`"]
pub type BO_R = crate::R<bool, BO_A>;
impl BO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BO_A {
        match self.bits {
            false => BO_A::B_0X0,
            true => BO_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BO_A::B_0X1
    }
}
#[doc = "Write proxy for field `BO`"]
pub struct BO_W<'a> {
    w: &'a mut W,
}
impl<'a> BO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The FDCAN is not Bus_Off"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BO_A::B_0X0)
    }
    #[doc = "The FDCAN is in Bus_Off state"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BO_A::B_0X1)
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
#[doc = "Write proxy for field `DLEC`"]
pub struct DLEC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "RESI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESI_A {
    #[doc = "0: Last received FDCAN message did not\r\n                  have its ESI flag set"]
    B_0X0 = 0,
    #[doc = "1: Last received FDCAN message had its\r\n                  ESI flag set"]
    B_0X1 = 1,
}
impl From<RESI_A> for bool {
    #[inline(always)]
    fn from(variant: RESI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESI`"]
pub type RESI_R = crate::R<bool, RESI_A>;
impl RESI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESI_A {
        match self.bits {
            false => RESI_A::B_0X0,
            true => RESI_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RESI_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RESI_A::B_0X1
    }
}
#[doc = "Write proxy for field `RESI`"]
pub struct RESI_W<'a> {
    w: &'a mut W,
}
impl<'a> RESI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Last received FDCAN message did not have its ESI flag set"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RESI_A::B_0X0)
    }
    #[doc = "Last received FDCAN message had its ESI flag set"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RESI_A::B_0X1)
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
#[doc = "RBRS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBRS_A {
    #[doc = "0: Last received FDCAN message did not\r\n                  ha ve its BRS flag set"]
    B_0X0 = 0,
    #[doc = "1: Last received FDCAN message had its\r\n                  BRS flag set"]
    B_0X1 = 1,
}
impl From<RBRS_A> for bool {
    #[inline(always)]
    fn from(variant: RBRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RBRS`"]
pub type RBRS_R = crate::R<bool, RBRS_A>;
impl RBRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBRS_A {
        match self.bits {
            false => RBRS_A::B_0X0,
            true => RBRS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RBRS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RBRS_A::B_0X1
    }
}
#[doc = "Write proxy for field `RBRS`"]
pub struct RBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBRS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Last received FDCAN message did not ha ve its BRS flag set"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RBRS_A::B_0X0)
    }
    #[doc = "Last received FDCAN message had its BRS flag set"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RBRS_A::B_0X1)
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
#[doc = "REDL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REDL_A {
    #[doc = "0: Since this bit was reset by the CPU,\r\n                  no FDCAN message has been received"]
    B_0X0 = 0,
    #[doc = "1: Message in FDCAN format with EDL\r\n                  flag set has been received"]
    B_0X1 = 1,
}
impl From<REDL_A> for bool {
    #[inline(always)]
    fn from(variant: REDL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REDL`"]
pub type REDL_R = crate::R<bool, REDL_A>;
impl REDL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REDL_A {
        match self.bits {
            false => REDL_A::B_0X0,
            true => REDL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REDL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REDL_A::B_0X1
    }
}
#[doc = "Write proxy for field `REDL`"]
pub struct REDL_W<'a> {
    w: &'a mut W,
}
impl<'a> REDL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REDL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Since this bit was reset by the CPU, no FDCAN message has been received"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(REDL_A::B_0X0)
    }
    #[doc = "Message in FDCAN format with EDL flag set has been received"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(REDL_A::B_0X1)
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
#[doc = "PXE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PXE_A {
    #[doc = "0: No protocol exception event occurred\r\n                  since last read access"]
    B_0X0 = 0,
    #[doc = "1: Protocol exception event\r\n                  occurred"]
    B_0X1 = 1,
}
impl From<PXE_A> for bool {
    #[inline(always)]
    fn from(variant: PXE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PXE`"]
pub type PXE_R = crate::R<bool, PXE_A>;
impl PXE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PXE_A {
        match self.bits {
            false => PXE_A::B_0X0,
            true => PXE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PXE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PXE_A::B_0X1
    }
}
#[doc = "Write proxy for field `PXE`"]
pub struct PXE_W<'a> {
    w: &'a mut W,
}
impl<'a> PXE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PXE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No protocol exception event occurred since last read access"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PXE_A::B_0X0)
    }
    #[doc = "Protocol exception event occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PXE_A::B_0X1)
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
#[doc = "Reader of field `TDCV`"]
pub type TDCV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDCV`"]
pub struct TDCV_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 5 - EP"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EW"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BO"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RESI"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RBRS"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - REDL"]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PXE"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - TDCV"]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LEC"]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    #[doc = "Bits 3:4 - ACT"]
    #[inline(always)]
    pub fn act(&mut self) -> ACT_W {
        ACT_W { w: self }
    }
    #[doc = "Bit 5 - EP"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    #[doc = "Bit 6 - EW"]
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W {
        EW_W { w: self }
    }
    #[doc = "Bit 7 - BO"]
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W {
        BO_W { w: self }
    }
    #[doc = "Bits 8:10 - DLEC"]
    #[inline(always)]
    pub fn dlec(&mut self) -> DLEC_W {
        DLEC_W { w: self }
    }
    #[doc = "Bit 11 - RESI"]
    #[inline(always)]
    pub fn resi(&mut self) -> RESI_W {
        RESI_W { w: self }
    }
    #[doc = "Bit 12 - RBRS"]
    #[inline(always)]
    pub fn rbrs(&mut self) -> RBRS_W {
        RBRS_W { w: self }
    }
    #[doc = "Bit 13 - REDL"]
    #[inline(always)]
    pub fn redl(&mut self) -> REDL_W {
        REDL_W { w: self }
    }
    #[doc = "Bit 14 - PXE"]
    #[inline(always)]
    pub fn pxe(&mut self) -> PXE_W {
        PXE_W { w: self }
    }
    #[doc = "Bits 16:22 - TDCV"]
    #[inline(always)]
    pub fn tdcv(&mut self) -> TDCV_W {
        TDCV_W { w: self }
    }
}
