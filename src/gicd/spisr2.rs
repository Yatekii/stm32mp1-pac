#[doc = "Reader of register SPISR2"]
pub type R = crate::R<u32, super::SPISR2>;
#[doc = "Reader of field `SPISR2`"]
pub type SPISR2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - shared peripheral interrupt"]
    #[inline(always)]
    pub fn spisr2(&self) -> SPISR2_R {
        SPISR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
