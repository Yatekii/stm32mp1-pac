#[doc = "Reader of register REGION_BASE_LOW0"]
pub type R = crate::R<u32, super::REGION_BASE_LOW0>;
#[doc = "Reader of field `BASE_ADDRESS_LOW`"]
pub type BASE_ADDRESS_LOW_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 12:31 - base address bits"]
    #[inline(always)]
    pub fn base_address_low(&self) -> BASE_ADDRESS_LOW_R {
        BASE_ADDRESS_LOW_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
