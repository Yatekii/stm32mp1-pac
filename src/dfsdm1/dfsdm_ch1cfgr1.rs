#[doc = "Reader of register DFSDM_CH1CFGR1"]
pub type R = crate::R<u32, super::DFSDM_CH1CFGR1>;
#[doc = "Writer for register DFSDM_CH1CFGR1"]
pub type W = crate::W<u32, super::DFSDM_CH1CFGR1>;
#[doc = "Register DFSDM_CH1CFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFSDM_CH1CFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SITP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SITP_A {
    #[doc = "0: SPI with rising edge to strobe\r\n                  data"]
    B_0X0 = 0,
    #[doc = "1: SPI with falling edge to strobe\r\n                  data"]
    B_0X1 = 1,
    #[doc = "2: Manchester coded input on DATINy\r\n                  pin: rising edge = logic 0, falling edge = logic\r\n                  1"]
    B_0X2 = 2,
    #[doc = "3: Manchester coded input on DATINy\r\n                  pin: rising edge = logic 1, falling edge = logic\r\n                  0"]
    B_0X3 = 3,
}
impl From<SITP_A> for u8 {
    #[inline(always)]
    fn from(variant: SITP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SITP`"]
pub type SITP_R = crate::R<u8, SITP_A>;
impl SITP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SITP_A {
        match self.bits {
            0 => SITP_A::B_0X0,
            1 => SITP_A::B_0X1,
            2 => SITP_A::B_0X2,
            3 => SITP_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SITP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SITP_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SITP_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SITP_A::B_0X3
    }
}
#[doc = "Write proxy for field `SITP`"]
pub struct SITP_W<'a> {
    w: &'a mut W,
}
impl<'a> SITP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SITP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SPI with rising edge to strobe data"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SITP_A::B_0X0)
    }
    #[doc = "SPI with falling edge to strobe data"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SITP_A::B_0X1)
    }
    #[doc = "Manchester coded input on DATINy pin: rising edge = logic 0, falling edge = logic 1"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SITP_A::B_0X2)
    }
    #[doc = "Manchester coded input on DATINy pin: rising edge = logic 1, falling edge = logic 0"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SITP_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "SPICKSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPICKSEL_A {
    #[doc = "0: clock coming from external CKINy\r\n                  input - sampling point according\r\n                  SITP\\[1:0\\]"]
    B_0X0 = 0,
    #[doc = "1: clock coming from internal CKOUT\r\n                  output - sampling point according\r\n                  SITP\\[1:0\\]"]
    B_0X1 = 1,
    #[doc = "2: clock coming from internal CKOUT -\r\n                  sampling point on each second CKOUT falling\r\n                  edge."]
    B_0X2 = 2,
    #[doc = "3: clock coming from internal CKOUT\r\n                  output - sampling point on each second CKOUT\r\n                  rising edge."]
    B_0X3 = 3,
}
impl From<SPICKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPICKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPICKSEL`"]
pub type SPICKSEL_R = crate::R<u8, SPICKSEL_A>;
impl SPICKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPICKSEL_A {
        match self.bits {
            0 => SPICKSEL_A::B_0X0,
            1 => SPICKSEL_A::B_0X1,
            2 => SPICKSEL_A::B_0X2,
            3 => SPICKSEL_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPICKSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPICKSEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SPICKSEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SPICKSEL_A::B_0X3
    }
}
#[doc = "Write proxy for field `SPICKSEL`"]
pub struct SPICKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPICKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPICKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "clock coming from external CKINy input - sampling point according SITP\\[1:0\\]"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SPICKSEL_A::B_0X0)
    }
    #[doc = "clock coming from internal CKOUT output - sampling point according SITP\\[1:0\\]"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SPICKSEL_A::B_0X1)
    }
    #[doc = "clock coming from internal CKOUT - sampling point on each second CKOUT falling edge."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SPICKSEL_A::B_0X2)
    }
    #[doc = "clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SPICKSEL_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "SCDEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCDEN_A {
    #[doc = "0: Input channel y will not be guarded\r\n                  by the short-circuit detector"]
    B_0X0 = 0,
    #[doc = "1: Input channel y will be continuously\r\n                  guarded by the short-circuit\r\n                  detector"]
    B_0X1 = 1,
}
impl From<SCDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCDEN`"]
pub type SCDEN_R = crate::R<bool, SCDEN_A>;
impl SCDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCDEN_A {
        match self.bits {
            false => SCDEN_A::B_0X0,
            true => SCDEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SCDEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SCDEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SCDEN`"]
