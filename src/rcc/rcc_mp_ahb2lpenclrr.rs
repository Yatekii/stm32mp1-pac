#[doc = "Reader of register RCC_MP_AHB2LPENCLRR"]
pub type R = crate::R<u32, super::RCC_MP_AHB2LPENCLRR>;
#[doc = "Writer for register RCC_MP_AHB2LPENCLRR"]
pub type W = crate::W<u32, super::RCC_MP_AHB2LPENCLRR>;
#[doc = "Register RCC_MP_AHB2LPENCLRR `reset()`'s with value 0x0001_0127"]
impl crate::ResetValue for super::RCC_MP_AHB2LPENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0127
    }
}
#[doc = "DMA1LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<DMA1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA1LPEN`"]
pub type DMA1LPEN_R = crate::R<bool, DMA1LPEN_A>;
impl DMA1LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1LPEN_A {
        match self.bits {
            false => DMA1LPEN_A::B_0X0,
            true => DMA1LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA1LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA1LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMA1LPEN`"]
pub struct DMA1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::B_0X1)
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
#[doc = "DMA2LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA2LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<DMA2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA2LPEN`"]
pub type DMA2LPEN_R = crate::R<bool, DMA2LPEN_A>;
impl DMA2LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2LPEN_A {
        match self.bits {
            false => DMA2LPEN_A::B_0X0,
            true => DMA2LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA2LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA2LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMA2LPEN`"]
pub struct DMA2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMA2LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMA2LPEN_A::B_0X1)
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
#[doc = "DMAMUXLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<DMAMUXLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUXLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAMUXLPEN`"]
pub type DMAMUXLPEN_R = crate::R<bool, DMAMUXLPEN_A>;
impl DMAMUXLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMUXLPEN_A {
        match self.bits {
            false => DMAMUXLPEN_A::B_0X0,
            true => DMAMUXLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAMUXLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAMUXLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMAMUXLPEN`"]
pub struct DMAMUXLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUXLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAMUXLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAMUXLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAMUXLPEN_A::B_0X1)
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
#[doc = "ADC12LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<ADC12LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12LPEN`"]
pub type ADC12LPEN_R = crate::R<bool, ADC12LPEN_A>;
impl ADC12LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12LPEN_A {
        match self.bits {
            false => ADC12LPEN_A::B_0X0,
            true => ADC12LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADC12LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADC12LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `ADC12LPEN`"]
pub struct ADC12LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADC12LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADC12LPEN_A::B_0X1)
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
#[doc = "USBOLPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBOLPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<USBOLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBOLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBOLPEN`"]
pub type USBOLPEN_R = crate::R<bool, USBOLPEN_A>;
impl USBOLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBOLPEN_A {
        match self.bits {
            false => USBOLPEN_A::B_0X0,
            true => USBOLPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBOLPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBOLPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `USBOLPEN`"]
pub struct USBOLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBOLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBOLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBOLPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBOLPEN_A::B_0X1)
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
#[doc = "SDMMC3LPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMC3LPEN_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the peripheral clocks are disabled in\r\n                  CSLEEP"]
    B_0X0 = 0,
    #[doc = "1: Writing disables the peripheral\r\n                  clocks in CSLEEP, reading means that the\r\n                  peripheral clocks are enabled in\r\n                  CSLEEP"]
    B_0X1 = 1,
}
impl From<SDMMC3LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC3LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDMMC3LPEN`"]
pub type SDMMC3LPEN_R = crate::R<bool, SDMMC3LPEN_A>;
impl SDMMC3LPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMC3LPEN_A {
        match self.bits {
            false => SDMMC3LPEN_A::B_0X0,
            true => SDMMC3LPEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SDMMC3LPEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SDMMC3LPEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `SDMMC3LPEN`"]
pub struct SDMMC3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC3LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the peripheral clocks are disabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SDMMC3LPEN_A::B_0X0)
    }
    #[doc = "Writing disables the peripheral clocks in CSLEEP, reading means that the peripheral clocks are enabled in CSLEEP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SDMMC3LPEN_A::B_0X1)
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
    #[doc = "Bit 0 - DMA1LPEN"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2LPEN"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMAMUXLPEN"]
    #[inline(always)]
    pub fn dmamuxlpen(&self) -> DMAMUXLPEN_R {
        DMAMUXLPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12LPEN"]
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USBOLPEN"]
    #[inline(always)]
    pub fn usbolpen(&self) -> USBOLPEN_R {
        USBOLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC3LPEN"]
    #[inline(always)]
    pub fn sdmmc3lpen(&self) -> SDMMC3LPEN_R {
        SDMMC3LPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1LPEN"]
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W {
        DMA1LPEN_W { w: self }
    }
    #[doc = "Bit 1 - DMA2LPEN"]
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W {
        DMA2LPEN_W { w: self }
    }
    #[doc = "Bit 2 - DMAMUXLPEN"]
    #[inline(always)]
    pub fn dmamuxlpen(&mut self) -> DMAMUXLPEN_W {
        DMAMUXLPEN_W { w: self }
    }
    #[doc = "Bit 5 - ADC12LPEN"]
    #[inline(always)]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W {
        ADC12LPEN_W { w: self }
    }
    #[doc = "Bit 8 - USBOLPEN"]
    #[inline(always)]
    pub fn usbolpen(&mut self) -> USBOLPEN_W {
        USBOLPEN_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC3LPEN"]
    #[inline(always)]
    pub fn sdmmc3lpen(&mut self) -> SDMMC3LPEN_W {
        SDMMC3LPEN_W { w: self }
    }
}
