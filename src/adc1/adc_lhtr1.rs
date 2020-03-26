#[doc = "Reader of register ADC_LHTR1"]
pub type R = crate::R<u32, super::ADC_LHTR1>;
#[doc = "Writer for register ADC_LHTR1"]
pub type W = crate::W<u32, super::ADC_LHTR1>;
#[doc = "Register ADC_LHTR1 `reset()`'s with value 0x0fff_0000"]
impl crate::ResetValue for super::ADC_LHTR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
    }
}
#[doc = "Reader of field `LHTR1`"]
pub type LHTR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LHTR1`"]
pub struct LHTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> LHTR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn lhtr1(&self) -> LHTR1_R {
        LHTR1_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn lhtr1(&mut self) -> LHTR1_W {
        LHTR1_W { w: self }
    }
}