pub struct SCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input channel y will not be guarded by the short-circuit detector"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SCDEN_A::B_0X0)
    }
    #[doc = "Input channel y will be continuously guarded by the short-circuit detector"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SCDEN_A::B_0X1)
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
#[doc = "CKABEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKABEN_A {
    #[doc = "0: Clock absence detector disabled on\r\n                  channel y"]
    B_0X0 = 0,
    #[doc = "1: Clock absence detector enabled on\r\n                  channel y"]
    B_0X1 = 1,
}
impl From<CKABEN_A> for bool {
    #[inline(always)]
    fn from(variant: CKABEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKABEN`"]
pub type CKABEN_R = crate::R<bool, CKABEN_A>;
impl CKABEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKABEN_A {
        match self.bits {
            false => CKABEN_A::B_0X0,
            true => CKABEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CKABEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CKABEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `CKABEN`"]
pub struct CKABEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKABEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKABEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock absence detector disabled on channel y"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKABEN_A::B_0X0)
    }
    #[doc = "Clock absence detector enabled on channel y"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CKABEN_A::B_0X1)
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
#[doc = "CHEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN_A {
    #[doc = "0: Channel y disabled"]
    B_0X0 = 0,
    #[doc = "1: Channel y enabled"]
    B_0X1 = 1,
}
impl From<CHEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHEN`"]
pub type CHEN_R = crate::R<bool, CHEN_A>;
impl CHEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN_A {
        match self.bits {
            false => CHEN_A::B_0X0,
            true => CHEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `CHEN`"]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel y disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CHEN_A::B_0X0)
    }
    #[doc = "Channel y enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CHEN_A::B_0X1)
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
#[doc = "CHINSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHINSEL_A {
    #[doc = "0: Channel inputs are taken from pins\r\n                  of the same channel y."]
    B_0X0 = 0,
    #[doc = "1: Channel inputs are taken from pins\r\n                  of the following channel (channel (y+1) modulo\r\n                  8)."]
    B_0X1 = 1,
}
impl From<CHINSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CHINSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CHINSEL`"]
pub type CHINSEL_R = crate::R<bool, CHINSEL_A>;
impl CHINSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHINSEL_A {
        match self.bits {
            false => CHINSEL_A::B_0X0,
            true => CHINSEL_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CHINSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CHINSEL_A::B_0X1
    }
}
#[doc = "Write proxy for field `CHINSEL`"]
pub struct CHINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHINSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel inputs are taken from pins of the same channel y."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CHINSEL_A::B_0X0)
    }
    #[doc = "Channel inputs are taken from pins of the following channel (channel (y+1) modulo 8)."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CHINSEL_A::B_0X1)
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
#[doc = "DATMPX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATMPX_A {
    #[doc = "0: Data to channel y are taken from\r\n                  external serial inputs as 1-bit values.\r\n                  DFSDM_CHyDATINR register is write\r\n                  protected."]
    B_0X0 = 0,
    #[doc = "1: Data to channel y are taken from\r\n                  internal analog to digital converter ADCy+1\r\n                  output register update as 16-bit values (if\r\n                  ADCy+1 is available). Data from ADCs are written\r\n                  into INDAT0\\[15:0\\]
part of DFSDM_CHyDATINR\r\n                  register."]
    B_0X1 = 1,
    #[doc = "2: Data to channel y are taken from\r\n                  internal DFSDM_CHyDATINR register by direct\r\n                  CPU/DMA write. There can be written one or two\r\n                  16-bit data samples according DATPACK\\[1:0\\]
bit\r\n                  field setting."]
    B_0X2 = 2,
}
impl From<DATMPX_A> for u8 {
    #[inline(always)]
    fn from(variant: DATMPX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATMPX`"]
pub type DATMPX_R = crate::R<u8, DATMPX_A>;
impl DATMPX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATMPX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DATMPX_A::B_0X0),
            1 => Val(DATMPX_A::B_0X1),
            2 => Val(DATMPX_A::B_0X2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DATMPX_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DATMPX_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DATMPX_A::B_0X2
    }
}
#[doc = "Write proxy for field `DATMPX`"]
pub struct DATMPX_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMPX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMPX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Data to channel y are taken from external serial inputs as 1-bit values. DFSDM_CHyDATINR register is write protected."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DATMPX_A::B_0X0)
    }
    #[doc = "Data to channel y are taken from internal analog to digital converter ADCy+1 output register update as 16-bit values (if ADCy+1 is available). Data from ADCs are written into INDAT0\\[15:0\\]
