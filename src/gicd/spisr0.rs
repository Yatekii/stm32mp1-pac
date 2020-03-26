#[doc = "Reader of register SPISR0"]
pub type R = crate::R<u32, super::SPISR0>;
#[doc = "Reader of field `SPISR0`"]
pub type SPISR0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - shared peripheral interrupt"]
    #[inline(always)]
    pub fn spisr0(&self) -> SPISR0_R {
        SPISR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
