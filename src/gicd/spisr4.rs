#[doc = "Reader of register SPISR4"]
pub type R = crate::R<u32, super::SPISR4>;
#[doc = "Reader of field `SPISR4`"]
pub type SPISR4_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - shared peripheral interrupt"]
    #[inline(always)]
    pub fn spisr4(&self) -> SPISR4_R {
        SPISR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