part of DFSDM_CHyDATINR register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DATMPX_A::B_0X1)
    }
    #[doc = "Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DATMPX_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "DATPACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATPACK_A {
    #[doc = "0: Standard: input data in\r\n                  DFSDM_CHyDATINR register are stored only in\r\n                  INDAT0\\[15:0\\]. To empty DFSDM_CHyDATINR register\r\n                  one sample must be read by the DFSDM filter from\r\n                  channel y."]
    B_0X0 = 0,
    #[doc = "1: Interleaved: input data in\r\n                  DFSDM_CHyDATINR register are stored as two\r\n                  samples:"]
    B_0X1 = 1,
    #[doc = "2: Dual: input data in DFSDM_CHyDATINR\r\n                  register are stored as two samples:"]
    B_0X2 = 2,
}
impl From<DATPACK_A> for u8 {
    #[inline(always)]
    fn from(variant: DATPACK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DATPACK`"]
pub type DATPACK_R = crate::R<u8, DATPACK_A>;
impl DATPACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATPACK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DATPACK_A::B_0X0),
            1 => Val(DATPACK_A::B_0X1),
            2 => Val(DATPACK_A::B_0X2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DATPACK_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DATPACK_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DATPACK_A::B_0X2
    }
}
#[doc = "Write proxy for field `DATPACK`"]
pub struct DATPACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DATPACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATPACK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Standard: input data in DFSDM_CHyDATINR register are stored only in INDAT0\\[15:0\\]. To empty DFSDM_CHyDATINR register one sample must be read by the DFSDM filter from channel y."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DATPACK_A::B_0X0)
    }
    #[doc = "Interleaved: input data in DFSDM_CHyDATINR register are stored as two samples:"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DATPACK_A::B_0X1)
    }
    #[doc = "Dual: input data in DFSDM_CHyDATINR register are stored as two samples:"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DATPACK_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "CKOUTDIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKOUTDIV_A {
    #[doc = "0: Output clock generation is disabled\r\n                  (CKOUT signal is set to low state)"]
    B_0X0 = 0,
}
impl From<CKOUTDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CKOUTDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CKOUTDIV`"]
pub type CKOUTDIV_R = crate::R<u8, CKOUTDIV_A>;
impl CKOUTDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CKOUTDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CKOUTDIV_A::B_0X0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CKOUTDIV_A::B_0X0
    }
}
#[doc = "Write proxy for field `CKOUTDIV`"]
pub struct CKOUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKOUTDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Output clock generation is disabled (CKOUT signal is set to low state)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKOUTDIV_A::B_0X0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "CKOUTSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKOUTSRC_A {
    #[doc = "0: Source for output clock is from\r\n                  system clock"]
    B_0X0 = 0,
    #[doc = "1: Source for output clock is from\r\n                  audio clock"]
    B_0X1 = 1,
}
impl From<CKOUTSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CKOUTSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKOUTSRC`"]
pub type CKOUTSRC_R = crate::R<bool, CKOUTSRC_A>;
impl CKOUTSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKOUTSRC_A {
        match self.bits {
            false => CKOUTSRC_A::B_0X0,
            true => CKOUTSRC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CKOUTSRC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CKOUTSRC_A::B_0X1
    }
}
#[doc = "Write proxy for field `CKOUTSRC`"]
pub struct CKOUTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKOUTSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKOUTSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Source for output clock is from system clock"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CKOUTSRC_A::B_0X0)
    }
    #[doc = "Source for output clock is from audio clock"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CKOUTSRC_A::B_0X1)
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
#[doc = "DFSDMEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFSDMEN_A {
    #[doc = "0: DFSDM interface\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: DFSDM interface\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<DFSDMEN_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFSDMEN`"]
