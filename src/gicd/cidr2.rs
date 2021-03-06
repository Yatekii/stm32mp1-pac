#[doc = "Reader of register CIDR2"]
pub type R = crate::R<u32, super::CIDR2>;
#[doc = "Reader of field `CIDR2`"]
pub type CIDR2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - component ID2"]
    #[inline(always)]
    pub fn cidr2(&self) -> CIDR2_R {
        CIDR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
