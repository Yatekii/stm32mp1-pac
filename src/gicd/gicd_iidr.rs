#[doc = "Reader of register GICD_IIDR"]
pub type R = crate::R<u32, super::GICD_IIDR>;
#[doc = "Reader of field `IMPLEMENTER`"]
pub type IMPLEMENTER_R = crate::R<u16, u16>;
#[doc = "Reader of field `VARIANT`"]
pub type VARIANT_R = crate::R<u8, u8>;
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRODUCTID`"]
pub type PRODUCTID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - GIC implementer"]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - major revision number of the GIC"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - minor revision number of the GIC"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - product ID of the GIC"]
    #[inline(always)]
    pub fn productid(&self) -> PRODUCTID_R {
        PRODUCTID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
