#[doc = "Reader of register SPISR6"]
pub type R = crate::R<u32, super::SPISR6>;
#[doc = "Reader of field `SPISR6`"]
pub type SPISR6_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - shared peripheral interrupt"]
    #[inline(always)]
    pub fn spisr6(&self) -> SPISR6_R {
        SPISR6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
