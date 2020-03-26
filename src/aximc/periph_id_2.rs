#[doc = "Reader of register PERIPH_ID_2"]
pub type R = crate::R<u32, super::PERIPH_ID_2>;
#[doc = "Reader of field `PERIPH_ID_2`"]
pub type PERIPH_ID_2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - part number"]
    #[inline(always)]
    pub fn periph_id_2(&self) -> PERIPH_ID_2_R {
        PERIPH_ID_2_R::new((self.bits & 0xff) as u8)
    }
}
