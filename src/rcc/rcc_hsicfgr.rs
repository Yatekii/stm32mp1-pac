#[doc = "Reader of register RCC_HSICFGR"]
pub type R = crate::R<u32, super::RCC_HSICFGR>;
#[doc = "Writer for register RCC_HSICFGR"]
pub type W = crate::W<u32, super::RCC_HSICFGR>;
#[doc = "Register RCC_HSICFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_HSICFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HSIDIV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HSIDIV_A {
    #[doc = "0: Division by 1, hsi_ck (hsi_ker_ck) =\r\n                  64 MHz (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: Division by 2, hsi_ck (hsi_ker_ck) =\r\n                  32 MHz"]
    B_0X1 = 1,
    #[doc = "2: Division by 4, hsi_ck (hsi_ker_ck) =\r\n                  16 MHz"]
    B_0X2 = 2,
    #[doc = "3: Division by 8, hsi_ck (hsi_ker_ck) =\r\n                  8 MHz"]
    B_0X3 = 3,
}
impl From<HSIDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: HSIDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HSIDIV`"]
pub type HSIDIV_R = crate::R<u8, HSIDIV_A>;
impl HSIDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIDIV_A {
        match self.bits {
            0 => HSIDIV_A::B_0X0,
            1 => HSIDIV_A::B_0X1,
            2 => HSIDIV_A::B_0X2,
            3 => HSIDIV_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIDIV_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIDIV_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == HSIDIV_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == HSIDIV_A::B_0X3
    }
}
#[doc = "Write proxy for field `HSIDIV`"]
pub struct HSIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSIDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Division by 1, hsi_ck (hsi_ker_ck) = 64 MHz (default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSIDIV_A::B_0X0)
    }
    #[doc = "Division by 2, hsi_ck (hsi_ker_ck) = 32 MHz"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(HSIDIV_A::B_0X1)
    }
    #[doc = "Division by 4, hsi_ck (hsi_ker_ck) = 16 MHz"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(HSIDIV_A::B_0X2)
    }
    #[doc = "Division by 8, hsi_ck (hsi_ker_ck) = 8 MHz"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(HSIDIV_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "HSITRIM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HSITRIM_A {
    #[doc = "0: bsec_hsi_cal\\[11:0\\]
(default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "62: bsec_hsi_cal\\[11:0\\]
+\r\n                  62"]
    B_0X3E = 62,
    #[doc = "63: bsec_hsi_cal\\[11:0\\]
+\r\n                  63"]
    B_0X3F = 63,
    #[doc = "64: bsec_hsi_cal\\[11:0\\]
-\r\n                  64"]
    B_0X40 = 64,
    #[doc = "65: bsec_hsi_cal\\[11:0\\]
-\r\n                  63"]
    B_0X41 = 65,
}
impl From<HSITRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: HSITRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `HSITRIM`"]
pub type HSITRIM_R = crate::R<u8, HSITRIM_A>;
impl HSITRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HSITRIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HSITRIM_A::B_0X0),
            62 => Val(HSITRIM_A::B_0X3E),
            63 => Val(HSITRIM_A::B_0X3F),
            64 => Val(HSITRIM_A::B_0X40),
            65 => Val(HSITRIM_A::B_0X41),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSITRIM_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X3E`"]
    #[inline(always)]
    pub fn is_b_0x3e(&self) -> bool {
        *self == HSITRIM_A::B_0X3E
    }
    #[doc = "Checks if the value of the field is `B_0X3F`"]
    #[inline(always)]
    pub fn is_b_0x3f(&self) -> bool {
        *self == HSITRIM_A::B_0X3F
    }
    #[doc = "Checks if the value of the field is `B_0X40`"]
    #[inline(always)]
    pub fn is_b_0x40(&self) -> bool {
        *self == HSITRIM_A::B_0X40
    }
    #[doc = "Checks if the value of the field is `B_0X41`"]
    #[inline(always)]
    pub fn is_b_0x41(&self) -> bool {
        *self == HSITRIM_A::B_0X41
    }
}
#[doc = "Write proxy for field `HSITRIM`"]
pub struct HSITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSITRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSITRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "bsec_hsi_cal\\[11:0\\]
(default after reset)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(HSITRIM_A::B_0X0)
    }
    #[doc = "bsec_hsi_cal\\[11:0\\]
+ 62"]
    #[inline(always)]
    pub fn b_0x3e(self) -> &'a mut W {
        self.variant(HSITRIM_A::B_0X3E)
    }
    #[doc = "bsec_hsi_cal\\[11:0\\]
+ 63"]
    #[inline(always)]
    pub fn b_0x3f(self) -> &'a mut W {
        self.variant(HSITRIM_A::B_0X3F)
    }
    #[doc = "bsec_hsi_cal\\[11:0\\]
- 64"]
    #[inline(always)]
    pub fn b_0x40(self) -> &'a mut W {
        self.variant(HSITRIM_A::B_0X40)
    }
    #[doc = "bsec_hsi_cal\\[11:0\\]
- 63"]
    #[inline(always)]
    pub fn b_0x41(self) -> &'a mut W {
        self.variant(HSITRIM_A::B_0X41)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HSICAL`"]
pub type HSICAL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:1 - HSIDIV"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HSIDIV_R {
        HSIDIV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:14 - HSITRIM"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - HSICAL"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - HSIDIV"]
    #[inline(always)]
    pub fn hsidiv(&mut self) -> HSIDIV_W {
        HSIDIV_W { w: self }
    }
    #[doc = "Bits 8:14 - HSITRIM"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W {
        HSITRIM_W { w: self }
    }
}
