#[doc = "Reader of register CRYP_SR"]
pub type R = crate::R<u32, super::CRYP_SR>;
#[doc = "Reader of field `IFEM`"]
pub type IFEM_R = crate::R<bool, bool>;
#[doc = "Reader of field `IFNF`"]
pub type IFNF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OFNE`"]
pub type OFNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `OFFU`"]
pub type OFFU_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - IFEM"]
    #[inline(always)]
    pub fn ifem(&self) -> IFEM_R {
        IFEM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IFNF"]
    #[inline(always)]
    pub fn ifnf(&self) -> IFNF_R {
        IFNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OFNE"]
    #[inline(always)]
    pub fn ofne(&self) -> OFNE_R {
        OFNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OFFU"]
    #[inline(always)]
    pub fn offu(&self) -> OFFU_R {
        OFFU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
