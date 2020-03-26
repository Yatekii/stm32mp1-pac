#[doc = "Reader of register DDRPHYC_BISTWCSR"]
pub type R = crate::R<u32, super::DDRPHYC_BISTWCSR>;
#[doc = "Reader of field `ACWCNT`"]
pub type ACWCNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `DXWCNT`"]
pub type DXWCNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ACWCNT"]
    #[inline(always)]
    pub fn acwcnt(&self) -> ACWCNT_R {
        ACWCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DXWCNT"]
    #[inline(always)]
    pub fn dxwcnt(&self) -> DXWCNT_R {
        DXWCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
