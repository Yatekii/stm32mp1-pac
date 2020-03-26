#[doc = "Reader of register ITARGETSR2"]
pub type R = crate::R<u32, super::ITARGETSR2>;
#[doc = "Reader of field `CPU_TARGETS0`"]
pub type CPU_TARGETS0_R = crate::R<u8, u8>;
#[doc = "Reader of field `CPU_TARGETS1`"]
pub type CPU_TARGETS1_R = crate::R<u8, u8>;
#[doc = "Reader of field `CPU_TARGETS2`"]
pub type CPU_TARGETS2_R = crate::R<u8, u8>;
#[doc = "Reader of field `CPU_TARGETS3`"]
pub type CPU_TARGETS3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - CPU(s) target for interrupt"]
    #[inline(always)]
    pub fn cpu_targets0(&self) -> CPU_TARGETS0_R {
        CPU_TARGETS0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - CPU(s) target for interrupt"]
    #[inline(always)]
    pub fn cpu_targets1(&self) -> CPU_TARGETS1_R {
        CPU_TARGETS1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - CPU(s) target for interrupt"]
    #[inline(always)]
    pub fn cpu_targets2(&self) -> CPU_TARGETS2_R {
        CPU_TARGETS2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - CPU(s) target for interrupt"]
    #[inline(always)]
    pub fn cpu_targets3(&self) -> CPU_TARGETS3_R {
        CPU_TARGETS3_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
