#[doc = "Reader of register DFSDM_FLT0CR1"]
pub type R = crate::R<u32, super::DFSDM_FLT0CR1>;
#[doc = "Writer for register DFSDM_FLT0CR1"]
pub type W = crate::W<u32, super::DFSDM_FLT0CR1>;
#[doc = "Register DFSDM_FLT0CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFSDM_FLT0CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DFEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFEN_A {
    #[doc = "0: DFSDM_FLTx is disabled. All\r\n                  conversions of given DFSDM_FLTx are stopped\r\n                  immediately and all DFSDM_FLTx functions are\r\n                  stopped."]
    B_0X0 = 0,
    #[doc = "1: DFSDM_FLTx is enabled. If DFSDM_FLTx\r\n                  is enabled, then DFSDM_FLTx starts operating\r\n                  according to its setting."]
    B_0X1 = 1,
}
impl From<DFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFEN`"]
pub type DFEN_R = crate::R<bool, DFEN_A>;
impl DFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFEN_A {
        match self.bits {
            false => DFEN_A::B_0X0,
            true => DFEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DFEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DFEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DFEN`"]
pub struct DFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DFSDM_FLTx is disabled. All conversions of given DFSDM_FLTx are stopped immediately and all DFSDM_FLTx functions are stopped."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DFEN_A::B_0X0)
    }
    #[doc = "DFSDM_FLTx is enabled. If DFSDM_FLTx is enabled, then DFSDM_FLTx starts operating according to its setting."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DFEN_A::B_0X1)
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
#[doc = "JSWSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSWSTART_AW {
    #[doc = "0: Writing 0 has no\r\n                  effect."]
    B_0X0 = 0,
    #[doc = "1: Writing 1 makes a request to convert\r\n                  the channels in the injected conversion group,\r\n                  causing JCIP to become 1 at the same time. If\r\n                  JCIP=1 already, then writing to JSWSTART has no\r\n                  effect. Writing 1 has no effect if\r\n                  JSYNC=1."]
    B_0X1 = 1,
}
impl From<JSWSTART_AW> for bool {
    #[inline(always)]
    fn from(variant: JSWSTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `JSWSTART`"]
pub struct JSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> JSWSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JSWSTART_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing 0 has no effect."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JSWSTART_AW::B_0X0)
    }
    #[doc = "Writing 1 makes a request to convert the channels in the injected conversion group, causing JCIP to become 1 at the same time. If JCIP=1 already, then writing to JSWSTART has no effect. Writing 1 has no effect if JSYNC=1."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JSWSTART_AW::B_0X1)
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
#[doc = "JSYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSYNC_A {
    #[doc = "0: Do not launch an injected conversion\r\n                  synchronously with DFSDM_FLT0"]
    B_0X0 = 0,
    #[doc = "1: Launch an injected conversion in\r\n                  this DFSDM_FLTx at the very moment when an\r\n                  injected conversion is launched in DFSDM_FLT0 by\r\n                  its JSWSTART trigger"]
    B_0X1 = 1,
}
impl From<JSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: JSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JSYNC`"]
pub type JSYNC_R = crate::R<bool, JSYNC_A>;
impl JSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSYNC_A {
        match self.bits {
            false => JSYNC_A::B_0X0,
            true => JSYNC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == JSYNC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == JSYNC_A::B_0X1
    }
}
#[doc = "Write proxy for field `JSYNC`"]
pub struct JSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> JSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not launch an injected conversion synchronously with DFSDM_FLT0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JSYNC_A::B_0X0)
    }
    #[doc = "Launch an injected conversion in this DFSDM_FLTx at the very moment when an injected conversion is launched in DFSDM_FLT0 by its JSWSTART trigger"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JSYNC_A::B_0X1)
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
#[doc = "JSCAN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSCAN_A {
    #[doc = "0: One channel conversion is performed\r\n                  from the injected channel group and next the\r\n                  selected channel from this group is\r\n                  selected."]
    B_0X0 = 0,
    #[doc = "1: The series of conversions for the\r\n                  injected group channels is executed, starting\r\n                  over with the lowest selected\r\n                  channel."]
    B_0X1 = 1,
}
impl From<JSCAN_A> for bool {
    #[inline(always)]
    fn from(variant: JSCAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JSCAN`"]
