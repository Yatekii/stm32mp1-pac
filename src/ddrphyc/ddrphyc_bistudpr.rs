#[doc = "Reader of register DDRPHYC_BISTUDPR"]
pub type R = crate::R<u32, super::DDRPHYC_BISTUDPR>;
#[doc = "Writer for register DDRPHYC_BISTUDPR"]
pub type W = crate::W<u32, super::DDRPHYC_BISTUDPR>;
#[doc = "Register DDRPHYC_BISTUDPR `reset()`'s with value 0xffff_0000"]
impl crate::ResetValue for super::DDRPHYC_BISTUDPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_0000
    }
}
#[doc = "Reader of field `BUDP0`"]
pub type BUDP0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BUDP0`"]
pub struct BUDP0_W<'a> {
    w: &'a mut W,
}
impl<'a> BUDP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `BUDP1`"]
pub type BUDP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BUDP1`"]
pub struct BUDP1_W<'a> {
    w: &'a mut W,
}
impl<'a> BUDP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - BUDP0"]
    #[inline(always)]
    pub fn budp0(&self) -> BUDP0_R {
        BUDP0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - BUDP1"]
    #[inline(always)]
    pub fn budp1(&self) -> BUDP1_R {
        BUDP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BUDP0"]
    #[inline(always)]
    pub fn budp0(&mut self) -> BUDP0_W {
        BUDP0_W { w: self }
    }
    #[doc = "Bits 16:31 - BUDP1"]
    #[inline(always)]
    pub fn budp1(&mut self) -> BUDP1_W {
        BUDP1_W { w: self }
    }
}
