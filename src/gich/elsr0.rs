#[doc = "Reader of register ELSR0"]
pub type R = crate::R<u32, super::ELSR0>;
#[doc = "Reader of field `ELSR0`"]
pub type ELSR0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - end of interrupt status"]
    #[inline(always)]
    pub fn elsr0(&self) -> ELSR0_R {
        ELSR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
