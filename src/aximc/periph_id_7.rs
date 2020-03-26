#[doc = "Reader of register PERIPH_ID_7"]
pub type R = crate::R<u32, super::PERIPH_ID_7>;
#[doc = "Reader of field `PERIPH_ID_7`"]
pub type PERIPH_ID_7_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - not used"]
    #[inline(always)]
    pub fn periph_id_7(&self) -> PERIPH_ID_7_R {
        PERIPH_ID_7_R::new((self.bits & 0xff) as u8)
    }
}
