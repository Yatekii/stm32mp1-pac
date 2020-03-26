#[doc = "Reader of register RCC_MC_RSTSCLRR"]
pub type R = crate::R<u32, super::RCC_MC_RSTSCLRR>;
#[doc = "Writer for register RCC_MC_RSTSCLRR"]
pub type W = crate::W<u32, super::RCC_MC_RSTSCLRR>;
#[doc = "Register RCC_MC_RSTSCLRR `reset()`'s with value 0x15"]
impl crate::ResetValue for super::RCC_MC_RSTSCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x15
    }
}
#[doc = "PORRSTF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORRSTF_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that no POR/PDR reset occurred"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the PORRSTF flag,\r\n                  reading means that a POR/PDR reset occurred\r\n                  (default after por_rst reset)"]
    B_0X1 = 1,
}
impl From<PORRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: PORRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORRSTF`"]
pub type PORRSTF_R = crate::R<bool, PORRSTF_A>;
impl PORRSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORRSTF_A {
        match self.bits {
            false => PORRSTF_A::B_0X0,
            true => PORRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PORRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PORRSTF_A::B_0X1
    }
}
#[doc = "Write proxy for field `PORRSTF`"]
pub struct PORRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PORRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that no POR/PDR reset occurred"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PORRSTF_A::B_0X0)
    }
    #[doc = "Writing clears the PORRSTF flag, reading means that a POR/PDR reset occurred (default after por_rst reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PORRSTF_A::B_0X1)
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
#[doc = "BORRSTF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BORRSTF_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that no BOR reset occurred (default after por_rst\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the BORRSTF flag,\r\n                  reading means that a BOR reset\r\n                  occurred"]
    B_0X1 = 1,
}
impl From<BORRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: BORRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BORRSTF`"]
pub type BORRSTF_R = crate::R<bool, BORRSTF_A>;
impl BORRSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BORRSTF_A {
        match self.bits {
            false => BORRSTF_A::B_0X0,
            true => BORRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BORRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BORRSTF_A::B_0X1
    }
}
#[doc = "Write proxy for field `BORRSTF`"]
pub struct BORRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> BORRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BORRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that no BOR reset occurred (default after por_rst reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BORRSTF_A::B_0X0)
    }
    #[doc = "Writing clears the BORRSTF flag, reading means that a BOR reset occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BORRSTF_A::B_0X1)
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
#[doc = "PADRSTF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PADRSTF_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that no PAD reset occurred (default after por_rst\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the PADRSTF flag,\r\n                  reading means that a PAD reset\r\n                  occurred"]
    B_0X1 = 1,
}
impl From<PADRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: PADRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PADRSTF`"]
pub type PADRSTF_R = crate::R<bool, PADRSTF_A>;
impl PADRSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PADRSTF_A {
        match self.bits {
            false => PADRSTF_A::B_0X0,
            true => PADRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PADRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PADRSTF_A::B_0X1
    }
}
#[doc = "Write proxy for field `PADRSTF`"]
pub struct PADRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PADRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PADRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that no PAD reset occurred (default after por_rst reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PADRSTF_A::B_0X0)
    }
    #[doc = "Writing clears the PADRSTF flag, reading means that a PAD reset occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PADRSTF_A::B_0X1)
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
#[doc = "HCSSRSTF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCSSRSTF_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that no HSE CSS reset occurred (default after\r\n                  por_rst reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the HCSSRSTF flag,\r\n                  reading means that a HSE CSS reset\r\n                  occurred"]
    B_0X1 = 1,
}
impl From<HCSSRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: HCSSRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HCSSRSTF`"]
pub type HCSSRSTF_R = crate::R<bool, HCSSRSTF_A>;
impl HCSSRSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCSSRSTF_A {
        match self.bits {
            false => HCSSRSTF_A::B_0X0,
            true => HCSSRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HCSSRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HCSSRSTF_A::B_0X1
    }
}
#[doc = "Write proxy for field `HCSSRSTF`"]
pub struct HCSSRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> HCSSRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCSSRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that no HSE CSS reset occurred (default after por_rst reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HCSSRSTF_A::B_0X0)
    }
    #[doc = "Writing clears the HCSSRSTF flag, reading means that a HSE CSS reset occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HCSSRSTF_A::B_0X1)
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
#[doc = "VCORERSTF\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCORERSTF_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that VDD_CORE is not the origin of the\r\n                  reset"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the VCORERSTF flag,\r\n                  reading means that VDD_CORE is the origin of the\r\n                  reset (default after por_rst reset)"]
    B_0X1 = 1,
}
impl From<VCORERSTF_A> for bool {
    #[inline(always)]
    fn from(variant: VCORERSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VCORERSTF`"]
pub type VCORERSTF_R = crate::R<bool, VCORERSTF_A>;
impl VCORERSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCORERSTF_A {
        match self.bits {
            false => VCORERSTF_A::B_0X0,
            true => VCORERSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == VCORERSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == VCORERSTF_A::B_0X1
    }
}
#[doc = "Write proxy for field `VCORERSTF`"]
pub struct VCORERSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> VCORERSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCORERSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that VDD_CORE is not the origin of the reset"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(VCORERSTF_A::B_0X0)
    }
    #[doc = "Writing clears the VCORERSTF flag, reading means that VDD_CORE is the origin of the reset (default after por_rst reset)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(VCORERSTF_A::B_0X1)
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
#[doc = "MCURSTF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCURSTF_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that no MCU reset occurred (default after por_rst\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the MCURSTF flag,\r\n                  reading means that a MCU reset\r\n                  occurred"]
    B_0X1 = 1,
}
impl From<MCURSTF_A> for bool {
    #[inline(always)]
    fn from(variant: MCURSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCURSTF`"]
pub type MCURSTF_R = crate::R<bool, MCURSTF_A>;
impl MCURSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCURSTF_A {
        match self.bits {
            false => MCURSTF_A::B_0X0,
            true => MCURSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCURSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCURSTF_A::B_0X1
    }
}
#[doc = "Write proxy for field `MCURSTF`"]
pub struct MCURSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> MCURSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCURSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that no MCU reset occurred (default after por_rst reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCURSTF_A::B_0X0)
    }
    #[doc = "Writing clears the MCURSTF flag, reading means that a MCU reset occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCURSTF_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "MPSYSRSTF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPSYSRSTF_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that no system reset generated by the MPU\r\n                  occurred (default after por_rst\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the MCURSTF flag,\r\n                  reading means that a system reset generated by\r\n                  the MPU occurred"]
    B_0X1 = 1,
}
impl From<MPSYSRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: MPSYSRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPSYSRSTF`"]
pub type MPSYSRSTF_R = crate::R<bool, MPSYSRSTF_A>;
impl MPSYSRSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPSYSRSTF_A {
        match self.bits {
            false => MPSYSRSTF_A::B_0X0,
            true => MPSYSRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MPSYSRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MPSYSRSTF_A::B_0X1
    }
}
#[doc = "Write proxy for field `MPSYSRSTF`"]
pub struct MPSYSRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> MPSYSRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPSYSRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that no system reset generated by the MPU occurred (default after por_rst reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MPSYSRSTF_A::B_0X0)
    }
    #[doc = "Writing clears the MCURSTF flag, reading means that a system reset generated by the MPU occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MPSYSRSTF_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "MCSYSRSTF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCSYSRSTF_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that no system reset generated by the MCU\r\n                  occurred (default after por_rst\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the MCURSTF flag,\r\n                  reading means that a system reset generated by\r\n                  the MCU occurred"]
    B_0X1 = 1,
}
impl From<MCSYSRSTF_A> for bool {
    #[inline(always)]
    fn from(variant: MCSYSRSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCSYSRSTF`"]
