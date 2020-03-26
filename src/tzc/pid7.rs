#[doc = "Reader of register PID7"]
pub type R = crate::R<u32, super::PID7>;
#[doc = "Reader of field `PER_ID_7`"]
pub type PER_ID_7_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID 7"]
    #[inline(always)]
    pub fn per_id_7(&self) -> PER_ID_7_R {
        PER_ID_7_R::new((self.bits & 0xff) as u8)
    }
}
