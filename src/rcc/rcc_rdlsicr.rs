#[doc = "Reader of register RCC_RDLSICR"]
pub type R = crate::R<u32, super::RCC_RDLSICR>;
#[doc = "Writer for register RCC_RDLSICR"]
pub type W = crate::W<u32, super::RCC_RDLSICR>;
#[doc = "Register RCC_RDLSICR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_RDLSICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LSION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSION_A {
    #[doc = "0: LSI oscillator OFF (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: LSI oscillator ON"]
    B_0X1 = 1,
}
impl From<LSION_A> for bool {
    #[inline(always)]
    fn from(variant: LSION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSION`"]
pub type LSION_R = crate::R<bool, LSION_A>;
impl LSION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSION_A {
        match self.bits {
            false => LSION_A::B_0X0,
            true => LSION_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSION_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSION_A::B_0X1
    }
}
#[doc = "Write proxy for field `LSION`"]
pub struct LSION_W<'a> {
    w: &'a mut W,
}
impl<'a> LSION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSI oscillator OFF (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSION_A::B_0X0)
    }
    #[doc = "LSI oscillator ON"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSION_A::B_0X1)
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
#[doc = "LSIRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDY_A {
    #[doc = "0: LSI oscillator not ready (default\r\n                  after reset)"]
    B_0X0 = 0,
    #[doc = "1: LSI oscillator ready"]
    B_0X1 = 1,
}
impl From<LSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSIRDY`"]
pub type LSIRDY_R = crate::R<bool, LSIRDY_A>;
impl LSIRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDY_A {
        match self.bits {
            false => LSIRDY_A::B_0X0,
            true => LSIRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSIRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSIRDY_A::B_0X1
    }
}
#[doc = "MRD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MRD_A {
    #[doc = "0: NRST low pulse duration is\r\n                  guaranteed by the pulse stretcher of the PAD. The\r\n                  RPCTL is bypassed (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: The guaranteed NRST low pulse\r\n                  duration is about 1 ms (1 x 32 lsi_ck\r\n                  cycles),"]
    B_0X1 = 1,
    #[doc = "2: The guaranteed NRST low pulse\r\n                  duration is about 2 ms (2 x 32 lsi_ck\r\n                  cycles),"]
    B_0X2 = 2,
    #[doc = "31: The guaranteed NRST low pulse\r\n                  duration is about 31 ms (31 x 32 lsi_ck\r\n                  cycles)."]
    B_0X1F = 31,
}
impl From<MRD_A> for u8 {
    #[inline(always)]
    fn from(variant: MRD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MRD`"]
pub type MRD_R = crate::R<u8, MRD_A>;
impl MRD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MRD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MRD_A::B_0X0),
            1 => Val(MRD_A::B_0X1),
            2 => Val(MRD_A::B_0X2),
            31 => Val(MRD_A::B_0X1F),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MRD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MRD_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MRD_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X1F`"]
    #[inline(always)]
    pub fn is_b_0x1f(&self) -> bool {
        *self == MRD_A::B_0X1F
    }
}
#[doc = "Write proxy for field `MRD`"]
pub struct MRD_W<'a> {
    w: &'a mut W,
}
impl<'a> MRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "NRST low pulse duration is guaranteed by the pulse stretcher of the PAD. The RPCTL is bypassed (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MRD_A::B_0X0)
    }
    #[doc = "The guaranteed NRST low pulse duration is about 1 ms (1 x 32 lsi_ck cycles),"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MRD_A::B_0X1)
    }
    #[doc = "The guaranteed NRST low pulse duration is about 2 ms (2 x 32 lsi_ck cycles),"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(MRD_A::B_0X2)
    }
    #[doc = "The guaranteed NRST low pulse duration is about 31 ms (31 x 32 lsi_ck cycles)."]
    #[inline(always)]
    pub fn b_0x1f(self) -> &'a mut W {
        self.variant(MRD_A::B_0X1F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "EADLY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EADLY_A {
    #[doc = "0: 10 ms (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: No extra delay added by the\r\n                  BOOTROM"]
    B_0X1 = 1,
    #[doc = "2: 100 us"]
    B_0X2 = 2,
    #[doc = "3: 200 us"]
    B_0X3 = 3,
    #[doc = "4: 500 us"]
    B_0X4 = 4,
    #[doc = "5: 1 ms"]
    B_0X5 = 5,
    #[doc = "6: 2 ms"]
    B_0X6 = 6,
    #[doc = "7: 5 ms"]
    B_0X7 = 7,
}
impl From<EADLY_A> for u8 {
    #[inline(always)]
    fn from(variant: EADLY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EADLY`"]
pub type EADLY_R = crate::R<u8, EADLY_A>;
impl EADLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EADLY_A {
        match self.bits {
            0 => EADLY_A::B_0X0,
            1 => EADLY_A::B_0X1,
            2 => EADLY_A::B_0X2,
            3 => EADLY_A::B_0X3,
            4 => EADLY_A::B_0X4,
            5 => EADLY_A::B_0X5,
            6 => EADLY_A::B_0X6,
            7 => EADLY_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EADLY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EADLY_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == EADLY_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == EADLY_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == EADLY_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == EADLY_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == EADLY_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == EADLY_A::B_0X7
    }
}
#[doc = "Write proxy for field `EADLY`"]
pub struct EADLY_W<'a> {
    w: &'a mut W,
}
impl<'a> EADLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EADLY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "10 ms (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EADLY_A::B_0X0)
    }
    #[doc = "No extra delay added by the BOOTROM"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EADLY_A::B_0X1)
    }
    #[doc = "100 us"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(EADLY_A::B_0X2)
    }
    #[doc = "200 us"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(EADLY_A::B_0X3)
    }
    #[doc = "500 us"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(EADLY_A::B_0X4)
    }
    #[doc = "1 ms"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(EADLY_A::B_0X5)
    }
    #[doc = "2 ms"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(EADLY_A::B_0X6)
    }
    #[doc = "5 ms"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(EADLY_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPARE`"]
pub type SPARE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPARE`"]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LSION"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - MRD"]
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - EADLY"]
    #[inline(always)]
    pub fn eadly(&self) -> EADLY_R {
        EADLY_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:31 - SPARE"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LSION"]
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W {
        LSION_W { w: self }
    }
    #[doc = "Bits 16:20 - MRD"]
    #[inline(always)]
    pub fn mrd(&mut self) -> MRD_W {
        MRD_W { w: self }
    }
    #[doc = "Bits 24:26 - EADLY"]
    #[inline(always)]
    pub fn eadly(&mut self) -> EADLY_W {
        EADLY_W { w: self }
    }
    #[doc = "Bits 27:31 - SPARE"]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
}
