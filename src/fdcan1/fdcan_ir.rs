#[doc = "Reader of register FDCAN_IR"]
pub type R = crate::R<u32, super::FDCAN_IR>;
#[doc = "Writer for register FDCAN_IR"]
pub type W = crate::W<u32, super::FDCAN_IR>;
#[doc = "Register FDCAN_IR `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_IR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RF0N\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0N_A {
    #[doc = "0: No new message written to Rx FIFO\r\n                  0"]
    B_0X0 = 0,
    #[doc = "1: New message written to Rx FIFO\r\n                  0"]
    B_0X1 = 1,
}
impl From<RF0N_A> for bool {
    #[inline(always)]
    fn from(variant: RF0N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF0N`"]
pub type RF0N_R = crate::R<bool, RF0N_A>;
impl RF0N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF0N_A {
        match self.bits {
            false => RF0N_A::B_0X0,
            true => RF0N_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0N_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0N_A::B_0X1
    }
}
#[doc = "Write proxy for field `RF0N`"]
pub struct RF0N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF0N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No new message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0N_A::B_0X0)
    }
    #[doc = "New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0N_A::B_0X1)
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
#[doc = "RF0W\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0W_A {
    #[doc = "0: Rx FIFO 0 fill level below\r\n                  watermark"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 0 fill level reached\r\n                  watermark"]
    B_0X1 = 1,
}
impl From<RF0W_A> for bool {
    #[inline(always)]
    fn from(variant: RF0W_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF0W`"]
pub type RF0W_R = crate::R<bool, RF0W_A>;
impl RF0W_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF0W_A {
        match self.bits {
            false => RF0W_A::B_0X0,
            true => RF0W_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0W_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0W_A::B_0X1
    }
}
#[doc = "Write proxy for field `RF0W`"]
pub struct RF0W_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0W_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF0W_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO 0 fill level below watermark"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0W_A::B_0X0)
    }
    #[doc = "Rx FIFO 0 fill level reached watermark"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0W_A::B_0X1)
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
#[doc = "RF0F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0F_A {
    #[doc = "0: Rx FIFO 0 not full"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 0 full"]
    B_0X1 = 1,
}
impl From<RF0F_A> for bool {
    #[inline(always)]
    fn from(variant: RF0F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF0F`"]
pub type RF0F_R = crate::R<bool, RF0F_A>;
impl RF0F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF0F_A {
        match self.bits {
            false => RF0F_A::B_0X0,
            true => RF0F_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0F_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0F_A::B_0X1
    }
}
#[doc = "Write proxy for field `RF0F`"]
pub struct RF0F_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF0F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO 0 not full"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0F_A::B_0X0)
    }
    #[doc = "Rx FIFO 0 full"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0F_A::B_0X1)
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
#[doc = "RF0L\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF0L_A {
    #[doc = "0: No Rx FIFO 0 message\r\n                  lost"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 0 message lost, also set\r\n                  after write attempt to Rx FIFO 0 of size\r\n                  zero"]
    B_0X1 = 1,
}
impl From<RF0L_A> for bool {
    #[inline(always)]
    fn from(variant: RF0L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF0L`"]
pub type RF0L_R = crate::R<bool, RF0L_A>;
impl RF0L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF0L_A {
        match self.bits {
            false => RF0L_A::B_0X0,
            true => RF0L_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0L_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0L_A::B_0X1
    }
}
#[doc = "Write proxy for field `RF0L`"]
pub struct RF0L_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF0L_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Rx FIFO 0 message lost"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF0L_A::B_0X0)
    }
    #[doc = "Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF0L_A::B_0X1)
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
#[doc = "RF1N\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1N_A {
    #[doc = "0: No new message written to Rx FIFO\r\n                  1"]
    B_0X0 = 0,
    #[doc = "1: New message written to Rx FIFO\r\n                  1"]
    B_0X1 = 1,
}
impl From<RF1N_A> for bool {
    #[inline(always)]
    fn from(variant: RF1N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF1N`"]
pub type RF1N_R = crate::R<bool, RF1N_A>;
impl RF1N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF1N_A {
        match self.bits {
            false => RF1N_A::B_0X0,
            true => RF1N_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1N_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1N_A::B_0X1
    }
}
#[doc = "Write proxy for field `RF1N`"]
pub struct RF1N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF1N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No new message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1N_A::B_0X0)
    }
    #[doc = "New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1N_A::B_0X1)
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
#[doc = "RF1W\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1W_A {
    #[doc = "0: Rx FIFO 1 fill level below\r\n                  watermark"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 1 fill level reached\r\n                  watermark"]
    B_0X1 = 1,
}
impl From<RF1W_A> for bool {
    #[inline(always)]
    fn from(variant: RF1W_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF1W`"]
pub type RF1W_R = crate::R<bool, RF1W_A>;
impl RF1W_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF1W_A {
        match self.bits {
            false => RF1W_A::B_0X0,
            true => RF1W_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1W_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1W_A::B_0X1
    }
}
#[doc = "Write proxy for field `RF1W`"]
pub struct RF1W_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1W_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF1W_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO 1 fill level below watermark"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1W_A::B_0X0)
    }
    #[doc = "Rx FIFO 1 fill level reached watermark"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1W_A::B_0X1)
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
#[doc = "RF1F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1F_A {
    #[doc = "0: Rx FIFO 1 not full"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 1 full"]
    B_0X1 = 1,
}
impl From<RF1F_A> for bool {
    #[inline(always)]
    fn from(variant: RF1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF1F`"]
pub type RF1F_R = crate::R<bool, RF1F_A>;
impl RF1F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF1F_A {
        match self.bits {
            false => RF1F_A::B_0X0,
            true => RF1F_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1F_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1F_A::B_0X1
    }
}
#[doc = "Write proxy for field `RF1F`"]
pub struct RF1F_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF1F_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx FIFO 1 not full"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1F_A::B_0X0)
    }
    #[doc = "Rx FIFO 1 full"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1F_A::B_0X1)
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
#[doc = "RF1L\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1L_A {
    #[doc = "0: No Rx FIFO 1 message\r\n                  lost"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 1 message lost, also set\r\n                  after write attempt to Rx FIFO 1 of size\r\n                  zero"]
    B_0X1 = 1,
}
impl From<RF1L_A> for bool {
    #[inline(always)]
    fn from(variant: RF1L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF1L`"]
pub type RF1L_R = crate::R<bool, RF1L_A>;
impl RF1L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF1L_A {
        match self.bits {
            false => RF1L_A::B_0X0,
            true => RF1L_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1L_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1L_A::B_0X1
    }
}
#[doc = "Write proxy for field `RF1L`"]
pub struct RF1L_W<'a> {
    w: &'a mut W,
}
impl<'a> RF1L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RF1L_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Rx FIFO 1 message lost"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RF1L_A::B_0X0)
    }
    #[doc = "Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RF1L_A::B_0X1)
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
#[doc = "HPM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPM_A {
    #[doc = "0: No high priority message\r\n                  received"]
    B_0X0 = 0,
    #[doc = "1: High priority message\r\n                  received"]
    B_0X1 = 1,
}
impl From<HPM_A> for bool {
    #[inline(always)]
    fn from(variant: HPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HPM`"]
pub type HPM_R = crate::R<bool, HPM_A>;
impl HPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HPM_A {
        match self.bits {
            false => HPM_A::B_0X0,
            true => HPM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HPM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HPM_A::B_0X1
    }
}
#[doc = "Write proxy for field `HPM`"]
pub struct HPM_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No high priority message received"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HPM_A::B_0X0)
    }
    #[doc = "High priority message received"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HPM_A::B_0X1)
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
#[doc = "TC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC_A {
    #[doc = "0: No transmission\r\n                  completed"]
    B_0X0 = 0,
    #[doc = "1: Transmission completed"]
    B_0X1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TC`"]
pub type TC_R = crate::R<bool, TC_A>;
impl TC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::B_0X0,
            true => TC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TC_A::B_0X1
    }
}
#[doc = "Write proxy for field `TC`"]
pub struct TC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No transmission completed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TC_A::B_0X0)
    }
    #[doc = "Transmission completed"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TC_A::B_0X1)
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
#[doc = "TCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_A {
    #[doc = "0: No transmission cancellation\r\n                  finished"]
    B_0X0 = 0,
    #[doc = "1: Transmission cancellation\r\n                  finished"]
    B_0X1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCF`"]
pub type TCF_R = crate::R<bool, TCF_A>;
impl TCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::B_0X0,
            true => TCF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCF_A::B_0X1
    }
}
#[doc = "Write proxy for field `TCF`"]
pub struct TCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No transmission cancellation finished"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TCF_A::B_0X0)
    }
    #[doc = "Transmission cancellation finished"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TCF_A::B_0X1)
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
#[doc = "TFE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFE_A {
    #[doc = "0: Tx FIFO non-empty"]
    B_0X0 = 0,
    #[doc = "1: Tx FIFO empty"]
    B_0X1 = 1,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TFE`"]
