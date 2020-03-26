#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GICV virtual machine control register"]
    pub ctlr: CTLR,
    #[doc = "0x04 - GICV VM priority mask register"]
    pub pmr: PMR,
    #[doc = "0x08 - GICV VM binary point register"]
    pub bpr: BPR,
    #[doc = "0x0c - GICV VM interrupt acknowledge register"]
    pub iar: IAR,
    #[doc = "0x10 - GICV VM end of interrupt register"]
    pub eoir: EOIR,
    #[doc = "0x14 - GICV VM running priority register"]
    pub rpr: RPR,
    #[doc = "0x18 - GICV VM highest priority pending interrupt register"]
    pub hppir: HPPIR,
    #[doc = "0x1c - GICV VM aliased binary point register"]
    pub abpr: ABPR,
    #[doc = "0x20 - GICV VM aliased interrupt register"]
    pub aiar: AIAR,
    #[doc = "0x24 - GICV VM aliased end of interrupt register"]
    pub aeoir: AEOIR,
    #[doc = "0x28 - GICV VM aliased highest priority pending interrupt register"]
    pub ahppir: AHPPIR,
    _reserved11: [u8; 164usize],
    #[doc = "0xd0 - GICV VM active priority register"]
    pub apr0: APR0,
    _reserved12: [u8; 40usize],
    #[doc = "0xfc - GICV VM CPU interface identification register"]
    pub iidr: IIDR,
    _reserved13: [u8; 3840usize],
    #[doc = "0x1000 - GICV VM deactivate interrupt register"]
    pub dir: DIR,
}
#[doc = "GICV virtual machine control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctlr](ctlr) module"]
pub type CTLR = crate::Reg<u32, _CTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTLR;
#[doc = "`read()` method returns [ctlr::R](ctlr::R) reader structure"]
impl crate::Readable for CTLR {}
#[doc = "`write(|w| ..)` method takes [ctlr::W](ctlr::W) writer structure"]
impl crate::Writable for CTLR {}
#[doc = "GICV virtual machine control register"]
pub mod ctlr;
#[doc = "GICV VM priority mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmr](pmr) module"]
pub type PMR = crate::Reg<u32, _PMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMR;
#[doc = "`read()` method returns [pmr::R](pmr::R) reader structure"]
impl crate::Readable for PMR {}
#[doc = "`write(|w| ..)` method takes [pmr::W](pmr::W) writer structure"]
impl crate::Writable for PMR {}
#[doc = "GICV VM priority mask register"]
pub mod pmr;
#[doc = "GICV VM binary point register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpr](bpr) module"]
pub type BPR = crate::Reg<u32, _BPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPR;
#[doc = "`read()` method returns [bpr::R](bpr::R) reader structure"]
impl crate::Readable for BPR {}
#[doc = "`write(|w| ..)` method takes [bpr::W](bpr::W) writer structure"]
impl crate::Writable for BPR {}
#[doc = "GICV VM binary point register"]
pub mod bpr;
#[doc = "GICV VM interrupt acknowledge register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iar](iar) module"]
pub type IAR = crate::Reg<u32, _IAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IAR;
#[doc = "`read()` method returns [iar::R](iar::R) reader structure"]
impl crate::Readable for IAR {}
#[doc = "GICV VM interrupt acknowledge register"]
pub mod iar;
#[doc = "GICV VM end of interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eoir](eoir) module"]
pub type EOIR = crate::Reg<u32, _EOIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EOIR;
#[doc = "`write(|w| ..)` method takes [eoir::W](eoir::W) writer structure"]
impl crate::Writable for EOIR {}
#[doc = "GICV VM end of interrupt register"]
pub mod eoir;
#[doc = "GICV VM running priority register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr](rpr) module"]
pub type RPR = crate::Reg<u32, _RPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPR;
#[doc = "`read()` method returns [rpr::R](rpr::R) reader structure"]
impl crate::Readable for RPR {}
#[doc = "GICV VM running priority register"]
pub mod rpr;
#[doc = "GICV VM highest priority pending interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hppir](hppir) module"]
pub type HPPIR = crate::Reg<u32, _HPPIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPPIR;
#[doc = "`read()` method returns [hppir::R](hppir::R) reader structure"]
impl crate::Readable for HPPIR {}
#[doc = "GICV VM highest priority pending interrupt register"]
pub mod hppir;
#[doc = "GICV VM aliased binary point register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abpr](abpr) module"]
pub type ABPR = crate::Reg<u32, _ABPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ABPR;
#[doc = "`read()` method returns [abpr::R](abpr::R) reader structure"]
impl crate::Readable for ABPR {}
#[doc = "`write(|w| ..)` method takes [abpr::W](abpr::W) writer structure"]
impl crate::Writable for ABPR {}
#[doc = "GICV VM aliased binary point register"]
pub mod abpr;
#[doc = "GICV VM aliased interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aiar](aiar) module"]
pub type AIAR = crate::Reg<u32, _AIAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIAR;
#[doc = "`read()` method returns [aiar::R](aiar::R) reader structure"]
impl crate::Readable for AIAR {}
#[doc = "GICV VM aliased interrupt register"]
pub mod aiar;
#[doc = "GICV VM aliased end of interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeoir](aeoir) module"]
pub type AEOIR = crate::Reg<u32, _AEOIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AEOIR;
#[doc = "`write(|w| ..)` method takes [aeoir::W](aeoir::W) writer structure"]
impl crate::Writable for AEOIR {}
#[doc = "GICV VM aliased end of interrupt register"]
pub mod aeoir;
#[doc = "GICV VM aliased highest priority pending interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahppir](ahppir) module"]
pub type AHPPIR = crate::Reg<u32, _AHPPIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHPPIR;
#[doc = "`read()` method returns [ahppir::R](ahppir::R) reader structure"]
impl crate::Readable for AHPPIR {}
#[doc = "GICV VM aliased highest priority pending interrupt register"]
pub mod ahppir;
#[doc = "GICV VM active priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apr0](apr0) module"]
pub type APR0 = crate::Reg<u32, _APR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APR0;
#[doc = "`read()` method returns [apr0::R](apr0::R) reader structure"]
impl crate::Readable for APR0 {}
#[doc = "`write(|w| ..)` method takes [apr0::W](apr0::W) writer structure"]
impl crate::Writable for APR0 {}
#[doc = "GICV VM active priority register"]
pub mod apr0;
#[doc = "GICV VM CPU interface identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iidr](iidr) module"]
pub type IIDR = crate::Reg<u32, _IIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IIDR;
#[doc = "`read()` method returns [iidr::R](iidr::R) reader structure"]
impl crate::Readable for IIDR {}
#[doc = "GICV VM CPU interface identification register"]
pub mod iidr;
#[doc = "GICV VM deactivate interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "GICV VM deactivate interrupt register"]
pub mod dir;
