#[doc = "Reader of register RCC_APB1DIVR"]
pub type R = crate::R<u32, super::RCC_APB1DIVR>;
#[doc = "Writer for register RCC_APB1DIVR"]
pub type W = crate::W<u32, super::RCC_APB1DIVR>;
#[doc = "Register RCC_APB1DIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_APB1DIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "APB1DIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APB1DIV_A {
    #[doc = "0: mlhclk (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: mlhclk / 2"]
    B_0X1 = 1,
    #[doc = "2: mlhclk / 4"]
    B_0X2 = 2,
    #[doc = "3: mlhclk / 8"]
    B_0X3 = 3,
    #[doc = "4: mlhclk / 16"]
    B_0X4 = 4,
}
impl From<APB1DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: APB1DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `APB1DIV`"]
pub type APB1DIV_R = crate::R<u8, APB1DIV_A>;
impl APB1DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, APB1DIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(APB1DIV_A::B_0X0),
            1 => Val(APB1DIV_A::B_0X1),
            2 => Val(APB1DIV_A::B_0X2),
            3 => Val(APB1DIV_A::B_0X3),
            4 => Val(APB1DIV_A::B_0X4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == APB1DIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == APB1DIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == APB1DIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == APB1DIV_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == APB1DIV_A::B_0X4
    }
}
#[doc = "Write proxy for field `APB1DIV`"]
pub struct APB1DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB1DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APB1DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "mlhclk (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(APB1DIV_A::B_0X0)
    }
    #[doc = "mlhclk / 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(APB1DIV_A::B_0X1)
    }
    #[doc = "mlhclk / 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(APB1DIV_A::B_0X2)
    }
    #[doc = "mlhclk / 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(APB1DIV_A::B_0X3)
    }
    #[doc = "mlhclk / 16"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(APB1DIV_A::B_0X4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "APB1DIVRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APB1DIVRDY_A {
    #[doc = "0: The new division factor is not yet\r\n                  taken into account."]
    B_0X0 = 0,
    #[doc = "1: The new division factor is taken\r\n                  into account. (default after reset)"]
    B_0X1 = 1,
}
impl From<APB1DIVRDY_A> for bool {
    #[inline(always)]
    fn from(variant: APB1DIVRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `APB1DIVRDY`"]
pub type APB1DIVRDY_R = crate::R<bool, APB1DIVRDY_A>;
impl APB1DIVRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APB1DIVRDY_A {
        match self.bits {
            false => APB1DIVRDY_A::B_0X0,
            true => APB1DIVRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == APB1DIVRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == APB1DIVRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:2 - APB1DIV"]
    #[inline(always)]
    pub fn apb1div(&self) -> APB1DIV_R {
        APB1DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - APB1DIVRDY"]
    #[inline(always)]
    pub fn apb1divrdy(&self) -> APB1DIVRDY_R {
        APB1DIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB1DIV"]
    #[inline(always)]
    pub fn apb1div(&mut self) -> APB1DIV_W {
        APB1DIV_W { w: self }
    }
}
