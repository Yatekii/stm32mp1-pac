#[doc = "Reader of register PIDR0"]
pub type R = crate::R<u32, super::PIDR0>;
#[doc = "Reader of field `PIDR0`"]
pub type PIDR0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - peripheral ID0"]
    #[inline(always)]
    pub fn pidr0(&self) -> PIDR0_R {
        PIDR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
