#[doc = "Reader of register CIDR0"]
pub type R = crate::R<u32, super::CIDR0>;
#[doc = "Reader of field `CIDR0`"]
pub type CIDR0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - component ID0"]
    #[inline(always)]
    pub fn cidr0(&self) -> CIDR0_R {
        CIDR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
