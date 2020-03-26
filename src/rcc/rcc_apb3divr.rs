#[doc = "Reader of register RCC_APB3DIVR"]
pub type R = crate::R<u32, super::RCC_APB3DIVR>;
#[doc = "Writer for register RCC_APB3DIVR"]
pub type W = crate::W<u32, super::RCC_APB3DIVR>;
#[doc = "Register RCC_APB3DIVR `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::RCC_APB3DIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "APB3DIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APB3DIV_A {
    #[doc = "0: hclk (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: hclk / 2"]
    B_0X1 = 1,
    #[doc = "2: hclk / 4"]
    B_0X2 = 2,
    #[doc = "3: hclk / 8"]
    B_0X3 = 3,
}
impl From<APB3DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: APB3DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `APB3DIV`"]
pub type APB3DIV_R = crate::R<u8, APB3DIV_A>;
impl APB3DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, APB3DIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(APB3DIV_A::B_0X0),
            1 => Val(APB3DIV_A::B_0X1),
            2 => Val(APB3DIV_A::B_0X2),
            3 => Val(APB3DIV_A::B_0X3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == APB3DIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == APB3DIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == APB3DIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == APB3DIV_A::B_0X3
    }
}
#[doc = "Write proxy for field `APB3DIV`"]
pub struct APB3DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> APB3DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APB3DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "hclk (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(APB3DIV_A::B_0X0)
    }
    #[doc = "hclk / 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(APB3DIV_A::B_0X1)
    }
    #[doc = "hclk / 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(APB3DIV_A::B_0X2)
    }
    #[doc = "hclk / 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(APB3DIV_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "APB3DIVRDY\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APB3DIVRDY_A {
    #[doc = "0: The new division factor is not yet\r\n                  taken into account."]
    B_0X0 = 0,
    #[doc = "1: The new division factor is taken\r\n                  into account. (default after reset)"]
    B_0X1 = 1,
}
impl From<APB3DIVRDY_A> for bool {
    #[inline(always)]
    fn from(variant: APB3DIVRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `APB3DIVRDY`"]
pub type APB3DIVRDY_R = crate::R<bool, APB3DIVRDY_A>;
impl APB3DIVRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APB3DIVRDY_A {
        match self.bits {
            false => APB3DIVRDY_A::B_0X0,
            true => APB3DIVRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == APB3DIVRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == APB3DIVRDY_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:2 - APB3DIV"]
    #[inline(always)]
    pub fn apb3div(&self) -> APB3DIV_R {
        APB3DIV_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 31 - APB3DIVRDY"]
    #[inline(always)]
    pub fn apb3divrdy(&self) -> APB3DIVRDY_R {
        APB3DIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - APB3DIV"]
    #[inline(always)]
    pub fn apb3div(&mut self) -> APB3DIV_W {
        APB3DIV_W { w: self }
    }
}
