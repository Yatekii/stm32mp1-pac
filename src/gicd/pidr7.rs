#[doc = "Reader of register PIDR7"]
pub type R = crate::R<u32, super::PIDR7>;
#[doc = "Reader of field `PIDR7`"]
pub type PIDR7_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - peripheral ID7"]
    #[inline(always)]
    pub fn pidr7(&self) -> PIDR7_R {
        PIDR7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
