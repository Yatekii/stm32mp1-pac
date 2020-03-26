#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GICH hypervisor control register"]
    pub hcr: HCR,
    #[doc = "0x04 - GICH VGIC type register"]
    pub vtr: VTR,
    #[doc = "0x08 - GICH virtual machine control register"]
    pub vmcr: VMCR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - GICH maintenance interrupt status register"]
    pub misr: MISR,
    _reserved4: [u8; 12usize],
    #[doc = "0x20 - GICH end of interrupt status register"]
    pub eisr0: EISR0,
    _reserved5: [u8; 12usize],
    #[doc = "0x30 - GICH empty list status register"]
    pub elsr0: ELSR0,
    _reserved6: [u8; 188usize],
    #[doc = "0xf0 - GICH active priority register"]
    pub apr0: APR0,
    _reserved7: [u8; 12usize],
    #[doc = "0x100 - GICH list register"]
    pub lr0: LR0,
    #[doc = "0x104 - GICH list register"]
    pub lr1: LR1,
    #[doc = "0x108 - GICH list register"]
    pub lr2: LR2,
    #[doc = "0x10c - GICH list register"]
    pub lr3: LR3,
}
#[doc = "GICH hypervisor control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcr](hcr) module"]
pub type HCR = crate::Reg<u32, _HCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCR;
#[doc = "`read()` method returns [hcr::R](hcr::R) reader structure"]
impl crate::Readable for HCR {}
#[doc = "`write(|w| ..)` method takes [hcr::W](hcr::W) writer structure"]
impl crate::Writable for HCR {}
#[doc = "GICH hypervisor control register"]
pub mod hcr;
#[doc = "GICH VGIC type register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vtr](vtr) module"]
pub type VTR = crate::Reg<u32, _VTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VTR;
#[doc = "`read()` method returns [vtr::R](vtr::R) reader structure"]
impl crate::Readable for VTR {}
#[doc = "GICH VGIC type register"]
pub mod vtr;
#[doc = "GICH virtual machine control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmcr](vmcr) module"]
pub type VMCR = crate::Reg<u32, _VMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VMCR;
#[doc = "`read()` method returns [vmcr::R](vmcr::R) reader structure"]
impl crate::Readable for VMCR {}
#[doc = "`write(|w| ..)` method takes [vmcr::W](vmcr::W) writer structure"]
impl crate::Writable for VMCR {}
#[doc = "GICH virtual machine control register"]
pub mod vmcr;
#[doc = "GICH maintenance interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](misr) module"]
pub type MISR = crate::Reg<u32, _MISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISR;
#[doc = "`read()` method returns [misr::R](misr::R) reader structure"]
impl crate::Readable for MISR {}
#[doc = "GICH maintenance interrupt status register"]
pub mod misr;
#[doc = "GICH end of interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eisr0](eisr0) module"]
pub type EISR0 = crate::Reg<u32, _EISR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EISR0;
#[doc = "`read()` method returns [eisr0::R](eisr0::R) reader structure"]
impl crate::Readable for EISR0 {}
#[doc = "GICH end of interrupt status register"]
pub mod eisr0;
#[doc = "GICH empty list status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elsr0](elsr0) module"]
pub type ELSR0 = crate::Reg<u32, _ELSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ELSR0;
#[doc = "`read()` method returns [elsr0::R](elsr0::R) reader structure"]
impl crate::Readable for ELSR0 {}
#[doc = "GICH empty list status register"]
pub mod elsr0;
#[doc = "GICH active priority register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apr0](apr0) module"]
pub type APR0 = crate::Reg<u32, _APR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APR0;
#[doc = "`read()` method returns [apr0::R](apr0::R) reader structure"]
impl crate::Readable for APR0 {}
#[doc = "GICH active priority register"]
pub mod apr0;
#[doc = "GICH list register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lr0](lr0) module"]
pub type LR0 = crate::Reg<u32, _LR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LR0;
#[doc = "`read()` method returns [lr0::R](lr0::R) reader structure"]
impl crate::Readable for LR0 {}
#[doc = "`write(|w| ..)` method takes [lr0::W](lr0::W) writer structure"]
impl crate::Writable for LR0 {}
#[doc = "GICH list register"]
pub mod lr0;
#[doc = "GICH list register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lr1](lr1) module"]
pub type LR1 = crate::Reg<u32, _LR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LR1;
#[doc = "`read()` method returns [lr1::R](lr1::R) reader structure"]
impl crate::Readable for LR1 {}
#[doc = "`write(|w| ..)` method takes [lr1::W](lr1::W) writer structure"]
impl crate::Writable for LR1 {}
#[doc = "GICH list register"]
pub mod lr1;
#[doc = "GICH list register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lr2](lr2) module"]
pub type LR2 = crate::Reg<u32, _LR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LR2;
#[doc = "`read()` method returns [lr2::R](lr2::R) reader structure"]
impl crate::Readable for LR2 {}
#[doc = "`write(|w| ..)` method takes [lr2::W](lr2::W) writer structure"]
impl crate::Writable for LR2 {}
#[doc = "GICH list register"]
pub mod lr2;
#[doc = "GICH list register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lr3](lr3) module"]
pub type LR3 = crate::Reg<u32, _LR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LR3;
#[doc = "`read()` method returns [lr3::R](lr3::R) reader structure"]
impl crate::Readable for LR3 {}
#[doc = "`write(|w| ..)` method takes [lr3::W](lr3::W) writer structure"]
impl crate::Writable for LR3 {}
#[doc = "GICH list register"]
pub mod lr3;
