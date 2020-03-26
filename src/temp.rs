#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DTS_CFGR1 is the configuration register for temperature sensor 1."]
    pub temp_cfgr1: TEMP_CFGR1,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed."]
    pub temp_t0valr1: TEMP_T0VALR1,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed."]
    pub temp_rampvalr: TEMP_RAMPVALR,
    #[doc = "0x14 - DTS_ITR1 contains the threshold values for sensor 1."]
    pub temp_itr1: TEMP_ITR1,
    _reserved4: [u8; 4usize],
    #[doc = "0x1c - The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency."]
    pub temp_dr: TEMP_DR,
    #[doc = "0x20 - Temperature sensor status register"]
    pub temp_sr: TEMP_SR,
    #[doc = "0x24 - Temperature sensor interrupt enable register"]
    pub temp_itenr: TEMP_ITENR,
    #[doc = "0x28 - DTS_ICIFR is the control register for the interrupt flags."]
    pub temp_icifr: TEMP_ICIFR,
    #[doc = "0x2c - The DTS_OR contains general-purpose option bits."]
    pub temp_or: TEMP_OR,
}
#[doc = "DTS_CFGR1 is the configuration register for temperature sensor 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_cfgr1](temp_cfgr1) module"]
pub type TEMP_CFGR1 = crate::Reg<u32, _TEMP_CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP_CFGR1;
#[doc = "`read()` method returns [temp_cfgr1::R](temp_cfgr1::R) reader structure"]
impl crate::Readable for TEMP_CFGR1 {}
#[doc = "`write(|w| ..)` method takes [temp_cfgr1::W](temp_cfgr1::W) writer structure"]
impl crate::Writable for TEMP_CFGR1 {}
#[doc = "DTS_CFGR1 is the configuration register for temperature sensor 1."]
pub mod temp_cfgr1;
#[doc = "DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_t0valr1](temp_t0valr1) module"]
pub type TEMP_T0VALR1 = crate::Reg<u32, _TEMP_T0VALR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP_T0VALR1;
#[doc = "`read()` method returns [temp_t0valr1::R](temp_t0valr1::R) reader structure"]
impl crate::Readable for TEMP_T0VALR1 {}
#[doc = "DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed."]
pub mod temp_t0valr1;
#[doc = "The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_rampvalr](temp_rampvalr) module"]
pub type TEMP_RAMPVALR = crate::Reg<u32, _TEMP_RAMPVALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP_RAMPVALR;
#[doc = "`read()` method returns [temp_rampvalr::R](temp_rampvalr::R) reader structure"]
impl crate::Readable for TEMP_RAMPVALR {}
#[doc = "The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed."]
pub mod temp_rampvalr;
#[doc = "DTS_ITR1 contains the threshold values for sensor 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_itr1](temp_itr1) module"]
pub type TEMP_ITR1 = crate::Reg<u32, _TEMP_ITR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP_ITR1;
#[doc = "`read()` method returns [temp_itr1::R](temp_itr1::R) reader structure"]
impl crate::Readable for TEMP_ITR1 {}
#[doc = "`write(|w| ..)` method takes [temp_itr1::W](temp_itr1::W) writer structure"]
impl crate::Writable for TEMP_ITR1 {}
#[doc = "DTS_ITR1 contains the threshold values for sensor 1."]
pub mod temp_itr1;
#[doc = "The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_dr](temp_dr) module"]
pub type TEMP_DR = crate::Reg<u32, _TEMP_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP_DR;
#[doc = "`read()` method returns [temp_dr::R](temp_dr::R) reader structure"]
impl crate::Readable for TEMP_DR {}
#[doc = "`write(|w| ..)` method takes [temp_dr::W](temp_dr::W) writer structure"]
impl crate::Writable for TEMP_DR {}
#[doc = "The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency."]
pub mod temp_dr;
#[doc = "Temperature sensor status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_sr](temp_sr) module"]
pub type TEMP_SR = crate::Reg<u32, _TEMP_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP_SR;
#[doc = "`read()` method returns [temp_sr::R](temp_sr::R) reader structure"]
impl crate::Readable for TEMP_SR {}
#[doc = "Temperature sensor status register"]
pub mod temp_sr;
#[doc = "Temperature sensor interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_itenr](temp_itenr) module"]
pub type TEMP_ITENR = crate::Reg<u32, _TEMP_ITENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP_ITENR;
#[doc = "`read()` method returns [temp_itenr::R](temp_itenr::R) reader structure"]
impl crate::Readable for TEMP_ITENR {}
#[doc = "`write(|w| ..)` method takes [temp_itenr::W](temp_itenr::W) writer structure"]
impl crate::Writable for TEMP_ITENR {}
#[doc = "Temperature sensor interrupt enable register"]
pub mod temp_itenr;
#[doc = "DTS_ICIFR is the control register for the interrupt flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_icifr](temp_icifr) module"]
pub type TEMP_ICIFR = crate::Reg<u32, _TEMP_ICIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP_ICIFR;
#[doc = "`read()` method returns [temp_icifr::R](temp_icifr::R) reader structure"]
impl crate::Readable for TEMP_ICIFR {}
#[doc = "`write(|w| ..)` method takes [temp_icifr::W](temp_icifr::W) writer structure"]
impl crate::Writable for TEMP_ICIFR {}
#[doc = "DTS_ICIFR is the control register for the interrupt flags."]
pub mod temp_icifr;
#[doc = "The DTS_OR contains general-purpose option bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp_or](temp_or) module"]
pub type TEMP_OR = crate::Reg<u32, _TEMP_OR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP_OR;
#[doc = "`read()` method returns [temp_or::R](temp_or::R) reader structure"]
impl crate::Readable for TEMP_OR {}
#[doc = "`write(|w| ..)` method takes [temp_or::W](temp_or::W) writer structure"]
impl crate::Writable for TEMP_OR {}
#[doc = "The DTS_OR contains general-purpose option bits."]
pub mod temp_or;
