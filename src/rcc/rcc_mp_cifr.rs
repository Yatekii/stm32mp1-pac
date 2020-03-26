#[doc = "Reader of register RCC_MP_CIFR"]
pub type R = crate::R<u32, super::RCC_MP_CIFR>;
#[doc = "Writer for register RCC_MP_CIFR"]
pub type W = crate::W<u32, super::RCC_MP_CIFR>;
#[doc = "Register RCC_MP_CIFR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_CIFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LSIRDYF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYF_A {
    #[doc = "0: No clock ready interrupt caused by\r\n                  the LSI (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by the\r\n                  LSI, writing clears this flag"]
    B_0X1 = 1,
}
impl From<LSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSIRDYF`"]
pub type LSIRDYF_R = crate::R<bool, LSIRDYF_A>;
impl LSIRDYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYF_A {
        match self.bits {
            false => LSIRDYF_A::B_0X0,
            true => LSIRDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSIRDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSIRDYF_A::B_0X1
    }
}
#[doc = "Write proxy for field `LSIRDYF`"]
pub struct LSIRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSIRDYF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock ready interrupt caused by the LSI (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSIRDYF_A::B_0X0)
    }
    #[doc = "Clock ready interrupt caused by the LSI, writing clears this flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSIRDYF_A::B_0X1)
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
#[doc = "LSERDYF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDYF_A {
    #[doc = "0: No clock ready interrupt caused by\r\n                  the LSE (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by the\r\n                  LSE, writing clears this flag"]
    B_0X1 = 1,
}
impl From<LSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSERDYF`"]
pub type LSERDYF_R = crate::R<bool, LSERDYF_A>;
impl LSERDYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDYF_A {
        match self.bits {
            false => LSERDYF_A::B_0X0,
            true => LSERDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSERDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSERDYF_A::B_0X1
    }
}
#[doc = "Write proxy for field `LSERDYF`"]
pub struct LSERDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSERDYF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock ready interrupt caused by the LSE (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSERDYF_A::B_0X0)
    }
    #[doc = "Clock ready interrupt caused by the LSE, writing clears this flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSERDYF_A::B_0X1)
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
#[doc = "HSIRDYF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIRDYF_A {
    #[doc = "0: No clock ready interrupt caused by\r\n                  the HSI (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by the\r\n                  HSI, writing clears this flag"]
    B_0X1 = 1,
}
impl From<HSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSIRDYF`"]
pub type HSIRDYF_R = crate::R<bool, HSIRDYF_A>;
impl HSIRDYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYF_A {
        match self.bits {
            false => HSIRDYF_A::B_0X0,
            true => HSIRDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIRDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIRDYF_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSIRDYF`"]
pub struct HSIRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIRDYF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock ready interrupt caused by the HSI (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSIRDYF_A::B_0X0)
    }
    #[doc = "Clock ready interrupt caused by the HSI, writing clears this flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSIRDYF_A::B_0X1)
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
#[doc = "HSERDYF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSERDYF_A {
    #[doc = "0: No clock ready interrupt caused by\r\n                  the HSE (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by the\r\n                  HSE, writing clears this flag"]
    B_0X1 = 1,
}
impl From<HSERDYF_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSERDYF`"]
pub type HSERDYF_R = crate::R<bool, HSERDYF_A>;
impl HSERDYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSERDYF_A {
        match self.bits {
            false => HSERDYF_A::B_0X0,
            true => HSERDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSERDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSERDYF_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSERDYF`"]
