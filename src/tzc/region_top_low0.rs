#[doc = "Reader of register REGION_TOP_LOW0"]
pub type R = crate::R<u32, super::REGION_TOP_LOW0>;
#[doc = "Reader of field `TOP_ADDRESS_LOW`"]
pub type TOP_ADDRESS_LOW_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 12:31 - Top address bits"]
    #[inline(always)]
    pub fn top_address_low(&self) -> TOP_ADDRESS_LOW_R {
        TOP_ADDRESS_LOW_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
