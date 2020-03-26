#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )"]
    pub syscfg_bootr: SYSCFG_BOOTR,
    #[doc = "0x04 - SYSCFG peripheral mode configuration register"]
    pub syscfg_pmcr: SYSCFG_PMCR,
    _reserved2: [u8; 16usize],
    #[doc = "0x18 - SYSCFG IO control register"]
    pub syscfg_ioctrlr: SYSCFG_IOCTRLR,
    #[doc = "0x1c - SYSCFG interconnect control register"]
    pub syscfg_icnr: SYSCFG_ICNR,
    #[doc = "0x20 - SYSCFG compensation cell control register"]
    pub syscfg_cmpcr: SYSCFG_CMPCR,
    #[doc = "0x24 - SYSCFG control timer break register"]
    pub syscfg_cbr: SYSCFG_CBR,
    _reserved6: [u8; 972usize],
    #[doc = "0x3f4 - SYSCFG version register"]
    pub syscfg_verr: SYSCFG_VERR,
    #[doc = "0x3f8 - SYSCFG identification register"]
    pub syscfg_ipidr: SYSCFG_IPIDR,
    #[doc = "0x3fc - SYSCFG size identification register"]
    pub syscfg_sidr: SYSCFG_SIDR,
}
#[doc = "This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_bootr](syscfg_bootr) module"]
pub type SYSCFG_BOOTR = crate::Reg<u32, _SYSCFG_BOOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_BOOTR;
#[doc = "`read()` method returns [syscfg_bootr::R](syscfg_bootr::R) reader structure"]
impl crate::Readable for SYSCFG_BOOTR {}
#[doc = "`write(|w| ..)` method takes [syscfg_bootr::W](syscfg_bootr::W) writer structure"]
impl crate::Writable for SYSCFG_BOOTR {}
#[doc = "This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )"]
pub mod syscfg_bootr;
#[doc = "SYSCFG peripheral mode configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_pmcr](syscfg_pmcr) module"]
pub type SYSCFG_PMCR = crate::Reg<u32, _SYSCFG_PMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_PMCR;
#[doc = "`read()` method returns [syscfg_pmcr::R](syscfg_pmcr::R) reader structure"]
impl crate::Readable for SYSCFG_PMCR {}
#[doc = "`write(|w| ..)` method takes [syscfg_pmcr::W](syscfg_pmcr::W) writer structure"]
impl crate::Writable for SYSCFG_PMCR {}
#[doc = "SYSCFG peripheral mode configuration register"]
pub mod syscfg_pmcr;
#[doc = "SYSCFG IO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_ioctrlr](syscfg_ioctrlr) module"]
pub type SYSCFG_IOCTRLR = crate::Reg<u32, _SYSCFG_IOCTRLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_IOCTRLR;
#[doc = "`read()` method returns [syscfg_ioctrlr::R](syscfg_ioctrlr::R) reader structure"]
impl crate::Readable for SYSCFG_IOCTRLR {}
#[doc = "`write(|w| ..)` method takes [syscfg_ioctrlr::W](syscfg_ioctrlr::W) writer structure"]
impl crate::Writable for SYSCFG_IOCTRLR {}
#[doc = "SYSCFG IO control register"]
pub mod syscfg_ioctrlr;
#[doc = "SYSCFG interconnect control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_icnr](syscfg_icnr) module"]
pub type SYSCFG_ICNR = crate::Reg<u32, _SYSCFG_ICNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_ICNR;
#[doc = "`read()` method returns [syscfg_icnr::R](syscfg_icnr::R) reader structure"]
impl crate::Readable for SYSCFG_ICNR {}
#[doc = "`write(|w| ..)` method takes [syscfg_icnr::W](syscfg_icnr::W) writer structure"]
impl crate::Writable for SYSCFG_ICNR {}
#[doc = "SYSCFG interconnect control register"]
pub mod syscfg_icnr;
#[doc = "SYSCFG compensation cell control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cmpcr](syscfg_cmpcr) module"]
pub type SYSCFG_CMPCR = crate::Reg<u32, _SYSCFG_CMPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_CMPCR;
#[doc = "`read()` method returns [syscfg_cmpcr::R](syscfg_cmpcr::R) reader structure"]
impl crate::Readable for SYSCFG_CMPCR {}
#[doc = "`write(|w| ..)` method takes [syscfg_cmpcr::W](syscfg_cmpcr::W) writer structure"]
impl crate::Writable for SYSCFG_CMPCR {}
#[doc = "SYSCFG compensation cell control register"]
pub mod syscfg_cmpcr;
#[doc = "SYSCFG control timer break register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cbr](syscfg_cbr) module"]
pub type SYSCFG_CBR = crate::Reg<u32, _SYSCFG_CBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_CBR;
#[doc = "`read()` method returns [syscfg_cbr::R](syscfg_cbr::R) reader structure"]
impl crate::Readable for SYSCFG_CBR {}
#[doc = "`write(|w| ..)` method takes [syscfg_cbr::W](syscfg_cbr::W) writer structure"]
impl crate::Writable for SYSCFG_CBR {}
#[doc = "SYSCFG control timer break register"]
pub mod syscfg_cbr;
#[doc = "SYSCFG version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_verr](syscfg_verr) module"]
pub type SYSCFG_VERR = crate::Reg<u32, _SYSCFG_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_VERR;
#[doc = "`read()` method returns [syscfg_verr::R](syscfg_verr::R) reader structure"]
impl crate::Readable for SYSCFG_VERR {}
#[doc = "SYSCFG version register"]
pub mod syscfg_verr;
#[doc = "SYSCFG identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_ipidr](syscfg_ipidr) module"]
pub type SYSCFG_IPIDR = crate::Reg<u32, _SYSCFG_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_IPIDR;
#[doc = "`read()` method returns [syscfg_ipidr::R](syscfg_ipidr::R) reader structure"]
impl crate::Readable for SYSCFG_IPIDR {}
#[doc = "SYSCFG identification register"]
pub mod syscfg_ipidr;
#[doc = "SYSCFG size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_sidr](syscfg_sidr) module"]
pub type SYSCFG_SIDR = crate::Reg<u32, _SYSCFG_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_SIDR;
#[doc = "`read()` method returns [syscfg_sidr::R](syscfg_sidr::R) reader structure"]
impl crate::Readable for SYSCFG_SIDR {}
#[doc = "SYSCFG size identification register"]
pub mod syscfg_sidr;
