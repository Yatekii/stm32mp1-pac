#[doc = "Reader of register RCC_MP_CIER"]
pub type R = crate::R<u32, super::RCC_MP_CIER>;
#[doc = "Writer for register RCC_MP_CIER"]
pub type W = crate::W<u32, super::RCC_MP_CIER>;
#[doc = "Register RCC_MP_CIER `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_CIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LSIRDYIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSIRDYIE_A {
    #[doc = "0: LSI ready interrupt disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: LSI ready interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<LSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSIRDYIE`"]
pub type LSIRDYIE_R = crate::R<bool, LSIRDYIE_A>;
impl LSIRDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSIRDYIE_A {
        match self.bits {
            false => LSIRDYIE_A::B_0X0,
            true => LSIRDYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSIRDYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSIRDYIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `LSIRDYIE`"]
pub struct LSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSIRDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSI ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::B_0X0)
    }
    #[doc = "LSI ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSIRDYIE_A::B_0X1)
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
#[doc = "LSERDYIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSERDYIE_A {
    #[doc = "0: LSE ready interrupt disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: LSE ready interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<LSERDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSERDYIE`"]
pub type LSERDYIE_R = crate::R<bool, LSERDYIE_A>;
impl LSERDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDYIE_A {
        match self.bits {
            false => LSERDYIE_A::B_0X0,
            true => LSERDYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSERDYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSERDYIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `LSERDYIE`"]
pub struct LSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSERDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSE ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSERDYIE_A::B_0X0)
    }
    #[doc = "LSE ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSERDYIE_A::B_0X1)
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
#[doc = "HSIRDYIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIRDYIE_A {
    #[doc = "0: HSI ready interrupt disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: HSI ready interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<HSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSIRDYIE`"]