pub type MCSYSRSTF_R = crate::R<bool, MCSYSRSTF_A>;
impl MCSYSRSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCSYSRSTF_A {
        match self.bits {
            false => MCSYSRSTF_A::B_0X0,
            true => MCSYSRSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MCSYSRSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MCSYSRSTF_A::B_0X1
    }
}
#[doc = "Write proxy for field `MCSYSRSTF`"]
pub struct MCSYSRSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> MCSYSRSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCSYSRSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that no system reset generated by the MCU occurred (default after por_rst reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(MCSYSRSTF_A::B_0X0)
    }
    #[doc = "Writing clears the MCURSTF flag, reading means that a system reset generated by the MCU occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(MCSYSRSTF_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "IWDG1RSTF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDG1RSTF_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that no IWDG1 reset occurred (default after\r\n                  por_rst reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the IWDG1RSTF flag,\r\n                  reading means that a IWDG1 reset\r\n                  occurred"]
    B_0X1 = 1,
}
impl From<IWDG1RSTF_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG1RSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IWDG1RSTF`"]
pub type IWDG1RSTF_R = crate::R<bool, IWDG1RSTF_A>;
impl IWDG1RSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDG1RSTF_A {
        match self.bits {
            false => IWDG1RSTF_A::B_0X0,
            true => IWDG1RSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IWDG1RSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IWDG1RSTF_A::B_0X1
    }
}
#[doc = "Write proxy for field `IWDG1RSTF`"]
pub struct IWDG1RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG1RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IWDG1RSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that no IWDG1 reset occurred (default after por_rst reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IWDG1RSTF_A::B_0X0)
    }
    #[doc = "Writing clears the IWDG1RSTF flag, reading means that a IWDG1 reset occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IWDG1RSTF_A::B_0X1)
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
#[doc = "IWDG2RSTF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWDG2RSTF_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that no IWDG2 reset occurred (default after\r\n                  por_rst reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the IWDG2RSTF flag,\r\n                  reading means that a IWDG2 reset\r\n                  occurred"]
    B_0X1 = 1,
}
impl From<IWDG2RSTF_A> for bool {
    #[inline(always)]
    fn from(variant: IWDG2RSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IWDG2RSTF`"]
