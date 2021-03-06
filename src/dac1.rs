#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC control register"]
    pub dac_cr: DAC_CR,
    #[doc = "0x04 - DAC software trigger register"]
    pub dac_swtrgr: DAC_SWTRGR,
    #[doc = "0x08 - DAC channel1 12-bit right-aligned data holding register"]
    pub dac_dhr12r1: DAC_DHR12R1,
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register"]
    pub dac_dhr12l1: DAC_DHR12L1,
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register"]
    pub dac_dhr8r1: DAC_DHR8R1,
    #[doc = "0x14 - This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation."]
    pub dac_dhr12r2: DAC_DHR12R2,
    #[doc = "0x18 - This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation."]
    pub dac_dhr12l2: DAC_DHR12L2,
    #[doc = "0x1c - This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation."]
    pub dac_dhr8r2: DAC_DHR8R2,
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    pub dac_dhr12rd: DAC_DHR12RD,
    #[doc = "0x24 - Dual DAC 12-bit left aligned data holding register"]
    pub dac_dhr12ld: DAC_DHR12LD,
    #[doc = "0x28 - Dual DAC 8-bit right aligned data holding register"]
    pub dac_dhr8rd: DAC_DHR8RD,
    #[doc = "0x2c - DAC channel1 data output register"]
    pub dac_dor1: DAC_DOR1,
    #[doc = "0x30 - This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation."]
    pub dac_dor2: DAC_DOR2,
    #[doc = "0x34 - DAC status register"]
    pub dac_sr: DAC_SR,
    #[doc = "0x38 - DAC calibration control register"]
    pub dac_ccr: DAC_CCR,
    #[doc = "0x3c - DAC mode control register"]
    pub dac_mcr: DAC_MCR,
    #[doc = "0x40 - DAC channel 1 sample and hold sample time register"]
    pub dac_shsr1: DAC_SHSR1,
    #[doc = "0x44 - This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation."]
    pub dac_shsr2: DAC_SHSR2,
    #[doc = "0x48 - DAC sample and hold time register"]
    pub dac_shhr: DAC_SHHR,
    #[doc = "0x4c - DAC sample and hold refresh time register"]
    pub dac_shrr: DAC_SHRR,
    _reserved20: [u8; 928usize],
    #[doc = "0x3f0 - DAC IP hardware configuration register"]
    pub dac_ip_hwcfgr0: DAC_IP_HWCFGR0,
    #[doc = "0x3f4 - DAC IP version register"]
    pub dac_verr: DAC_VERR,
    #[doc = "0x3f8 - DAC IP identification register"]
    pub dac_ipidr: DAC_IPIDR,
    #[doc = "0x3fc - DAC size identification register"]
    pub dac_sidr: DAC_SIDR,
}
#[doc = "DAC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_cr](dac_cr) module"]
pub type DAC_CR = crate::Reg<u32, _DAC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_CR;
#[doc = "`read()` method returns [dac_cr::R](dac_cr::R) reader structure"]
impl crate::Readable for DAC_CR {}
#[doc = "`write(|w| ..)` method takes [dac_cr::W](dac_cr::W) writer structure"]
impl crate::Writable for DAC_CR {}
#[doc = "DAC control register"]
pub mod dac_cr;
#[doc = "DAC software trigger register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_swtrgr](dac_swtrgr) module"]
pub type DAC_SWTRGR = crate::Reg<u32, _DAC_SWTRGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_SWTRGR;
#[doc = "`read()` method returns [dac_swtrgr::R](dac_swtrgr::R) reader structure"]
impl crate::Readable for DAC_SWTRGR {}
#[doc = "`write(|w| ..)` method takes [dac_swtrgr::W](dac_swtrgr::W) writer structure"]
impl crate::Writable for DAC_SWTRGR {}
#[doc = "DAC software trigger register"]
pub mod dac_swtrgr;
#[doc = "DAC channel1 12-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr12r1](dac_dhr12r1) module"]
pub type DAC_DHR12R1 = crate::Reg<u32, _DAC_DHR12R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_DHR12R1;
#[doc = "`read()` method returns [dac_dhr12r1::R](dac_dhr12r1::R) reader structure"]
impl crate::Readable for DAC_DHR12R1 {}
#[doc = "`write(|w| ..)` method takes [dac_dhr12r1::W](dac_dhr12r1::W) writer structure"]
impl crate::Writable for DAC_DHR12R1 {}
#[doc = "DAC channel1 12-bit right-aligned data holding register"]
pub mod dac_dhr12r1;
#[doc = "DAC channel1 12-bit left aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr12l1](dac_dhr12l1) module"]
pub type DAC_DHR12L1 = crate::Reg<u32, _DAC_DHR12L1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_DHR12L1;
#[doc = "`read()` method returns [dac_dhr12l1::R](dac_dhr12l1::R) reader structure"]
impl crate::Readable for DAC_DHR12L1 {}
#[doc = "`write(|w| ..)` method takes [dac_dhr12l1::W](dac_dhr12l1::W) writer structure"]
impl crate::Writable for DAC_DHR12L1 {}
#[doc = "DAC channel1 12-bit left aligned data holding register"]
pub mod dac_dhr12l1;
#[doc = "DAC channel1 8-bit right aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr8r1](dac_dhr8r1) module"]
pub type DAC_DHR8R1 = crate::Reg<u32, _DAC_DHR8R1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_DHR8R1;
#[doc = "`read()` method returns [dac_dhr8r1::R](dac_dhr8r1::R) reader structure"]
impl crate::Readable for DAC_DHR8R1 {}
#[doc = "`write(|w| ..)` method takes [dac_dhr8r1::W](dac_dhr8r1::W) writer structure"]
impl crate::Writable for DAC_DHR8R1 {}
#[doc = "DAC channel1 8-bit right aligned data holding register"]
pub mod dac_dhr8r1;
#[doc = "This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr12r2](dac_dhr12r2) module"]
pub type DAC_DHR12R2 = crate::Reg<u32, _DAC_DHR12R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_DHR12R2;
#[doc = "`read()` method returns [dac_dhr12r2::R](dac_dhr12r2::R) reader structure"]
impl crate::Readable for DAC_DHR12R2 {}
#[doc = "`write(|w| ..)` method takes [dac_dhr12r2::W](dac_dhr12r2::W) writer structure"]
impl crate::Writable for DAC_DHR12R2 {}
#[doc = "This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation."]
pub mod dac_dhr12r2;
#[doc = "This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr12l2](dac_dhr12l2) module"]
pub type DAC_DHR12L2 = crate::Reg<u32, _DAC_DHR12L2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_DHR12L2;
#[doc = "`read()` method returns [dac_dhr12l2::R](dac_dhr12l2::R) reader structure"]
impl crate::Readable for DAC_DHR12L2 {}
#[doc = "`write(|w| ..)` method takes [dac_dhr12l2::W](dac_dhr12l2::W) writer structure"]
impl crate::Writable for DAC_DHR12L2 {}
#[doc = "This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation."]
pub mod dac_dhr12l2;
#[doc = "This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr8r2](dac_dhr8r2) module"]
pub type DAC_DHR8R2 = crate::Reg<u32, _DAC_DHR8R2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_DHR8R2;
#[doc = "`read()` method returns [dac_dhr8r2::R](dac_dhr8r2::R) reader structure"]
impl crate::Readable for DAC_DHR8R2 {}
#[doc = "`write(|w| ..)` method takes [dac_dhr8r2::W](dac_dhr8r2::W) writer structure"]
impl crate::Writable for DAC_DHR8R2 {}
#[doc = "This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation."]
pub mod dac_dhr8r2;
#[doc = "Dual DAC 12-bit right-aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr12rd](dac_dhr12rd) module"]
pub type DAC_DHR12RD = crate::Reg<u32, _DAC_DHR12RD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_DHR12RD;
#[doc = "`read()` method returns [dac_dhr12rd::R](dac_dhr12rd::R) reader structure"]
impl crate::Readable for DAC_DHR12RD {}
#[doc = "`write(|w| ..)` method takes [dac_dhr12rd::W](dac_dhr12rd::W) writer structure"]
impl crate::Writable for DAC_DHR12RD {}
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dac_dhr12rd;
#[doc = "Dual DAC 12-bit left aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr12ld](dac_dhr12ld) module"]
pub type DAC_DHR12LD = crate::Reg<u32, _DAC_DHR12LD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_DHR12LD;
#[doc = "`read()` method returns [dac_dhr12ld::R](dac_dhr12ld::R) reader structure"]
impl crate::Readable for DAC_DHR12LD {}
#[doc = "`write(|w| ..)` method takes [dac_dhr12ld::W](dac_dhr12ld::W) writer structure"]
impl crate::Writable for DAC_DHR12LD {}
#[doc = "Dual DAC 12-bit left aligned data holding register"]
pub mod dac_dhr12ld;
#[doc = "Dual DAC 8-bit right aligned data holding register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dhr8rd](dac_dhr8rd) module"]
pub type DAC_DHR8RD = crate::Reg<u32, _DAC_DHR8RD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_DHR8RD;
#[doc = "`read()` method returns [dac_dhr8rd::R](dac_dhr8rd::R) reader structure"]
impl crate::Readable for DAC_DHR8RD {}
#[doc = "`write(|w| ..)` method takes [dac_dhr8rd::W](dac_dhr8rd::W) writer structure"]
impl crate::Writable for DAC_DHR8RD {}
#[doc = "Dual DAC 8-bit right aligned data holding register"]
pub mod dac_dhr8rd;
#[doc = "DAC channel1 data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dor1](dac_dor1) module"]
pub type DAC_DOR1 = crate::Reg<u32, _DAC_DOR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_DOR1;
#[doc = "`read()` method returns [dac_dor1::R](dac_dor1::R) reader structure"]
impl crate::Readable for DAC_DOR1 {}
#[doc = "DAC channel1 data output register"]
pub mod dac_dor1;
#[doc = "This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dor2](dac_dor2) module"]
pub type DAC_DOR2 = crate::Reg<u32, _DAC_DOR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_DOR2;
#[doc = "`read()` method returns [dac_dor2::R](dac_dor2::R) reader structure"]
impl crate::Readable for DAC_DOR2 {}
#[doc = "This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation."]
pub mod dac_dor2;
#[doc = "DAC status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_sr](dac_sr) module"]
pub type DAC_SR = crate::Reg<u32, _DAC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_SR;
#[doc = "`read()` method returns [dac_sr::R](dac_sr::R) reader structure"]
impl crate::Readable for DAC_SR {}
#[doc = "`write(|w| ..)` method takes [dac_sr::W](dac_sr::W) writer structure"]
impl crate::Writable for DAC_SR {}
#[doc = "DAC status register"]
pub mod dac_sr;
#[doc = "DAC calibration control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_ccr](dac_ccr) module"]
pub type DAC_CCR = crate::Reg<u32, _DAC_CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_CCR;
#[doc = "`read()` method returns [dac_ccr::R](dac_ccr::R) reader structure"]
impl crate::Readable for DAC_CCR {}
#[doc = "`write(|w| ..)` method takes [dac_ccr::W](dac_ccr::W) writer structure"]
impl crate::Writable for DAC_CCR {}
#[doc = "DAC calibration control register"]
pub mod dac_ccr;
#[doc = "DAC mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_mcr](dac_mcr) module"]
pub type DAC_MCR = crate::Reg<u32, _DAC_MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_MCR;
#[doc = "`read()` method returns [dac_mcr::R](dac_mcr::R) reader structure"]
impl crate::Readable for DAC_MCR {}
#[doc = "`write(|w| ..)` method takes [dac_mcr::W](dac_mcr::W) writer structure"]
impl crate::Writable for DAC_MCR {}
#[doc = "DAC mode control register"]
pub mod dac_mcr;
#[doc = "DAC channel 1 sample and hold sample time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_shsr1](dac_shsr1) module"]
pub type DAC_SHSR1 = crate::Reg<u32, _DAC_SHSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_SHSR1;
#[doc = "`read()` method returns [dac_shsr1::R](dac_shsr1::R) reader structure"]
impl crate::Readable for DAC_SHSR1 {}
#[doc = "`write(|w| ..)` method takes [dac_shsr1::W](dac_shsr1::W) writer structure"]
impl crate::Writable for DAC_SHSR1 {}
#[doc = "DAC channel 1 sample and hold sample time register"]
pub mod dac_shsr1;
#[doc = "This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_shsr2](dac_shsr2) module"]
pub type DAC_SHSR2 = crate::Reg<u32, _DAC_SHSR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_SHSR2;
#[doc = "`read()` method returns [dac_shsr2::R](dac_shsr2::R) reader structure"]
impl crate::Readable for DAC_SHSR2 {}
#[doc = "`write(|w| ..)` method takes [dac_shsr2::W](dac_shsr2::W) writer structure"]
impl crate::Writable for DAC_SHSR2 {}
#[doc = "This register is available only on dual-channel DACs. Refer to Section1.3: DAC implementation."]
pub mod dac_shsr2;
#[doc = "DAC sample and hold time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_shhr](dac_shhr) module"]
pub type DAC_SHHR = crate::Reg<u32, _DAC_SHHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_SHHR;
#[doc = "`read()` method returns [dac_shhr::R](dac_shhr::R) reader structure"]
impl crate::Readable for DAC_SHHR {}
#[doc = "`write(|w| ..)` method takes [dac_shhr::W](dac_shhr::W) writer structure"]
impl crate::Writable for DAC_SHHR {}
#[doc = "DAC sample and hold time register"]
pub mod dac_shhr;
#[doc = "DAC sample and hold refresh time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_shrr](dac_shrr) module"]
pub type DAC_SHRR = crate::Reg<u32, _DAC_SHRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_SHRR;
#[doc = "`read()` method returns [dac_shrr::R](dac_shrr::R) reader structure"]
impl crate::Readable for DAC_SHRR {}
#[doc = "`write(|w| ..)` method takes [dac_shrr::W](dac_shrr::W) writer structure"]
impl crate::Writable for DAC_SHRR {}
#[doc = "DAC sample and hold refresh time register"]
pub mod dac_shrr;
#[doc = "DAC IP hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_ip_hwcfgr0](dac_ip_hwcfgr0) module"]
pub type DAC_IP_HWCFGR0 = crate::Reg<u32, _DAC_IP_HWCFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_IP_HWCFGR0;
#[doc = "`read()` method returns [dac_ip_hwcfgr0::R](dac_ip_hwcfgr0::R) reader structure"]
impl crate::Readable for DAC_IP_HWCFGR0 {}
#[doc = "DAC IP hardware configuration register"]
pub mod dac_ip_hwcfgr0;
#[doc = "DAC IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_verr](dac_verr) module"]
pub type DAC_VERR = crate::Reg<u32, _DAC_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_VERR;
#[doc = "`read()` method returns [dac_verr::R](dac_verr::R) reader structure"]
impl crate::Readable for DAC_VERR {}
#[doc = "DAC IP version register"]
pub mod dac_verr;
#[doc = "DAC IP identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_ipidr](dac_ipidr) module"]
pub type DAC_IPIDR = crate::Reg<u32, _DAC_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_IPIDR;
#[doc = "`read()` method returns [dac_ipidr::R](dac_ipidr::R) reader structure"]
impl crate::Readable for DAC_IPIDR {}
#[doc = "DAC IP identification register"]
pub mod dac_ipidr;
#[doc = "DAC size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_sidr](dac_sidr) module"]
pub type DAC_SIDR = crate::Reg<u32, _DAC_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAC_SIDR;
#[doc = "`read()` method returns [dac_sidr::R](dac_sidr::R) reader structure"]
impl crate::Readable for DAC_SIDR {}
#[doc = "DAC size identification register"]
pub mod dac_sidr;
