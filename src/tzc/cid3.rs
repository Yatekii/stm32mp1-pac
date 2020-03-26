#[doc = "Reader of register CID3"]
pub type R = crate::R<u32, super::CID3>;
#[doc = "Reader of field `COMP_ID_3`"]
pub type COMP_ID_3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Component ID 3"]
    #[inline(always)]
    pub fn comp_id_3(&self) -> COMP_ID_3_R {
        COMP_ID_3_R::new((self.bits & 0xff) as u8)
    }
}
