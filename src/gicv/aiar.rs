#[doc = "Reader of register AIAR"]
pub type R = crate::R<u32, super::AIAR>;
#[doc = "Reader of field `INTERRUPT_ID`"]
pub type INTERRUPT_ID_R = crate::R<u16, u16>;
#[doc = "Reader of field `CPUID`"]
pub type CPUID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:9 - The interrupt ID"]
    #[inline(always)]
    pub fn interrupt_id(&self) -> INTERRUPT_ID_R {
        INTERRUPT_ID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}