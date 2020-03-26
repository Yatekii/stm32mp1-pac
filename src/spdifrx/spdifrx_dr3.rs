#[doc = "Reader of register SPDIFRX_DR3"]
pub type R = crate::R<u32, super::SPDIFRX_DR3>;
#[doc = "Reader of field `DRNL1`"]
pub type DRNL1_R = crate::R<u16, u16>;
#[doc = "Reader of field `DRNL2`"]
pub type DRNL2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - DRNL1"]
    #[inline(always)]
    pub fn drnl1(&self) -> DRNL1_R {
        DRNL1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DRNL2"]
    #[inline(always)]
    pub fn drnl2(&self) -> DRNL2_R {
        DRNL2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
