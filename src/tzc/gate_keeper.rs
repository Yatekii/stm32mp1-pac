#[doc = "Reader of register GATE_KEEPER"]
pub type R = crate::R<u32, super::GATE_KEEPER>;
#[doc = "Writer for register GATE_KEEPER"]
pub type W = crate::W<u32, super::GATE_KEEPER>;
#[doc = "Register GATE_KEEPER `reset()`'s with value 0"]
impl crate::ResetValue for super::GATE_KEEPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OPENREQ`"]
pub type OPENREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPENREQ`"]
pub struct OPENREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> OPENREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `OPENSTAT`"]
pub type OPENSTAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPENSTAT`"]
pub struct OPENSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPENSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Gate keeper open request"]
    #[inline(always)]
    pub fn openreq(&self) -> OPENREQ_R {
        OPENREQ_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Gate keeper status for each filter"]
    #[inline(always)]
    pub fn openstat(&self) -> OPENSTAT_R {
        OPENSTAT_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Gate keeper open request"]
    #[inline(always)]
    pub fn openreq(&mut self) -> OPENREQ_W {
        OPENREQ_W { w: self }
    }
    #[doc = "Bits 16:17 - Gate keeper status for each filter"]
    #[inline(always)]
    pub fn openstat(&mut self) -> OPENSTAT_W {
        OPENSTAT_W { w: self }
    }
}
