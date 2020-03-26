#[doc = "Reader of register MDMA_C17TCR"]
pub type R = crate::R<u32, super::MDMA_C17TCR>;
#[doc = "Writer for register MDMA_C17TCR"]
pub type W = crate::W<u32, super::MDMA_C17TCR>;
#[doc = "Register MDMA_C17TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C17TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SINC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SINC_A {
    #[doc = "0: Source address pointer is\r\n                  fixed"]
    B_0X0 = 0,
    #[doc = "2: Source address pointer is\r\n                  incremented after each data transfer (increment\r\n                  is done according to SINCOS)"]
    B_0X2 = 2,
    #[doc = "3: Source address pointer is\r\n                  decremented after each data transfer (increment\r\n                  is done according to SINCOS)"]
    B_0X3 = 3,
}
impl From<SINC_A> for u8 {
    #[inline(always)]
    fn from(variant: SINC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SINC`"]
pub type SINC_R = crate::R<u8, SINC_A>;
impl SINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SINC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SINC_A::B_0X0),
            2 => Val(SINC_A::B_0X2),
            3 => Val(SINC_A::B_0X3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SINC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SINC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SINC_A::B_0X3
    }
}
#[doc = "Write proxy for field `SINC`"]
pub struct SINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SINC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Source address pointer is fixed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SINC_A::B_0X0)
    }
    #[doc = "Source address pointer is incremented after each data transfer (increment is done according to SINCOS)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SINC_A::B_0X2)
    }
    #[doc = "Source address pointer is decremented after each data transfer (increment is done according to SINCOS)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SINC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "DINC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DINC_A {
    #[doc = "0: Destination address pointer is\r\n                  fixed"]
    B_0X0 = 0,
    #[doc = "2: Destination address pointer is\r\n                  incremented after each data transfer (increment\r\n                  is done according to DINCOS)"]
    B_0X2 = 2,
    #[doc = "3: Destination address pointer is\r\n                  decremented after each data transfer (increment\r\n                  is done according to DINCOS)"]
    B_0X3 = 3,
}
impl From<DINC_A> for u8 {
    #[inline(always)]
    fn from(variant: DINC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DINC`"]
pub type DINC_R = crate::R<u8, DINC_A>;
impl DINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DINC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DINC_A::B_0X0),
            2 => Val(DINC_A::B_0X2),
            3 => Val(DINC_A::B_0X3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DINC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DINC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == DINC_A::B_0X3
    }
}
#[doc = "Write proxy for field `DINC`"]
pub struct DINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Destination address pointer is fixed"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DINC_A::B_0X0)
    }
    #[doc = "Destination address pointer is incremented after each data transfer (increment is done according to DINCOS)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DINC_A::B_0X2)
    }
    #[doc = "Destination address pointer is decremented after each data transfer (increment is done according to DINCOS)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(DINC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "SSIZE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SSIZE_A {
    #[doc = "0: Byte (8-bit)"]
    B_0X0 = 0,
    #[doc = "1: Half-word (16-bit)"]
    B_0X1 = 1,
    #[doc = "2: Word (32-bit)"]
    B_0X2 = 2,
    #[doc = "3: Double-Word (64-bit)"]
    B_0X3 = 3,
}
impl From<SSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SSIZE`"]
pub type SSIZE_R = crate::R<u8, SSIZE_A>;
impl SSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSIZE_A {
        match self.bits {
            0 => SSIZE_A::B_0X0,
            1 => SSIZE_A::B_0X1,
            2 => SSIZE_A::B_0X2,
            3 => SSIZE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SSIZE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SSIZE_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SSIZE_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SSIZE_A::B_0X3
    }
}
#[doc = "Write proxy for field `SSIZE`"]
pub struct SSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Byte (8-bit)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SSIZE_A::B_0X0)
    }
    #[doc = "Half-word (16-bit)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SSIZE_A::B_0X1)
    }
    #[doc = "Word (32-bit)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SSIZE_A::B_0X2)
    }
    #[doc = "Double-Word (64-bit)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SSIZE_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "DSIZE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSIZE_A {
    #[doc = "0: byte (8-bit)"]
    B_0X0 = 0,
    #[doc = "1: half-word (16-bit)"]
    B_0X1 = 1,
    #[doc = "2: word (32-bit)"]
    B_0X2 = 2,
    #[doc = "3: Double-Word (64-bit) -"]
    B_0X3 = 3,
}
impl From<DSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSIZE`"]
pub type DSIZE_R = crate::R<u8, DSIZE_A>;
impl DSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSIZE_A {
        match self.bits {
            0 => DSIZE_A::B_0X0,
            1 => DSIZE_A::B_0X1,
            2 => DSIZE_A::B_0X2,
            3 => DSIZE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DSIZE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DSIZE_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DSIZE_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == DSIZE_A::B_0X3
    }
}
#[doc = "Write proxy for field `DSIZE`"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "byte (8-bit)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DSIZE_A::B_0X0)
    }
    #[doc = "half-word (16-bit)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DSIZE_A::B_0X1)
    }
    #[doc = "word (32-bit)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DSIZE_A::B_0X2)
    }
    #[doc = "Double-Word (64-bit) -"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(DSIZE_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "SINCOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SINCOS_A {
    #[doc = "0: byte (8-bit)"]
    B_0X0 = 0,
    #[doc = "1: half-word (16-bit)"]
    B_0X1 = 1,
    #[doc = "2: word (32-bit)"]
    B_0X2 = 2,
    #[doc = "3: Double-Word (64-bit) -"]
    B_0X3 = 3,
}
impl From<SINCOS_A> for u8 {
    #[inline(always)]
    fn from(variant: SINCOS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SINCOS`"]
pub type SINCOS_R = crate::R<u8, SINCOS_A>;
impl SINCOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINCOS_A {
        match self.bits {
            0 => SINCOS_A::B_0X0,
            1 => SINCOS_A::B_0X1,
            2 => SINCOS_A::B_0X2,
            3 => SINCOS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SINCOS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SINCOS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == SINCOS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == SINCOS_A::B_0X3
    }
}
#[doc = "Write proxy for field `SINCOS`"]
pub struct SINCOS_W<'a> {
    w: &'a mut W,
}
impl<'a> SINCOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SINCOS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "byte (8-bit)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SINCOS_A::B_0X0)
    }
    #[doc = "half-word (16-bit)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SINCOS_A::B_0X1)
    }
    #[doc = "word (32-bit)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(SINCOS_A::B_0X2)
    }
    #[doc = "Double-Word (64-bit) -"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(SINCOS_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "DINCOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DINCOS_A {
    #[doc = "0: byte (8-bit)"]
    B_0X0 = 0,
    #[doc = "1: half-word (16-bit)"]
    B_0X1 = 1,
    #[doc = "2: word (32-bit)"]
    B_0X2 = 2,
    #[doc = "3: Double-Word (64-bit) -"]
    B_0X3 = 3,
}
impl From<DINCOS_A> for u8 {
    #[inline(always)]
    fn from(variant: DINCOS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DINCOS`"]
pub type DINCOS_R = crate::R<u8, DINCOS_A>;
impl DINCOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINCOS_A {
        match self.bits {
            0 => DINCOS_A::B_0X0,
            1 => DINCOS_A::B_0X1,
            2 => DINCOS_A::B_0X2,
            3 => DINCOS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DINCOS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DINCOS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DINCOS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == DINCOS_A::B_0X3
    }
}
#[doc = "Write proxy for field `DINCOS`"]
pub struct DINCOS_W<'a> {
    w: &'a mut W,
}
impl<'a> DINCOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINCOS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "byte (8-bit)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DINCOS_A::B_0X0)
    }
    #[doc = "half-word (16-bit)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DINCOS_A::B_0X1)
    }
    #[doc = "word (32-bit)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(DINCOS_A::B_0X2)
    }
    #[doc = "Double-Word (64-bit) -"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(DINCOS_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "SBURST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SBURST_A {
    #[doc = "0: single transfer N: burst of 2^N\r\n                  beats"]
    B_0X0 = 0,
}
impl From<SBURST_A> for u8 {
    #[inline(always)]
    fn from(variant: SBURST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SBURST`"]
pub type SBURST_R = crate::R<u8, SBURST_A>;
impl SBURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SBURST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SBURST_A::B_0X0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SBURST_A::B_0X0
    }
}
#[doc = "Write proxy for field `SBURST`"]
pub struct SBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> SBURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBURST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "single transfer N: burst of 2^N beats"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SBURST_A::B_0X0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "DBURST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBURST_A {
    #[doc = "0: single transfer N: burst of 2^N\r\n                  beats These bits are protected and can be written\r\n                  only if EN is 0"]
    B_0X0 = 0,
}
impl From<DBURST_A> for u8 {
    #[inline(always)]
    fn from(variant: DBURST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DBURST`"]
pub type DBURST_R = crate::R<u8, DBURST_A>;
impl DBURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DBURST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DBURST_A::B_0X0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DBURST_A::B_0X0
    }
}
#[doc = "Write proxy for field `DBURST`"]
pub struct DBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> DBURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBURST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "single transfer N: burst of 2^N beats These bits are protected and can be written only if EN is 0"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DBURST_A::B_0X0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Reader of field `TLEN`"]
pub type TLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLEN`"]
pub struct TLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 18)) | (((value as u32) & 0x7f) << 18);
        self.w
    }
}
#[doc = "PKE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PKE_A {
    #[doc = "0: The source data is written to the\r\n                  destination as is."]
    B_0X0 = 0,
    #[doc = "1: The source data is packed/un-packed\r\n                  into the destination data size. All data are\r\n                  right aligned, in Little Endian\r\n                  mode."]
    B_0X1 = 1,
}
impl From<PKE_A> for bool {
    #[inline(always)]
    fn from(variant: PKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PKE`"]
