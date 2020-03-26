#[doc = "Reader of register RNG_MID"]
pub type R = crate::R<u32, super::RNG_MID>;
#[doc = "Reader of field `MID`"]
pub type MID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - MID"]
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