pub type JSCAN_R = crate::R<bool, JSCAN_A>;
impl JSCAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSCAN_A {
        match self.bits {
            false => JSCAN_A::B_0X0,
            true => JSCAN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == JSCAN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == JSCAN_A::B_0X1
    }
}
#[doc = "Write proxy for field `JSCAN`"]
pub struct JSCAN_W<'a> {
    w: &'a mut W,
}
impl<'a> JSCAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JSCAN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "One channel conversion is performed from the injected channel group and next the selected channel from this group is selected."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JSCAN_A::B_0X0)
    }
    #[doc = "The series of conversions for the injected group channels is executed, starting over with the lowest selected channel."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JSCAN_A::B_0X1)
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
#[doc = "JDMAEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JDMAEN_A {
    #[doc = "0: The DMA channel is not enabled to\r\n                  read injected data"]
    B_0X0 = 0,
    #[doc = "1: The DMA channel is enabled to read\r\n                  injected data"]
    B_0X1 = 1,
}
impl From<JDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: JDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JDMAEN`"]
pub type JDMAEN_R = crate::R<bool, JDMAEN_A>;
impl JDMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JDMAEN_A {
        match self.bits {
            false => JDMAEN_A::B_0X0,
            true => JDMAEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == JDMAEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == JDMAEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `JDMAEN`"]
pub struct JDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA channel is not enabled to read injected data"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JDMAEN_A::B_0X0)
    }
    #[doc = "The DMA channel is enabled to read injected data"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JDMAEN_A::B_0X1)
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
#[doc = "Reader of field `JEXTSEL`"]
pub type JEXTSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `JEXTSEL`"]
pub struct JEXTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "JEXTEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum JEXTEN_A {
    #[doc = "0: Trigger detection is\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Each rising edge on the selected\r\n                  trigger makes a request to launch an injected\r\n                  conversion"]
    B_0X1 = 1,
    #[doc = "2: Each falling edge on the selected\r\n                  trigger makes a request to launch an injected\r\n                  conversion"]
    B_0X2 = 2,
    #[doc = "3: Both rising edges and falling edges\r\n                  on the selected trigger make requests to launch\r\n                  injected conversions"]
    B_0X3 = 3,
}
impl From<JEXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `JEXTEN`"]
pub type JEXTEN_R = crate::R<u8, JEXTEN_A>;
impl JEXTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEXTEN_A {
        match self.bits {
            0 => JEXTEN_A::B_0X0,
            1 => JEXTEN_A::B_0X1,
            2 => JEXTEN_A::B_0X2,
            3 => JEXTEN_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == JEXTEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == JEXTEN_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == JEXTEN_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == JEXTEN_A::B_0X3
    }
}
#[doc = "Write proxy for field `JEXTEN`"]
pub struct JEXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JEXTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEXTEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger detection is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(JEXTEN_A::B_0X0)
    }
    #[doc = "Each rising edge on the selected trigger makes a request to launch an injected conversion"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(JEXTEN_A::B_0X1)
    }
    #[doc = "Each falling edge on the selected trigger makes a request to launch an injected conversion"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(JEXTEN_A::B_0X2)
    }
    #[doc = "Both rising edges and falling edges on the selected trigger make requests to launch injected conversions"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(JEXTEN_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "RSWSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSWSTART_AW {
    #[doc = "0: Writing 0 has no\r\n                  effect"]
    B_0X0 = 0,
    #[doc = "1: Writing 1 makes a request to start a\r\n                  conversion on the regular channel and causes RCIP\r\n                  to become 1 . If RCIP=1 already, writing to\r\n                  RSWSTART has no effect. Writing 1 has no effect\r\n                  if RSYNC=1."]
    B_0X1 = 1,
}
impl From<RSWSTART_AW> for bool {
    #[inline(always)]
    fn from(variant: RSWSTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RSWSTART`"]
