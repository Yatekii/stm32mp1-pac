#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAMUX1 request line multiplexer channel 0 configuration register"]
    pub dmamux1_c0cr: DMAMUX1_C0CR,
    #[doc = "0x04 - DMAMUX1 request line multiplexer channel 1 configuration register"]
    pub dmamux1_c1cr: DMAMUX1_C1CR,
    #[doc = "0x08 - DMAMUX1 request line multiplexer channel 2 configuration register"]
    pub dmamux1_c2cr: DMAMUX1_C2CR,
    #[doc = "0x0c - DMAMUX1 request line multiplexer channel 3 configuration register"]
    pub dmamux1_c3cr: DMAMUX1_C3CR,
    #[doc = "0x10 - DMAMUX1 request line multiplexer channel 4 configuration register"]
    pub dmamux1_c4cr: DMAMUX1_C4CR,
    #[doc = "0x14 - DMAMUX1 request line multiplexer channel 5 configuration register"]
    pub dmamux1_c5cr: DMAMUX1_C5CR,
    #[doc = "0x18 - DMAMUX1 request line multiplexer channel 6 configuration register"]
    pub dmamux1_c6cr: DMAMUX1_C6CR,
    #[doc = "0x1c - DMAMUX1 request line multiplexer channel 7 configuration register"]
    pub dmamux1_c7cr: DMAMUX1_C7CR,
    #[doc = "0x20 - DMAMUX1 request line multiplexer channel 8 configuration register"]
    pub dmamux1_c8cr: DMAMUX1_C8CR,
    #[doc = "0x24 - DMAMUX1 request line multiplexer channel 9 configuration register"]
    pub dmamux1_c9cr: DMAMUX1_C9CR,
    #[doc = "0x28 - DMAMUX1 request line multiplexer channel 10 configuration register"]
    pub dmamux1_c10cr: DMAMUX1_C10CR,
    #[doc = "0x2c - DMAMUX1 request line multiplexer channel 11 configuration register"]
    pub dmamux1_c11cr: DMAMUX1_C11CR,
    #[doc = "0x30 - DMAMUX1 request line multiplexer channel 12 configuration register"]
    pub dmamux1_c12cr: DMAMUX1_C12CR,
    #[doc = "0x34 - DMAMUX1 request line multiplexer channel 13 configuration register"]
    pub dmamux1_c13cr: DMAMUX1_C13CR,
    #[doc = "0x38 - DMAMUX1 request line multiplexer channel 14 configuration register"]
    pub dmamux1_c14cr: DMAMUX1_C14CR,
    #[doc = "0x3c - DMAMUX1 request line multiplexer channel 15 configuration register"]
    pub dmamux1_c15cr: DMAMUX1_C15CR,
    _reserved16: [u8; 64usize],
    #[doc = "0x80 - DMAMUX1 request line multiplexer interrupt channel status register"]
    pub dmamux1_csr: DMAMUX1_CSR,
    #[doc = "0x84 - DMAMUX1 request line multiplexer interrupt clear flag register"]
    pub dmamux1_cfr: DMAMUX1_CFR,
    _reserved18: [u8; 120usize],
    #[doc = "0x100 - DMAMUX1 request generator channel 0 configuration register"]
    pub dmamux1_rg0cr: DMAMUX1_RG0CR,
    #[doc = "0x104 - DMAMUX1 request generator channel 1 configuration register"]
    pub dmamux1_rg1cr: DMAMUX1_RG1CR,
    #[doc = "0x108 - DMAMUX1 request generator channel 2 configuration register"]
    pub dmamux1_rg2cr: DMAMUX1_RG2CR,
    #[doc = "0x10c - DMAMUX1 request generator channel 3 configuration register"]
    pub dmamux1_rg3cr: DMAMUX1_RG3CR,
    #[doc = "0x110 - DMAMUX1 request generator channel 4 configuration register"]
    pub dmamux1_rg4cr: DMAMUX1_RG4CR,
    #[doc = "0x114 - DMAMUX1 request generator channel 5 configuration register"]
    pub dmamux1_rg5cr: DMAMUX1_RG5CR,
    #[doc = "0x118 - DMAMUX1 request generator channel 6 configuration register"]
    pub dmamux1_rg6cr: DMAMUX1_RG6CR,
    #[doc = "0x11c - DMAMUX1 request generator channel 7 configuration register"]
    pub dmamux1_rg7cr: DMAMUX1_RG7CR,
    _reserved26: [u8; 32usize],
    #[doc = "0x140 - DMAMUX1 request generator interrupt status register"]
    pub dmamux1_rgsr: DMAMUX1_RGSR,
    #[doc = "0x144 - DMAMUX1 request generator interrupt clear flag register"]
    pub dmamux1_rgcfr: DMAMUX1_RGCFR,
    _reserved28: [u8; 676usize],
    #[doc = "0x3ec - DMAMUX hardware configuration 2 register"]
    pub dmamux1_hwcfgr2: DMAMUX1_HWCFGR2,
    #[doc = "0x3f0 - DMAMUX hardware configuration 1 register"]
    pub dmamux1_hwcfgr1: DMAMUX1_HWCFGR1,
    #[doc = "0x3f4 - DMAMUX IP Version Register"]
    pub dmamux1_verr: DMAMUX1_VERR,
    #[doc = "0x3f8 - DMAMUX IP Version Register"]
    pub dmamux1_ipidr: DMAMUX1_IPIDR,
    #[doc = "0x3fc - DMAMUX IP Version Register"]
    pub dmamux1_sidr: DMAMUX1_SIDR,
}
#[doc = "DMAMUX1 request line multiplexer channel 0 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c0cr](dmamux1_c0cr) module"]
pub type DMAMUX1_C0CR = crate::Reg<u32, _DMAMUX1_C0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C0CR;
#[doc = "`read()` method returns [dmamux1_c0cr::R](dmamux1_c0cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C0CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c0cr::W](dmamux1_c0cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C0CR {}
#[doc = "DMAMUX1 request line multiplexer channel 0 configuration register"]
pub mod dmamux1_c0cr;
#[doc = "DMAMUX1 request line multiplexer channel 1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c1cr](dmamux1_c1cr) module"]
pub type DMAMUX1_C1CR = crate::Reg<u32, _DMAMUX1_C1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C1CR;
#[doc = "`read()` method returns [dmamux1_c1cr::R](dmamux1_c1cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C1CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c1cr::W](dmamux1_c1cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C1CR {}
#[doc = "DMAMUX1 request line multiplexer channel 1 configuration register"]
pub mod dmamux1_c1cr;
#[doc = "DMAMUX1 request line multiplexer channel 2 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c2cr](dmamux1_c2cr) module"]
pub type DMAMUX1_C2CR = crate::Reg<u32, _DMAMUX1_C2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C2CR;
#[doc = "`read()` method returns [dmamux1_c2cr::R](dmamux1_c2cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C2CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c2cr::W](dmamux1_c2cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C2CR {}
#[doc = "DMAMUX1 request line multiplexer channel 2 configuration register"]
pub mod dmamux1_c2cr;
#[doc = "DMAMUX1 request line multiplexer channel 3 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c3cr](dmamux1_c3cr) module"]
pub type DMAMUX1_C3CR = crate::Reg<u32, _DMAMUX1_C3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C3CR;
#[doc = "`read()` method returns [dmamux1_c3cr::R](dmamux1_c3cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C3CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c3cr::W](dmamux1_c3cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C3CR {}
#[doc = "DMAMUX1 request line multiplexer channel 3 configuration register"]
pub mod dmamux1_c3cr;
#[doc = "DMAMUX1 request line multiplexer channel 4 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c4cr](dmamux1_c4cr) module"]
pub type DMAMUX1_C4CR = crate::Reg<u32, _DMAMUX1_C4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C4CR;
#[doc = "`read()` method returns [dmamux1_c4cr::R](dmamux1_c4cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C4CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c4cr::W](dmamux1_c4cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C4CR {}
#[doc = "DMAMUX1 request line multiplexer channel 4 configuration register"]
pub mod dmamux1_c4cr;
#[doc = "DMAMUX1 request line multiplexer channel 5 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c5cr](dmamux1_c5cr) module"]
pub type DMAMUX1_C5CR = crate::Reg<u32, _DMAMUX1_C5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C5CR;
#[doc = "`read()` method returns [dmamux1_c5cr::R](dmamux1_c5cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C5CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c5cr::W](dmamux1_c5cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C5CR {}
#[doc = "DMAMUX1 request line multiplexer channel 5 configuration register"]
pub mod dmamux1_c5cr;
#[doc = "DMAMUX1 request line multiplexer channel 6 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c6cr](dmamux1_c6cr) module"]
pub type DMAMUX1_C6CR = crate::Reg<u32, _DMAMUX1_C6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C6CR;
#[doc = "`read()` method returns [dmamux1_c6cr::R](dmamux1_c6cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C6CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c6cr::W](dmamux1_c6cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C6CR {}
#[doc = "DMAMUX1 request line multiplexer channel 6 configuration register"]
pub mod dmamux1_c6cr;
#[doc = "DMAMUX1 request line multiplexer channel 7 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c7cr](dmamux1_c7cr) module"]
pub type DMAMUX1_C7CR = crate::Reg<u32, _DMAMUX1_C7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C7CR;
#[doc = "`read()` method returns [dmamux1_c7cr::R](dmamux1_c7cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C7CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c7cr::W](dmamux1_c7cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C7CR {}
#[doc = "DMAMUX1 request line multiplexer channel 7 configuration register"]
pub mod dmamux1_c7cr;
#[doc = "DMAMUX1 request line multiplexer channel 8 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c8cr](dmamux1_c8cr) module"]
pub type DMAMUX1_C8CR = crate::Reg<u32, _DMAMUX1_C8CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C8CR;
#[doc = "`read()` method returns [dmamux1_c8cr::R](dmamux1_c8cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C8CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c8cr::W](dmamux1_c8cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C8CR {}
#[doc = "DMAMUX1 request line multiplexer channel 8 configuration register"]
pub mod dmamux1_c8cr;
#[doc = "DMAMUX1 request line multiplexer channel 9 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c9cr](dmamux1_c9cr) module"]
pub type DMAMUX1_C9CR = crate::Reg<u32, _DMAMUX1_C9CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C9CR;
#[doc = "`read()` method returns [dmamux1_c9cr::R](dmamux1_c9cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C9CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c9cr::W](dmamux1_c9cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C9CR {}
#[doc = "DMAMUX1 request line multiplexer channel 9 configuration register"]
pub mod dmamux1_c9cr;
#[doc = "DMAMUX1 request line multiplexer channel 10 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c10cr](dmamux1_c10cr) module"]
pub type DMAMUX1_C10CR = crate::Reg<u32, _DMAMUX1_C10CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C10CR;
#[doc = "`read()` method returns [dmamux1_c10cr::R](dmamux1_c10cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C10CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c10cr::W](dmamux1_c10cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C10CR {}
#[doc = "DMAMUX1 request line multiplexer channel 10 configuration register"]
pub mod dmamux1_c10cr;
#[doc = "DMAMUX1 request line multiplexer channel 11 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c11cr](dmamux1_c11cr) module"]
pub type DMAMUX1_C11CR = crate::Reg<u32, _DMAMUX1_C11CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C11CR;
#[doc = "`read()` method returns [dmamux1_c11cr::R](dmamux1_c11cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C11CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c11cr::W](dmamux1_c11cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C11CR {}
#[doc = "DMAMUX1 request line multiplexer channel 11 configuration register"]
pub mod dmamux1_c11cr;
#[doc = "DMAMUX1 request line multiplexer channel 12 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c12cr](dmamux1_c12cr) module"]
pub type DMAMUX1_C12CR = crate::Reg<u32, _DMAMUX1_C12CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C12CR;
#[doc = "`read()` method returns [dmamux1_c12cr::R](dmamux1_c12cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C12CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c12cr::W](dmamux1_c12cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C12CR {}
#[doc = "DMAMUX1 request line multiplexer channel 12 configuration register"]
pub mod dmamux1_c12cr;
#[doc = "DMAMUX1 request line multiplexer channel 13 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c13cr](dmamux1_c13cr) module"]
pub type DMAMUX1_C13CR = crate::Reg<u32, _DMAMUX1_C13CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C13CR;
#[doc = "`read()` method returns [dmamux1_c13cr::R](dmamux1_c13cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C13CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c13cr::W](dmamux1_c13cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C13CR {}
#[doc = "DMAMUX1 request line multiplexer channel 13 configuration register"]
pub mod dmamux1_c13cr;
#[doc = "DMAMUX1 request line multiplexer channel 14 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c14cr](dmamux1_c14cr) module"]
pub type DMAMUX1_C14CR = crate::Reg<u32, _DMAMUX1_C14CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C14CR;
#[doc = "`read()` method returns [dmamux1_c14cr::R](dmamux1_c14cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C14CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c14cr::W](dmamux1_c14cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C14CR {}
#[doc = "DMAMUX1 request line multiplexer channel 14 configuration register"]
pub mod dmamux1_c14cr;
#[doc = "DMAMUX1 request line multiplexer channel 15 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_c15cr](dmamux1_c15cr) module"]
pub type DMAMUX1_C15CR = crate::Reg<u32, _DMAMUX1_C15CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_C15CR;
#[doc = "`read()` method returns [dmamux1_c15cr::R](dmamux1_c15cr::R) reader structure"]
impl crate::Readable for DMAMUX1_C15CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_c15cr::W](dmamux1_c15cr::W) writer structure"]
impl crate::Writable for DMAMUX1_C15CR {}
#[doc = "DMAMUX1 request line multiplexer channel 15 configuration register"]
pub mod dmamux1_c15cr;
#[doc = "DMAMUX1 request line multiplexer interrupt channel status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_csr](dmamux1_csr) module"]
pub type DMAMUX1_CSR = crate::Reg<u32, _DMAMUX1_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_CSR;
#[doc = "`read()` method returns [dmamux1_csr::R](dmamux1_csr::R) reader structure"]
impl crate::Readable for DMAMUX1_CSR {}
#[doc = "DMAMUX1 request line multiplexer interrupt channel status register"]
pub mod dmamux1_csr;
#[doc = "DMAMUX1 request line multiplexer interrupt clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_cfr](dmamux1_cfr) module"]
pub type DMAMUX1_CFR = crate::Reg<u32, _DMAMUX1_CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_CFR;
#[doc = "`read()` method returns [dmamux1_cfr::R](dmamux1_cfr::R) reader structure"]
impl crate::Readable for DMAMUX1_CFR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_cfr::W](dmamux1_cfr::W) writer structure"]
impl crate::Writable for DMAMUX1_CFR {}
#[doc = "DMAMUX1 request line multiplexer interrupt clear flag register"]
pub mod dmamux1_cfr;
#[doc = "DMAMUX1 request generator channel 0 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_rg0cr](dmamux1_rg0cr) module"]
pub type DMAMUX1_RG0CR = crate::Reg<u32, _DMAMUX1_RG0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_RG0CR;
#[doc = "`read()` method returns [dmamux1_rg0cr::R](dmamux1_rg0cr::R) reader structure"]
impl crate::Readable for DMAMUX1_RG0CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_rg0cr::W](dmamux1_rg0cr::W) writer structure"]
impl crate::Writable for DMAMUX1_RG0CR {}
#[doc = "DMAMUX1 request generator channel 0 configuration register"]
pub mod dmamux1_rg0cr;
#[doc = "DMAMUX1 request generator channel 1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_rg1cr](dmamux1_rg1cr) module"]
pub type DMAMUX1_RG1CR = crate::Reg<u32, _DMAMUX1_RG1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_RG1CR;
#[doc = "`read()` method returns [dmamux1_rg1cr::R](dmamux1_rg1cr::R) reader structure"]
impl crate::Readable for DMAMUX1_RG1CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_rg1cr::W](dmamux1_rg1cr::W) writer structure"]
impl crate::Writable for DMAMUX1_RG1CR {}
#[doc = "DMAMUX1 request generator channel 1 configuration register"]
pub mod dmamux1_rg1cr;
#[doc = "DMAMUX1 request generator channel 2 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_rg2cr](dmamux1_rg2cr) module"]
pub type DMAMUX1_RG2CR = crate::Reg<u32, _DMAMUX1_RG2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_RG2CR;
#[doc = "`read()` method returns [dmamux1_rg2cr::R](dmamux1_rg2cr::R) reader structure"]
impl crate::Readable for DMAMUX1_RG2CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_rg2cr::W](dmamux1_rg2cr::W) writer structure"]
impl crate::Writable for DMAMUX1_RG2CR {}
#[doc = "DMAMUX1 request generator channel 2 configuration register"]
pub mod dmamux1_rg2cr;
#[doc = "DMAMUX1 request generator channel 3 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_rg3cr](dmamux1_rg3cr) module"]
pub type DMAMUX1_RG3CR = crate::Reg<u32, _DMAMUX1_RG3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_RG3CR;
#[doc = "`read()` method returns [dmamux1_rg3cr::R](dmamux1_rg3cr::R) reader structure"]
impl crate::Readable for DMAMUX1_RG3CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_rg3cr::W](dmamux1_rg3cr::W) writer structure"]
impl crate::Writable for DMAMUX1_RG3CR {}
#[doc = "DMAMUX1 request generator channel 3 configuration register"]
pub mod dmamux1_rg3cr;
#[doc = "DMAMUX1 request generator channel 4 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_rg4cr](dmamux1_rg4cr) module"]
pub type DMAMUX1_RG4CR = crate::Reg<u32, _DMAMUX1_RG4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_RG4CR;
#[doc = "`read()` method returns [dmamux1_rg4cr::R](dmamux1_rg4cr::R) reader structure"]
impl crate::Readable for DMAMUX1_RG4CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_rg4cr::W](dmamux1_rg4cr::W) writer structure"]
impl crate::Writable for DMAMUX1_RG4CR {}
#[doc = "DMAMUX1 request generator channel 4 configuration register"]
pub mod dmamux1_rg4cr;
#[doc = "DMAMUX1 request generator channel 5 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_rg5cr](dmamux1_rg5cr) module"]
pub type DMAMUX1_RG5CR = crate::Reg<u32, _DMAMUX1_RG5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_RG5CR;
#[doc = "`read()` method returns [dmamux1_rg5cr::R](dmamux1_rg5cr::R) reader structure"]
impl crate::Readable for DMAMUX1_RG5CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_rg5cr::W](dmamux1_rg5cr::W) writer structure"]
impl crate::Writable for DMAMUX1_RG5CR {}
#[doc = "DMAMUX1 request generator channel 5 configuration register"]
pub mod dmamux1_rg5cr;
#[doc = "DMAMUX1 request generator channel 6 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_rg6cr](dmamux1_rg6cr) module"]
pub type DMAMUX1_RG6CR = crate::Reg<u32, _DMAMUX1_RG6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_RG6CR;
#[doc = "`read()` method returns [dmamux1_rg6cr::R](dmamux1_rg6cr::R) reader structure"]
impl crate::Readable for DMAMUX1_RG6CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_rg6cr::W](dmamux1_rg6cr::W) writer structure"]
impl crate::Writable for DMAMUX1_RG6CR {}
#[doc = "DMAMUX1 request generator channel 6 configuration register"]
pub mod dmamux1_rg6cr;
#[doc = "DMAMUX1 request generator channel 7 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_rg7cr](dmamux1_rg7cr) module"]
pub type DMAMUX1_RG7CR = crate::Reg<u32, _DMAMUX1_RG7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_RG7CR;
#[doc = "`read()` method returns [dmamux1_rg7cr::R](dmamux1_rg7cr::R) reader structure"]
impl crate::Readable for DMAMUX1_RG7CR {}
#[doc = "`write(|w| ..)` method takes [dmamux1_rg7cr::W](dmamux1_rg7cr::W) writer structure"]
impl crate::Writable for DMAMUX1_RG7CR {}
#[doc = "DMAMUX1 request generator channel 7 configuration register"]
pub mod dmamux1_rg7cr;
#[doc = "DMAMUX1 request generator interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_rgsr](dmamux1_rgsr) module"]
pub type DMAMUX1_RGSR = crate::Reg<u32, _DMAMUX1_RGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_RGSR;
#[doc = "`read()` method returns [dmamux1_rgsr::R](dmamux1_rgsr::R) reader structure"]
impl crate::Readable for DMAMUX1_RGSR {}
#[doc = "DMAMUX1 request generator interrupt status register"]
pub mod dmamux1_rgsr;
#[doc = "DMAMUX1 request generator interrupt clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_rgcfr](dmamux1_rgcfr) module"]
pub type DMAMUX1_RGCFR = crate::Reg<u32, _DMAMUX1_RGCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_RGCFR;
#[doc = "`read()` method returns [dmamux1_rgcfr::R](dmamux1_rgcfr::R) reader structure"]
impl crate::Readable for DMAMUX1_RGCFR {}
#[doc = "DMAMUX1 request generator interrupt clear flag register"]
pub mod dmamux1_rgcfr;
#[doc = "DMAMUX IP Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_verr](dmamux1_verr) module"]
pub type DMAMUX1_VERR = crate::Reg<u32, _DMAMUX1_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_VERR;
#[doc = "`read()` method returns [dmamux1_verr::R](dmamux1_verr::R) reader structure"]
impl crate::Readable for DMAMUX1_VERR {}
#[doc = "DMAMUX IP Version Register"]
pub mod dmamux1_verr;
#[doc = "DMAMUX IP Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_ipidr](dmamux1_ipidr) module"]
pub type DMAMUX1_IPIDR = crate::Reg<u32, _DMAMUX1_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_IPIDR;
#[doc = "`read()` method returns [dmamux1_ipidr::R](dmamux1_ipidr::R) reader structure"]
impl crate::Readable for DMAMUX1_IPIDR {}
#[doc = "DMAMUX IP Version Register"]
pub mod dmamux1_ipidr;
#[doc = "DMAMUX IP Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_sidr](dmamux1_sidr) module"]
pub type DMAMUX1_SIDR = crate::Reg<u32, _DMAMUX1_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_SIDR;
#[doc = "`read()` method returns [dmamux1_sidr::R](dmamux1_sidr::R) reader structure"]
impl crate::Readable for DMAMUX1_SIDR {}
#[doc = "DMAMUX IP Version Register"]
pub mod dmamux1_sidr;
#[doc = "DMAMUX hardware configuration 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_hwcfgr1](dmamux1_hwcfgr1) module"]
pub type DMAMUX1_HWCFGR1 = crate::Reg<u32, _DMAMUX1_HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_HWCFGR1;
#[doc = "`read()` method returns [dmamux1_hwcfgr1::R](dmamux1_hwcfgr1::R) reader structure"]
impl crate::Readable for DMAMUX1_HWCFGR1 {}
#[doc = "DMAMUX hardware configuration 1 register"]
pub mod dmamux1_hwcfgr1;
#[doc = "DMAMUX hardware configuration 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux1_hwcfgr2](dmamux1_hwcfgr2) module"]
pub type DMAMUX1_HWCFGR2 = crate::Reg<u32, _DMAMUX1_HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAMUX1_HWCFGR2;
#[doc = "`read()` method returns [dmamux1_hwcfgr2::R](dmamux1_hwcfgr2::R) reader structure"]
impl crate::Readable for DMAMUX1_HWCFGR2 {}
#[doc = "DMAMUX hardware configuration 2 register"]
pub mod dmamux1_hwcfgr2;
