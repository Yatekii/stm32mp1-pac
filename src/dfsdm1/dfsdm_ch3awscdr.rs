#[doc = "Reader of register DFSDM_CH3AWSCDR"]
pub type R = crate::R<u32, super::DFSDM_CH3AWSCDR>;
#[doc = "Writer for register DFSDM_CH3AWSCDR"]
pub type W = crate::W<u32, super::DFSDM_CH3AWSCDR>;
#[doc = "Register DFSDM_CH3AWSCDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DFSDM_CH3AWSCDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCDT`"]
pub type SCDT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCDT`"]
pub struct SCDT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `BKSCD`"]
pub type BKSCD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BKSCD`"]
pub struct BKSCD_W<'a> {
    w: &'a mut W,
}
impl<'a> BKSCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `AWFOSR`"]
pub type AWFOSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AWFOSR`"]
pub struct AWFOSR_W<'a> {
    w: &'a mut W,
}
impl<'a> AWFOSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "AWFORD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AWFORD_A {
    #[doc = "0: FastSinc filter type"]
    B_0X0 = 0,
    #[doc = "1: Sinc1 filter type"]
    B_0X1 = 1,
    #[doc = "2: Sinc2 filter type"]
    B_0X2 = 2,
    #[doc = "3: Sinc3 filter type"]
    B_0X3 = 3,
}
impl From<AWFORD_A> for u8 {
    #[inline(always)]
    fn from(variant: AWFORD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AWFORD`"]
pub type AWFORD_R = crate::R<u8, AWFORD_A>;
impl AWFORD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWFORD_A {
        match self.bits {
            0 => AWFORD_A::B_0X0,
            1 => AWFORD_A::B_0X1,
            2 => AWFORD_A::B_0X2,
            3 => AWFORD_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWFORD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWFORD_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == AWFORD_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == AWFORD_A::B_0X3
    }
}
#[doc = "Write proxy for field `AWFORD`"]
pub struct AWFORD_W<'a> {
    w: &'a mut W,
}
impl<'a> AWFORD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWFORD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FastSinc filter type"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(AWFORD_A::B_0X0)
    }
    #[doc = "Sinc1 filter type"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(AWFORD_A::B_0X1)
    }
    #[doc = "Sinc2 filter type"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(AWFORD_A::B_0X2)
    }
    #[doc = "Sinc3 filter type"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(AWFORD_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SCDT"]
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - BKSCD"]
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - AWFOSR"]
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - AWFORD"]
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCDT"]
    #[inline(always)]
    pub fn scdt(&mut self) -> SCDT_W {
        SCDT_W { w: self }
    }
    #[doc = "Bits 12:15 - BKSCD"]
    #[inline(always)]
    pub fn bkscd(&mut self) -> BKSCD_W {
        BKSCD_W { w: self }
    }
    #[doc = "Bits 16:20 - AWFOSR"]
    #[inline(always)]
    pub fn awfosr(&mut self) -> AWFOSR_W {
        AWFOSR_W { w: self }
    }
    #[doc = "Bits 22:23 - AWFORD"]
    #[inline(always)]
    pub fn awford(&mut self) -> AWFORD_W {
        AWFORD_W { w: self }
    }
}
