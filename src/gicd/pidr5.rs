#[doc = "Reader of register PIDR5"]
pub type R = crate::R<u32, super::PIDR5>;
#[doc = "Reader of field `PIDR5`"]
pub type PIDR5_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - peripheral ID5"]
    #[inline(always)]
    pub fn pidr5(&self) -> PIDR5_R {
        PIDR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
