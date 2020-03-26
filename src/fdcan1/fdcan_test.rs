#[doc = "Reader of register FDCAN_TEST"]
pub type R = crate::R<u32, super::FDCAN_TEST>;
#[doc = "Writer for register FDCAN_TEST"]
pub type W = crate::W<u32, super::FDCAN_TEST>;
#[doc = "Register FDCAN_TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LBCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBCK_A {
    #[doc = "0: Reset value, Loop Back mode is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Loop Back mode is enabled (see Test\r\n                  modes)"]
    B_0X1 = 1,
}
impl From<LBCK_A> for bool {
    #[inline(always)]
    fn from(variant: LBCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBCK`"]
pub type LBCK_R = crate::R<bool, LBCK_A>;
impl LBCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBCK_A {
        match self.bits {
            false => LBCK_A::B_0X0,
            true => LBCK_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LBCK_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LBCK_A::B_0X1
    }
}
#[doc = "Write proxy for field `LBCK`"]
pub struct LBCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LBCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset value, Loop Back mode is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LBCK_A::B_0X0)
    }
    #[doc = "Loop Back mode is enabled (see Test modes)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LBCK_A::B_0X1)
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
#[doc = "TX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_A {
    #[doc = "0: Reset value , FDCANx_TX TX is\r\n                  controlled by the CAN core, updated at the end of\r\n                  the CAN bit time"]
    B_0X0 = 0,
    #[doc = "1: Sample point can be monitored at pin\r\n                  FDCANx_TX"]
    B_0X1 = 1,
    #[doc = "2: Dominant ( 0 ) level at pin\r\n                  FDCANx_TX"]
    B_0X2 = 2,
    #[doc = "3: Recessive ( 1 ) at pin\r\n                  FDCANx_TX"]
    B_0X3 = 3,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<u8, TX_A>;
impl TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            0 => TX_A::B_0X0,
            1 => TX_A::B_0X1,
            2 => TX_A::B_0X2,
            3 => TX_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TX_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TX_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TX_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TX_A::B_0X3
    }
}
#[doc = "Write proxy for field `TX`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Reset value , FDCANx_TX TX is controlled by the CAN core, updated at the end of the CAN bit time"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TX_A::B_0X0)
    }
    #[doc = "Sample point can be monitored at pin FDCANx_TX"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TX_A::B_0X1)
    }
    #[doc = "Dominant ( 0 ) level at pin FDCANx_TX"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TX_A::B_0X2)
    }
    #[doc = "Recessive ( 1 ) at pin FDCANx_TX"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TX_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_A {
    #[doc = "0: The CAN bus is dominant (FDCANx_RX =\r\n                  0 )"]
    B_0X0 = 0,
    #[doc = "1: The CAN bus is recessive (FDCANx_RX\r\n                  = 1 )"]
    B_0X1 = 1,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, RX_A>;
impl RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            false => RX_A::B_0X0,
            true => RX_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RX_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RX_A::B_0X1
    }
}
#[doc = "Write proxy for field `RX`"]
pub struct RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The CAN bus is dominant (FDCANx_RX = 0 )"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RX_A::B_0X0)
    }
    #[doc = "The CAN bus is recessive (FDCANx_RX = 1 )"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RX_A::B_0X1)
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
impl R {
    #[doc = "Bit 4 - LBCK"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - TX"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - RX"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - LBCK"]
    #[inline(always)]
    pub fn lbck(&mut self) -> LBCK_W {
        LBCK_W { w: self }
    }
    #[doc = "Bits 5:6 - TX"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Bit 7 - RX"]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W {
        RX_W { w: self }
    }
}
