#[doc = "Reader of register FAIL_ADDRESS_LOW0"]
pub type R = crate::R<u32, super::FAIL_ADDRESS_LOW0>;
#[doc = "Reader of field `ADDR_STATUS_LOW`"]
pub type ADDR_STATUS_LOW_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fail address low bits"]
    #[inline(always)]
    pub fn addr_status_low(&self) -> ADDR_STATUS_LOW_R {
        ADDR_STATUS_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
