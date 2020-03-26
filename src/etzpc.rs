#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ETZPC ROM secure size definition"]
    pub etzpc_tzma0_size: ETZPC_TZMA0_SIZE,
    #[doc = "0x04 - ETZPC RAM secure size definition"]
    pub etzpc_tzma1_size: ETZPC_TZMA1_SIZE,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - ETZPC securable peripheral definition register 0"]
    pub etzpc_decprot0: ETZPC_DECPROT0,
    _reserved3: [u8; 12usize],
    #[doc = "0x20 - ETZPC securable peripheral definition register 0"]
    pub etzpc_decprot_lock0: ETZPC_DECPROT_LOCK0,
    _reserved4: [u8; 972usize],
    #[doc = "0x3f0 - ETZPC IP hardware configuration register"]
    pub etzpc_hwcfgr: ETZPC_HWCFGR,
    #[doc = "0x3f4 - ETZPC IP version register"]
    pub etzpc_verr: ETZPC_VERR,
    #[doc = "0x3f8 - ETZPC IP version register"]
    pub etzpc_ipidr: ETZPC_IPIDR,
    #[doc = "0x3fc - ETZPC IP version register"]
    pub etzpc_sidr: ETZPC_SIDR,
}
#[doc = "ETZPC ROM secure size definition\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_tzma0_size](etzpc_tzma0_size) module"]
pub type ETZPC_TZMA0_SIZE = crate::Reg<u32, _ETZPC_TZMA0_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_TZMA0_SIZE;
#[doc = "`read()` method returns [etzpc_tzma0_size::R](etzpc_tzma0_size::R) reader structure"]
impl crate::Readable for ETZPC_TZMA0_SIZE {}
#[doc = "`write(|w| ..)` method takes [etzpc_tzma0_size::W](etzpc_tzma0_size::W) writer structure"]
impl crate::Writable for ETZPC_TZMA0_SIZE {}
#[doc = "ETZPC ROM secure size definition"]
pub mod etzpc_tzma0_size;
#[doc = "ETZPC RAM secure size definition\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_tzma1_size](etzpc_tzma1_size) module"]
pub type ETZPC_TZMA1_SIZE = crate::Reg<u32, _ETZPC_TZMA1_SIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_TZMA1_SIZE;
#[doc = "`read()` method returns [etzpc_tzma1_size::R](etzpc_tzma1_size::R) reader structure"]
impl crate::Readable for ETZPC_TZMA1_SIZE {}
#[doc = "`write(|w| ..)` method takes [etzpc_tzma1_size::W](etzpc_tzma1_size::W) writer structure"]
impl crate::Writable for ETZPC_TZMA1_SIZE {}
#[doc = "ETZPC RAM secure size definition"]
pub mod etzpc_tzma1_size;
#[doc = "ETZPC securable peripheral definition register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot0](etzpc_decprot0) module"]
pub type ETZPC_DECPROT0 = crate::Reg<u32, _ETZPC_DECPROT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_DECPROT0;
#[doc = "`read()` method returns [etzpc_decprot0::R](etzpc_decprot0::R) reader structure"]
impl crate::Readable for ETZPC_DECPROT0 {}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot0::W](etzpc_decprot0::W) writer structure"]
impl crate::Writable for ETZPC_DECPROT0 {}
#[doc = "ETZPC securable peripheral definition register 0"]
pub mod etzpc_decprot0;
#[doc = "ETZPC securable peripheral definition register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_decprot_lock0](etzpc_decprot_lock0) module"]
pub type ETZPC_DECPROT_LOCK0 = crate::Reg<u32, _ETZPC_DECPROT_LOCK0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_DECPROT_LOCK0;
#[doc = "`read()` method returns [etzpc_decprot_lock0::R](etzpc_decprot_lock0::R) reader structure"]
impl crate::Readable for ETZPC_DECPROT_LOCK0 {}
#[doc = "`write(|w| ..)` method takes [etzpc_decprot_lock0::W](etzpc_decprot_lock0::W) writer structure"]
impl crate::Writable for ETZPC_DECPROT_LOCK0 {}
#[doc = "ETZPC securable peripheral definition register 0"]
pub mod etzpc_decprot_lock0;
#[doc = "ETZPC IP hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_hwcfgr](etzpc_hwcfgr) module"]
pub type ETZPC_HWCFGR = crate::Reg<u32, _ETZPC_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_HWCFGR;
#[doc = "`read()` method returns [etzpc_hwcfgr::R](etzpc_hwcfgr::R) reader structure"]
impl crate::Readable for ETZPC_HWCFGR {}
#[doc = "ETZPC IP hardware configuration register"]
pub mod etzpc_hwcfgr;
#[doc = "ETZPC IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_verr](etzpc_verr) module"]
pub type ETZPC_VERR = crate::Reg<u32, _ETZPC_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_VERR;
#[doc = "`read()` method returns [etzpc_verr::R](etzpc_verr::R) reader structure"]
impl crate::Readable for ETZPC_VERR {}
#[doc = "ETZPC IP version register"]
pub mod etzpc_verr;
#[doc = "ETZPC IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_ipidr](etzpc_ipidr) module"]
pub type ETZPC_IPIDR = crate::Reg<u32, _ETZPC_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_IPIDR;
#[doc = "`read()` method returns [etzpc_ipidr::R](etzpc_ipidr::R) reader structure"]
impl crate::Readable for ETZPC_IPIDR {}
#[doc = "ETZPC IP version register"]
pub mod etzpc_ipidr;
#[doc = "ETZPC IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_sidr](etzpc_sidr) module"]
pub type ETZPC_SIDR = crate::Reg<u32, _ETZPC_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ETZPC_SIDR;
#[doc = "`read()` method returns [etzpc_sidr::R](etzpc_sidr::R) reader structure"]
impl crate::Readable for ETZPC_SIDR {}
#[doc = "ETZPC IP version register"]
pub mod etzpc_sidr;
