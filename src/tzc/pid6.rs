#[doc = "Reader of register PID6"]
pub type R = crate::R<u32, super::PID6>;
#[doc = "Reader of field `PER_ID_6`"]
pub type PER_ID_6_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID 6"]
    #[inline(always)]
    pub fn per_id_6(&self) -> PER_ID_6_R {
        PER_ID_6_R::new((self.bits & 0xff) as u8)
    }
}