pub struct HSERDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSERDYF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock ready interrupt caused by the HSE (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSERDYF_A::B_0X0)
    }
    #[doc = "Clock ready interrupt caused by the HSE, writing clears this flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSERDYF_A::B_0X1)
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
#[doc = "CSIRDYF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIRDYF_A {
    #[doc = "0: No clock ready interrupt caused by\r\n                  the CSI (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by the\r\n                  CSI, writing clears this flag"]
    B_0X1 = 1,
}
impl From<CSIRDYF_A> for bool {
    #[inline(always)]
    fn from(variant: CSIRDYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSIRDYF`"]
pub type CSIRDYF_R = crate::R<bool, CSIRDYF_A>;
impl CSIRDYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSIRDYF_A {
        match self.bits {
            false => CSIRDYF_A::B_0X0,
            true => CSIRDYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSIRDYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSIRDYF_A::B_0X1
    }
}
#[doc = "Write proxy for field `CSIRDYF`"]
pub struct CSIRDYF_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIRDYF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSIRDYF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock ready interrupt caused by the CSI (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CSIRDYF_A::B_0X0)
    }
    #[doc = "Clock ready interrupt caused by the CSI, writing clears this flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CSIRDYF_A::B_0X1)
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
#[doc = "PLL1DYF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL1DYF_A {
    #[doc = "0: No clock ready interrupt caused by\r\n                  PLL1 lock (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by PLL1\r\n                  lock, writing clears this flag"]
    B_0X1 = 1,
}
impl From<PLL1DYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1DYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL1DYF`"]
pub type PLL1DYF_R = crate::R<bool, PLL1DYF_A>;
impl PLL1DYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL1DYF_A {
        match self.bits {
            false => PLL1DYF_A::B_0X0,
            true => PLL1DYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL1DYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL1DYF_A::B_0X1
    }
}
#[doc = "Write proxy for field `PLL1DYF`"]
pub struct PLL1DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1DYF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL1DYF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock ready interrupt caused by PLL1 lock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLL1DYF_A::B_0X0)
    }
    #[doc = "Clock ready interrupt caused by PLL1 lock, writing clears this flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLL1DYF_A::B_0X1)
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
#[doc = "PLL2DYF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL2DYF_A {
    #[doc = "0: No clock ready interrupt caused by\r\n                  PLL2 lock (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by PLL2\r\n                  lock, writing clears this flag"]
    B_0X1 = 1,
}
impl From<PLL2DYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2DYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL2DYF`"]
pub type PLL2DYF_R = crate::R<bool, PLL2DYF_A>;
impl PLL2DYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL2DYF_A {
        match self.bits {
            false => PLL2DYF_A::B_0X0,
            true => PLL2DYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL2DYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL2DYF_A::B_0X1
    }
}
#[doc = "Write proxy for field `PLL2DYF`"]
pub struct PLL2DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2DYF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL2DYF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock ready interrupt caused by PLL2 lock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLL2DYF_A::B_0X0)
    }
    #[doc = "Clock ready interrupt caused by PLL2 lock, writing clears this flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLL2DYF_A::B_0X1)
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
#[doc = "PLL3DYF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL3DYF_A {
    #[doc = "0: No clock ready interrupt caused by\r\n                  PLL3 lock (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by PLL3\r\n                  lock, writing clears this flag"]
    B_0X1 = 1,
}
impl From<PLL3DYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLL3DYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL3DYF`"]
pub type PLL3DYF_R = crate::R<bool, PLL3DYF_A>;
impl PLL3DYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL3DYF_A {
        match self.bits {
            false => PLL3DYF_A::B_0X0,
            true => PLL3DYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL3DYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL3DYF_A::B_0X1
    }
}
#[doc = "Write proxy for field `PLL3DYF`"]
pub struct PLL3DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3DYF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3DYF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock ready interrupt caused by PLL3 lock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLL3DYF_A::B_0X0)
    }
    #[doc = "Clock ready interrupt caused by PLL3 lock, writing clears this flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLL3DYF_A::B_0X1)
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
#[doc = "PLL4DYF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL4DYF_A {
    #[doc = "0: No clock ready interrupt caused by\r\n                  PLL4 lock (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Clock ready interrupt caused by PLL4\r\n                  lock, writing clears this flag"]
    B_0X1 = 1,
}
impl From<PLL4DYF_A> for bool {
    #[inline(always)]
    fn from(variant: PLL4DYF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL4DYF`"]
pub type PLL4DYF_R = crate::R<bool, PLL4DYF_A>;
impl PLL4DYF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL4DYF_A {
        match self.bits {
            false => PLL4DYF_A::B_0X0,
            true => PLL4DYF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL4DYF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL4DYF_A::B_0X1
    }
}
#[doc = "Write proxy for field `PLL4DYF`"]
pub struct PLL4DYF_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL4DYF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL4DYF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No clock ready interrupt caused by PLL4 lock (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLL4DYF_A::B_0X0)
    }
    #[doc = "Clock ready interrupt caused by PLL4 lock, writing clears this flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLL4DYF_A::B_0X1)
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
#[doc = "LSECSSF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSECSSF_A {
    #[doc = "0: No failure detected on the external\r\n                  32 kHz oscillator (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: A failure is detected on the\r\n                  external 32 kHz oscillator, writing clears this\r\n                  flag"]
    B_0X1 = 1,
}
impl From<LSECSSF_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSECSSF`"]
pub type LSECSSF_R = crate::R<bool, LSECSSF_A>;
impl LSECSSF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSF_A {
        match self.bits {
            false => LSECSSF_A::B_0X0,
            true => LSECSSF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSECSSF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSECSSF_A::B_0X1
    }
}
#[doc = "Write proxy for field `LSECSSF`"]
pub struct LSECSSF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSECSSF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No failure detected on the external 32 kHz oscillator (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSECSSF_A::B_0X0)
    }
    #[doc = "A failure is detected on the external 32 kHz oscillator, writing clears this flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSECSSF_A::B_0X1)
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
#[doc = "WKUPF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPF_A {
    #[doc = "0: No wake-up interrupt pending\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Wake-up interrupt pending, writing\r\n                  clears this flag"]
    B_0X1 = 1,
}
impl From<WKUPF_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WKUPF`"]
pub type WKUPF_R = crate::R<bool, WKUPF_A>;
impl WKUPF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPF_A {
        match self.bits {
            false => WKUPF_A::B_0X0,
            true => WKUPF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WKUPF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WKUPF_A::B_0X1
    }
}
#[doc = "Write proxy for field `WKUPF`"]
pub struct WKUPF_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No wake-up interrupt pending (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WKUPF_A::B_0X0)
    }
    #[doc = "Wake-up interrupt pending, writing clears this flag"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WKUPF_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSIRDYF"]
    #[inline(always)]
    pub fn csirdyf(&self) -> CSIRDYF_R {
        CSIRDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL1DYF"]
    #[inline(always)]
    pub fn pll1dyf(&self) -> PLL1DYF_R {
        PLL1DYF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLL2DYF"]
    #[inline(always)]
    pub fn pll2dyf(&self) -> PLL2DYF_R {
        PLL2DYF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PLL3DYF"]
    #[inline(always)]
    pub fn pll3dyf(&self) -> PLL3DYF_R {
        PLL3DYF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PLL4DYF"]
    #[inline(always)]
    pub fn pll4dyf(&self) -> PLL4DYF_R {
        PLL4DYF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LSECSSF"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LSECSSF_R {
        LSECSSF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WKUPF"]
    #[inline(always)]
    pub fn wkupf(&self) -> WKUPF_R {
        WKUPF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSIRDYF"]
    #[inline(always)]
    pub fn lsirdyf(&mut self) -> LSIRDYF_W {
        LSIRDYF_W { w: self }
    }
    #[doc = "Bit 1 - LSERDYF"]
    #[inline(always)]
    pub fn lserdyf(&mut self) -> LSERDYF_W {
        LSERDYF_W { w: self }
    }
    #[doc = "Bit 2 - HSIRDYF"]
    #[inline(always)]
    pub fn hsirdyf(&mut self) -> HSIRDYF_W {
        HSIRDYF_W { w: self }
    }
    #[doc = "Bit 3 - HSERDYF"]
    #[inline(always)]
    pub fn hserdyf(&mut self) -> HSERDYF_W {
        HSERDYF_W { w: self }
    }
    #[doc = "Bit 4 - CSIRDYF"]
    #[inline(always)]
    pub fn csirdyf(&mut self) -> CSIRDYF_W {
        CSIRDYF_W { w: self }
    }
    #[doc = "Bit 8 - PLL1DYF"]
    #[inline(always)]
    pub fn pll1dyf(&mut self) -> PLL1DYF_W {
        PLL1DYF_W { w: self }
    }
    #[doc = "Bit 9 - PLL2DYF"]
    #[inline(always)]
    pub fn pll2dyf(&mut self) -> PLL2DYF_W {
        PLL2DYF_W { w: self }
    }
    #[doc = "Bit 10 - PLL3DYF"]
    #[inline(always)]
    pub fn pll3dyf(&mut self) -> PLL3DYF_W {
        PLL3DYF_W { w: self }
    }
    #[doc = "Bit 11 - PLL4DYF"]
    #[inline(always)]
    pub fn pll4dyf(&mut self) -> PLL4DYF_W {
        PLL4DYF_W { w: self }
    }
    #[doc = "Bit 16 - LSECSSF"]
    #[inline(always)]
    pub fn lsecssf(&mut self) -> LSECSSF_W {
        LSECSSF_W { w: self }
    }
    #[doc = "Bit 20 - WKUPF"]
    #[inline(always)]
    pub fn wkupf(&mut self) -> WKUPF_W {
        WKUPF_W { w: self }
    }
}
