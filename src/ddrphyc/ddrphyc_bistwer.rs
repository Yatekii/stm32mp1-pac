#[doc = "Reader of register DDRPHYC_BISTWER"]
pub type R = crate::R<u32, super::DDRPHYC_BISTWER>;
#[doc = "Reader of field `ACWER`"]
pub type ACWER_R = crate::R<u16, u16>;
#[doc = "Reader of field `DXWER`"]
pub type DXWER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ACWER"]
    #[inline(always)]
    pub fn acwer(&self) -> ACWER_R {
        ACWER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DXWER"]
    #[inline(always)]
    pub fn dxwer(&self) -> DXWER_R {
        DXWER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