pub type PKE_R = crate::R<bool, PKE_A>;
impl PKE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PKE_A {
        match self.bits {
            false => PKE_A::B_0X0,
            true => PKE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PKE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PKE_A::B_0X1
    }
}
#[doc = "Write proxy for field `PKE`"]
pub struct PKE_W<'a> {
    w: &'a mut W,
}
impl<'a> PKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PKE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The source data is written to the destination as is."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PKE_A::B_0X0)
    }
    #[doc = "The source data is packed/un-packed into the destination data size. All data are right aligned, in Little Endian mode."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PKE_A::B_0X1)
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
#[doc = "PAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAM_A {
    #[doc = "0: Right Aligned - only the LSBs part\r\n                  of the Source is written to the destination\r\n                  address"]
    B_0X0 = 0,
    #[doc = "1: Right Aligned, Sign\r\n                  extended"]
    B_0X1 = 1,
    #[doc = "2: Left Aligned - only the MSBs part of\r\n                  the Source is written to the destination\r\n                  address"]
    B_0X2 = 2,
}
impl From<PAM_A> for u8 {
    #[inline(always)]
    fn from(variant: PAM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAM`"]
pub type PAM_R = crate::R<u8, PAM_A>;
impl PAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PAM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PAM_A::B_0X0),
            1 => Val(PAM_A::B_0X1),
            2 => Val(PAM_A::B_0X2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PAM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PAM_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PAM_A::B_0X2
    }
}
#[doc = "Write proxy for field `PAM`"]
pub struct PAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Right Aligned - only the LSBs part of the Source is written to the destination address"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PAM_A::B_0X0)
    }
    #[doc = "Right Aligned, Sign extended"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PAM_A::B_0X1)
    }
    #[doc = "Left Aligned - only the MSBs part of the Source is written to the destination address"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PAM_A::B_0X2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "TRGM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGM_A {
    #[doc = "0: Each MDMA request (SW or HW)\r\n                  triggers a buffer transfer"]
    B_0X0 = 0,
    #[doc = "1: Each MDMA request (SW or HW)\r\n                  triggers a block transfer"]
    B_0X1 = 1,
    #[doc = "2: Each MDMA request (SW or HW)\r\n                  triggers a repeated block transfer (if the block\r\n                  repeat is 0, a single block is\r\n                  transferred)"]
    B_0X2 = 2,
    #[doc = "3: Each MDMA request (SW or HW)\r\n                  triggers the transfer of the whole data for the\r\n                  respective channel (e.g. linked list) until the\r\n                  channel reach the end and it is\r\n                  disabled."]
    B_0X3 = 3,
}
impl From<TRGM_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRGM`"]
pub type TRGM_R = crate::R<u8, TRGM_A>;
impl TRGM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGM_A {
        match self.bits {
            0 => TRGM_A::B_0X0,
            1 => TRGM_A::B_0X1,
            2 => TRGM_A::B_0X2,
            3 => TRGM_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TRGM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TRGM_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TRGM_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TRGM_A::B_0X3
    }
}
#[doc = "Write proxy for field `TRGM`"]
pub struct TRGM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Each MDMA request (SW or HW) triggers a buffer transfer"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TRGM_A::B_0X0)
    }
    #[doc = "Each MDMA request (SW or HW) triggers a block transfer"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TRGM_A::B_0X1)
    }
    #[doc = "Each MDMA request (SW or HW) triggers a repeated block transfer (if the block repeat is 0, a single block is transferred)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TRGM_A::B_0X2)
    }
    #[doc = "Each MDMA request (SW or HW) triggers the transfer of the whole data for the respective channel (e.g. linked list) until the channel reach the end and it is disabled."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TRGM_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "SWRM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRM_A {
    #[doc = "0: HW request are taken into account:\r\n                  the transfer is initiated as defined by TRGM\r\n                  value and acknowledged by the MDMA ACKx\r\n                  signal."]
    B_0X0 = 0,
    #[doc = "1: HW request are ignored. Transfer is\r\n                  trigerred by SW writing 1 to the SWRQ\r\n                  bit."]
    B_0X1 = 1,
}
impl From<SWRM_A> for bool {
    #[inline(always)]
    fn from(variant: SWRM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWRM`"]
pub type SWRM_R = crate::R<bool, SWRM_A>;
impl SWRM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRM_A {
        match self.bits {
            false => SWRM_A::B_0X0,
            true => SWRM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SWRM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SWRM_A::B_0X1
    }
}
#[doc = "Write proxy for field `SWRM`"]
pub struct SWRM_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HW request are taken into account: the transfer is initiated as defined by TRGM value and acknowledged by the MDMA ACKx signal."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SWRM_A::B_0X0)
    }
    #[doc = "HW request are ignored. Transfer is trigerred by SW writing 1 to the SWRQ bit."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SWRM_A::B_0X1)
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
#[doc = "BWM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWM_A {
    #[doc = "0: The destination write operation is\r\n                  non-bufferable."]
    B_0X0 = 0,
    #[doc = "1: The destination write operation is\r\n                  bufferable."]
    B_0X1 = 1,
}
impl From<BWM_A> for bool {
    #[inline(always)]
    fn from(variant: BWM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BWM`"]
pub type BWM_R = crate::R<bool, BWM_A>;
impl BWM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWM_A {
        match self.bits {
            false => BWM_A::B_0X0,
            true => BWM_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BWM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BWM_A::B_0X1
    }
}
#[doc = "Write proxy for field `BWM`"]
pub struct BWM_W<'a> {
    w: &'a mut W,
}
impl<'a> BWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BWM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The destination write operation is non-bufferable."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BWM_A::B_0X0)
    }
    #[doc = "The destination write operation is bufferable."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BWM_A::B_0X1)
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
    #[doc = "Bits 0:1 - SINC"]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - DINC"]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - SSIZE"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - DSIZE"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - SINCOS"]
    #[inline(always)]
    pub fn sincos(&self) -> SINCOS_R {
        SINCOS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - DINCOS"]
    #[inline(always)]
    pub fn dincos(&self) -> DINCOS_R {
        DINCOS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - SBURST"]
    #[inline(always)]
    pub fn sburst(&self) -> SBURST_R {
        SBURST_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - DBURST"]
    #[inline(always)]
    pub fn dburst(&self) -> DBURST_R {
        DBURST_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:24 - TLEN"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bit 25 - PKE"]
    #[inline(always)]
    pub fn pke(&self) -> PKE_R {
        PKE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - PAM"]
    #[inline(always)]
    pub fn pam(&self) -> PAM_R {
        PAM_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - TRGM"]
    #[inline(always)]
    pub fn trgm(&self) -> TRGM_R {
        TRGM_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - SWRM"]
    #[inline(always)]
    pub fn swrm(&self) -> SWRM_R {
        SWRM_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - BWM"]
    #[inline(always)]
    pub fn bwm(&self) -> BWM_R {
        BWM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SINC"]
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W {
        SINC_W { w: self }
    }
    #[doc = "Bits 2:3 - DINC"]
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W {
        DINC_W { w: self }
    }
    #[doc = "Bits 4:5 - SSIZE"]
    #[inline(always)]
    pub fn ssize(&mut self) -> SSIZE_W {
        SSIZE_W { w: self }
    }
    #[doc = "Bits 6:7 - DSIZE"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bits 8:9 - SINCOS"]
    #[inline(always)]
    pub fn sincos(&mut self) -> SINCOS_W {
        SINCOS_W { w: self }
    }
    #[doc = "Bits 10:11 - DINCOS"]
    #[inline(always)]
    pub fn dincos(&mut self) -> DINCOS_W {
        DINCOS_W { w: self }
    }
    #[doc = "Bits 12:14 - SBURST"]
    #[inline(always)]
    pub fn sburst(&mut self) -> SBURST_W {
        SBURST_W { w: self }
    }
    #[doc = "Bits 15:17 - DBURST"]
    #[inline(always)]
    pub fn dburst(&mut self) -> DBURST_W {
        DBURST_W { w: self }
    }
    #[doc = "Bits 18:24 - TLEN"]
    #[inline(always)]
    pub fn tlen(&mut self) -> TLEN_W {
        TLEN_W { w: self }
    }
    #[doc = "Bit 25 - PKE"]
    #[inline(always)]
    pub fn pke(&mut self) -> PKE_W {
        PKE_W { w: self }
    }
    #[doc = "Bits 26:27 - PAM"]
    #[inline(always)]
    pub fn pam(&mut self) -> PAM_W {
        PAM_W { w: self }
    }
    #[doc = "Bits 28:29 - TRGM"]
    #[inline(always)]
    pub fn trgm(&mut self) -> TRGM_W {
        TRGM_W { w: self }
    }
    #[doc = "Bit 30 - SWRM"]
    #[inline(always)]
    pub fn swrm(&mut self) -> SWRM_W {
        SWRM_W { w: self }
    }
    #[doc = "Bit 31 - BWM"]
    #[inline(always)]
    pub fn bwm(&mut self) -> BWM_W {
        BWM_W { w: self }
    }
}
