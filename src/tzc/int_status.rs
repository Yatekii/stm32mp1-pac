#[doc = "Reader of register INT_STATUS"]
pub type R = crate::R<u32, super::INT_STATUS>;
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<u8, u8>;
#[doc = "Reader of field `OVERLAP`"]
pub type OVERLAP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Interrupt status for each filter"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Permission failure overrun"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Overlap violation for each filter"]
    #[inline(always)]
    pub fn overlap(&self) -> OVERLAP_R {
        OVERLAP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
