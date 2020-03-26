#[doc = "Reader of register PID3"]
pub type R = crate::R<u32, super::PID3>;
#[doc = "Reader of field `PER_ID_3`"]
pub type PER_ID_3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID 3"]
    #[inline(always)]
    pub fn per_id_3(&self) -> PER_ID_3_R {
        PER_ID_3_R::new((self.bits & 0xff) as u8)
    }
}
