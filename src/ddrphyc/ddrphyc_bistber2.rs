#[doc = "Reader of register DDRPHYC_BISTBER2"]
pub type R = crate::R<u32, super::DDRPHYC_BISTBER2>;
#[doc = "Reader of field `DQBER`"]
pub type DQBER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DQBER"]
    #[inline(always)]
    pub fn dqber(&self) -> DQBER_R {
        DQBER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