pub struct RSWSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RSWSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSWSTART_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RSWSTART_AW::B_0X0)
    }
    #[doc = "Writing 1 makes a request to start a conversion on the regular channel and causes RCIP to become 1 . If RCIP=1 already, writing to RSWSTART has no effect. Writing 1 has no effect if RSYNC=1."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RSWSTART_AW::B_0X1)
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
#[doc = "RCONT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCONT_A {
    #[doc = "0: The regular channel is converted\r\n                  just once for each conversion\r\n                  request"]
    B_0X0 = 0,
    #[doc = "1: The regular channel is converted\r\n                  repeatedly after each conversion\r\n                  request"]
    B_0X1 = 1,
}
impl From<RCONT_A> for bool {
    #[inline(always)]
    fn from(variant: RCONT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RCONT`"]
pub type RCONT_R = crate::R<bool, RCONT_A>;
impl RCONT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCONT_A {
        match self.bits {
            false => RCONT_A::B_0X0,
            true => RCONT_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RCONT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RCONT_A::B_0X1
    }
}
#[doc = "Write proxy for field `RCONT`"]
pub struct RCONT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCONT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCONT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The regular channel is converted just once for each conversion request"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RCONT_A::B_0X0)
    }
    #[doc = "The regular channel is converted repeatedly after each conversion request"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RCONT_A::B_0X1)
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
#[doc = "RSYNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSYNC_A {
    #[doc = "0: Do not launch a regular conversion\r\n                  synchronously with DFSDM_FLT0"]
    B_0X0 = 0,
    #[doc = "1: Launch a regular conversion in this\r\n                  DFSDM_FLTx at the very moment when a regular\r\n                  conversion is launched in\r\n                  DFSDM_FLT0"]
    B_0X1 = 1,
}
impl From<RSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSYNC`"]
pub type RSYNC_R = crate::R<bool, RSYNC_A>;
impl RSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSYNC_A {
        match self.bits {
            false => RSYNC_A::B_0X0,
            true => RSYNC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RSYNC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RSYNC_A::B_0X1
    }
}
#[doc = "Write proxy for field `RSYNC`"]
pub struct RSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not launch a regular conversion synchronously with DFSDM_FLT0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RSYNC_A::B_0X0)
    }
    #[doc = "Launch a regular conversion in this DFSDM_FLTx at the very moment when a regular conversion is launched in DFSDM_FLT0"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RSYNC_A::B_0X1)
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
#[doc = "RDMAEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAEN_A {
    #[doc = "0: The DMA channel is not enabled to\r\n                  read regular data"]
    B_0X0 = 0,
    #[doc = "1: The DMA channel is enabled to read\r\n                  regular data"]
    B_0X1 = 1,
}
impl From<RDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDMAEN`"]
pub type RDMAEN_R = crate::R<bool, RDMAEN_A>;
impl RDMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMAEN_A {
        match self.bits {
            false => RDMAEN_A::B_0X0,
            true => RDMAEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RDMAEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RDMAEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `RDMAEN`"]
pub struct RDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The DMA channel is not enabled to read regular data"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RDMAEN_A::B_0X0)
    }
    #[doc = "The DMA channel is enabled to read regular data"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RDMAEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "RCH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RCH_A {
    #[doc = "0: Channel 0 is selected as the regular\r\n                  channel"]
    B_0X0 = 0,
    #[doc = "1: Channel 1 is selected as the regular\r\n                  channel"]
    B_0X1 = 1,
    #[doc = "7: Channel 7 is selected as the regular\r\n                  channel"]
    B_0X7 = 7,
}
impl From<RCH_A> for u8 {
    #[inline(always)]
    fn from(variant: RCH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RCH`"]
