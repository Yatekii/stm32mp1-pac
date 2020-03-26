#[doc = "Reader of register DDRPHYC_BISTBER1"]
pub type R = crate::R<u32, super::DDRPHYC_BISTBER1>;
#[doc = "Reader of field `BABER`"]
pub type BABER_R = crate::R<u8, u8>;
#[doc = "Reader of field `WEBER`"]
pub type WEBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `CKEBER`"]
pub type CKEBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `CSBER`"]
pub type CSBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `ODTBER`"]
pub type ODTBER_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - BABER"]
    #[inline(always)]
    pub fn baber(&self) -> BABER_R {
        BABER_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - WEBER"]
    #[inline(always)]
    pub fn weber(&self) -> WEBER_R {
        WEBER_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - CKEBER"]
    #[inline(always)]
    pub fn ckeber(&self) -> CKEBER_R {
        CKEBER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CSBER"]
    #[inline(always)]
    pub fn csber(&self) -> CSBER_R {
        CSBER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ODTBER"]
    #[inline(always)]
    pub fn odtber(&self) -> ODTBER_R {
        ODTBER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
