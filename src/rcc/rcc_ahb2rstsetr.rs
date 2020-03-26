#[doc = "Reader of register RCC_AHB2RSTSETR"]
pub type R = crate::R<u32, super::RCC_AHB2RSTSETR>;
#[doc = "Writer for register RCC_AHB2RSTSETR"]
pub type W = crate::W<u32, super::RCC_AHB2RSTSETR>;
#[doc = "Register RCC_AHB2RSTSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_AHB2RSTSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA1RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<DMA1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA1RST`"]
pub type DMA1RST_R = crate::R<bool, DMA1RST_A>;
impl DMA1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1RST_A {
        match self.bits {
            false => DMA1RST_A::B_0X0,
            true => DMA1RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA1RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA1RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMA1RST`"]
pub struct DMA1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMA1RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMA1RST_A::B_0X1)
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
#[doc = "DMA2RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA2RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<DMA2RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA2RST`"]
pub type DMA2RST_R = crate::R<bool, DMA2RST_A>;
impl DMA2RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA2RST_A {
        match self.bits {
            false => DMA2RST_A::B_0X0,
            true => DMA2RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMA2RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMA2RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMA2RST`"]
pub struct DMA2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMA2RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMA2RST_A::B_0X1)
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
#[doc = "DMAMUXRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAMUXRST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<DMAMUXRST_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUXRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAMUXRST`"]
pub type DMAMUXRST_R = crate::R<bool, DMAMUXRST_A>;
impl DMAMUXRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMUXRST_A {
        match self.bits {
            false => DMAMUXRST_A::B_0X0,
            true => DMAMUXRST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAMUXRST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAMUXRST_A::B_0X1
    }
}
#[doc = "Write proxy for field `DMAMUXRST`"]
pub struct DMAMUXRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUXRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAMUXRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(DMAMUXRST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(DMAMUXRST_A::B_0X1)
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
#[doc = "ADC12RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC12RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<ADC12RST_A> for bool {
    #[inline(always)]
    fn from(variant: ADC12RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC12RST`"]
pub type ADC12RST_R = crate::R<bool, ADC12RST_A>;
impl ADC12RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC12RST_A {
        match self.bits {
            false => ADC12RST_A::B_0X0,
            true => ADC12RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ADC12RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ADC12RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `ADC12RST`"]
pub struct ADC12RST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ADC12RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ADC12RST_A::B_0X1)
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
#[doc = "USBORST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBORST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<USBORST_A> for bool {
    #[inline(always)]
    fn from(variant: USBORST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBORST`"]
pub type USBORST_R = crate::R<bool, USBORST_A>;
impl USBORST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBORST_A {
        match self.bits {
            false => USBORST_A::B_0X0,
            true => USBORST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == USBORST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == USBORST_A::B_0X1
    }
}
#[doc = "Write proxy for field `USBORST`"]
pub struct USBORST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBORST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBORST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(USBORST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(USBORST_A::B_0X1)
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
#[doc = "SDMMC3RST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMC3RST_A {
    #[doc = "0: Writing has no effect, reading means\r\n                  that the block reset is released"]
    B_0X0 = 0,
    #[doc = "1: Writing asserts the block reset,\r\n                  reading means that the block reset is\r\n                  asserted"]
    B_0X1 = 1,
}
impl From<SDMMC3RST_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC3RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDMMC3RST`"]
pub type SDMMC3RST_R = crate::R<bool, SDMMC3RST_A>;
impl SDMMC3RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMC3RST_A {
        match self.bits {
            false => SDMMC3RST_A::B_0X0,
            true => SDMMC3RST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SDMMC3RST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SDMMC3RST_A::B_0X1
    }
}
#[doc = "Write proxy for field `SDMMC3RST`"]
pub struct SDMMC3RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC3RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMC3RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Writing has no effect, reading means that the block reset is released"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SDMMC3RST_A::B_0X0)
    }
    #[doc = "Writing asserts the block reset, reading means that the block reset is asserted"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SDMMC3RST_A::B_0X1)
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
    #[doc = "Bit 0 - DMA1RST"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2RST"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    pub fn dmamuxrst(&self) -> DMAMUXRST_R {
        DMAMUXRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC12RST"]
    #[inline(always)]
    pub fn adc12rst(&self) -> ADC12RST_R {
        ADC12RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USBORST"]
    #[inline(always)]
    pub fn usborst(&self) -> USBORST_R {
        USBORST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SDMMC3RST"]
    #[inline(always)]
    pub fn sdmmc3rst(&self) -> SDMMC3RST_R {
        SDMMC3RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1RST"]
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W {
        DMA1RST_W { w: self }
    }
    #[doc = "Bit 1 - DMA2RST"]
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W {
        DMA2RST_W { w: self }
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    pub fn dmamuxrst(&mut self) -> DMAMUXRST_W {
        DMAMUXRST_W { w: self }
    }
    #[doc = "Bit 5 - ADC12RST"]
    #[inline(always)]
    pub fn adc12rst(&mut self) -> ADC12RST_W {
        ADC12RST_W { w: self }
    }
    #[doc = "Bit 8 - USBORST"]
    #[inline(always)]
    pub fn usborst(&mut self) -> USBORST_W {
        USBORST_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC3RST"]
    #[inline(always)]
    pub fn sdmmc3rst(&mut self) -> SDMMC3RST_W {
        SDMMC3RST_W { w: self }
    }
}
