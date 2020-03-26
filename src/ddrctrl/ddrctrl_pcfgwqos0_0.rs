#[doc = "Reader of register DDRCTRL_PCFGWQOS0_0"]
pub type R = crate::R<u32, super::DDRCTRL_PCFGWQOS0_0>;
#[doc = "Writer for register DDRCTRL_PCFGWQOS0_0"]
pub type W = crate::W<u32, super::DDRCTRL_PCFGWQOS0_0>;
#[doc = "Register DDRCTRL_PCFGWQOS0_0 `reset()`'s with value 0x0e00"]
impl crate::ResetValue for super::DDRCTRL_PCFGWQOS0_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0e00
    }
}
#[doc = "Reader of field `WQOS_MAP_LEVEL1`"]
pub type WQOS_MAP_LEVEL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WQOS_MAP_LEVEL1`"]
pub struct WQOS_MAP_LEVEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> WQOS_MAP_LEVEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `WQOS_MAP_LEVEL2`"]
pub type WQOS_MAP_LEVEL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WQOS_MAP_LEVEL2`"]
pub struct WQOS_MAP_LEVEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> WQOS_MAP_LEVEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "WQOS_MAP_REGION0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WQOS_MAP_REGION0_A {
    #[doc = "0: NPW, 1: VPW."]
    B_0X0 = 0,
}
impl From<WQOS_MAP_REGION0_A> for u8 {
    #[inline(always)]
    fn from(variant: WQOS_MAP_REGION0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WQOS_MAP_REGION0`"]
pub type WQOS_MAP_REGION0_R = crate::R<u8, WQOS_MAP_REGION0_A>;
impl WQOS_MAP_REGION0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WQOS_MAP_REGION0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WQOS_MAP_REGION0_A::B_0X0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WQOS_MAP_REGION0_A::B_0X0
    }
}
#[doc = "Write proxy for field `WQOS_MAP_REGION0`"]
pub struct WQOS_MAP_REGION0_W<'a> {
    w: &'a mut W,
}
impl<'a> WQOS_MAP_REGION0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WQOS_MAP_REGION0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "NPW, 1: VPW."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WQOS_MAP_REGION0_A::B_0X0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "WQOS_MAP_REGION1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WQOS_MAP_REGION1_A {
    #[doc = "0: NPW, 1: VPW."]
    B_0X0 = 0,
}
impl From<WQOS_MAP_REGION1_A> for u8 {
    #[inline(always)]
    fn from(variant: WQOS_MAP_REGION1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WQOS_MAP_REGION1`"]
pub type WQOS_MAP_REGION1_R = crate::R<u8, WQOS_MAP_REGION1_A>;
impl WQOS_MAP_REGION1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WQOS_MAP_REGION1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WQOS_MAP_REGION1_A::B_0X0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WQOS_MAP_REGION1_A::B_0X0
    }
}
#[doc = "Write proxy for field `WQOS_MAP_REGION1`"]
pub struct WQOS_MAP_REGION1_W<'a> {
    w: &'a mut W,
}
impl<'a> WQOS_MAP_REGION1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WQOS_MAP_REGION1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "NPW, 1: VPW."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WQOS_MAP_REGION1_A::B_0X0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "WQOS_MAP_REGION2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WQOS_MAP_REGION2_A {
    #[doc = "0: NPW, 1: VPW."]
    B_0X0 = 0,
}
impl From<WQOS_MAP_REGION2_A> for u8 {
    #[inline(always)]
    fn from(variant: WQOS_MAP_REGION2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WQOS_MAP_REGION2`"]
pub type WQOS_MAP_REGION2_R = crate::R<u8, WQOS_MAP_REGION2_A>;
impl WQOS_MAP_REGION2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WQOS_MAP_REGION2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WQOS_MAP_REGION2_A::B_0X0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WQOS_MAP_REGION2_A::B_0X0
    }
}
#[doc = "Write proxy for field `WQOS_MAP_REGION2`"]
pub struct WQOS_MAP_REGION2_W<'a> {
    w: &'a mut W,
}
impl<'a> WQOS_MAP_REGION2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WQOS_MAP_REGION2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "NPW, 1: VPW."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WQOS_MAP_REGION2_A::B_0X0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - WQOS_MAP_LEVEL1"]
    #[inline(always)]
    pub fn wqos_map_level1(&self) -> WQOS_MAP_LEVEL1_R {
        WQOS_MAP_LEVEL1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - WQOS_MAP_LEVEL2"]
    #[inline(always)]
    pub fn wqos_map_level2(&self) -> WQOS_MAP_LEVEL2_R {
        WQOS_MAP_LEVEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - WQOS_MAP_REGION0"]
    #[inline(always)]
    pub fn wqos_map_region0(&self) -> WQOS_MAP_REGION0_R {
        WQOS_MAP_REGION0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - WQOS_MAP_REGION1"]
    #[inline(always)]
    pub fn wqos_map_region1(&self) -> WQOS_MAP_REGION1_R {
        WQOS_MAP_REGION1_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - WQOS_MAP_REGION2"]
    #[inline(always)]
    pub fn wqos_map_region2(&self) -> WQOS_MAP_REGION2_R {
        WQOS_MAP_REGION2_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - WQOS_MAP_LEVEL1"]
    #[inline(always)]
    pub fn wqos_map_level1(&mut self) -> WQOS_MAP_LEVEL1_W {
        WQOS_MAP_LEVEL1_W { w: self }
    }
    #[doc = "Bits 8:11 - WQOS_MAP_LEVEL2"]
    #[inline(always)]
    pub fn wqos_map_level2(&mut self) -> WQOS_MAP_LEVEL2_W {
        WQOS_MAP_LEVEL2_W { w: self }
    }
    #[doc = "Bits 16:17 - WQOS_MAP_REGION0"]
    #[inline(always)]
    pub fn wqos_map_region0(&mut self) -> WQOS_MAP_REGION0_W {
        WQOS_MAP_REGION0_W { w: self }
    }
    #[doc = "Bits 20:21 - WQOS_MAP_REGION1"]
    #[inline(always)]
    pub fn wqos_map_region1(&mut self) -> WQOS_MAP_REGION1_W {
        WQOS_MAP_REGION1_W { w: self }
    }
    #[doc = "Bits 24:25 - WQOS_MAP_REGION2"]
    #[inline(always)]
    pub fn wqos_map_region2(&mut self) -> WQOS_MAP_REGION2_W {
        WQOS_MAP_REGION2_W { w: self }
    }
}
