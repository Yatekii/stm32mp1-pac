#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RIDR register"]
    pub ddrphyc_ridr: DDRPHYC_RIDR,
    #[doc = "0x04 - PIR register"]
    pub ddrphyc_pir: DDRPHYC_PIR,
    #[doc = "0x08 - PGCR register"]
    pub ddrphyc_pgcr: DDRPHYC_PGCR,
    #[doc = "0x0c - PGSR register"]
    pub ddrphyc_pgsr: DDRPHYC_PGSR,
    #[doc = "0x10 - DLLGCR register"]
    pub ddrphyc_dllgcr: DDRPHYC_DLLGCR,
    #[doc = "0x14 - ACDLLCR register"]
    pub ddrphyc_acdllcr: DDRPHYC_ACDLLCR,
    #[doc = "0x18 - PTR0 register"]
    pub ddrphyc_ptr0: DDRPHYC_PTR0,
    #[doc = "0x1c - PTR1 register"]
    pub ddrphyc_ptr1: DDRPHYC_PTR1,
    #[doc = "0x20 - PTR2 register"]
    pub ddrphyc_ptr2: DDRPHYC_PTR2,
    #[doc = "0x24 - ACIOCR register"]
    pub ddrphyc_aciocr: DDRPHYC_ACIOCR,
    #[doc = "0x28 - DXCCR register"]
    pub ddrphyc_dxccr: DDRPHYC_DXCCR,
    #[doc = "0x2c - DSGCR register"]
    pub ddrphyc_dsgcr: DDRPHYC_DSGCR,
    #[doc = "0x30 - DCR register"]
    pub ddrphyc_dcr: DDRPHYC_DCR,
    #[doc = "0x34 - DTPR0 register"]
    pub ddrphyc_dtpr0: DDRPHYC_DTPR0,
    #[doc = "0x38 - DTPR1 register"]
    pub ddrphyc_dtpr1: DDRPHYC_DTPR1,
    #[doc = "0x3c - DTPR2 register"]
    pub ddrphyc_dtpr2: DDRPHYC_DTPR2,
    #[doc = "0x40 - MR0 register for DDR3"]
    pub ddrphyc_mr0: DDRPHYC_MR0,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - MR1 register for LPDDR2"]
    pub ddrphyc_mr1: DDRPHYC_MR1,
    _reserved18: [u8; 2usize],
    #[doc = "0x48 - MR2 register for LPDDR2"]
    pub ddrphyc_mr2: DDRPHYC_MR2,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - MR3 register for DDR3"]
    pub ddrphyc_mr3: DDRPHYC_MR3,
    _reserved20: [u8; 3usize],
    #[doc = "0x50 - ODTCR register"]
    pub ddrphyc_odtcr: DDRPHYC_ODTCR,
    #[doc = "0x54 - DTAR register"]
    pub ddrphyc_dtar: DDRPHYC_DTAR,
    #[doc = "0x58 - DTDR0 register"]
    pub ddrphyc_dtdr0: DDRPHYC_DTDR0,
    #[doc = "0x5c - DTDR1 register"]
    pub ddrphyc_dtdr1: DDRPHYC_DTDR1,
    _reserved24: [u8; 96usize],
    #[doc = "0xc0 - DCUAR register"]
    pub ddrphyc_dcuar: DDRPHYC_DCUAR,
    _reserved25: [u8; 2usize],
    #[doc = "0xc4 - DCUDR register"]
    pub ddrphyc_dcudr: DDRPHYC_DCUDR,
    #[doc = "0xc8 - DCURR register"]
    pub ddrphyc_dcurr: DDRPHYC_DCURR,
    #[doc = "0xcc - DCULR register"]
    pub ddrphyc_dculr: DDRPHYC_DCULR,
    #[doc = "0xd0 - DCUGCR register"]
    pub ddrphyc_dcugcr: DDRPHYC_DCUGCR,
    _reserved29: [u8; 2usize],
    #[doc = "0xd4 - DCUTPR register"]
    pub ddrphyc_dcutpr: DDRPHYC_DCUTPR,
    #[doc = "0xd8 - DCUSR0 register"]
    pub ddrphyc_dcusr0: DDRPHYC_DCUSR0,
    _reserved31: [u8; 3usize],
    #[doc = "0xdc - DCUSR1 register"]
    pub ddrphyc_dcusr1: DDRPHYC_DCUSR1,
    _reserved32: [u8; 32usize],
    #[doc = "0x100 - BISTRR register"]
    pub ddrphyc_bistrr: DDRPHYC_BISTRR,
    #[doc = "0x104 - BISTMSKR0 register"]
    pub ddrphyc_bistmskr0: DDRPHYC_BISTMSKR0,
    #[doc = "0x108 - BISTMSKR1 register"]
    pub ddrphyc_bistmskr1: DDRPHYC_BISTMSKR1,
    #[doc = "0x10c - BISTWCR register"]
    pub ddrphyc_bistwcr: DDRPHYC_BISTWCR,
    _reserved36: [u8; 2usize],
    #[doc = "0x110 - BISTLSR register"]
    pub ddrphyc_bistlsr: DDRPHYC_BISTLSR,
    #[doc = "0x114 - BISTAR0 register"]
    pub ddrphyc_bistar0: DDRPHYC_BISTAR0,
    #[doc = "0x118 - BISTAR1 register"]
    pub ddrphyc_bistar1: DDRPHYC_BISTAR1,
    _reserved39: [u8; 2usize],
    #[doc = "0x11c - BISTAR2 register"]
    pub ddrphyc_bistar2: DDRPHYC_BISTAR2,
    #[doc = "0x120 - BISTUDPR register"]
    pub ddrphyc_bistudpr: DDRPHYC_BISTUDPR,
    #[doc = "0x124 - BISTGSR register"]
    pub ddrphyc_bistgsr: DDRPHYC_BISTGSR,
    #[doc = "0x128 - BISTWER register"]
    pub ddrphyc_bistwer: DDRPHYC_BISTWER,
    #[doc = "0x12c - BISTBER0 register"]
    pub ddrphyc_bistber0: DDRPHYC_BISTBER0,
    #[doc = "0x130 - BISTBER1 register"]
    pub ddrphyc_bistber1: DDRPHYC_BISTBER1,
    #[doc = "0x134 - BISTBER2 register"]
    pub ddrphyc_bistber2: DDRPHYC_BISTBER2,
    #[doc = "0x138 - BISTWCSR register"]
    pub ddrphyc_bistwcsr: DDRPHYC_BISTWCSR,
    #[doc = "0x13c - BISTFWR0 register"]
    pub ddrphyc_bistfwr0: DDRPHYC_BISTFWR0,
    #[doc = "0x140 - BISTFWR1 register"]
    pub ddrphyc_bistfwr1: DDRPHYC_BISTFWR1,
    _reserved49: [u8; 52usize],
    #[doc = "0x178 - General Purpose Register 0"]
    pub ddrphyc_gpr0: DDRPHYC_GPR0,
    #[doc = "0x17c - General Purpose Register register 1"]
    pub ddrphyc_gpr1: DDRPHYC_GPR1,
    #[doc = "0x180 - ZQ0CR0 register"]
    pub ddrphyc_zq0cr0: DDRPHYC_ZQ0CR0,
    #[doc = "0x184 - ZQ0CR1 register"]
    pub ddrphyc_zq0cr1: DDRPHYC_ZQ0CR1,
    _reserved53: [u8; 3usize],
    #[doc = "0x188 - ZQ0SR0 register"]
    pub ddrphyc_zq0sr0: DDRPHYC_ZQ0SR0,
    #[doc = "0x18c - ZQ0SR1 register"]
    pub ddrphyc_zq0sr1: DDRPHYC_ZQ0SR1,
    _reserved55: [u8; 51usize],
    #[doc = "0x1c0 - DX 0 GCR register"]
    pub ddrphyc_dx0gcr: DDRPHYC_DX0GCR,
    #[doc = "0x1c4 - DX 0 GSR0 register"]
    pub ddrphyc_dx0gsr0: DDRPHYC_DX0GSR0,
    _reserved57: [u8; 2usize],
    #[doc = "0x1c8 - DX 0 GSR1 register"]
    pub ddrphyc_dx0gsr1: DDRPHYC_DX0GSR1,
    #[doc = "0x1cc - DX 0 DLLCR register"]
    pub ddrphyc_dx0dllcr: DDRPHYC_DX0DLLCR,
    #[doc = "0x1d0 - DX 0 DQTR register"]
    pub ddrphyc_dx0dqtr: DDRPHYC_DX0DQTR,
    #[doc = "0x1d4 - DX 0 DQSTR register"]
    pub ddrphyc_dx0dqstr: DDRPHYC_DX0DQSTR,
    _reserved61: [u8; 40usize],
    #[doc = "0x200 - DX 1 GCR register"]
    pub ddrphyc_dx1gcr: DDRPHYC_DX1GCR,
    #[doc = "0x204 - DX 1 GSR0 register"]
    pub ddrphyc_dx1gsr0: DDRPHYC_DX1GSR0,
    _reserved63: [u8; 2usize],
    #[doc = "0x208 - DX 1 GSR1 register"]
    pub ddrphyc_dx1gsr1: DDRPHYC_DX1GSR1,
    #[doc = "0x20c - DX 1 DLLCR register"]
    pub ddrphyc_dx1dllcr: DDRPHYC_DX1DLLCR,
    #[doc = "0x210 - DX 1 DQTR register"]
    pub ddrphyc_dx1dqtr: DDRPHYC_DX1DQTR,
    #[doc = "0x214 - DX 1 DQSTR register"]
    pub ddrphyc_dx1dqstr: DDRPHYC_DX1DQSTR,
    _reserved67: [u8; 40usize],
    #[doc = "0x240 - DX 2 GCR register"]
    pub ddrphyc_dx2gcr: DDRPHYC_DX2GCR,
    #[doc = "0x244 - DX 2 GSR0 register"]
    pub ddrphyc_dx2gsr0: DDRPHYC_DX2GSR0,
    _reserved69: [u8; 2usize],
    #[doc = "0x248 - DX 2 GSR1 register"]
    pub ddrphyc_dx2gsr1: DDRPHYC_DX2GSR1,
    #[doc = "0x24c - DX 2 DLLCR register"]
    pub ddrphyc_dx2dllcr: DDRPHYC_DX2DLLCR,
    #[doc = "0x250 - DX 2 DQTR register"]
    pub ddrphyc_dx2dqtr: DDRPHYC_DX2DQTR,
    #[doc = "0x254 - DX 2 DQSTR register"]
    pub ddrphyc_dx2dqstr: DDRPHYC_DX2DQSTR,
    _reserved73: [u8; 40usize],
    #[doc = "0x280 - DX 3 GCR register"]
    pub ddrphyc_dx3gcr: DDRPHYC_DX3GCR,
    #[doc = "0x284 - DX 3 GSR0 register"]
    pub ddrphyc_dx3gsr0: DDRPHYC_DX3GSR0,
    _reserved75: [u8; 2usize],
    #[doc = "0x288 - DX 3 GSR1 register"]
    pub ddrphyc_dx3gsr1: DDRPHYC_DX3GSR1,
    #[doc = "0x28c - DX 3 DLLCR register"]
    pub ddrphyc_dx3dllcr: DDRPHYC_DX3DLLCR,
    #[doc = "0x290 - DX 3 DQTR register"]
    pub ddrphyc_dx3dqtr: DDRPHYC_DX3DQTR,
    #[doc = "0x294 - DX 3 DQSTR register"]
    pub ddrphyc_dx3dqstr: DDRPHYC_DX3DQSTR,
}
#[doc = "RIDR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ridr](ddrphyc_ridr) module"]
pub type DDRPHYC_RIDR = crate::Reg<u32, _DDRPHYC_RIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_RIDR;
#[doc = "`read()` method returns [ddrphyc_ridr::R](ddrphyc_ridr::R) reader structure"]
impl crate::Readable for DDRPHYC_RIDR {}
#[doc = "RIDR register"]
pub mod ddrphyc_ridr;
#[doc = "PIR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_pir](ddrphyc_pir) module"]
pub type DDRPHYC_PIR = crate::Reg<u32, _DDRPHYC_PIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PIR;
#[doc = "`read()` method returns [ddrphyc_pir::R](ddrphyc_pir::R) reader structure"]
impl crate::Readable for DDRPHYC_PIR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_pir::W](ddrphyc_pir::W) writer structure"]
impl crate::Writable for DDRPHYC_PIR {}
#[doc = "PIR register"]
pub mod ddrphyc_pir;
#[doc = "PGCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_pgcr](ddrphyc_pgcr) module"]
pub type DDRPHYC_PGCR = crate::Reg<u32, _DDRPHYC_PGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PGCR;
#[doc = "`read()` method returns [ddrphyc_pgcr::R](ddrphyc_pgcr::R) reader structure"]
impl crate::Readable for DDRPHYC_PGCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_pgcr::W](ddrphyc_pgcr::W) writer structure"]
impl crate::Writable for DDRPHYC_PGCR {}
#[doc = "PGCR register"]
pub mod ddrphyc_pgcr;
#[doc = "PGSR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_pgsr](ddrphyc_pgsr) module"]
pub type DDRPHYC_PGSR = crate::Reg<u32, _DDRPHYC_PGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PGSR;
#[doc = "`read()` method returns [ddrphyc_pgsr::R](ddrphyc_pgsr::R) reader structure"]
impl crate::Readable for DDRPHYC_PGSR {}
#[doc = "PGSR register"]
pub mod ddrphyc_pgsr;
#[doc = "DLLGCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dllgcr](ddrphyc_dllgcr) module"]
pub type DDRPHYC_DLLGCR = crate::Reg<u32, _DDRPHYC_DLLGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DLLGCR;
#[doc = "`read()` method returns [ddrphyc_dllgcr::R](ddrphyc_dllgcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DLLGCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dllgcr::W](ddrphyc_dllgcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DLLGCR {}
#[doc = "DLLGCR register"]
pub mod ddrphyc_dllgcr;
#[doc = "ACDLLCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_acdllcr](ddrphyc_acdllcr) module"]
pub type DDRPHYC_ACDLLCR = crate::Reg<u32, _DDRPHYC_ACDLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ACDLLCR;
#[doc = "`read()` method returns [ddrphyc_acdllcr::R](ddrphyc_acdllcr::R) reader structure"]
impl crate::Readable for DDRPHYC_ACDLLCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_acdllcr::W](ddrphyc_acdllcr::W) writer structure"]
impl crate::Writable for DDRPHYC_ACDLLCR {}
#[doc = "ACDLLCR register"]
pub mod ddrphyc_acdllcr;
#[doc = "PTR0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ptr0](ddrphyc_ptr0) module"]
pub type DDRPHYC_PTR0 = crate::Reg<u32, _DDRPHYC_PTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PTR0;
#[doc = "`read()` method returns [ddrphyc_ptr0::R](ddrphyc_ptr0::R) reader structure"]
impl crate::Readable for DDRPHYC_PTR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ptr0::W](ddrphyc_ptr0::W) writer structure"]
impl crate::Writable for DDRPHYC_PTR0 {}
#[doc = "PTR0 register"]
pub mod ddrphyc_ptr0;
#[doc = "PTR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ptr1](ddrphyc_ptr1) module"]
pub type DDRPHYC_PTR1 = crate::Reg<u32, _DDRPHYC_PTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PTR1;
#[doc = "`read()` method returns [ddrphyc_ptr1::R](ddrphyc_ptr1::R) reader structure"]
impl crate::Readable for DDRPHYC_PTR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ptr1::W](ddrphyc_ptr1::W) writer structure"]
impl crate::Writable for DDRPHYC_PTR1 {}
#[doc = "PTR1 register"]
pub mod ddrphyc_ptr1;
#[doc = "PTR2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_ptr2](ddrphyc_ptr2) module"]
pub type DDRPHYC_PTR2 = crate::Reg<u32, _DDRPHYC_PTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_PTR2;
#[doc = "`read()` method returns [ddrphyc_ptr2::R](ddrphyc_ptr2::R) reader structure"]
impl crate::Readable for DDRPHYC_PTR2 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_ptr2::W](ddrphyc_ptr2::W) writer structure"]
impl crate::Writable for DDRPHYC_PTR2 {}
#[doc = "PTR2 register"]
pub mod ddrphyc_ptr2;
#[doc = "ACIOCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_aciocr](ddrphyc_aciocr) module"]
pub type DDRPHYC_ACIOCR = crate::Reg<u32, _DDRPHYC_ACIOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ACIOCR;
#[doc = "`read()` method returns [ddrphyc_aciocr::R](ddrphyc_aciocr::R) reader structure"]
impl crate::Readable for DDRPHYC_ACIOCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_aciocr::W](ddrphyc_aciocr::W) writer structure"]
impl crate::Writable for DDRPHYC_ACIOCR {}
#[doc = "ACIOCR register"]
pub mod ddrphyc_aciocr;
#[doc = "DXCCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dxccr](ddrphyc_dxccr) module"]
pub type DDRPHYC_DXCCR = crate::Reg<u32, _DDRPHYC_DXCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DXCCR;
#[doc = "`read()` method returns [ddrphyc_dxccr::R](ddrphyc_dxccr::R) reader structure"]
impl crate::Readable for DDRPHYC_DXCCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dxccr::W](ddrphyc_dxccr::W) writer structure"]
impl crate::Writable for DDRPHYC_DXCCR {}
#[doc = "DXCCR register"]
pub mod ddrphyc_dxccr;
#[doc = "DSGCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dsgcr](ddrphyc_dsgcr) module"]
pub type DDRPHYC_DSGCR = crate::Reg<u32, _DDRPHYC_DSGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DSGCR;
#[doc = "`read()` method returns [ddrphyc_dsgcr::R](ddrphyc_dsgcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DSGCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dsgcr::W](ddrphyc_dsgcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DSGCR {}
#[doc = "DSGCR register"]
pub mod ddrphyc_dsgcr;
#[doc = "DCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dcr](ddrphyc_dcr) module"]
pub type DDRPHYC_DCR = crate::Reg<u32, _DDRPHYC_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DCR;
#[doc = "`read()` method returns [ddrphyc_dcr::R](ddrphyc_dcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dcr::W](ddrphyc_dcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DCR {}
#[doc = "DCR register"]
pub mod ddrphyc_dcr;
#[doc = "DTPR0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtpr0](ddrphyc_dtpr0) module"]
pub type DDRPHYC_DTPR0 = crate::Reg<u32, _DDRPHYC_DTPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTPR0;
#[doc = "`read()` method returns [ddrphyc_dtpr0::R](ddrphyc_dtpr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtpr0::W](ddrphyc_dtpr0::W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR0 {}
#[doc = "DTPR0 register"]
pub mod ddrphyc_dtpr0;
#[doc = "DTPR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtpr1](ddrphyc_dtpr1) module"]
pub type DDRPHYC_DTPR1 = crate::Reg<u32, _DDRPHYC_DTPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTPR1;
#[doc = "`read()` method returns [ddrphyc_dtpr1::R](ddrphyc_dtpr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtpr1::W](ddrphyc_dtpr1::W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR1 {}
#[doc = "DTPR1 register"]
pub mod ddrphyc_dtpr1;
#[doc = "DTPR2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtpr2](ddrphyc_dtpr2) module"]
pub type DDRPHYC_DTPR2 = crate::Reg<u32, _DDRPHYC_DTPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTPR2;
#[doc = "`read()` method returns [ddrphyc_dtpr2::R](ddrphyc_dtpr2::R) reader structure"]
impl crate::Readable for DDRPHYC_DTPR2 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtpr2::W](ddrphyc_dtpr2::W) writer structure"]
impl crate::Writable for DDRPHYC_DTPR2 {}
#[doc = "DTPR2 register"]
pub mod ddrphyc_dtpr2;
#[doc = "MR0 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_mr0](ddrphyc_mr0) module"]
pub type DDRPHYC_MR0 = crate::Reg<u16, _DDRPHYC_MR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_MR0;
#[doc = "`read()` method returns [ddrphyc_mr0::R](ddrphyc_mr0::R) reader structure"]
impl crate::Readable for DDRPHYC_MR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_mr0::W](ddrphyc_mr0::W) writer structure"]
impl crate::Writable for DDRPHYC_MR0 {}
#[doc = "MR0 register for DDR3"]
pub mod ddrphyc_mr0;
#[doc = "MR1 register for LPDDR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_mr1](ddrphyc_mr1) module"]
pub type DDRPHYC_MR1 = crate::Reg<u16, _DDRPHYC_MR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_MR1;
#[doc = "`read()` method returns [ddrphyc_mr1::R](ddrphyc_mr1::R) reader structure"]
impl crate::Readable for DDRPHYC_MR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_mr1::W](ddrphyc_mr1::W) writer structure"]
impl crate::Writable for DDRPHYC_MR1 {}
#[doc = "MR1 register for LPDDR2"]
pub mod ddrphyc_mr1;
#[doc = "MR2 register for LPDDR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_mr2](ddrphyc_mr2) module"]
pub type DDRPHYC_MR2 = crate::Reg<u16, _DDRPHYC_MR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_MR2;
#[doc = "`read()` method returns [ddrphyc_mr2::R](ddrphyc_mr2::R) reader structure"]
impl crate::Readable for DDRPHYC_MR2 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_mr2::W](ddrphyc_mr2::W) writer structure"]
impl crate::Writable for DDRPHYC_MR2 {}
#[doc = "MR2 register for LPDDR2"]
pub mod ddrphyc_mr2;
#[doc = "MR3 register for DDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_mr3](ddrphyc_mr3) module"]
pub type DDRPHYC_MR3 = crate::Reg<u8, _DDRPHYC_MR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_MR3;
#[doc = "`read()` method returns [ddrphyc_mr3::R](ddrphyc_mr3::R) reader structure"]
impl crate::Readable for DDRPHYC_MR3 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_mr3::W](ddrphyc_mr3::W) writer structure"]
impl crate::Writable for DDRPHYC_MR3 {}
#[doc = "MR3 register for DDR3"]
pub mod ddrphyc_mr3;
#[doc = "ODTCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_odtcr](ddrphyc_odtcr) module"]
pub type DDRPHYC_ODTCR = crate::Reg<u32, _DDRPHYC_ODTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ODTCR;
#[doc = "`read()` method returns [ddrphyc_odtcr::R](ddrphyc_odtcr::R) reader structure"]
impl crate::Readable for DDRPHYC_ODTCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_odtcr::W](ddrphyc_odtcr::W) writer structure"]
impl crate::Writable for DDRPHYC_ODTCR {}
#[doc = "ODTCR register"]
pub mod ddrphyc_odtcr;
#[doc = "DTAR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtar](ddrphyc_dtar) module"]
pub type DDRPHYC_DTAR = crate::Reg<u32, _DDRPHYC_DTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTAR;
#[doc = "`read()` method returns [ddrphyc_dtar::R](ddrphyc_dtar::R) reader structure"]
impl crate::Readable for DDRPHYC_DTAR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtar::W](ddrphyc_dtar::W) writer structure"]
impl crate::Writable for DDRPHYC_DTAR {}
#[doc = "DTAR register"]
pub mod ddrphyc_dtar;
#[doc = "DTDR0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtdr0](ddrphyc_dtdr0) module"]
pub type DDRPHYC_DTDR0 = crate::Reg<u32, _DDRPHYC_DTDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTDR0;
#[doc = "`read()` method returns [ddrphyc_dtdr0::R](ddrphyc_dtdr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DTDR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtdr0::W](ddrphyc_dtdr0::W) writer structure"]
impl crate::Writable for DDRPHYC_DTDR0 {}
#[doc = "DTDR0 register"]
pub mod ddrphyc_dtdr0;
#[doc = "DTDR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtdr1](ddrphyc_dtdr1) module"]
pub type DDRPHYC_DTDR1 = crate::Reg<u32, _DDRPHYC_DTDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DTDR1;
#[doc = "`read()` method returns [ddrphyc_dtdr1::R](ddrphyc_dtdr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DTDR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtdr1::W](ddrphyc_dtdr1::W) writer structure"]
impl crate::Writable for DDRPHYC_DTDR1 {}
#[doc = "DTDR1 register"]
pub mod ddrphyc_dtdr1;
#[doc = "DCUAR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dcuar](ddrphyc_dcuar) module"]
pub type DDRPHYC_DCUAR = crate::Reg<u16, _DDRPHYC_DCUAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DCUAR;
#[doc = "`read()` method returns [ddrphyc_dcuar::R](ddrphyc_dcuar::R) reader structure"]
impl crate::Readable for DDRPHYC_DCUAR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dcuar::W](ddrphyc_dcuar::W) writer structure"]
impl crate::Writable for DDRPHYC_DCUAR {}
#[doc = "DCUAR register"]
pub mod ddrphyc_dcuar;
#[doc = "DCUDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dcudr](ddrphyc_dcudr) module"]
pub type DDRPHYC_DCUDR = crate::Reg<u32, _DDRPHYC_DCUDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DCUDR;
#[doc = "`read()` method returns [ddrphyc_dcudr::R](ddrphyc_dcudr::R) reader structure"]
impl crate::Readable for DDRPHYC_DCUDR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dcudr::W](ddrphyc_dcudr::W) writer structure"]
impl crate::Writable for DDRPHYC_DCUDR {}
#[doc = "DCUDR register"]
pub mod ddrphyc_dcudr;
#[doc = "DCURR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dcurr](ddrphyc_dcurr) module"]
pub type DDRPHYC_DCURR = crate::Reg<u32, _DDRPHYC_DCURR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DCURR;
#[doc = "`read()` method returns [ddrphyc_dcurr::R](ddrphyc_dcurr::R) reader structure"]
impl crate::Readable for DDRPHYC_DCURR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dcurr::W](ddrphyc_dcurr::W) writer structure"]
impl crate::Writable for DDRPHYC_DCURR {}
#[doc = "DCURR register"]
pub mod ddrphyc_dcurr;
#[doc = "DCULR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dculr](ddrphyc_dculr) module"]
pub type DDRPHYC_DCULR = crate::Reg<u32, _DDRPHYC_DCULR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DCULR;
#[doc = "`read()` method returns [ddrphyc_dculr::R](ddrphyc_dculr::R) reader structure"]
impl crate::Readable for DDRPHYC_DCULR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dculr::W](ddrphyc_dculr::W) writer structure"]
impl crate::Writable for DDRPHYC_DCULR {}
#[doc = "DCULR register"]
pub mod ddrphyc_dculr;
#[doc = "DCUGCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dcugcr](ddrphyc_dcugcr) module"]
pub type DDRPHYC_DCUGCR = crate::Reg<u16, _DDRPHYC_DCUGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DCUGCR;
#[doc = "`read()` method returns [ddrphyc_dcugcr::R](ddrphyc_dcugcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DCUGCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dcugcr::W](ddrphyc_dcugcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DCUGCR {}
#[doc = "DCUGCR register"]
pub mod ddrphyc_dcugcr;
#[doc = "DCUTPR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dcutpr](ddrphyc_dcutpr) module"]
pub type DDRPHYC_DCUTPR = crate::Reg<u32, _DDRPHYC_DCUTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DCUTPR;
#[doc = "`read()` method returns [ddrphyc_dcutpr::R](ddrphyc_dcutpr::R) reader structure"]
impl crate::Readable for DDRPHYC_DCUTPR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dcutpr::W](ddrphyc_dcutpr::W) writer structure"]
impl crate::Writable for DDRPHYC_DCUTPR {}
#[doc = "DCUTPR register"]
pub mod ddrphyc_dcutpr;
#[doc = "DCUSR0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dcusr0](ddrphyc_dcusr0) module"]
pub type DDRPHYC_DCUSR0 = crate::Reg<u8, _DDRPHYC_DCUSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DCUSR0;
#[doc = "`read()` method returns [ddrphyc_dcusr0::R](ddrphyc_dcusr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DCUSR0 {}
#[doc = "DCUSR0 register"]
pub mod ddrphyc_dcusr0;
#[doc = "DCUSR1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dcusr1](ddrphyc_dcusr1) module"]
pub type DDRPHYC_DCUSR1 = crate::Reg<u32, _DDRPHYC_DCUSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DCUSR1;
#[doc = "`read()` method returns [ddrphyc_dcusr1::R](ddrphyc_dcusr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DCUSR1 {}
#[doc = "DCUSR1 register"]
pub mod ddrphyc_dcusr1;
#[doc = "BISTRR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistrr](ddrphyc_bistrr) module"]
pub type DDRPHYC_BISTRR = crate::Reg<u32, _DDRPHYC_BISTRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTRR;
#[doc = "`read()` method returns [ddrphyc_bistrr::R](ddrphyc_bistrr::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTRR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_bistrr::W](ddrphyc_bistrr::W) writer structure"]
impl crate::Writable for DDRPHYC_BISTRR {}
#[doc = "BISTRR register"]
pub mod ddrphyc_bistrr;
#[doc = "BISTMSKR0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistmskr0](ddrphyc_bistmskr0) module"]
pub type DDRPHYC_BISTMSKR0 = crate::Reg<u32, _DDRPHYC_BISTMSKR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTMSKR0;
#[doc = "`read()` method returns [ddrphyc_bistmskr0::R](ddrphyc_bistmskr0::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTMSKR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_bistmskr0::W](ddrphyc_bistmskr0::W) writer structure"]
impl crate::Writable for DDRPHYC_BISTMSKR0 {}
#[doc = "BISTMSKR0 register"]
pub mod ddrphyc_bistmskr0;
#[doc = "BISTMSKR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistmskr1](ddrphyc_bistmskr1) module"]
pub type DDRPHYC_BISTMSKR1 = crate::Reg<u32, _DDRPHYC_BISTMSKR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTMSKR1;
#[doc = "`read()` method returns [ddrphyc_bistmskr1::R](ddrphyc_bistmskr1::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTMSKR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_bistmskr1::W](ddrphyc_bistmskr1::W) writer structure"]
impl crate::Writable for DDRPHYC_BISTMSKR1 {}
#[doc = "BISTMSKR1 register"]
pub mod ddrphyc_bistmskr1;
#[doc = "BISTWCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistwcr](ddrphyc_bistwcr) module"]
pub type DDRPHYC_BISTWCR = crate::Reg<u16, _DDRPHYC_BISTWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTWCR;
#[doc = "`read()` method returns [ddrphyc_bistwcr::R](ddrphyc_bistwcr::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTWCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_bistwcr::W](ddrphyc_bistwcr::W) writer structure"]
impl crate::Writable for DDRPHYC_BISTWCR {}
#[doc = "BISTWCR register"]
pub mod ddrphyc_bistwcr;
#[doc = "BISTLSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistlsr](ddrphyc_bistlsr) module"]
pub type DDRPHYC_BISTLSR = crate::Reg<u32, _DDRPHYC_BISTLSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTLSR;
#[doc = "`read()` method returns [ddrphyc_bistlsr::R](ddrphyc_bistlsr::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTLSR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_bistlsr::W](ddrphyc_bistlsr::W) writer structure"]
impl crate::Writable for DDRPHYC_BISTLSR {}
#[doc = "BISTLSR register"]
pub mod ddrphyc_bistlsr;
#[doc = "BISTAR0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistar0](ddrphyc_bistar0) module"]
pub type DDRPHYC_BISTAR0 = crate::Reg<u32, _DDRPHYC_BISTAR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTAR0;
#[doc = "`read()` method returns [ddrphyc_bistar0::R](ddrphyc_bistar0::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTAR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_bistar0::W](ddrphyc_bistar0::W) writer structure"]
impl crate::Writable for DDRPHYC_BISTAR0 {}
#[doc = "BISTAR0 register"]
pub mod ddrphyc_bistar0;
#[doc = "BISTAR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistar1](ddrphyc_bistar1) module"]
pub type DDRPHYC_BISTAR1 = crate::Reg<u16, _DDRPHYC_BISTAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTAR1;
#[doc = "`read()` method returns [ddrphyc_bistar1::R](ddrphyc_bistar1::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTAR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_bistar1::W](ddrphyc_bistar1::W) writer structure"]
impl crate::Writable for DDRPHYC_BISTAR1 {}
#[doc = "BISTAR1 register"]
pub mod ddrphyc_bistar1;
#[doc = "BISTAR2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistar2](ddrphyc_bistar2) module"]
pub type DDRPHYC_BISTAR2 = crate::Reg<u32, _DDRPHYC_BISTAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTAR2;
#[doc = "`read()` method returns [ddrphyc_bistar2::R](ddrphyc_bistar2::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTAR2 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_bistar2::W](ddrphyc_bistar2::W) writer structure"]
impl crate::Writable for DDRPHYC_BISTAR2 {}
#[doc = "BISTAR2 register"]
pub mod ddrphyc_bistar2;
#[doc = "BISTUDPR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistudpr](ddrphyc_bistudpr) module"]
pub type DDRPHYC_BISTUDPR = crate::Reg<u32, _DDRPHYC_BISTUDPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTUDPR;
#[doc = "`read()` method returns [ddrphyc_bistudpr::R](ddrphyc_bistudpr::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTUDPR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_bistudpr::W](ddrphyc_bistudpr::W) writer structure"]
impl crate::Writable for DDRPHYC_BISTUDPR {}
#[doc = "BISTUDPR register"]
pub mod ddrphyc_bistudpr;
#[doc = "BISTGSR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistgsr](ddrphyc_bistgsr) module"]
pub type DDRPHYC_BISTGSR = crate::Reg<u32, _DDRPHYC_BISTGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTGSR;
#[doc = "`read()` method returns [ddrphyc_bistgsr::R](ddrphyc_bistgsr::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTGSR {}
#[doc = "BISTGSR register"]
pub mod ddrphyc_bistgsr;
#[doc = "BISTWER register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistwer](ddrphyc_bistwer) module"]
pub type DDRPHYC_BISTWER = crate::Reg<u32, _DDRPHYC_BISTWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTWER;
#[doc = "`read()` method returns [ddrphyc_bistwer::R](ddrphyc_bistwer::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTWER {}
#[doc = "BISTWER register"]
pub mod ddrphyc_bistwer;
#[doc = "BISTBER0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistber0](ddrphyc_bistber0) module"]
pub type DDRPHYC_BISTBER0 = crate::Reg<u32, _DDRPHYC_BISTBER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTBER0;
#[doc = "`read()` method returns [ddrphyc_bistber0::R](ddrphyc_bistber0::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTBER0 {}
#[doc = "BISTBER0 register"]
pub mod ddrphyc_bistber0;
#[doc = "BISTBER1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistber1](ddrphyc_bistber1) module"]
pub type DDRPHYC_BISTBER1 = crate::Reg<u32, _DDRPHYC_BISTBER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTBER1;
#[doc = "`read()` method returns [ddrphyc_bistber1::R](ddrphyc_bistber1::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTBER1 {}
#[doc = "BISTBER1 register"]
pub mod ddrphyc_bistber1;
#[doc = "BISTBER2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistber2](ddrphyc_bistber2) module"]
pub type DDRPHYC_BISTBER2 = crate::Reg<u32, _DDRPHYC_BISTBER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTBER2;
#[doc = "`read()` method returns [ddrphyc_bistber2::R](ddrphyc_bistber2::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTBER2 {}
#[doc = "BISTBER2 register"]
pub mod ddrphyc_bistber2;
#[doc = "BISTWCSR register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistwcsr](ddrphyc_bistwcsr) module"]
pub type DDRPHYC_BISTWCSR = crate::Reg<u32, _DDRPHYC_BISTWCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTWCSR;
#[doc = "`read()` method returns [ddrphyc_bistwcsr::R](ddrphyc_bistwcsr::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTWCSR {}
#[doc = "BISTWCSR register"]
pub mod ddrphyc_bistwcsr;
#[doc = "BISTFWR0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistfwr0](ddrphyc_bistfwr0) module"]
pub type DDRPHYC_BISTFWR0 = crate::Reg<u32, _DDRPHYC_BISTFWR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTFWR0;
#[doc = "`read()` method returns [ddrphyc_bistfwr0::R](ddrphyc_bistfwr0::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTFWR0 {}
#[doc = "BISTFWR0 register"]
pub mod ddrphyc_bistfwr0;
#[doc = "BISTFWR1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_bistfwr1](ddrphyc_bistfwr1) module"]
pub type DDRPHYC_BISTFWR1 = crate::Reg<u32, _DDRPHYC_BISTFWR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_BISTFWR1;
#[doc = "`read()` method returns [ddrphyc_bistfwr1::R](ddrphyc_bistfwr1::R) reader structure"]
impl crate::Readable for DDRPHYC_BISTFWR1 {}
#[doc = "BISTFWR1 register"]
pub mod ddrphyc_bistfwr1;
#[doc = "General Purpose Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_gpr0](ddrphyc_gpr0) module"]
pub type DDRPHYC_GPR0 = crate::Reg<u32, _DDRPHYC_GPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_GPR0;
#[doc = "`read()` method returns [ddrphyc_gpr0::R](ddrphyc_gpr0::R) reader structure"]
impl crate::Readable for DDRPHYC_GPR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_gpr0::W](ddrphyc_gpr0::W) writer structure"]
impl crate::Writable for DDRPHYC_GPR0 {}
#[doc = "General Purpose Register 0"]
pub mod ddrphyc_gpr0;
#[doc = "General Purpose Register register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_gpr1](ddrphyc_gpr1) module"]
pub type DDRPHYC_GPR1 = crate::Reg<u32, _DDRPHYC_GPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_GPR1;
#[doc = "`read()` method returns [ddrphyc_gpr1::R](ddrphyc_gpr1::R) reader structure"]
impl crate::Readable for DDRPHYC_GPR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_gpr1::W](ddrphyc_gpr1::W) writer structure"]
impl crate::Writable for DDRPHYC_GPR1 {}
#[doc = "General Purpose Register register 1"]
pub mod ddrphyc_gpr1;
#[doc = "ZQ0CR0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0cr0](ddrphyc_zq0cr0) module"]
pub type DDRPHYC_ZQ0CR0 = crate::Reg<u32, _DDRPHYC_ZQ0CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ZQ0CR0;
#[doc = "`read()` method returns [ddrphyc_zq0cr0::R](ddrphyc_zq0cr0::R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0CR0 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_zq0cr0::W](ddrphyc_zq0cr0::W) writer structure"]
impl crate::Writable for DDRPHYC_ZQ0CR0 {}
#[doc = "ZQ0CR0 register"]
pub mod ddrphyc_zq0cr0;
#[doc = "ZQ0CR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0cr1](ddrphyc_zq0cr1) module"]
pub type DDRPHYC_ZQ0CR1 = crate::Reg<u8, _DDRPHYC_ZQ0CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ZQ0CR1;
#[doc = "`read()` method returns [ddrphyc_zq0cr1::R](ddrphyc_zq0cr1::R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0CR1 {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_zq0cr1::W](ddrphyc_zq0cr1::W) writer structure"]
impl crate::Writable for DDRPHYC_ZQ0CR1 {}
#[doc = "ZQ0CR1 register"]
pub mod ddrphyc_zq0cr1;
#[doc = "ZQ0SR0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0sr0](ddrphyc_zq0sr0) module"]
pub type DDRPHYC_ZQ0SR0 = crate::Reg<u32, _DDRPHYC_ZQ0SR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ZQ0SR0;
#[doc = "`read()` method returns [ddrphyc_zq0sr0::R](ddrphyc_zq0sr0::R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0SR0 {}
#[doc = "ZQ0SR0 register"]
pub mod ddrphyc_zq0sr0;
#[doc = "ZQ0SR1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_zq0sr1](ddrphyc_zq0sr1) module"]
pub type DDRPHYC_ZQ0SR1 = crate::Reg<u8, _DDRPHYC_ZQ0SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_ZQ0SR1;
#[doc = "`read()` method returns [ddrphyc_zq0sr1::R](ddrphyc_zq0sr1::R) reader structure"]
impl crate::Readable for DDRPHYC_ZQ0SR1 {}
#[doc = "ZQ0SR1 register"]
pub mod ddrphyc_zq0sr1;
#[doc = "DX 0 GCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0gcr](ddrphyc_dx0gcr) module"]
pub type DDRPHYC_DX0GCR = crate::Reg<u32, _DDRPHYC_DX0GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0GCR;
#[doc = "`read()` method returns [ddrphyc_dx0gcr::R](ddrphyc_dx0gcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0GCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx0gcr::W](ddrphyc_dx0gcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX0GCR {}
#[doc = "DX 0 GCR register"]
pub mod ddrphyc_dx0gcr;
#[doc = "DX 0 GSR0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0gsr0](ddrphyc_dx0gsr0) module"]
pub type DDRPHYC_DX0GSR0 = crate::Reg<u16, _DDRPHYC_DX0GSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0GSR0;
#[doc = "`read()` method returns [ddrphyc_dx0gsr0::R](ddrphyc_dx0gsr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0GSR0 {}
#[doc = "DX 0 GSR0 register"]
pub mod ddrphyc_dx0gsr0;
#[doc = "DX 0 GSR1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0gsr1](ddrphyc_dx0gsr1) module"]
pub type DDRPHYC_DX0GSR1 = crate::Reg<u32, _DDRPHYC_DX0GSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0GSR1;
#[doc = "`read()` method returns [ddrphyc_dx0gsr1::R](ddrphyc_dx0gsr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0GSR1 {}
#[doc = "DX 0 GSR1 register"]
pub mod ddrphyc_dx0gsr1;
#[doc = "DX 0 DLLCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0dllcr](ddrphyc_dx0dllcr) module"]
pub type DDRPHYC_DX0DLLCR = crate::Reg<u32, _DDRPHYC_DX0DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0DLLCR;
#[doc = "`read()` method returns [ddrphyc_dx0dllcr::R](ddrphyc_dx0dllcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0DLLCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx0dllcr::W](ddrphyc_dx0dllcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX0DLLCR {}
#[doc = "DX 0 DLLCR register"]
pub mod ddrphyc_dx0dllcr;
#[doc = "DX 0 DQTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0dqtr](ddrphyc_dx0dqtr) module"]
pub type DDRPHYC_DX0DQTR = crate::Reg<u32, _DDRPHYC_DX0DQTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0DQTR;
#[doc = "`read()` method returns [ddrphyc_dx0dqtr::R](ddrphyc_dx0dqtr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0DQTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx0dqtr::W](ddrphyc_dx0dqtr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX0DQTR {}
#[doc = "DX 0 DQTR register"]
pub mod ddrphyc_dx0dqtr;
#[doc = "DX 0 DQSTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx0dqstr](ddrphyc_dx0dqstr) module"]
pub type DDRPHYC_DX0DQSTR = crate::Reg<u32, _DDRPHYC_DX0DQSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX0DQSTR;
#[doc = "`read()` method returns [ddrphyc_dx0dqstr::R](ddrphyc_dx0dqstr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX0DQSTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx0dqstr::W](ddrphyc_dx0dqstr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX0DQSTR {}
#[doc = "DX 0 DQSTR register"]
pub mod ddrphyc_dx0dqstr;
#[doc = "DX 1 GCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1gcr](ddrphyc_dx1gcr) module"]
pub type DDRPHYC_DX1GCR = crate::Reg<u32, _DDRPHYC_DX1GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1GCR;
#[doc = "`read()` method returns [ddrphyc_dx1gcr::R](ddrphyc_dx1gcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1GCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx1gcr::W](ddrphyc_dx1gcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX1GCR {}
#[doc = "DX 1 GCR register"]
pub mod ddrphyc_dx1gcr;
#[doc = "DX 1 GSR0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1gsr0](ddrphyc_dx1gsr0) module"]
pub type DDRPHYC_DX1GSR0 = crate::Reg<u16, _DDRPHYC_DX1GSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1GSR0;
#[doc = "`read()` method returns [ddrphyc_dx1gsr0::R](ddrphyc_dx1gsr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1GSR0 {}
#[doc = "DX 1 GSR0 register"]
pub mod ddrphyc_dx1gsr0;
#[doc = "DX 1 GSR1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1gsr1](ddrphyc_dx1gsr1) module"]
pub type DDRPHYC_DX1GSR1 = crate::Reg<u32, _DDRPHYC_DX1GSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1GSR1;
#[doc = "`read()` method returns [ddrphyc_dx1gsr1::R](ddrphyc_dx1gsr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1GSR1 {}
#[doc = "DX 1 GSR1 register"]
pub mod ddrphyc_dx1gsr1;
#[doc = "DX 1 DLLCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1dllcr](ddrphyc_dx1dllcr) module"]
pub type DDRPHYC_DX1DLLCR = crate::Reg<u32, _DDRPHYC_DX1DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1DLLCR;
#[doc = "`read()` method returns [ddrphyc_dx1dllcr::R](ddrphyc_dx1dllcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1DLLCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx1dllcr::W](ddrphyc_dx1dllcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX1DLLCR {}
#[doc = "DX 1 DLLCR register"]
pub mod ddrphyc_dx1dllcr;
#[doc = "DX 1 DQTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1dqtr](ddrphyc_dx1dqtr) module"]
pub type DDRPHYC_DX1DQTR = crate::Reg<u32, _DDRPHYC_DX1DQTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1DQTR;
#[doc = "`read()` method returns [ddrphyc_dx1dqtr::R](ddrphyc_dx1dqtr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1DQTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx1dqtr::W](ddrphyc_dx1dqtr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX1DQTR {}
#[doc = "DX 1 DQTR register"]
pub mod ddrphyc_dx1dqtr;
#[doc = "DX 1 DQSTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx1dqstr](ddrphyc_dx1dqstr) module"]
pub type DDRPHYC_DX1DQSTR = crate::Reg<u32, _DDRPHYC_DX1DQSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX1DQSTR;
#[doc = "`read()` method returns [ddrphyc_dx1dqstr::R](ddrphyc_dx1dqstr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX1DQSTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx1dqstr::W](ddrphyc_dx1dqstr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX1DQSTR {}
#[doc = "DX 1 DQSTR register"]
pub mod ddrphyc_dx1dqstr;
#[doc = "DX 2 GCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2gcr](ddrphyc_dx2gcr) module"]
pub type DDRPHYC_DX2GCR = crate::Reg<u32, _DDRPHYC_DX2GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2GCR;
#[doc = "`read()` method returns [ddrphyc_dx2gcr::R](ddrphyc_dx2gcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2GCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx2gcr::W](ddrphyc_dx2gcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX2GCR {}
#[doc = "DX 2 GCR register"]
pub mod ddrphyc_dx2gcr;
#[doc = "DX 2 GSR0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2gsr0](ddrphyc_dx2gsr0) module"]
pub type DDRPHYC_DX2GSR0 = crate::Reg<u16, _DDRPHYC_DX2GSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2GSR0;
#[doc = "`read()` method returns [ddrphyc_dx2gsr0::R](ddrphyc_dx2gsr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2GSR0 {}
#[doc = "DX 2 GSR0 register"]
pub mod ddrphyc_dx2gsr0;
#[doc = "DX 2 GSR1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2gsr1](ddrphyc_dx2gsr1) module"]
pub type DDRPHYC_DX2GSR1 = crate::Reg<u32, _DDRPHYC_DX2GSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2GSR1;
#[doc = "`read()` method returns [ddrphyc_dx2gsr1::R](ddrphyc_dx2gsr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2GSR1 {}
#[doc = "DX 2 GSR1 register"]
pub mod ddrphyc_dx2gsr1;
#[doc = "DX 2 DLLCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2dllcr](ddrphyc_dx2dllcr) module"]
pub type DDRPHYC_DX2DLLCR = crate::Reg<u32, _DDRPHYC_DX2DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2DLLCR;
#[doc = "`read()` method returns [ddrphyc_dx2dllcr::R](ddrphyc_dx2dllcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2DLLCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx2dllcr::W](ddrphyc_dx2dllcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX2DLLCR {}
#[doc = "DX 2 DLLCR register"]
pub mod ddrphyc_dx2dllcr;
#[doc = "DX 2 DQTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2dqtr](ddrphyc_dx2dqtr) module"]
pub type DDRPHYC_DX2DQTR = crate::Reg<u32, _DDRPHYC_DX2DQTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2DQTR;
#[doc = "`read()` method returns [ddrphyc_dx2dqtr::R](ddrphyc_dx2dqtr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2DQTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx2dqtr::W](ddrphyc_dx2dqtr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX2DQTR {}
#[doc = "DX 2 DQTR register"]
pub mod ddrphyc_dx2dqtr;
#[doc = "DX 2 DQSTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx2dqstr](ddrphyc_dx2dqstr) module"]
pub type DDRPHYC_DX2DQSTR = crate::Reg<u32, _DDRPHYC_DX2DQSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX2DQSTR;
#[doc = "`read()` method returns [ddrphyc_dx2dqstr::R](ddrphyc_dx2dqstr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX2DQSTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx2dqstr::W](ddrphyc_dx2dqstr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX2DQSTR {}
#[doc = "DX 2 DQSTR register"]
pub mod ddrphyc_dx2dqstr;
#[doc = "DX 3 GCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3gcr](ddrphyc_dx3gcr) module"]
pub type DDRPHYC_DX3GCR = crate::Reg<u32, _DDRPHYC_DX3GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3GCR;
#[doc = "`read()` method returns [ddrphyc_dx3gcr::R](ddrphyc_dx3gcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3GCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx3gcr::W](ddrphyc_dx3gcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX3GCR {}
#[doc = "DX 3 GCR register"]
pub mod ddrphyc_dx3gcr;
#[doc = "DX 3 GSR0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3gsr0](ddrphyc_dx3gsr0) module"]
pub type DDRPHYC_DX3GSR0 = crate::Reg<u16, _DDRPHYC_DX3GSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3GSR0;
#[doc = "`read()` method returns [ddrphyc_dx3gsr0::R](ddrphyc_dx3gsr0::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3GSR0 {}
#[doc = "DX 3 GSR0 register"]
pub mod ddrphyc_dx3gsr0;
#[doc = "DX 3 GSR1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3gsr1](ddrphyc_dx3gsr1) module"]
pub type DDRPHYC_DX3GSR1 = crate::Reg<u32, _DDRPHYC_DX3GSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3GSR1;
#[doc = "`read()` method returns [ddrphyc_dx3gsr1::R](ddrphyc_dx3gsr1::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3GSR1 {}
#[doc = "DX 3 GSR1 register"]
pub mod ddrphyc_dx3gsr1;
#[doc = "DX 3 DLLCR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3dllcr](ddrphyc_dx3dllcr) module"]
pub type DDRPHYC_DX3DLLCR = crate::Reg<u32, _DDRPHYC_DX3DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3DLLCR;
#[doc = "`read()` method returns [ddrphyc_dx3dllcr::R](ddrphyc_dx3dllcr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3DLLCR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx3dllcr::W](ddrphyc_dx3dllcr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX3DLLCR {}
#[doc = "DX 3 DLLCR register"]
pub mod ddrphyc_dx3dllcr;
#[doc = "DX 3 DQTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3dqtr](ddrphyc_dx3dqtr) module"]
pub type DDRPHYC_DX3DQTR = crate::Reg<u32, _DDRPHYC_DX3DQTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3DQTR;
#[doc = "`read()` method returns [ddrphyc_dx3dqtr::R](ddrphyc_dx3dqtr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3DQTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx3dqtr::W](ddrphyc_dx3dqtr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX3DQTR {}
#[doc = "DX 3 DQTR register"]
pub mod ddrphyc_dx3dqtr;
#[doc = "DX 3 DQSTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3dqstr](ddrphyc_dx3dqstr) module"]
pub type DDRPHYC_DX3DQSTR = crate::Reg<u32, _DDRPHYC_DX3DQSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDRPHYC_DX3DQSTR;
#[doc = "`read()` method returns [ddrphyc_dx3dqstr::R](ddrphyc_dx3dqstr::R) reader structure"]
impl crate::Readable for DDRPHYC_DX3DQSTR {}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx3dqstr::W](ddrphyc_dx3dqstr::W) writer structure"]
impl crate::Writable for DDRPHYC_DX3DQSTR {}
#[doc = "DX 3 DQSTR register"]
pub mod ddrphyc_dx3dqstr;
