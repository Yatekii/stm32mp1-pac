#[doc = "Reader of register FAIL_ID0"]
pub type R = crate::R<u32, super::FAIL_ID0>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - AXI fail ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x07ff) as u16)
    }
}
