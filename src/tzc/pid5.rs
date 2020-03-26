#[doc = "Reader of register PID5"]
pub type R = crate::R<u32, super::PID5>;
#[doc = "Reader of field `PER_ID_5`"]
pub type PER_ID_5_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID 5"]
    #[inline(always)]
    pub fn per_id_5(&self) -> PER_ID_5_R {
        PER_ID_5_R::new((self.bits & 0xff) as u8)
    }
}
