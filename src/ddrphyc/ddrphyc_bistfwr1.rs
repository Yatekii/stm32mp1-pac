#[doc = "Reader of register DDRPHYC_BISTFWR1"]
pub type R = crate::R<u32, super::DDRPHYC_BISTFWR1>;
#[doc = "Reader of field `DQWEBS`"]
pub type DQWEBS_R = crate::R<u16, u16>;
#[doc = "Reader of field `DMWEBS`"]
pub type DMWEBS_R = crate::R<u8, u8>;
#[doc = "Reader of field `RASWEBS`"]
pub type RASWEBS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CASWEBS`"]
pub type CASWEBS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PARWEBS`"]
pub type PARWEBS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TPDWEBS`"]
pub type TPDWEBS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - DQWEBS"]
    #[inline(always)]
    pub fn dqwebs(&self) -> DQWEBS_R {
        DQWEBS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - DMWEBS"]
    #[inline(always)]
    pub fn dmwebs(&self) -> DMWEBS_R {
        DMWEBS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - RASWEBS"]
    #[inline(always)]
    pub fn raswebs(&self) -> RASWEBS_R {
        RASWEBS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CASWEBS"]
    #[inline(always)]
    pub fn caswebs(&self) -> CASWEBS_R {
        CASWEBS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 30 - PARWEBS"]
    #[inline(always)]
    pub fn parwebs(&self) -> PARWEBS_R {
        PARWEBS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - TPDWEBS"]
    #[inline(always)]
    pub fn tpdwebs(&self) -> TPDWEBS_R {
        TPDWEBS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
