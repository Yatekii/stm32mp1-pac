#[doc = "Reader of register ADC_JDR3"]
pub type R = crate::R<u32, super::ADC_JDR3>;
#[doc = "Reader of field `JDATA3`"]
pub type JDATA3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADC group injected sequencer rank 3 conversion data"]
    #[inline(always)]
    pub fn jdata3(&self) -> JDATA3_R {
        JDATA3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