pub type RCH_R = crate::R<u8, RCH_A>;
impl RCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RCH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RCH_A::B_0X0),
            1 => Val(RCH_A::B_0X1),
            7 => Val(RCH_A::B_0X7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RCH_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RCH_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == RCH_A::B_0X7
    }
}
#[doc = "Write proxy for field `RCH`"]
pub struct RCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RCH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel 0 is selected as the regular channel"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RCH_A::B_0X0)
    }
    #[doc = "Channel 1 is selected as the regular channel"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RCH_A::B_0X1)
    }
    #[doc = "Channel 7 is selected as the regular channel"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(RCH_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "FAST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_A {
    #[doc = "0: Fast conversion mode\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Fast conversion mode\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<FAST_A> for bool {
    #[inline(always)]
    fn from(variant: FAST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAST`"]
pub type FAST_R = crate::R<bool, FAST_A>;
impl FAST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAST_A {
        match self.bits {
            false => FAST_A::B_0X0,
            true => FAST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FAST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FAST_A::B_0X1
    }
}
#[doc = "Write proxy for field `FAST`"]
pub struct FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FAST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fast conversion mode disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(FAST_A::B_0X0)
    }
    #[doc = "Fast conversion mode enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(FAST_A::B_0X1)
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
#[doc = "AWFSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWFSEL_A {
    #[doc = "0: Analog watchdog on data output value\r\n                  (after the digital filter). The comparison is\r\n                  done after offset correction and\r\n                  shift"]
    B_0X0 = 0,
    #[doc = "1: Analog watchdog on channel\r\n                  transceivers value (after watchdog\r\n                  filter)"]
    B_0X1 = 1,
}
impl From<AWFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: AWFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWFSEL`"]
pub type AWFSEL_R = crate::R<bool, AWFSEL_A>;
impl AWFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWFSEL_A {
        match self.bits {
            false => AWFSEL_A::B_0X0,
            true => AWFSEL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWFSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWFSEL_A::B_0X1
    }
}
#[doc = "Write proxy for field `AWFSEL`"]
pub struct AWFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWFSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Analog watchdog on data output value (after the digital filter). The comparison is done after offset correction and shift"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AWFSEL_A::B_0X0)
    }
    #[doc = "Analog watchdog on channel transceivers value (after watchdog filter)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AWFSEL_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DFEN"]
    #[inline(always)]
    pub fn dfen(&self) -> DFEN_R {
        DFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - JSYNC"]
    #[inline(always)]
    pub fn jsync(&self) -> JSYNC_R {
        JSYNC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - JSCAN"]
    #[inline(always)]
    pub fn jscan(&self) -> JSCAN_R {
        JSCAN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - JDMAEN"]
    #[inline(always)]
    pub fn jdmaen(&self) -> JDMAEN_R {
        JDMAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - JEXTSEL"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 13:14 - JEXTEN"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 18 - RCONT"]
    #[inline(always)]
    pub fn rcont(&self) -> RCONT_R {
        RCONT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RSYNC"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - RDMAEN"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RDMAEN_R {
        RDMAEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - RCH"]
    #[inline(always)]
    pub fn rch(&self) -> RCH_R {
        RCH_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 29 - FAST"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - AWFSEL"]
    #[inline(always)]
    pub fn awfsel(&self) -> AWFSEL_R {
        AWFSEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFEN"]
    #[inline(always)]
    pub fn dfen(&mut self) -> DFEN_W {
        DFEN_W { w: self }
    }
    #[doc = "Bit 1 - JSWSTART"]
    #[inline(always)]
    pub fn jswstart(&mut self) -> JSWSTART_W {
        JSWSTART_W { w: self }
    }
    #[doc = "Bit 3 - JSYNC"]
    #[inline(always)]
    pub fn jsync(&mut self) -> JSYNC_W {
        JSYNC_W { w: self }
    }
    #[doc = "Bit 4 - JSCAN"]
    #[inline(always)]
    pub fn jscan(&mut self) -> JSCAN_W {
        JSCAN_W { w: self }
    }
    #[doc = "Bit 5 - JDMAEN"]
    #[inline(always)]
    pub fn jdmaen(&mut self) -> JDMAEN_W {
        JDMAEN_W { w: self }
    }
    #[doc = "Bits 8:10 - JEXTSEL"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W {
        JEXTSEL_W { w: self }
    }
    #[doc = "Bits 13:14 - JEXTEN"]
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W {
        JEXTEN_W { w: self }
    }
    #[doc = "Bit 17 - RSWSTART"]
    #[inline(always)]
    pub fn rswstart(&mut self) -> RSWSTART_W {
        RSWSTART_W { w: self }
    }
    #[doc = "Bit 18 - RCONT"]
    #[inline(always)]
    pub fn rcont(&mut self) -> RCONT_W {
        RCONT_W { w: self }
    }
    #[doc = "Bit 19 - RSYNC"]
    #[inline(always)]
    pub fn rsync(&mut self) -> RSYNC_W {
        RSYNC_W { w: self }
    }
    #[doc = "Bit 21 - RDMAEN"]
    #[inline(always)]
    pub fn rdmaen(&mut self) -> RDMAEN_W {
        RDMAEN_W { w: self }
    }
    #[doc = "Bits 24:26 - RCH"]
    #[inline(always)]
    pub fn rch(&mut self) -> RCH_W {
        RCH_W { w: self }
    }
    #[doc = "Bit 29 - FAST"]
    #[inline(always)]
    pub fn fast(&mut self) -> FAST_W {
        FAST_W { w: self }
    }
    #[doc = "Bit 30 - AWFSEL"]
    #[inline(always)]
    pub fn awfsel(&mut self) -> AWFSEL_W {
        AWFSEL_W { w: self }
    }
}