pub type TFE_R = crate::R<bool, TFE_A>;
impl TFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFE_A {
        match self.bits {
            false => TFE_A::B_0X0,
            true => TFE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TFE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TFE_A::B_0X1
    }
}
#[doc = "Write proxy for field `TFE`"]
pub struct TFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx FIFO non-empty"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TFE_A::B_0X0)
    }
    #[doc = "Tx FIFO empty"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TFE_A::B_0X1)
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
#[doc = "TEFN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFN_A {
    #[doc = "0: Tx Event FIFO\r\n                  unchanged"]
    B_0X0 = 0,
    #[doc = "1: Tx Handler wrote Tx Event FIFO\r\n                  element"]
    B_0X1 = 1,
}
impl From<TEFN_A> for bool {
    #[inline(always)]
    fn from(variant: TEFN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEFN`"]
pub type TEFN_R = crate::R<bool, TEFN_A>;
impl TEFN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEFN_A {
        match self.bits {
            false => TEFN_A::B_0X0,
            true => TEFN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFN_A::B_0X1
    }
}
#[doc = "Write proxy for field `TEFN`"]
pub struct TEFN_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEFN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx Event FIFO unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFN_A::B_0X0)
    }
    #[doc = "Tx Handler wrote Tx Event FIFO element"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFN_A::B_0X1)
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
#[doc = "TEFW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFW_A {
    #[doc = "0: Tx Event FIFO fill level below\r\n                  watermark"]
    B_0X0 = 0,
    #[doc = "1: Tx Event FIFO fill level reached\r\n                  watermark"]
    B_0X1 = 1,
}
impl From<TEFW_A> for bool {
    #[inline(always)]
    fn from(variant: TEFW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEFW`"]
pub type TEFW_R = crate::R<bool, TEFW_A>;
impl TEFW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEFW_A {
        match self.bits {
            false => TEFW_A::B_0X0,
            true => TEFW_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFW_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFW_A::B_0X1
    }
}
#[doc = "Write proxy for field `TEFW`"]
pub struct TEFW_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEFW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx Event FIFO fill level below watermark"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFW_A::B_0X0)
    }
    #[doc = "Tx Event FIFO fill level reached watermark"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFW_A::B_0X1)
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
#[doc = "TEFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFF_A {
    #[doc = "0: Tx Event FIFO not full"]
    B_0X0 = 0,
    #[doc = "1: Tx Event FIFO full"]
    B_0X1 = 1,
}
impl From<TEFF_A> for bool {
    #[inline(always)]
    fn from(variant: TEFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEFF`"]
pub type TEFF_R = crate::R<bool, TEFF_A>;
impl TEFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEFF_A {
        match self.bits {
            false => TEFF_A::B_0X0,
            true => TEFF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFF_A::B_0X1
    }
}
#[doc = "Write proxy for field `TEFF`"]
pub struct TEFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tx Event FIFO not full"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFF_A::B_0X0)
    }
    #[doc = "Tx Event FIFO full"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFF_A::B_0X1)
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
#[doc = "TEFL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFL_A {
    #[doc = "0: No Tx Event FIFO element\r\n                  lost"]
    B_0X0 = 0,
    #[doc = "1: Tx Event FIFO element lost, also set\r\n                  after write attempt to Tx Event FIFO of size\r\n                  zero"]
    B_0X1 = 1,
}
impl From<TEFL_A> for bool {
    #[inline(always)]
    fn from(variant: TEFL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEFL`"]
pub type TEFL_R = crate::R<bool, TEFL_A>;
impl TEFL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEFL_A {
        match self.bits {
            false => TEFL_A::B_0X0,
            true => TEFL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFL_A::B_0X1
    }
}
#[doc = "Write proxy for field `TEFL`"]
pub struct TEFL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEFL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEFL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Tx Event FIFO element lost"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TEFL_A::B_0X0)
    }
    #[doc = "Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TEFL_A::B_0X1)
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
#[doc = "TSW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSW_A {
    #[doc = "0: No timestamp counter\r\n                  wrap-around"]
    B_0X0 = 0,
    #[doc = "1: Timestamp counter wrapped\r\n                  around"]
    B_0X1 = 1,
}
impl From<TSW_A> for bool {
    #[inline(always)]
    fn from(variant: TSW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TSW`"]
pub type TSW_R = crate::R<bool, TSW_A>;
impl TSW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSW_A {
        match self.bits {
            false => TSW_A::B_0X0,
            true => TSW_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSW_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSW_A::B_0X1
    }
}
#[doc = "Write proxy for field `TSW`"]
pub struct TSW_W<'a> {
    w: &'a mut W,
}
impl<'a> TSW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No timestamp counter wrap-around"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSW_A::B_0X0)
    }
    #[doc = "Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSW_A::B_0X1)
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
#[doc = "MRAF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRAF_A {
    #[doc = "0: No Message RAM access failure\r\n                  occurred"]
    B_0X0 = 0,
    #[doc = "1: Message RAM access failure\r\n                  occurred"]
    B_0X1 = 1,
}
impl From<MRAF_A> for bool {
    #[inline(always)]
    fn from(variant: MRAF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MRAF`"]
pub type MRAF_R = crate::R<bool, MRAF_A>;
impl MRAF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRAF_A {
        match self.bits {
            false => MRAF_A::B_0X0,
            true => MRAF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MRAF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MRAF_A::B_0X1
    }
}
#[doc = "Write proxy for field `MRAF`"]
pub struct MRAF_W<'a> {
    w: &'a mut W,
}
impl<'a> MRAF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRAF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Message RAM access failure occurred"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MRAF_A::B_0X0)
    }
    #[doc = "Message RAM access failure occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MRAF_A::B_0X1)
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
#[doc = "TOO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOO_A {
    #[doc = "0: No timeout"]
    B_0X0 = 0,
    #[doc = "1: Timeout reached"]
    B_0X1 = 1,
}
impl From<TOO_A> for bool {
    #[inline(always)]
    fn from(variant: TOO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TOO`"]
pub type TOO_R = crate::R<bool, TOO_A>;
impl TOO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOO_A {
        match self.bits {
            false => TOO_A::B_0X0,
            true => TOO_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TOO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TOO_A::B_0X1
    }
}
#[doc = "Write proxy for field `TOO`"]
pub struct TOO_W<'a> {
    w: &'a mut W,
}
impl<'a> TOO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No timeout"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TOO_A::B_0X0)
    }
    #[doc = "Timeout reached"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TOO_A::B_0X1)
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
#[doc = "DRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRX_A {
    #[doc = "0: No timeout"]
    B_0X0 = 0,
    #[doc = "1: Timeout reached"]
    B_0X1 = 1,
}
impl From<DRX_A> for bool {
    #[inline(always)]
    fn from(variant: DRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRX`"]
pub type DRX_R = crate::R<bool, DRX_A>;
impl DRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRX_A {
        match self.bits {
            false => DRX_A::B_0X0,
            true => DRX_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DRX_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DRX_A::B_0X1
    }
}
#[doc = "Write proxy for field `DRX`"]
pub struct DRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No timeout"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DRX_A::B_0X0)
    }
    #[doc = "Timeout reached"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DRX_A::B_0X1)
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
#[doc = "ELO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELO_A {
    #[doc = "0: CAN Error Logging Counter did not\r\n                  overflow"]
    B_0X0 = 0,
    #[doc = "1: Overflow of CAN Error Logging\r\n                  Counter occurred"]
    B_0X1 = 1,
}
impl From<ELO_A> for bool {
    #[inline(always)]
    fn from(variant: ELO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ELO`"]
pub type ELO_R = crate::R<bool, ELO_A>;
impl ELO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELO_A {
        match self.bits {
            false => ELO_A::B_0X0,
            true => ELO_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ELO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ELO_A::B_0X1
    }
}
#[doc = "Write proxy for field `ELO`"]
pub struct ELO_W<'a> {
    w: &'a mut W,
}
impl<'a> ELO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ELO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAN Error Logging Counter did not overflow"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ELO_A::B_0X0)
    }
    #[doc = "Overflow of CAN Error Logging Counter occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ELO_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_A {
    #[doc = "0: Error_Passive status\r\n                  unchanged"]
    B_0X0 = 0,
    #[doc = "1: Error_Passive status\r\n                  changed"]
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
    #[doc = "Error_Passive status unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EP_A::B_0X0)
    }
    #[doc = "Error_Passive status changed"]
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "EW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EW_A {
    #[doc = "0: Error_Warning status\r\n                  unchanged"]
    B_0X0 = 0,
    #[doc = "1: Error_Warning status\r\n                  changed"]
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
    #[doc = "Error_Warning status unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EW_A::B_0X0)
    }
    #[doc = "Error_Warning status changed"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "BO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BO_A {
    #[doc = "0: Bus_Off status\r\n                  unchanged"]
    B_0X0 = 0,
    #[doc = "1: Bus_Off status changed"]
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
    #[doc = "Bus_Off status unchanged"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BO_A::B_0X0)
    }
    #[doc = "Bus_Off status changed"]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "WDI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDI_A {
    #[doc = "0: No Message RAM Watchdog event\r\n                  occurred"]
    B_0X0 = 0,
    #[doc = "1: Message RAM Watchdog event due to\r\n                  missing READY"]
    B_0X1 = 1,
}
impl From<WDI_A> for bool {
    #[inline(always)]
    fn from(variant: WDI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDI`"]
pub type WDI_R = crate::R<bool, WDI_A>;
impl WDI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDI_A {
        match self.bits {
            false => WDI_A::B_0X0,
            true => WDI_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDI_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDI_A::B_0X1
    }
}
#[doc = "Write proxy for field `WDI`"]
pub struct WDI_W<'a> {
    w: &'a mut W,
}
impl<'a> WDI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Message RAM Watchdog event occurred"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WDI_A::B_0X0)
    }
    #[doc = "Message RAM Watchdog event due to missing READY"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WDI_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "PEA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEA_A {
    #[doc = "0: No protocol error in arbitration\r\n                  phase"]
    B_0X0 = 0,
    #[doc = "1: Protocol error in arbitration phase\r\n                  detected (PSR.LEC different from\r\n                  0,7)"]
    B_0X1 = 1,
}
impl From<PEA_A> for bool {
    #[inline(always)]
    fn from(variant: PEA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PEA`"]
pub type PEA_R = crate::R<bool, PEA_A>;
impl PEA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEA_A {
        match self.bits {
            false => PEA_A::B_0X0,
            true => PEA_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PEA_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PEA_A::B_0X1
    }
}
#[doc = "Write proxy for field `PEA`"]
pub struct PEA_W<'a> {
    w: &'a mut W,
}
impl<'a> PEA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No protocol error in arbitration phase"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PEA_A::B_0X0)
    }
    #[doc = "Protocol error in arbitration phase detected (PSR.LEC different from 0,7)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PEA_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "PED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PED_A {
    #[doc = "0: No protocol error in data\r\n                  phase"]
    B_0X0 = 0,
    #[doc = "1: Protocol error in data phase\r\n                  detected (PSR.DLEC different from\r\n                  0,7)"]
    B_0X1 = 1,
}
impl From<PED_A> for bool {
    #[inline(always)]
    fn from(variant: PED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PED`"]
pub type PED_R = crate::R<bool, PED_A>;
impl PED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PED_A {
        match self.bits {
            false => PED_A::B_0X0,
            true => PED_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PED_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PED_A::B_0X1
    }
}
#[doc = "Write proxy for field `PED`"]
pub struct PED_W<'a> {
    w: &'a mut W,
}
impl<'a> PED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No protocol error in data phase"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PED_A::B_0X0)
    }
    #[doc = "Protocol error in data phase detected (PSR.DLEC different from 0,7)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PED_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "ARA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARA_A {
    #[doc = "0: No access to reserved address\r\n                  occurred"]
    B_0X0 = 0,
    #[doc = "1: Access to reserved address\r\n                  occurred"]
    B_0X1 = 1,
}
impl From<ARA_A> for bool {
    #[inline(always)]
    fn from(variant: ARA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARA`"]
pub type ARA_R = crate::R<bool, ARA_A>;
impl ARA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARA_A {
        match self.bits {
            false => ARA_A::B_0X0,
            true => ARA_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ARA_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ARA_A::B_0X1
    }
}
#[doc = "Write proxy for field `ARA`"]
pub struct ARA_W<'a> {
    w: &'a mut W,
}
impl<'a> ARA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No access to reserved address occurred"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ARA_A::B_0X0)
    }
    #[doc = "Access to reserved address occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ARA_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RF0N"]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RF0W"]
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RF0F"]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RF0L"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RF1N"]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RF1W"]
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RF1F"]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RF1L"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HPM"]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TC"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TCF"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TFE"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TEFN"]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TEFW"]
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TEFF"]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TEFL"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TSW"]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MRAF"]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TOO"]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DRX"]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ELO"]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - EP"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - EW"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - BO"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - WDI"]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PEA"]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PED"]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ARA"]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RF0N"]
    #[inline(always)]
    pub fn rf0n(&mut self) -> RF0N_W {
        RF0N_W { w: self }
    }
    #[doc = "Bit 1 - RF0W"]
    #[inline(always)]
    pub fn rf0w(&mut self) -> RF0W_W {
        RF0W_W { w: self }
    }
    #[doc = "Bit 2 - RF0F"]
    #[inline(always)]
    pub fn rf0f(&mut self) -> RF0F_W {
        RF0F_W { w: self }
    }
    #[doc = "Bit 3 - RF0L"]
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W {
        RF0L_W { w: self }
    }
    #[doc = "Bit 4 - RF1N"]
    #[inline(always)]
    pub fn rf1n(&mut self) -> RF1N_W {
        RF1N_W { w: self }
    }
    #[doc = "Bit 5 - RF1W"]
    #[inline(always)]
    pub fn rf1w(&mut self) -> RF1W_W {
        RF1W_W { w: self }
    }
    #[doc = "Bit 6 - RF1F"]
    #[inline(always)]
    pub fn rf1f(&mut self) -> RF1F_W {
        RF1F_W { w: self }
    }
    #[doc = "Bit 7 - RF1L"]
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W {
        RF1L_W { w: self }
    }
    #[doc = "Bit 8 - HPM"]
    #[inline(always)]
    pub fn hpm(&mut self) -> HPM_W {
        HPM_W { w: self }
    }
    #[doc = "Bit 9 - TC"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W {
        TC_W { w: self }
    }
    #[doc = "Bit 10 - TCF"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W {
        TCF_W { w: self }
    }
    #[doc = "Bit 11 - TFE"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W {
        TFE_W { w: self }
    }
    #[doc = "Bit 12 - TEFN"]
    #[inline(always)]
    pub fn tefn(&mut self) -> TEFN_W {
        TEFN_W { w: self }
    }
    #[doc = "Bit 13 - TEFW"]
    #[inline(always)]
    pub fn tefw(&mut self) -> TEFW_W {
        TEFW_W { w: self }
    }
    #[doc = "Bit 14 - TEFF"]
    #[inline(always)]
    pub fn teff(&mut self) -> TEFF_W {
        TEFF_W { w: self }
    }
    #[doc = "Bit 15 - TEFL"]
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W {
        TEFL_W { w: self }
    }
    #[doc = "Bit 16 - TSW"]
    #[inline(always)]
    pub fn tsw(&mut self) -> TSW_W {
        TSW_W { w: self }
    }
    #[doc = "Bit 17 - MRAF"]
    #[inline(always)]
    pub fn mraf(&mut self) -> MRAF_W {
        MRAF_W { w: self }
    }
    #[doc = "Bit 18 - TOO"]
    #[inline(always)]
    pub fn too(&mut self) -> TOO_W {
        TOO_W { w: self }
    }
    #[doc = "Bit 19 - DRX"]
    #[inline(always)]
    pub fn drx(&mut self) -> DRX_W {
        DRX_W { w: self }
    }
    #[doc = "Bit 22 - ELO"]
    #[inline(always)]
    pub fn elo(&mut self) -> ELO_W {
        ELO_W { w: self }
    }
    #[doc = "Bit 23 - EP"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    #[doc = "Bit 24 - EW"]
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W {
        EW_W { w: self }
    }
    #[doc = "Bit 25 - BO"]
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W {
        BO_W { w: self }
    }
    #[doc = "Bit 26 - WDI"]
    #[inline(always)]
    pub fn wdi(&mut self) -> WDI_W {
        WDI_W { w: self }
    }
    #[doc = "Bit 27 - PEA"]
    #[inline(always)]
    pub fn pea(&mut self) -> PEA_W {
        PEA_W { w: self }
    }
    #[doc = "Bit 28 - PED"]
    #[inline(always)]
    pub fn ped(&mut self) -> PED_W {
        PED_W { w: self }
    }
    #[doc = "Bit 29 - ARA"]
    #[inline(always)]
    pub fn ara(&mut self) -> ARA_W {
        ARA_W { w: self }
    }
}
