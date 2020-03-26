#[doc = "Reader of register PID4"]
pub type R = crate::R<u32, super::PID4>;
#[doc = "Reader of field `PER_ID_4`"]
pub type PER_ID_4_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID 4"]
    #[inline(always)]
    pub fn per_id_4(&self) -> PER_ID_4_R {
        PER_ID_4_R::new((self.bits & 0xff) as u8)
    }
}
