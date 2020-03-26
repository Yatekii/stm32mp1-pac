#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    pub rtsr1: RTSR1,
    #[doc = "0x04 - EXTI falling trigger selection register"]
    pub ftsr1: FTSR1,
    #[doc = "0x08 - EXTI software interrupt event register"]
    pub swier1: SWIER1,
    #[doc = "0x0c - EXTI rising edge pending register"]
    pub rpr1: RPR1,
    #[doc = "0x10 - EXTI falling edge pending register"]
    pub fpr1: FPR1,
    #[doc = "0x14 - EXTI TrustZone enable register"]
    pub tzenr1: TZENR1,
    _reserved6: [u8; 28usize],
    #[doc = "0x34 - EXTI TrustZone enable register"]
    pub tzenr2: TZENR2,
    _reserved7: [u8; 8usize],
    #[doc = "0x40 - EXTI rising trigger selection register"]
    pub rtsr3: RTSR3,
    #[doc = "0x44 - EXTI falling trigger selection register"]
    pub ftsr3: FTSR3,
    #[doc = "0x48 - EXTI software interrupt event register"]
    pub swier3: SWIER3,
    #[doc = "0x4c - EXTI rising edge pending register"]
    pub rpr3: RPR3,
    #[doc = "0x50 - EXTI falling edge pending register"]
    pub fpr3: FPR3,
    _reserved12: [u8; 12usize],
    #[doc = "0x60 - EXTI external interrupt selection register"]
    pub exticr1: EXTICR1,
    #[doc = "0x64 - EXTI external interrupt selection register"]
    pub exticr2: EXTICR2,
    #[doc = "0x68 - EXTI external interrupt selection register"]
    pub exticr3: EXTICR3,
    #[doc = "0x6c - EXTI external interrupt selection register"]
    pub exticr4: EXTICR4,
    _reserved16: [u8; 16usize],
    #[doc = "0x80 - EXTI CPU wakeup with interrupt mask register"]
    pub imr1: IMR1,
    #[doc = "0x84 - EXTI CPUm wakeup with event mask register"]
    pub emr1: EMR1,
    _reserved18: [u8; 8usize],
    #[doc = "0x90 - EXTI CPUm wakeup with interrupt mask register"]
    pub imr2: IMR2,
    _reserved19: [u8; 12usize],
    #[doc = "0xa0 - EXTI CPUm wakeup with interrupt mask register"]
    pub imr3: IMR3,
    #[doc = "0xa4 - EXTI CPUm wakeup with event mask register"]
    pub emr3: EMR3,
    _reserved21: [u8; 24usize],
    #[doc = "0xc0 - EXTI CPU wakeup with event mask register"]
    pub c2imr1: C2IMR1,
    #[doc = "0xc4 - EXTI CPUm wakeup with event mask register"]
    pub c2emr1: C2EMR1,
    _reserved23: [u8; 8usize],
    #[doc = "0xd0 - EXTI CPUm wakeup with interrupt mask register"]
    pub c2imr2: C2IMR2,
    _reserved24: [u8; 12usize],
    #[doc = "0xe0 - EXTI CPUm wakeup with interrupt mask register"]
    pub c2imr3: C2IMR3,
    #[doc = "0xe4 - EXTI CPUm wakeup with event mask register"]
    pub c2emr3: C2EMR3,
    _reserved26: [u8; 728usize],
    #[doc = "0x3c0 - Hardware configuration registers"]
    pub hwcfgr13: HWCFGR13,
    #[doc = "0x3c4 - Hardware configuration registers"]
    pub hwcfgr12: HWCFGR12,
    #[doc = "0x3c8 - Hardware configuration registers"]
    pub hwcfgr11: HWCFGR11,
    _reserved29: [u8; 12usize],
    #[doc = "0x3d8 - Hardware configuration registers"]
    pub hwcfgr7: HWCFGR7,
    #[doc = "0x3dc - Hardware configuration registers"]
    pub hwcfgr6: HWCFGR6,
    #[doc = "0x3e0 - Hardware configuration registers"]
    pub hwcfgr5: HWCFGR5,
    #[doc = "0x3e4 - Hardware configuration registers"]
    pub hwcfgr4: HWCFGR4,
    #[doc = "0x3e8 - Hardware configuration registers"]
    pub hwcfgr3: HWCFGR3,
    #[doc = "0x3ec - Hardware configuration registers"]
    pub hwcfgr2: HWCFGR2,
    #[doc = "0x3f0 - Hardware configuration registers"]
    pub hwcfgr1: HWCFGR1,
    #[doc = "0x3f4 - AES version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - AES identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - AES size ID register"]
    pub sidr: SIDR,
}
#[doc = "EXTI rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr1](rtsr1) module"]
pub type RTSR1 = crate::Reg<u32, _RTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR1;
#[doc = "`read()` method returns [rtsr1::R](rtsr1::R) reader structure"]
impl crate::Readable for RTSR1 {}
#[doc = "`write(|w| ..)` method takes [rtsr1::W](rtsr1::W) writer structure"]
impl crate::Writable for RTSR1 {}
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr1;
#[doc = "EXTI falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr1](ftsr1) module"]
pub type FTSR1 = crate::Reg<u32, _FTSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR1;
#[doc = "`read()` method returns [ftsr1::R](ftsr1::R) reader structure"]
impl crate::Readable for FTSR1 {}
#[doc = "`write(|w| ..)` method takes [ftsr1::W](ftsr1::W) writer structure"]
impl crate::Writable for FTSR1 {}
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr1;
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier1](swier1) module"]
pub type SWIER1 = crate::Reg<u32, _SWIER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER1;
#[doc = "`read()` method returns [swier1::R](swier1::R) reader structure"]
impl crate::Readable for SWIER1 {}
#[doc = "`write(|w| ..)` method takes [swier1::W](swier1::W) writer structure"]
impl crate::Writable for SWIER1 {}
#[doc = "EXTI software interrupt event register"]
pub mod swier1;
#[doc = "EXTI rising edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr1](rpr1) module"]
pub type RPR1 = crate::Reg<u32, _RPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPR1;
#[doc = "`read()` method returns [rpr1::R](rpr1::R) reader structure"]
impl crate::Readable for RPR1 {}
#[doc = "`write(|w| ..)` method takes [rpr1::W](rpr1::W) writer structure"]
impl crate::Writable for RPR1 {}
#[doc = "EXTI rising edge pending register"]
pub mod rpr1;
#[doc = "EXTI falling edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr1](fpr1) module"]
pub type FPR1 = crate::Reg<u32, _FPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPR1;
#[doc = "`read()` method returns [fpr1::R](fpr1::R) reader structure"]
impl crate::Readable for FPR1 {}
#[doc = "`write(|w| ..)` method takes [fpr1::W](fpr1::W) writer structure"]
impl crate::Writable for FPR1 {}
#[doc = "EXTI falling edge pending register"]
pub mod fpr1;
#[doc = "EXTI TrustZone enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzenr1](tzenr1) module"]
pub type TZENR1 = crate::Reg<u32, _TZENR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZENR1;
#[doc = "`read()` method returns [tzenr1::R](tzenr1::R) reader structure"]
impl crate::Readable for TZENR1 {}
#[doc = "`write(|w| ..)` method takes [tzenr1::W](tzenr1::W) writer structure"]
impl crate::Writable for TZENR1 {}
#[doc = "EXTI TrustZone enable register"]
pub mod tzenr1;
#[doc = "EXTI TrustZone enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzenr2](tzenr2) module"]
pub type TZENR2 = crate::Reg<u32, _TZENR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TZENR2;
#[doc = "`read()` method returns [tzenr2::R](tzenr2::R) reader structure"]
impl crate::Readable for TZENR2 {}
#[doc = "`write(|w| ..)` method takes [tzenr2::W](tzenr2::W) writer structure"]
impl crate::Writable for TZENR2 {}
#[doc = "EXTI TrustZone enable register"]
pub mod tzenr2;
#[doc = "EXTI rising trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtsr3](rtsr3) module"]
pub type RTSR3 = crate::Reg<u32, _RTSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTSR3;
#[doc = "`read()` method returns [rtsr3::R](rtsr3::R) reader structure"]
impl crate::Readable for RTSR3 {}
#[doc = "`write(|w| ..)` method takes [rtsr3::W](rtsr3::W) writer structure"]
impl crate::Writable for RTSR3 {}
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr3;
#[doc = "EXTI falling trigger selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftsr3](ftsr3) module"]
pub type FTSR3 = crate::Reg<u32, _FTSR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTSR3;
#[doc = "`read()` method returns [ftsr3::R](ftsr3::R) reader structure"]
impl crate::Readable for FTSR3 {}
#[doc = "`write(|w| ..)` method takes [ftsr3::W](ftsr3::W) writer structure"]
impl crate::Writable for FTSR3 {}
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr3;
#[doc = "EXTI software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier3](swier3) module"]
pub type SWIER3 = crate::Reg<u32, _SWIER3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWIER3;
#[doc = "`read()` method returns [swier3::R](swier3::R) reader structure"]
impl crate::Readable for SWIER3 {}
#[doc = "`write(|w| ..)` method takes [swier3::W](swier3::W) writer structure"]
impl crate::Writable for SWIER3 {}
#[doc = "EXTI software interrupt event register"]
pub mod swier3;
#[doc = "EXTI rising edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr3](rpr3) module"]
pub type RPR3 = crate::Reg<u32, _RPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPR3;
#[doc = "`read()` method returns [rpr3::R](rpr3::R) reader structure"]
impl crate::Readable for RPR3 {}
#[doc = "`write(|w| ..)` method takes [rpr3::W](rpr3::W) writer structure"]
impl crate::Writable for RPR3 {}
#[doc = "EXTI rising edge pending register"]
pub mod rpr3;
#[doc = "EXTI falling edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpr3](fpr3) module"]
pub type FPR3 = crate::Reg<u32, _FPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FPR3;
#[doc = "`read()` method returns [fpr3::R](fpr3::R) reader structure"]
impl crate::Readable for FPR3 {}
#[doc = "`write(|w| ..)` method takes [fpr3::W](fpr3::W) writer structure"]
impl crate::Writable for FPR3 {}
#[doc = "EXTI falling edge pending register"]
pub mod fpr3;
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr1](exticr1) module"]
pub type EXTICR1 = crate::Reg<u32, _EXTICR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR1;
#[doc = "`read()` method returns [exticr1::R](exticr1::R) reader structure"]
impl crate::Readable for EXTICR1 {}
#[doc = "`write(|w| ..)` method takes [exticr1::W](exticr1::W) writer structure"]
impl crate::Writable for EXTICR1 {}
#[doc = "EXTI external interrupt selection register"]
pub mod exticr1;
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr2](exticr2) module"]
pub type EXTICR2 = crate::Reg<u32, _EXTICR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR2;
#[doc = "`read()` method returns [exticr2::R](exticr2::R) reader structure"]
impl crate::Readable for EXTICR2 {}
#[doc = "`write(|w| ..)` method takes [exticr2::W](exticr2::W) writer structure"]
impl crate::Writable for EXTICR2 {}
#[doc = "EXTI external interrupt selection register"]
pub mod exticr2;
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](exticr3) module"]
pub type EXTICR3 = crate::Reg<u32, _EXTICR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR3;
#[doc = "`read()` method returns [exticr3::R](exticr3::R) reader structure"]
impl crate::Readable for EXTICR3 {}
#[doc = "`write(|w| ..)` method takes [exticr3::W](exticr3::W) writer structure"]
impl crate::Writable for EXTICR3 {}
#[doc = "EXTI external interrupt selection register"]
pub mod exticr3;
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](exticr4) module"]
pub type EXTICR4 = crate::Reg<u32, _EXTICR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTICR4;
#[doc = "`read()` method returns [exticr4::R](exticr4::R) reader structure"]
impl crate::Readable for EXTICR4 {}
#[doc = "`write(|w| ..)` method takes [exticr4::W](exticr4::W) writer structure"]
impl crate::Writable for EXTICR4 {}
#[doc = "EXTI external interrupt selection register"]
pub mod exticr4;
#[doc = "EXTI CPU wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](imr1) module"]
pub type IMR1 = crate::Reg<u32, _IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR1;
#[doc = "`read()` method returns [imr1::R](imr1::R) reader structure"]
impl crate::Readable for IMR1 {}
#[doc = "`write(|w| ..)` method takes [imr1::W](imr1::W) writer structure"]
impl crate::Writable for IMR1 {}
#[doc = "EXTI CPU wakeup with interrupt mask register"]
pub mod imr1;
#[doc = "EXTI CPU wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr1](c2imr1) module"]
pub type C2IMR1 = crate::Reg<u32, _C2IMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR1;
#[doc = "`read()` method returns [c2imr1::R](c2imr1::R) reader structure"]
impl crate::Readable for C2IMR1 {}
#[doc = "`write(|w| ..)` method takes [c2imr1::W](c2imr1::W) writer structure"]
impl crate::Writable for C2IMR1 {}
#[doc = "EXTI CPU wakeup with event mask register"]
pub mod c2imr1;
#[doc = "EXTI CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr1](emr1) module"]
pub type EMR1 = crate::Reg<u32, _EMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMR1;
#[doc = "`read()` method returns [emr1::R](emr1::R) reader structure"]
impl crate::Readable for EMR1 {}
#[doc = "`write(|w| ..)` method takes [emr1::W](emr1::W) writer structure"]
impl crate::Writable for EMR1 {}
#[doc = "EXTI CPUm wakeup with event mask register"]
pub mod emr1;
#[doc = "EXTI CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr1](c2emr1) module"]
pub type C2EMR1 = crate::Reg<u32, _C2EMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2EMR1;
#[doc = "`read()` method returns [c2emr1::R](c2emr1::R) reader structure"]
impl crate::Readable for C2EMR1 {}
#[doc = "`write(|w| ..)` method takes [c2emr1::W](c2emr1::W) writer structure"]
impl crate::Writable for C2EMR1 {}
#[doc = "EXTI CPUm wakeup with event mask register"]
pub mod c2emr1;
#[doc = "EXTI CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr2](imr2) module"]
pub type IMR2 = crate::Reg<u32, _IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR2;
#[doc = "`read()` method returns [imr2::R](imr2::R) reader structure"]
impl crate::Readable for IMR2 {}
#[doc = "`write(|w| ..)` method takes [imr2::W](imr2::W) writer structure"]
impl crate::Writable for IMR2 {}
#[doc = "EXTI CPUm wakeup with interrupt mask register"]
pub mod imr2;
#[doc = "EXTI CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr2](c2imr2) module"]
pub type C2IMR2 = crate::Reg<u32, _C2IMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR2;
#[doc = "`read()` method returns [c2imr2::R](c2imr2::R) reader structure"]
impl crate::Readable for C2IMR2 {}
#[doc = "`write(|w| ..)` method takes [c2imr2::W](c2imr2::W) writer structure"]
impl crate::Writable for C2IMR2 {}
#[doc = "EXTI CPUm wakeup with interrupt mask register"]
pub mod c2imr2;
#[doc = "EXTI CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr3](imr3) module"]
pub type IMR3 = crate::Reg<u32, _IMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR3;
#[doc = "`read()` method returns [imr3::R](imr3::R) reader structure"]
impl crate::Readable for IMR3 {}
#[doc = "`write(|w| ..)` method takes [imr3::W](imr3::W) writer structure"]
impl crate::Writable for IMR3 {}
#[doc = "EXTI CPUm wakeup with interrupt mask register"]
pub mod imr3;
#[doc = "EXTI CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr3](c2imr3) module"]
pub type C2IMR3 = crate::Reg<u32, _C2IMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2IMR3;
#[doc = "`read()` method returns [c2imr3::R](c2imr3::R) reader structure"]
impl crate::Readable for C2IMR3 {}
#[doc = "`write(|w| ..)` method takes [c2imr3::W](c2imr3::W) writer structure"]
impl crate::Writable for C2IMR3 {}
#[doc = "EXTI CPUm wakeup with interrupt mask register"]
pub mod c2imr3;
#[doc = "EXTI CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr3](emr3) module"]
pub type EMR3 = crate::Reg<u32, _EMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMR3;
#[doc = "`read()` method returns [emr3::R](emr3::R) reader structure"]
impl crate::Readable for EMR3 {}
#[doc = "`write(|w| ..)` method takes [emr3::W](emr3::W) writer structure"]
impl crate::Writable for EMR3 {}
#[doc = "EXTI CPUm wakeup with event mask register"]
pub mod emr3;
#[doc = "EXTI CPUm wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2emr3](c2emr3) module"]
pub type C2EMR3 = crate::Reg<u32, _C2EMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2EMR3;
#[doc = "`read()` method returns [c2emr3::R](c2emr3::R) reader structure"]
impl crate::Readable for C2EMR3 {}
#[doc = "`write(|w| ..)` method takes [c2emr3::W](c2emr3::W) writer structure"]
impl crate::Writable for C2EMR3 {}
#[doc = "EXTI CPUm wakeup with event mask register"]
pub mod c2emr3;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr11](hwcfgr11) module"]
pub type HWCFGR11 = crate::Reg<u32, _HWCFGR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR11;
#[doc = "`read()` method returns [hwcfgr11::R](hwcfgr11::R) reader structure"]
impl crate::Readable for HWCFGR11 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr11;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr12](hwcfgr12) module"]
pub type HWCFGR12 = crate::Reg<u32, _HWCFGR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR12;
#[doc = "`read()` method returns [hwcfgr12::R](hwcfgr12::R) reader structure"]
impl crate::Readable for HWCFGR12 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr12;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr13](hwcfgr13) module"]
pub type HWCFGR13 = crate::Reg<u32, _HWCFGR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR13;
#[doc = "`read()` method returns [hwcfgr13::R](hwcfgr13::R) reader structure"]
impl crate::Readable for HWCFGR13 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr13;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr7](hwcfgr7) module"]
pub type HWCFGR7 = crate::Reg<u32, _HWCFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR7;
#[doc = "`read()` method returns [hwcfgr7::R](hwcfgr7::R) reader structure"]
impl crate::Readable for HWCFGR7 {}
#[doc = "`write(|w| ..)` method takes [hwcfgr7::W](hwcfgr7::W) writer structure"]
impl crate::Writable for HWCFGR7 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr7;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr6](hwcfgr6) module"]
pub type HWCFGR6 = crate::Reg<u32, _HWCFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR6;
#[doc = "`read()` method returns [hwcfgr6::R](hwcfgr6::R) reader structure"]
impl crate::Readable for HWCFGR6 {}
#[doc = "`write(|w| ..)` method takes [hwcfgr6::W](hwcfgr6::W) writer structure"]
impl crate::Writable for HWCFGR6 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr6;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr5](hwcfgr5) module"]
pub type HWCFGR5 = crate::Reg<u32, _HWCFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR5;
#[doc = "`read()` method returns [hwcfgr5::R](hwcfgr5::R) reader structure"]
impl crate::Readable for HWCFGR5 {}
#[doc = "`write(|w| ..)` method takes [hwcfgr5::W](hwcfgr5::W) writer structure"]
impl crate::Writable for HWCFGR5 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr5;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr4](hwcfgr4) module"]
pub type HWCFGR4 = crate::Reg<u32, _HWCFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR4;
#[doc = "`read()` method returns [hwcfgr4::R](hwcfgr4::R) reader structure"]
impl crate::Readable for HWCFGR4 {}
#[doc = "`write(|w| ..)` method takes [hwcfgr4::W](hwcfgr4::W) writer structure"]
impl crate::Writable for HWCFGR4 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr4;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr3](hwcfgr3) module"]
pub type HWCFGR3 = crate::Reg<u32, _HWCFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR3;
#[doc = "`read()` method returns [hwcfgr3::R](hwcfgr3::R) reader structure"]
impl crate::Readable for HWCFGR3 {}
#[doc = "`write(|w| ..)` method takes [hwcfgr3::W](hwcfgr3::W) writer structure"]
impl crate::Writable for HWCFGR3 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr3;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr2](hwcfgr2) module"]
pub type HWCFGR2 = crate::Reg<u32, _HWCFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR2;
#[doc = "`read()` method returns [hwcfgr2::R](hwcfgr2::R) reader structure"]
impl crate::Readable for HWCFGR2 {}
#[doc = "`write(|w| ..)` method takes [hwcfgr2::W](hwcfgr2::W) writer structure"]
impl crate::Writable for HWCFGR2 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr2;
#[doc = "Hardware configuration registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr1](hwcfgr1) module"]
pub type HWCFGR1 = crate::Reg<u32, _HWCFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR1;
#[doc = "`read()` method returns [hwcfgr1::R](hwcfgr1::R) reader structure"]
impl crate::Readable for HWCFGR1 {}
#[doc = "Hardware configuration registers"]
pub mod hwcfgr1;
#[doc = "AES version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verr](verr) module"]
pub type VERR = crate::Reg<u32, _VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERR;
#[doc = "`read()` method returns [verr::R](verr::R) reader structure"]
impl crate::Readable for VERR {}
#[doc = "AES version register"]
pub mod verr;
#[doc = "AES identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipidr](ipidr) module"]
pub type IPIDR = crate::Reg<u32, _IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPIDR;
#[doc = "`read()` method returns [ipidr::R](ipidr::R) reader structure"]
impl crate::Readable for IPIDR {}
#[doc = "AES identification register"]
pub mod ipidr;
#[doc = "AES size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidr](sidr) module"]
pub type SIDR = crate::Reg<u32, _SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIDR;
#[doc = "`read()` method returns [sidr::R](sidr::R) reader structure"]
impl crate::Readable for SIDR {}
#[doc = "AES size ID register"]
pub mod sidr;
