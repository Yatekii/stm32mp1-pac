#[doc = "Reader of register PERIPH_ID_0"]
pub type R = crate::R<u32, super::PERIPH_ID_0>;
#[doc = "Reader of field `PERIPH_ID_0`"]
pub type PERIPH_ID_0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - part number"]
    #[inline(always)]
    pub fn periph_id_0(&self) -> PERIPH_ID_0_R {
        PERIPH_ID_0_R::new((self.bits & 0xff) as u8)
    }
}
