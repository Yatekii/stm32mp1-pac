#[doc = "Reader of register PID2"]
pub type R = crate::R<u32, super::PID2>;
#[doc = "Reader of field `PER_ID_2`"]
pub type PER_ID_2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID 2"]
    #[inline(always)]
    pub fn per_id_2(&self) -> PER_ID_2_R {
        PER_ID_2_R::new((self.bits & 0xff) as u8)
    }
}
