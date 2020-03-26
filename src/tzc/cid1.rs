#[doc = "Reader of register CID1"]
pub type R = crate::R<u32, super::CID1>;
#[doc = "Reader of field `COMP_ID_1`"]
pub type COMP_ID_1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Component ID 1"]
    #[inline(always)]
    pub fn comp_id_1(&self) -> COMP_ID_1_R {
        COMP_ID_1_R::new((self.bits & 0xff) as u8)
    }
}
