#[doc = "Reader of register DDRPHYC_DCUSR0"]
pub type R = crate::R<u8, super::DDRPHYC_DCUSR0>;
#[doc = "Reader of field `RDONE`"]
pub type RDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CFAIL`"]
pub type CFAIL_R = crate::R<bool, bool>;
#[doc = "Reader of field `CFULL`"]
pub type CFULL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RDONE"]
    #[inline(always)]
    pub fn rdone(&self) -> RDONE_R {
        RDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CFAIL"]
    #[inline(always)]
    pub fn cfail(&self) -> CFAIL_R {
        CFAIL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CFULL"]
    #[inline(always)]
    pub fn cfull(&self) -> CFULL_R {
        CFULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
