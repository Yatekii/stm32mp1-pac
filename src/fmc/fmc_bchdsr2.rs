#[doc = "Reader of register FMC_BCHDSR2"]
pub type R = crate::R<u32, super::FMC_BCHDSR2>;
#[doc = "Reader of field `EBP0_13`"]
pub type EBP0_13_R = crate::R<u16, u16>;
#[doc = "Reader of field `EBP16_28`"]
pub type EBP16_28_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:12 - EBP0_13"]
    #[inline(always)]
    pub fn ebp0_13(&self) -> EBP0_13_R {
        EBP0_13_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - EBP16_28"]
    #[inline(always)]
    pub fn ebp16_28(&self) -> EBP16_28_R {
        EBP16_28_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
