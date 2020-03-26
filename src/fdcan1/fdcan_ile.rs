#[doc = "Reader of register FDCAN_ILE"]
pub type R = crate::R<u32, super::FDCAN_ILE>;
#[doc = "Writer for register FDCAN_ILE"]
pub type W = crate::W<u32, super::FDCAN_ILE>;
#[doc = "Register FDCAN_ILE `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_ILE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "EINT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT0_A {
    #[doc = "0: Interrupt line fdcan_intr1_it\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt line fdcan_intr1_it\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<EINT0_A> for bool {
    #[inline(always)]
    fn from(variant: EINT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EINT0`"]
pub type EINT0_R = crate::R<bool, EINT0_A>;
impl EINT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT0_A {
        match self.bits {
            false => EINT0_A::B_0X0,
            true => EINT0_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EINT0_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EINT0_A::B_0X1
    }
}
#[doc = "Write proxy for field `EINT0`"]
pub struct EINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt line fdcan_intr1_it disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EINT0_A::B_0X0)
    }
    #[doc = "Interrupt line fdcan_intr1_it enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EINT0_A::B_0X1)
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
#[doc = "EINT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EINT1_A {
    #[doc = "0: Interrupt line fdcan_intr0_it\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Interrupt line fdcan_intr0_it\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<EINT1_A> for bool {
    #[inline(always)]
    fn from(variant: EINT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EINT1`"]
pub type EINT1_R = crate::R<bool, EINT1_A>;
impl EINT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EINT1_A {
        match self.bits {
            false => EINT1_A::B_0X0,
            true => EINT1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EINT1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EINT1_A::B_0X1
    }
}
#[doc = "Write proxy for field `EINT1`"]
pub struct EINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> EINT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EINT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt line fdcan_intr0_it disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(EINT1_A::B_0X0)
    }
    #[doc = "Interrupt line fdcan_intr0_it enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(EINT1_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - EINT0"]
    #[inline(always)]
    pub fn eint0(&self) -> EINT0_R {
        EINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EINT1"]
    #[inline(always)]
    pub fn eint1(&self) -> EINT1_R {
        EINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EINT0"]
    #[inline(always)]
    pub fn eint0(&mut self) -> EINT0_W {
        EINT0_W { w: self }
    }
    #[doc = "Bit 1 - EINT1"]
    #[inline(always)]
    pub fn eint1(&mut self) -> EINT1_W {
        EINT1_W { w: self }
    }
}