pub type IWDG2RSTF_R = crate::R<bool, IWDG2RSTF_A>;
impl IWDG2RSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWDG2RSTF_A {
        match self.bits {
            false => IWDG2RSTF_A::B_0X0,
            true => IWDG2RSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IWDG2RSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IWDG2RSTF_A::B_0X1
    }
}
#[doc = "Write proxy for field `IWDG2RSTF`"]
pub struct IWDG2RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG2RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IWDG2RSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that no IWDG2 reset occurred (default after por_rst reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IWDG2RSTF_A::B_0X0)
    }
    #[doc = "Writing clears the IWDG2RSTF flag, reading means that a IWDG2 reset occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IWDG2RSTF_A::B_0X1)
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
#[doc = "WWDG1RSTF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDG1RSTF_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that no WWDG1 reset occurred (default after\r\n                  por_rst reset)"]
    B_0X0 = 0,
    #[doc = "1: Writing clears the WWDG1RSTF flag,\r\n                  reading means that a WWDG1 reset\r\n                  occurred"]
    B_0X1 = 1,
}
impl From<WWDG1RSTF_A> for bool {
    #[inline(always)]
    fn from(variant: WWDG1RSTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WWDG1RSTF`"]
pub type WWDG1RSTF_R = crate::R<bool, WWDG1RSTF_A>;
impl WWDG1RSTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDG1RSTF_A {
        match self.bits {
            false => WWDG1RSTF_A::B_0X0,
            true => WWDG1RSTF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WWDG1RSTF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WWDG1RSTF_A::B_0X1
    }
}
#[doc = "Write proxy for field `WWDG1RSTF`"]
pub struct WWDG1RSTF_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDG1RSTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDG1RSTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that no WWDG1 reset occurred (default after por_rst reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WWDG1RSTF_A::B_0X0)
    }
    #[doc = "Writing clears the WWDG1RSTF flag, reading means that a WWDG1 reset occurred"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WWDG1RSTF_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - PORRSTF"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BORRSTF"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PADRSTF"]
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HCSSRSTF"]
    #[inline(always)]
    pub fn hcssrstf(&self) -> HCSSRSTF_R {
        HCSSRSTF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VCORERSTF"]
    #[inline(always)]
    pub fn vcorerstf(&self) -> VCORERSTF_R {
        VCORERSTF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MCURSTF"]
    #[inline(always)]
    pub fn mcurstf(&self) -> MCURSTF_R {
        MCURSTF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MPSYSRSTF"]
    #[inline(always)]
    pub fn mpsysrstf(&self) -> MPSYSRSTF_R {
        MPSYSRSTF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MCSYSRSTF"]
    #[inline(always)]
    pub fn mcsysrstf(&self) -> MCSYSRSTF_R {
        MCSYSRSTF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IWDG1RSTF"]
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IWDG2RSTF"]
    #[inline(always)]
    pub fn iwdg2rstf(&self) -> IWDG2RSTF_R {
        IWDG2RSTF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - WWDG1RSTF"]
    #[inline(always)]
    pub fn wwdg1rstf(&self) -> WWDG1RSTF_R {
        WWDG1RSTF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORRSTF"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W {
        PORRSTF_W { w: self }
    }
    #[doc = "Bit 1 - BORRSTF"]
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W {
        BORRSTF_W { w: self }
    }
    #[doc = "Bit 2 - PADRSTF"]
    #[inline(always)]
    pub fn padrstf(&mut self) -> PADRSTF_W {
        PADRSTF_W { w: self }
    }
    #[doc = "Bit 3 - HCSSRSTF"]
    #[inline(always)]
    pub fn hcssrstf(&mut self) -> HCSSRSTF_W {
        HCSSRSTF_W { w: self }
    }
    #[doc = "Bit 4 - VCORERSTF"]
    #[inline(always)]
    pub fn vcorerstf(&mut self) -> VCORERSTF_W {
        VCORERSTF_W { w: self }
    }
    #[doc = "Bit 5 - MCURSTF"]
    #[inline(always)]
    pub fn mcurstf(&mut self) -> MCURSTF_W {
        MCURSTF_W { w: self }
    }
    #[doc = "Bit 6 - MPSYSRSTF"]
    #[inline(always)]
    pub fn mpsysrstf(&mut self) -> MPSYSRSTF_W {
        MPSYSRSTF_W { w: self }
    }
    #[doc = "Bit 7 - MCSYSRSTF"]
    #[inline(always)]
    pub fn mcsysrstf(&mut self) -> MCSYSRSTF_W {
        MCSYSRSTF_W { w: self }
    }
    #[doc = "Bit 8 - IWDG1RSTF"]
    #[inline(always)]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W {
        IWDG1RSTF_W { w: self }
    }
    #[doc = "Bit 9 - IWDG2RSTF"]
    #[inline(always)]
    pub fn iwdg2rstf(&mut self) -> IWDG2RSTF_W {
        IWDG2RSTF_W { w: self }
    }
    #[doc = "Bit 10 - WWDG1RSTF"]
    #[inline(always)]
    pub fn wwdg1rstf(&mut self) -> WWDG1RSTF_W {
        WWDG1RSTF_W { w: self }
    }
}
