#[doc = "Reader of register RCC_MP_BOOTCR"]
pub type R = crate::R<u32, super::RCC_MP_BOOTCR>;
#[doc = "Writer for register RCC_MP_BOOTCR"]
pub type W = crate::W<u32, super::RCC_MP_BOOTCR>;
#[doc = "Register RCC_MP_BOOTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_BOOTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "MCU_BEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCU_BEN_A {
    #[doc = "0: The MCU will remain on HOLD_BOOT\r\n                  when the system exits from STANDBY"]
    B_0X0 = 0,
    #[doc = "1: The MCU is allowed to restart when\r\n                  the system exits from STANDBY"]
    B_0X1 = 1,
}
impl From<MCU_BEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCU_BEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCU_BEN`"]
pub type MCU_BEN_R = crate::R<bool, MCU_BEN_A>;
impl MCU_BEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCU_BEN_A {
        match self.bits {
            false => MCU_BEN_A::B_0X0,
            true => MCU_BEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCU_BEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCU_BEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `MCU_BEN`"]
pub struct MCU_BEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_BEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCU_BEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The MCU will remain on HOLD_BOOT when the system exits from STANDBY"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCU_BEN_A::B_0X0)
    }
    #[doc = "The MCU is allowed to restart when the system exits from STANDBY"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCU_BEN_A::B_0X1)
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
#[doc = "MPU_BEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPU_BEN_A {
    #[doc = "0: The MPU will remain on CSTANDBY when\r\n                  the system exits from STANDBY"]
    B_0X0 = 0,
    #[doc = "1: The MPU is allowed to restart when\r\n                  the system exits from STANDBY"]
    B_0X1 = 1,
}
impl From<MPU_BEN_A> for bool {
    #[inline(always)]
    fn from(variant: MPU_BEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPU_BEN`"]
pub type MPU_BEN_R = crate::R<bool, MPU_BEN_A>;
impl MPU_BEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPU_BEN_A {
        match self.bits {
            false => MPU_BEN_A::B_0X0,
            true => MPU_BEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MPU_BEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MPU_BEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `MPU_BEN`"]
pub struct MPU_BEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MPU_BEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPU_BEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The MPU will remain on CSTANDBY when the system exits from STANDBY"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MPU_BEN_A::B_0X0)
    }
    #[doc = "The MPU is allowed to restart when the system exits from STANDBY"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MPU_BEN_A::B_0X1)
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
    #[doc = "Bit 0 - MCU_BEN"]
    #[inline(always)]
    pub fn mcu_ben(&self) -> MCU_BEN_R {
        MCU_BEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU_BEN"]
    #[inline(always)]
    pub fn mpu_ben(&self) -> MPU_BEN_R {
        MPU_BEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCU_BEN"]
    #[inline(always)]
    pub fn mcu_ben(&mut self) -> MCU_BEN_W {
        MCU_BEN_W { w: self }
    }
    #[doc = "Bit 1 - MPU_BEN"]
    #[inline(always)]
    pub fn mpu_ben(&mut self) -> MPU_BEN_W {
        MPU_BEN_W { w: self }
    }
}
