#[doc = "Reader of register PIDR2"]
pub type R = crate::R<u32, super::PIDR2>;
#[doc = "Reader of field `PIDR2`"]
pub type PIDR2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - peripheral ID2"]
    #[inline(always)]
    pub fn pidr2(&self) -> PIDR2_R {
        PIDR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