pub type HSIRDYIE_R = crate::R<bool, HSIRDYIE_A>;
impl HSIRDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYIE_A {
        match self.bits {
            false => HSIRDYIE_A::B_0X0,
            true => HSIRDYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIRDYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIRDYIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSIRDYIE`"]
pub struct HSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIRDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSI ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSIRDYIE_A::B_0X0)
    }
    #[doc = "HSI ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSIRDYIE_A::B_0X1)
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
#[doc = "HSERDYIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSERDYIE_A {
    #[doc = "0: HSE ready interrupt disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: HSE ready interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<HSERDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSERDYIE`"]
pub type HSERDYIE_R = crate::R<bool, HSERDYIE_A>;
impl HSERDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSERDYIE_A {
        match self.bits {
            false => HSERDYIE_A::B_0X0,
            true => HSERDYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSERDYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSERDYIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `HSERDYIE`"]
pub struct HSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSERDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSE ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSERDYIE_A::B_0X0)
    }
    #[doc = "HSE ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSERDYIE_A::B_0X1)
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
#[doc = "CSIRDYIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIRDYIE_A {
    #[doc = "0: CSI ready interrupt disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: CSI ready interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<CSIRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: CSIRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSIRDYIE`"]
pub type CSIRDYIE_R = crate::R<bool, CSIRDYIE_A>;
impl CSIRDYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSIRDYIE_A {
        match self.bits {
            false => CSIRDYIE_A::B_0X0,
            true => CSIRDYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSIRDYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSIRDYIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `CSIRDYIE`"]
pub struct CSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSIRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSIRDYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CSI ready interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CSIRDYIE_A::B_0X0)
    }
    #[doc = "CSI ready interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CSIRDYIE_A::B_0X1)
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
#[doc = "PLL1DYIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL1DYIE_A {
    #[doc = "0: PLL1 lock interrupt disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: PLL1 lock interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<PLL1DYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL1DYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL1DYIE`"]
pub type PLL1DYIE_R = crate::R<bool, PLL1DYIE_A>;
impl PLL1DYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL1DYIE_A {
        match self.bits {
            false => PLL1DYIE_A::B_0X0,
            true => PLL1DYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL1DYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL1DYIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `PLL1DYIE`"]
pub struct PLL1DYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL1DYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL1DYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL1 lock interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLL1DYIE_A::B_0X0)
    }
    #[doc = "PLL1 lock interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLL1DYIE_A::B_0X1)
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
#[doc = "PLL2DYIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL2DYIE_A {
    #[doc = "0: PLL2 lock interrupt disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: PLL2 lock interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<PLL2DYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL2DYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL2DYIE`"]
pub type PLL2DYIE_R = crate::R<bool, PLL2DYIE_A>;
impl PLL2DYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL2DYIE_A {
        match self.bits {
            false => PLL2DYIE_A::B_0X0,
            true => PLL2DYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL2DYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL2DYIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `PLL2DYIE`"]
pub struct PLL2DYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL2DYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL2DYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL2 lock interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLL2DYIE_A::B_0X0)
    }
    #[doc = "PLL2 lock interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLL2DYIE_A::B_0X1)
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
#[doc = "PLL3DYIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL3DYIE_A {
    #[doc = "0: PLL3 lock interrupt disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: PLL3 lock interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<PLL3DYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL3DYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL3DYIE`"]
pub type PLL3DYIE_R = crate::R<bool, PLL3DYIE_A>;
impl PLL3DYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL3DYIE_A {
        match self.bits {
            false => PLL3DYIE_A::B_0X0,
            true => PLL3DYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL3DYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL3DYIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `PLL3DYIE`"]
pub struct PLL3DYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3DYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL3DYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL3 lock interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLL3DYIE_A::B_0X0)
    }
    #[doc = "PLL3 lock interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLL3DYIE_A::B_0X1)
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
#[doc = "PLL4DYIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL4DYIE_A {
    #[doc = "0: PLL4 lock interrupt disabled\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: PLL4 lock interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<PLL4DYIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLL4DYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLL4DYIE`"]
pub type PLL4DYIE_R = crate::R<bool, PLL4DYIE_A>;
impl PLL4DYIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL4DYIE_A {
        match self.bits {
            false => PLL4DYIE_A::B_0X0,
            true => PLL4DYIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PLL4DYIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PLL4DYIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `PLL4DYIE`"]
pub struct PLL4DYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL4DYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL4DYIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL4 lock interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PLL4DYIE_A::B_0X0)
    }
    #[doc = "PLL4 lock interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PLL4DYIE_A::B_0X1)
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
#[doc = "LSECSSIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSECSSIE_A {
    #[doc = "0: LSE CSS interrupt disabled (default\r\n                  after reset)"]
    B_0X0 = 0,
    #[doc = "1: LSE CSS interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<LSECSSIE_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSECSSIE`"]
pub type LSECSSIE_R = crate::R<bool, LSECSSIE_A>;
impl LSECSSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSECSSIE_A {
        match self.bits {
            false => LSECSSIE_A::B_0X0,
            true => LSECSSIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSECSSIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSECSSIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `LSECSSIE`"]
pub struct LSECSSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSECSSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSECSSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSE CSS interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LSECSSIE_A::B_0X0)
    }
    #[doc = "LSE CSS interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LSECSSIE_A::B_0X1)
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
#[doc = "WKUPIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIE_A {
    #[doc = "0: Wake-up interrupt disabled (default\r\n                  after reset)"]
    B_0X0 = 0,
    #[doc = "1: Wake-up interrupt\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<WKUPIE_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WKUPIE`"]
pub type WKUPIE_R = crate::R<bool, WKUPIE_A>;
impl WKUPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIE_A {
        match self.bits {
            false => WKUPIE_A::B_0X0,
            true => WKUPIE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WKUPIE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WKUPIE_A::B_0X1
    }
}
#[doc = "Write proxy for field `WKUPIE`"]
pub struct WKUPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WKUPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up interrupt disabled (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WKUPIE_A::B_0X0)
    }
    #[doc = "Wake-up interrupt enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WKUPIE_A::B_0X1)
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
    #[doc = "Bit 0 - LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSIRDYIE"]
    #[inline(always)]
    pub fn csirdyie(&self) -> CSIRDYIE_R {
        CSIRDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL1DYIE"]
    #[inline(always)]
    pub fn pll1dyie(&self) -> PLL1DYIE_R {
        PLL1DYIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLL2DYIE"]
    #[inline(always)]
    pub fn pll2dyie(&self) -> PLL2DYIE_R {
        PLL2DYIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PLL3DYIE"]
    #[inline(always)]
    pub fn pll3dyie(&self) -> PLL3DYIE_R {
        PLL3DYIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PLL4DYIE"]
    #[inline(always)]
    pub fn pll4dyie(&self) -> PLL4DYIE_R {
        PLL4DYIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LSECSSIE"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WKUPIE"]
    #[inline(always)]
    pub fn wkupie(&self) -> WKUPIE_R {
        WKUPIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSIRDYIE"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W {
        LSIRDYIE_W { w: self }
    }
    #[doc = "Bit 1 - LSERDYIE"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W {
        LSERDYIE_W { w: self }
    }
    #[doc = "Bit 2 - HSIRDYIE"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W {
        HSIRDYIE_W { w: self }
    }
    #[doc = "Bit 3 - HSERDYIE"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W {
        HSERDYIE_W { w: self }
    }
    #[doc = "Bit 4 - CSIRDYIE"]
    #[inline(always)]
    pub fn csirdyie(&mut self) -> CSIRDYIE_W {
        CSIRDYIE_W { w: self }
    }
    #[doc = "Bit 8 - PLL1DYIE"]
    #[inline(always)]
    pub fn pll1dyie(&mut self) -> PLL1DYIE_W {
        PLL1DYIE_W { w: self }
    }
    #[doc = "Bit 9 - PLL2DYIE"]
    #[inline(always)]
    pub fn pll2dyie(&mut self) -> PLL2DYIE_W {
        PLL2DYIE_W { w: self }
    }
    #[doc = "Bit 10 - PLL3DYIE"]
    #[inline(always)]
    pub fn pll3dyie(&mut self) -> PLL3DYIE_W {
        PLL3DYIE_W { w: self }
    }
    #[doc = "Bit 11 - PLL4DYIE"]
    #[inline(always)]
    pub fn pll4dyie(&mut self) -> PLL4DYIE_W {
        PLL4DYIE_W { w: self }
    }
    #[doc = "Bit 16 - LSECSSIE"]
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LSECSSIE_W {
        LSECSSIE_W { w: self }
    }
    #[doc = "Bit 20 - WKUPIE"]
    #[inline(always)]
    pub fn wkupie(&mut self) -> WKUPIE_W {
        WKUPIE_W { w: self }
    }
}
