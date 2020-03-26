#[doc = "Reader of register DDRPHYC_BISTBER0"]
pub type R = crate::R<u32, super::DDRPHYC_BISTBER0>;
#[doc = "Reader of field `ABER`"]
pub type ABER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ABER"]
    #[inline(always)]
    pub fn aber(&self) -> ABER_R {
        ABER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
