#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ctlr: [u8; 4usize],
    #[doc = "0x04 - GICC input priority mask register"]
    pub pmr: PMR,
    _reserved_2_bpr: [u8; 4usize],
    #[doc = "0x0c - GICC interrupt acknowledge register"]
    pub iar: IAR,
    #[doc = "0x10 - GICC end of interrupt register"]
    pub eoir: EOIR,
    #[doc = "0x14 - GICC running priority register"]
    pub rpr: RPR,
    #[doc = "0x18 - GICC highest priority pending interrupt register"]
    pub hppir: HPPIR,
    #[doc = "0x1c - GICC aliased binary point register"]
    pub abpr: ABPR,
    #[doc = "0x20 - GICC aliased interrupt acknowledge register"]
    pub aiar: AIAR,
    #[doc = "0x24 - GICC aliased end of interrupt register"]
    pub aeoir: AEOIR,
    #[doc = "0x28 - GICC aliased highest priority pending interrupt register"]
    pub ahppir: AHPPIR,
    _reserved11: [u8; 164usize],
    #[doc = "0xd0 - GICC active priority register"]
    pub apr0: APR0,
    _reserved12: [u8; 12usize],
    #[doc = "0xe0 - GICC non-secure active priority register"]
    pub nsapr0: NSAPR0,
    _reserved13: [u8; 24usize],
    #[doc = "0xfc - GICC interface identification register"]
    pub iidr: IIDR,
    _reserved14: [u8; 3840usize],
    #[doc = "0x1000 - GICC deactivate interrupt register"]
    pub dir: DIR,
}
impl RegisterBlock {
    #[doc = "0x00 - GICC control (non-secure access) register"]
    #[inline(always)]
    pub fn ctlrns(&self) -> &CTLRNS {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CTLRNS) }
    }
    #[doc = "0x00 - GICC control (non-secure access) register"]
    #[inline(always)]
    pub fn ctlrns_mut(&self) -> &mut CTLRNS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CTLRNS) }
    }
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub fn ctlr(&self) -> &CTLR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CTLR) }
    }
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub fn ctlr_mut(&self) -> &mut CTLR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CTLR) }
    }
    #[doc = "0x08 - GICC binary point (non-secure access) register"]
    #[inline(always)]
    pub fn bprns(&self) -> &BPRNS {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const BPRNS) }
    }
    #[doc = "0x08 - GICC binary point (non-secure access) register"]
    #[inline(always)]
    pub fn bprns_mut(&self) -> &mut BPRNS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut BPRNS) }
    }
    #[doc = "0x08 - GICC binary point register"]
    #[inline(always)]
    pub fn bpr(&self) -> &BPR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const BPR) }
    }
    #[doc = "0x08 - GICC binary point register"]
    #[inline(always)]
    pub fn bpr_mut(&self) -> &mut BPR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut BPR) }
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr](ctlr) module"]
pub type CTLR = crate::Reg<u32, _CTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTLR;
#[doc = "`read()` method returns [ctlr::R](ctlr::R) reader structure"]
impl crate::Readable for CTLR {}
#[doc = "`write(|w| ..)` method takes [ctlr::W](ctlr::W) writer structure"]
impl crate::Writable for CTLR {}
#[doc = "Control register"]
pub mod ctlr;
#[doc = "GICC control (non-secure access) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlrns](ctlrns) module"]
pub type CTLRNS = crate::Reg<u32, _CTLRNS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTLRNS;
#[doc = "`read()` method returns [ctlrns::R](ctlrns::R) reader structure"]
impl crate::Readable for CTLRNS {}
#[doc = "`write(|w| ..)` method takes [ctlrns::W](ctlrns::W) writer structure"]
impl crate::Writable for CTLRNS {}
#[doc = "GICC control (non-secure access) register"]
pub mod ctlrns;
#[doc = "GICC input priority mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr](pmr) module"]
pub type PMR = crate::Reg<u32, _PMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR;
#[doc = "`read()` method returns [pmr::R](pmr::R) reader structure"]
impl crate::Readable for PMR {}
#[doc = "`write(|w| ..)` method takes [pmr::W](pmr::W) writer structure"]
impl crate::Writable for PMR {}
#[doc = "GICC input priority mask register"]
pub mod pmr;
#[doc = "GICC binary point register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpr](bpr) module"]
pub type BPR = crate::Reg<u32, _BPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPR;
#[doc = "`read()` method returns [bpr::R](bpr::R) reader structure"]
impl crate::Readable for BPR {}
#[doc = "`write(|w| ..)` method takes [bpr::W](bpr::W) writer structure"]
impl crate::Writable for BPR {}
#[doc = "GICC binary point register"]
pub mod bpr;
#[doc = "GICC binary point (non-secure access) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bprns](bprns) module"]
pub type BPRNS = crate::Reg<u32, _BPRNS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPRNS;
#[doc = "`read()` method returns [bprns::R](bprns::R) reader structure"]
impl crate::Readable for BPRNS {}
#[doc = "`write(|w| ..)` method takes [bprns::W](bprns::W) writer structure"]
impl crate::Writable for BPRNS {}
#[doc = "GICC binary point (non-secure access) register"]
pub mod bprns;
#[doc = "GICC interrupt acknowledge register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iar](iar) module"]
pub type IAR = crate::Reg<u32, _IAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IAR;
#[doc = "`read()` method returns [iar::R](iar::R) reader structure"]
impl crate::Readable for IAR {}
#[doc = "GICC interrupt acknowledge register"]
pub mod iar;
#[doc = "GICC end of interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eoir](eoir) module"]
pub type EOIR = crate::Reg<u32, _EOIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EOIR;
#[doc = "`write(|w| ..)` method takes [eoir::W](eoir::W) writer structure"]
impl crate::Writable for EOIR {}
#[doc = "GICC end of interrupt register"]
pub mod eoir;
#[doc = "GICC running priority register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr](rpr) module"]
pub type RPR = crate::Reg<u32, _RPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPR;
#[doc = "`write(|w| ..)` method takes [rpr::W](rpr::W) writer structure"]
impl crate::Writable for RPR {}
#[doc = "GICC running priority register"]
pub mod rpr;
#[doc = "GICC highest priority pending interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hppir](hppir) module"]
pub type HPPIR = crate::Reg<u32, _HPPIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPPIR;
#[doc = "`read()` method returns [hppir::R](hppir::R) reader structure"]
impl crate::Readable for HPPIR {}
#[doc = "GICC highest priority pending interrupt register"]
pub mod hppir;
#[doc = "GICC aliased binary point register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abpr](abpr) module"]
pub type ABPR = crate::Reg<u32, _ABPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ABPR;
#[doc = "`read()` method returns [abpr::R](abpr::R) reader structure"]
impl crate::Readable for ABPR {}
#[doc = "`write(|w| ..)` method takes [abpr::W](abpr::W) writer structure"]
impl crate::Writable for ABPR {}
#[doc = "GICC aliased binary point register"]
pub mod abpr;
#[doc = "GICC aliased interrupt acknowledge register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aiar](aiar) module"]
pub type AIAR = crate::Reg<u32, _AIAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIAR;
#[doc = "`read()` method returns [aiar::R](aiar::R) reader structure"]
impl crate::Readable for AIAR {}
#[doc = "GICC aliased interrupt acknowledge register"]
pub mod aiar;
#[doc = "GICC aliased end of interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeoir](aeoir) module"]
pub type AEOIR = crate::Reg<u32, _AEOIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AEOIR;
#[doc = "`write(|w| ..)` method takes [aeoir::W](aeoir::W) writer structure"]
impl crate::Writable for AEOIR {}
#[doc = "GICC aliased end of interrupt register"]
pub mod aeoir;
#[doc = "GICC aliased highest priority pending interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahppir](ahppir) module"]
pub type AHPPIR = crate::Reg<u32, _AHPPIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHPPIR;
#[doc = "`read()` method returns [ahppir::R](ahppir::R) reader structure"]
impl crate::Readable for AHPPIR {}
#[doc = "GICC aliased highest priority pending interrupt register"]
pub mod ahppir;
#[doc = "GICC active priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apr0](apr0) module"]
pub type APR0 = crate::Reg<u32, _APR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APR0;
#[doc = "`read()` method returns [apr0::R](apr0::R) reader structure"]
impl crate::Readable for APR0 {}
#[doc = "`write(|w| ..)` method takes [apr0::W](apr0::W) writer structure"]
impl crate::Writable for APR0 {}
#[doc = "GICC active priority register"]
pub mod apr0;
#[doc = "GICC non-secure active priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsapr0](nsapr0) module"]
pub type NSAPR0 = crate::Reg<u32, _NSAPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSAPR0;
#[doc = "`read()` method returns [nsapr0::R](nsapr0::R) reader structure"]
impl crate::Readable for NSAPR0 {}
#[doc = "`write(|w| ..)` method takes [nsapr0::W](nsapr0::W) writer structure"]
impl crate::Writable for NSAPR0 {}
#[doc = "GICC non-secure active priority register"]
pub mod nsapr0;
#[doc = "GICC interface identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iidr](iidr) module"]
pub type IIDR = crate::Reg<u32, _IIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IIDR;
#[doc = "`read()` method returns [iidr::R](iidr::R) reader structure"]
impl crate::Readable for IIDR {}
#[doc = "GICC interface identification register"]
pub mod iidr;
#[doc = "GICC deactivate interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "GICC deactivate interrupt register"]
pub mod dir;