pub type DFSDMEN_R = crate::R<bool, DFSDMEN_A>;
impl DFSDMEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDMEN_A {
        match self.bits {
            false => DFSDMEN_A::B_0X0,
            true => DFSDMEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DFSDMEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DFSDMEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DFSDMEN`"]
pub struct DFSDMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDMEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFSDMEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DFSDM interface disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DFSDMEN_A::B_0X0)
    }
    #[doc = "DFSDM interface enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DFSDMEN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    pub fn sitp(&self) -> SITP_R {
        SITP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    pub fn spicksel(&self) -> SPICKSEL_R {
        SPICKSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    pub fn ckaben(&self) -> CKABEN_R {
        CKABEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    pub fn chinsel(&self) -> CHINSEL_R {
        CHINSEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    pub fn datmpx(&self) -> DATMPX_R {
        DATMPX_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    pub fn datpack(&self) -> DATPACK_R {
        DATPACK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - CKOUTDIV"]
    #[inline(always)]
    pub fn ckoutdiv(&self) -> CKOUTDIV_R {
        CKOUTDIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&self) -> CKOUTSRC_R {
        CKOUTSRC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&self) -> DFSDMEN_R {
        DFSDMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SITP"]
    #[inline(always)]
    pub fn sitp(&mut self) -> SITP_W {
        SITP_W { w: self }
    }
    #[doc = "Bits 2:3 - SPICKSEL"]
    #[inline(always)]
    pub fn spicksel(&mut self) -> SPICKSEL_W {
        SPICKSEL_W { w: self }
    }
    #[doc = "Bit 5 - SCDEN"]
    #[inline(always)]
    pub fn scden(&mut self) -> SCDEN_W {
        SCDEN_W { w: self }
    }
    #[doc = "Bit 6 - CKABEN"]
    #[inline(always)]
    pub fn ckaben(&mut self) -> CKABEN_W {
        CKABEN_W { w: self }
    }
    #[doc = "Bit 7 - CHEN"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Bit 8 - CHINSEL"]
    #[inline(always)]
    pub fn chinsel(&mut self) -> CHINSEL_W {
        CHINSEL_W { w: self }
    }
    #[doc = "Bits 12:13 - DATMPX"]
    #[inline(always)]
    pub fn datmpx(&mut self) -> DATMPX_W {
        DATMPX_W { w: self }
    }
    #[doc = "Bits 14:15 - DATPACK"]
    #[inline(always)]
    pub fn datpack(&mut self) -> DATPACK_W {
        DATPACK_W { w: self }
    }
    #[doc = "Bits 16:23 - CKOUTDIV"]
    #[inline(always)]
    pub fn ckoutdiv(&mut self) -> CKOUTDIV_W {
        CKOUTDIV_W { w: self }
    }
    #[doc = "Bit 30 - CKOUTSRC"]
    #[inline(always)]
    pub fn ckoutsrc(&mut self) -> CKOUTSRC_W {
        CKOUTSRC_W { w: self }
    }
    #[doc = "Bit 31 - DFSDMEN"]
    #[inline(always)]
    pub fn dfsdmen(&mut self) -> DFSDMEN_W {
        DFSDMEN_W { w: self }
    }
}
