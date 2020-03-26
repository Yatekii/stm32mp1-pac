#[doc = "Reader of register DDRPHYC_BISTFWR0"]
pub type R = crate::R<u32, super::DDRPHYC_BISTFWR0>;
#[doc = "Reader of field `AWEBS`"]
pub type AWEBS_R = crate::R<u16, u16>;
#[doc = "Reader of field `BAWEBS`"]
pub type BAWEBS_R = crate::R<u8, u8>;
#[doc = "Reader of field `WEWEBS`"]
pub type WEWEBS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CKEWEBS`"]
pub type CKEWEBS_R = crate::R<u8, u8>;
#[doc = "Reader of field `CSWEBS`"]
pub type CSWEBS_R = crate::R<u8, u8>;
#[doc = "Reader of field `ODTWEBS`"]
pub type ODTWEBS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - AWEBS"]
    #[inline(always)]
    pub fn awebs(&self) -> AWEBS_R {
        AWEBS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - BAWEBS"]
    #[inline(always)]
    pub fn bawebs(&self) -> BAWEBS_R {
        BAWEBS_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - WEWEBS"]
    #[inline(always)]
    pub fn wewebs(&self) -> WEWEBS_R {
        WEWEBS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - CKEWEBS"]
    #[inline(always)]
    pub fn ckewebs(&self) -> CKEWEBS_R {
        CKEWEBS_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CSWEBS"]
    #[inline(always)]
    pub fn cswebs(&self) -> CSWEBS_R {
        CSWEBS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - ODTWEBS"]
    #[inline(always)]
    pub fn odtwebs(&self) -> ODTWEBS_R {
        ODTWEBS_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
