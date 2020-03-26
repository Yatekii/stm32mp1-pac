#[doc = "Reader of register FDCAN_TSCV"]
pub type R = crate::R<u32, super::FDCAN_TSCV>;
#[doc = "Reader of field `TSC`"]
pub type TSC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - TSC"]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
