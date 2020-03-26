#[doc = "Reader of register FDCAN_DBTP"]
pub type R = crate::R<u32, super::FDCAN_DBTP>;
#[doc = "Writer for register FDCAN_DBTP"]
pub type W = crate::W<u32, super::FDCAN_DBTP>;
#[doc = "Register FDCAN_DBTP `reset()`'s with value 0x0a33"]
impl crate::ResetValue for super::FDCAN_DBTP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a33
    }
}
#[doc = "Reader of field `DSJW`"]
pub type DSJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSJW`"]
pub struct DSJW_W<'a> {
    w: &'a mut W,
}
impl<'a> DSJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DTSEG2`"]
pub type DTSEG2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTSEG2`"]
pub struct DTSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `DTSEG1`"]
pub struct DTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DBRP`"]
pub type DBRP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBRP`"]
pub struct DBRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "TDC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDC_A {
    #[doc = "0: Transceiver Delay Compensation\r\n                  disabled"]
    B_0X0 = 0,
    #[doc = "1: Transceiver Delay Compensation\r\n                  enabled"]
    B_0X1 = 1,
}
impl From<TDC_A> for bool {
    #[inline(always)]
    fn from(variant: TDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDC`"]
pub type TDC_R = crate::R<bool, TDC_A>;
impl TDC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDC_A {
        match self.bits {
            false => TDC_A::B_0X0,
            true => TDC_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TDC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TDC_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:3 - DSJW"]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DTSEG2"]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - DBRP"]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - TDC"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DSJW"]
    #[inline(always)]
    pub fn dsjw(&mut self) -> DSJW_W {
        DSJW_W { w: self }
    }
    #[doc = "Bits 4:7 - DTSEG2"]
    #[inline(always)]
    pub fn dtseg2(&mut self) -> DTSEG2_W {
        DTSEG2_W { w: self }
    }
    #[doc = "Bits 8:12 - DTSEG1"]
    #[inline(always)]
    pub fn dtseg1(&mut self) -> DTSEG1_W {
        DTSEG1_W { w: self }
    }
    #[doc = "Bits 16:20 - DBRP"]
    #[inline(always)]
    pub fn dbrp(&mut self) -> DBRP_W {
        DBRP_W { w: self }
    }
}
