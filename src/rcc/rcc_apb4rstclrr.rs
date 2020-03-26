#[doc = "Reader of register RCC_APB4RSTCLRR"]
pub type R = crate::R<u32, super::RCC_APB4RSTCLRR>;
#[doc = "Writer for register RCC_APB4RSTCLRR"]
pub type W = crate::W<u32, super::RCC_APB4RSTCLRR>;
#[doc = "Register RCC_APB4RSTCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_APB4RSTCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LTDCRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LTDCRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<LTDCRST_A> for bool {
    #[inline(always)]
    fn from(variant: LTDCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LTDCRST`"]
pub type LTDCRST_R = crate::R<bool, LTDCRST_A>;
impl LTDCRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LTDCRST_A {
        match self.bits {
            false => LTDCRST_A::B_0X0,
            true => LTDCRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LTDCRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LTDCRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `LTDCRST`"]
pub struct LTDCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LTDCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LTDCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(LTDCRST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(LTDCRST_A::B_0X1)
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
#[doc = "DSIRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSIRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<DSIRST_A> for bool {
    #[inline(always)]
    fn from(variant: DSIRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSIRST`"]
pub type DSIRST_R = crate::R<bool, DSIRST_A>;
impl DSIRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSIRST_A {
        match self.bits {
            false => DSIRST_A::B_0X0,
            true => DSIRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DSIRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DSIRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DSIRST`"]
pub struct DSIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DSIRST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DSIRST_A::B_0X1)
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
#[doc = "DDRPERFMRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRPERFMRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<DDRPERFMRST_A> for bool {
    #[inline(always)]
    fn from(variant: DDRPERFMRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDRPERFMRST`"]
pub type DDRPERFMRST_R = crate::R<bool, DDRPERFMRST_A>;
impl DDRPERFMRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRPERFMRST_A {
        match self.bits {
            false => DDRPERFMRST_A::B_0X0,
            true => DDRPERFMRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRPERFMRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRPERFMRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DDRPERFMRST`"]
pub struct DDRPERFMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPERFMRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDRPERFMRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DDRPERFMRST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DDRPERFMRST_A::B_0X1)
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
#[doc = "USBPHYRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBPHYRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing releases the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<USBPHYRST_A> for bool {
    #[inline(always)]
    fn from(variant: USBPHYRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBPHYRST`"]
pub type USBPHYRST_R = crate::R<bool, USBPHYRST_A>;
impl USBPHYRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPHYRST_A {
        match self.bits {
            false => USBPHYRST_A::B_0X0,
            true => USBPHYRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBPHYRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBPHYRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `USBPHYRST`"]
pub struct USBPHYRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPHYRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBPHYRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBPHYRST_A::B_0X0)
    }
    #[doc = "Writing releases the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBPHYRST_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - LTDCRST"]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - DSIRST"]
    #[inline(always)]
    pub fn dsirst(&self) -> DSIRST_R {
        DSIRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DDRPERFMRST"]
    #[inline(always)]
    pub fn ddrperfmrst(&self) -> DDRPERFMRST_R {
        DDRPERFMRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USBPHYRST"]
    #[inline(always)]
    pub fn usbphyrst(&self) -> USBPHYRST_R {
        USBPHYRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LTDCRST"]
    #[inline(always)]
    pub fn ltdcrst(&mut self) -> LTDCRST_W {
        LTDCRST_W { w: self }
    }
    #[doc = "Bit 4 - DSIRST"]
    #[inline(always)]
    pub fn dsirst(&mut self) -> DSIRST_W {
        DSIRST_W { w: self }
    }
    #[doc = "Bit 8 - DDRPERFMRST"]
    #[inline(always)]
    pub fn ddrperfmrst(&mut self) -> DDRPERFMRST_W {
        DDRPERFMRST_W { w: self }
    }
    #[doc = "Bit 16 - USBPHYRST"]
    #[inline(always)]
    pub fn usbphyrst(&mut self) -> USBPHYRST_W {
        USBPHYRST_W { w: self }
    }
}
