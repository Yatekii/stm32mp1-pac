#[doc = "Reader of register DDRPHYC_BISTGSR"]
pub type R = crate::R<u32, super::DDRPHYC_BISTGSR>;
#[doc = "Reader of field `BDONE`"]
pub type BDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BACERR`"]
pub type BACERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BDXERR`"]
pub type BDXERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PARBER`"]
pub type PARBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `TPDBER`"]
pub type TPDBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `DMBER`"]
pub type DMBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `RASBER`"]
pub type RASBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `CASBER`"]
pub type CASBER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - BDONE"]
    #[inline(always)]
    pub fn bdone(&self) -> BDONE_R {
        BDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BACERR"]
    #[inline(always)]
    pub fn bacerr(&self) -> BACERR_R {
        BACERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BDXERR"]
    #[inline(always)]
    pub fn bdxerr(&self) -> BDXERR_R {
        BDXERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - PARBER"]
    #[inline(always)]
    pub fn parber(&self) -> PARBER_R {
        PARBER_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - TPDBER"]
    #[inline(always)]
    pub fn tpdber(&self) -> TPDBER_R {
        TPDBER_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - DMBER"]
    #[inline(always)]
    pub fn dmber(&self) -> DMBER_R {
        DMBER_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - RASBER"]
    #[inline(always)]
    pub fn rasber(&self) -> RASBER_R {
        RASBER_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - CASBER"]
    #[inline(always)]
    pub fn casber(&self) -> CASBER_R {
        CASBER_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
