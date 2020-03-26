#[doc = "Reader of register RCC_MP_GCR"]
pub type R = crate::R<u32, super::RCC_MP_GCR>;
#[doc = "Writer for register RCC_MP_GCR"]
pub type W = crate::W<u32, super::RCC_MP_GCR>;
#[doc = "Register RCC_MP_GCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_GCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "BOOT_MCU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOOT_MCU_A {
    #[doc = "0: The MCU will be set in HOLD_BOOT\r\n                  when the next MCU core reset occurs. (default\r\n                  after reset)"]
    B_0X0 = 0,
    #[doc = "1: The MCU will not be in HOLD_BOOT\r\n                  mode when the next MCU core reset\r\n                  occurs."]
    B_0X1 = 1,
}
impl From<BOOT_MCU_A> for bool {
    #[inline(always)]
    fn from(variant: BOOT_MCU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOOT_MCU`"]
pub type BOOT_MCU_R = crate::R<bool, BOOT_MCU_A>;
impl BOOT_MCU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOT_MCU_A {
        match self.bits {
            false => BOOT_MCU_A::B_0X0,
            true => BOOT_MCU_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BOOT_MCU_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BOOT_MCU_A::B_0X1
    }
}
#[doc = "Write proxy for field `BOOT_MCU`"]
pub struct BOOT_MCU_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_MCU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOOT_MCU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The MCU will be set in HOLD_BOOT when the next MCU core reset occurs. (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BOOT_MCU_A::B_0X0)
    }
    #[doc = "The MCU will not be in HOLD_BOOT mode when the next MCU core reset occurs."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BOOT_MCU_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - BOOT_MCU"]
    #[inline(always)]
    pub fn boot_mcu(&self) -> BOOT_MCU_R {
        BOOT_MCU_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOOT_MCU"]
    #[inline(always)]
    pub fn boot_mcu(&mut self) -> BOOT_MCU_W {
        BOOT_MCU_W { w: self }
    }
}
