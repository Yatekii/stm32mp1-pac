#[doc = "Reader of register RCC_APB5DIVR"]
pub type R = crate::R<u32, super::RCC_APB5DIVR>;
#[doc = "Writer for register RCC_APB5DIVR"]
pub type W = crate::W<u32, super::RCC_APB5DIVR>;
#[doc = "Register RCC_APB5DIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_APB5DIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "APB5DIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APB5DIV_A {
    #[doc = "0: aclk (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: aclk / 2"]
    B_0X1 = 1,
    #[doc = "2: aclk / 4"]
    B_0X2 = 2,
    #[doc = "3: aclk / 8"]
    B_0X3 = 3,
}
impl From<APB5DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: APB5DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `APB5DIV`"]
pub type APB5DIV_R = crate::R<u8, APB5DIV_A>;
impl APB5DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, APB5DIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(APB5DIV_A::B_0X0),
            1 => Val(APB5DIV_A::B_0X1),
            2 => Val(APB5DIV_A::B_0X2),
            3 => Val(APB5DIV_A::B_0X3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == APB5DIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == APB5DIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == APB5DIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == APB5DIV_A::B_0X3
    }
}
#[doc = "Write proxy for field `APB5DIV`"]
pub struct APB5DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB5DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APB5DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "aclk (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(APB5DIV_A::B_0X0)
    }
    #[doc = "aclk / 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(APB5DIV_A::B_0X1)
    }
    #[doc = "aclk / 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(APB5DIV_A::B_0X2)
    }
    #[doc = "aclk / 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(APB5DIV_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "APB5DIVRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APB5DIVRDY_A {
    #[doc = "0: The new division factor is not yet\r\n                  taken into account."]
    B_0X0 = 0,
    #[doc = "1: The new division factor is taken\r\n                  into account. (default after reset)"]
    B_0X1 = 1,
}
impl From<APB5DIVRDY_A> for bool {
    #[inline(always)]
    fn from(variant: APB5DIVRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `APB5DIVRDY`"]
pub type APB5DIVRDY_R = crate::R<bool, APB5DIVRDY_A>;
impl APB5DIVRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APB5DIVRDY_A {
        match self.bits {
            false => APB5DIVRDY_A::B_0X0,
            true => APB5DIVRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == APB5DIVRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == APB5DIVRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:2 - APB5DIV"]
    #[inline(always)]
    pub fn apb5div(&self) -> APB5DIV_R {
        APB5DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - APB5DIVRDY"]
    #[inline(always)]
    pub fn apb5divrdy(&self) -> APB5DIVRDY_R {
        APB5DIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB5DIV"]
    #[inline(always)]
    pub fn apb5div(&mut self) -> APB5DIV_W {
        APB5DIV_W { w: self }
    }
}
