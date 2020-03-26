#[doc = "Reader of register PID0"]
pub type R = crate::R<u32, super::PID0>;
#[doc = "Reader of field `PER_ID_0`"]
pub type PER_ID_0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID 0"]
    #[inline(always)]
    pub fn per_id_0(&self) -> PER_ID_0_R {
        PER_ID_0_R::new((self.bits & 0xff) as u8)
    }
}
