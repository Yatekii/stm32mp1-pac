#[doc = "Reader of register FDCAN_TOCC"]
pub type R = crate::R<u32, super::FDCAN_TOCC>;
#[doc = "Writer for register FDCAN_TOCC"]
pub type W = crate::W<u32, super::FDCAN_TOCC>;
#[doc = "Register FDCAN_TOCC `reset()`'s with value 0xffff_0000"]
impl crate::ResetValue for super::FDCAN_TOCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_0000
    }
}
#[doc = "ETOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETOC_A {
    #[doc = "0: Timeout Counter\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Timeout Counter\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<ETOC_A> for bool {
    #[inline(always)]
    fn from(variant: ETOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETOC`"]
pub type ETOC_R = crate::R<bool, ETOC_A>;
impl ETOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETOC_A {
        match self.bits {
            false => ETOC_A::B_0X0,
            true => ETOC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETOC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETOC_A::B_0X1
    }
}
#[doc = "Write proxy for field `ETOC`"]
pub struct ETOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timeout Counter disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETOC_A::B_0X0)
    }
    #[doc = "Timeout Counter enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETOC_A::B_0X1)
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
#[doc = "TOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOS_AW {
    #[doc = "0: Continuous operation"]
    B_0X0 = 0,
    #[doc = "1: Timeout controlled by Tx Event\r\n                  FIFO"]
    B_0X1 = 1,
    #[doc = "2: Timeout controlled by Rx FIFO\r\n                  0"]
    B_0X2 = 2,
    #[doc = "3: Timeout controlled by Rx FIFO\r\n                  1"]
    B_0X3 = 3,
}
impl From<TOS_AW> for u8 {
    #[inline(always)]
    fn from(variant: TOS_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `TOS`"]
pub struct TOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TOS_AW::B_0X0)
    }
    #[doc = "Timeout controlled by Tx Event FIFO"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TOS_AW::B_0X1)
    }
    #[doc = "Timeout controlled by Rx FIFO 0"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TOS_AW::B_0X2)
    }
    #[doc = "Timeout controlled by Rx FIFO 1"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TOS_AW::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `TOP`"]
pub type TOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOP`"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ETOC"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - TOP"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - ETOC"]
    #[inline(always)]
    pub fn etoc(&mut self) -> ETOC_W {
        ETOC_W { w: self }
    }
    #[doc = "Bits 1:2 - TOS"]
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W {
        TOS_W { w: self }
    }
    #[doc = "Bits 16:31 - TOP"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
}
