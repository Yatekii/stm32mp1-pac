#[doc = "Reader of register SPISR1"]
pub type R = crate::R<u32, super::SPISR1>;
#[doc = "Reader of field `SPISR1`"]
pub type SPISR1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - shared peripheral interrupt"]
    #[inline(always)]
    pub fn spisr1(&self) -> SPISR1_R {
        SPISR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
