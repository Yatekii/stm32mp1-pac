#[doc = "Reader of register APR0"]
pub type R = crate::R<u32, super::APR0>;
#[doc = "Reader of field `APR0`"]
pub type APR0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - active priority"]
    #[inline(always)]
    pub fn apr0(&self) -> APR0_R {
        APR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
