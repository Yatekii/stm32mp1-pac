#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM1/TIM8 control register 1"]
    pub timx_cr1: TIMX_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM1/TIM8 control register 2"]
    pub timx_cr2: TIMX_CR2,
    #[doc = "0x08 - TIM1/TIM8 slave mode control register"]
    pub timx_smcr: TIMX_SMCR,
    #[doc = "0x0c - TIM1/TIM8 DMA/interrupt enable register"]
    pub timx_dier: TIMX_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIM1/TIM8 status register"]
    pub timx_sr: TIMX_SR,
    #[doc = "0x14 - TIM1/TIM8 event generation register"]
    pub timx_egr: TIMX_EGR,
    _reserved_6_timx_ccmr1: [u8; 4usize],
    _reserved_7_timx_ccmr2: [u8; 4usize],
    #[doc = "0x20 - capture/compare enable register"]
    pub timx_ccer: TIMX_CCER,
    #[doc = "0x24 - TIM1/TIM8 counter"]
    pub timx_cnt: TIMX_CNT,
    #[doc = "0x28 - TIM1/TIM8 prescaler"]
    pub timx_psc: TIMX_PSC,
    _reserved11: [u8; 2usize],
    #[doc = "0x2c - TIM1/TIM8 auto-reload register"]
    pub timx_arr: TIMX_ARR,
    _reserved12: [u8; 2usize],
    #[doc = "0x30 - TIM1/TIM8 repetition counter register"]
    pub timx_rcr: TIMX_RCR,
    _reserved13: [u8; 2usize],
    #[doc = "0x34 - TIM1/TIM8 capture/compare register 1"]
    pub timx_ccr1: TIMX_CCR1,
    _reserved14: [u8; 2usize],
    #[doc = "0x38 - TIM1/TIM8 capture/compare register 2"]
    pub timx_ccr2: TIMX_CCR2,
    _reserved15: [u8; 2usize],
    #[doc = "0x3c - TIM1/TIM8 capture/compare register 3"]
    pub timx_ccr3: TIMX_CCR3,
    _reserved16: [u8; 2usize],
    #[doc = "0x40 - TIM1/TIM8 capture/compare register 4"]
    pub timx_ccr4: TIMX_CCR4,
    _reserved17: [u8; 2usize],
    #[doc = "0x44 - As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
    pub timx_bdtr: TIMX_BDTR,
    #[doc = "0x48 - TIM1/TIM8 DMA control register"]
    pub timx_dcr: TIMX_DCR,
    _reserved19: [u8; 2usize],
    #[doc = "0x4c - DMA address for full transfer"]
    pub timx_dmar: TIMX_DMAR,
    _reserved20: [u8; 4usize],
    #[doc = "0x54 - capture/compare mode register 3 (output mode)"]
    pub timx_ccmr3: TIMX_CCMR3,
    #[doc = "0x58 - TIM1/TIM8 capture/compare register 5"]
    pub timx_ccr5: TIMX_CCR5,
    #[doc = "0x5c - TIM1/TIM8 capture/compare register 6"]
    pub timx_ccr6: TIMX_CCR6,
    _reserved23: [u8; 2usize],
    #[doc = "0x60 - DMA address for full transfer"]
    pub timx_af1: TIMX_AF1,
    #[doc = "0x64 - DMA address for full transfer"]
    pub timx_af2: TIMX_AF2,
    #[doc = "0x68 - TIM1 timer input selection register"]
    pub timx_tisel: TIMX_TISEL,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn timx_ccmr1_input(&self) -> &TIMX_CCMR1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const TIMX_CCMR1_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn timx_ccmr1_input_mut(&self) -> &mut TIMX_CCMR1_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut TIMX_CCMR1_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn timx_ccmr1_output(&self) -> &TIMX_CCMR1_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const TIMX_CCMR1_OUTPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn timx_ccmr1_output_mut(&self) -> &mut TIMX_CCMR1_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(24usize) as *mut TIMX_CCMR1_OUTPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn timx_ccmr2_input(&self) -> &TIMX_CCMR2_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const TIMX_CCMR2_INPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn timx_ccmr2_input_mut(&self) -> &mut TIMX_CCMR2_INPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut TIMX_CCMR2_INPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn timx_ccmr2_output(&self) -> &TIMX_CCMR2_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const TIMX_CCMR2_OUTPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn timx_ccmr2_output_mut(&self) -> &mut TIMX_CCMR2_OUTPUT {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(28usize) as *mut TIMX_CCMR2_OUTPUT) }
    }
}
#[doc = "TIM1/TIM8 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cr1](timx_cr1) module"]
pub type TIMX_CR1 = crate::Reg<u16, _TIMX_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CR1;
#[doc = "`read()` method returns [timx_cr1::R](timx_cr1::R) reader structure"]
impl crate::Readable for TIMX_CR1 {}
#[doc = "`write(|w| ..)` method takes [timx_cr1::W](timx_cr1::W) writer structure"]
impl crate::Writable for TIMX_CR1 {}
#[doc = "TIM1/TIM8 control register 1"]
pub mod timx_cr1;
#[doc = "TIM1/TIM8 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cr2](timx_cr2) module"]
pub type TIMX_CR2 = crate::Reg<u32, _TIMX_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CR2;
#[doc = "`read()` method returns [timx_cr2::R](timx_cr2::R) reader structure"]
impl crate::Readable for TIMX_CR2 {}
#[doc = "`write(|w| ..)` method takes [timx_cr2::W](timx_cr2::W) writer structure"]
impl crate::Writable for TIMX_CR2 {}
#[doc = "TIM1/TIM8 control register 2"]
pub mod timx_cr2;
#[doc = "TIM1/TIM8 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_smcr](timx_smcr) module"]
pub type TIMX_SMCR = crate::Reg<u32, _TIMX_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_SMCR;
#[doc = "`read()` method returns [timx_smcr::R](timx_smcr::R) reader structure"]
impl crate::Readable for TIMX_SMCR {}
#[doc = "`write(|w| ..)` method takes [timx_smcr::W](timx_smcr::W) writer structure"]
impl crate::Writable for TIMX_SMCR {}
#[doc = "TIM1/TIM8 slave mode control register"]
pub mod timx_smcr;
#[doc = "TIM1/TIM8 DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dier](timx_dier) module"]
pub type TIMX_DIER = crate::Reg<u16, _TIMX_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_DIER;
#[doc = "`read()` method returns [timx_dier::R](timx_dier::R) reader structure"]
impl crate::Readable for TIMX_DIER {}
#[doc = "`write(|w| ..)` method takes [timx_dier::W](timx_dier::W) writer structure"]
impl crate::Writable for TIMX_DIER {}
#[doc = "TIM1/TIM8 DMA/interrupt enable register"]
pub mod timx_dier;
#[doc = "TIM1/TIM8 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_sr](timx_sr) module"]
pub type TIMX_SR = crate::Reg<u32, _TIMX_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_SR;
#[doc = "`read()` method returns [timx_sr::R](timx_sr::R) reader structure"]
impl crate::Readable for TIMX_SR {}
#[doc = "`write(|w| ..)` method takes [timx_sr::W](timx_sr::W) writer structure"]
impl crate::Writable for TIMX_SR {}
#[doc = "TIM1/TIM8 status register"]
pub mod timx_sr;
#[doc = "TIM1/TIM8 event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_egr](timx_egr) module"]
pub type TIMX_EGR = crate::Reg<u32, _TIMX_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_EGR;
#[doc = "`write(|w| ..)` method takes [timx_egr::W](timx_egr::W) writer structure"]
impl crate::Writable for TIMX_EGR {}
#[doc = "TIM1/TIM8 event generation register"]
pub mod timx_egr;
#[doc = "capture/compare mode register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmr1_output](timx_ccmr1_output) module"]
pub type TIMX_CCMR1_OUTPUT = crate::Reg<u32, _TIMX_CCMR1_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCMR1_OUTPUT;
#[doc = "`read()` method returns [timx_ccmr1_output::R](timx_ccmr1_output::R) reader structure"]
impl crate::Readable for TIMX_CCMR1_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [timx_ccmr1_output::W](timx_ccmr1_output::W) writer structure"]
impl crate::Writable for TIMX_CCMR1_OUTPUT {}
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod timx_ccmr1_output;
#[doc = "capture/compare mode register 1 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmr1_input](timx_ccmr1_input) module"]
pub type TIMX_CCMR1_INPUT = crate::Reg<u32, _TIMX_CCMR1_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCMR1_INPUT;
#[doc = "`read()` method returns [timx_ccmr1_input::R](timx_ccmr1_input::R) reader structure"]
impl crate::Readable for TIMX_CCMR1_INPUT {}
#[doc = "`write(|w| ..)` method takes [timx_ccmr1_input::W](timx_ccmr1_input::W) writer structure"]
impl crate::Writable for TIMX_CCMR1_INPUT {}
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod timx_ccmr1_input;
#[doc = "capture/compare mode register 2 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmr2_output](timx_ccmr2_output) module"]
pub type TIMX_CCMR2_OUTPUT = crate::Reg<u32, _TIMX_CCMR2_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCMR2_OUTPUT;
#[doc = "`read()` method returns [timx_ccmr2_output::R](timx_ccmr2_output::R) reader structure"]
impl crate::Readable for TIMX_CCMR2_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [timx_ccmr2_output::W](timx_ccmr2_output::W) writer structure"]
impl crate::Writable for TIMX_CCMR2_OUTPUT {}
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod timx_ccmr2_output;
#[doc = "capture/compare mode register 2 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmr2_input](timx_ccmr2_input) module"]
pub type TIMX_CCMR2_INPUT = crate::Reg<u32, _TIMX_CCMR2_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCMR2_INPUT;
#[doc = "`read()` method returns [timx_ccmr2_input::R](timx_ccmr2_input::R) reader structure"]
impl crate::Readable for TIMX_CCMR2_INPUT {}
#[doc = "`write(|w| ..)` method takes [timx_ccmr2_input::W](timx_ccmr2_input::W) writer structure"]
impl crate::Writable for TIMX_CCMR2_INPUT {}
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod timx_ccmr2_input;
#[doc = "capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccer](timx_ccer) module"]
pub type TIMX_CCER = crate::Reg<u32, _TIMX_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCER;
#[doc = "`read()` method returns [timx_ccer::R](timx_ccer::R) reader structure"]
impl crate::Readable for TIMX_CCER {}
#[doc = "`write(|w| ..)` method takes [timx_ccer::W](timx_ccer::W) writer structure"]
impl crate::Writable for TIMX_CCER {}
#[doc = "capture/compare enable register"]
pub mod timx_ccer;
#[doc = "TIM1/TIM8 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cnt](timx_cnt) module"]
pub type TIMX_CNT = crate::Reg<u32, _TIMX_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CNT;
#[doc = "`read()` method returns [timx_cnt::R](timx_cnt::R) reader structure"]
impl crate::Readable for TIMX_CNT {}
#[doc = "`write(|w| ..)` method takes [timx_cnt::W](timx_cnt::W) writer structure"]
impl crate::Writable for TIMX_CNT {}
#[doc = "TIM1/TIM8 counter"]
pub mod timx_cnt;
#[doc = "TIM1/TIM8 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_psc](timx_psc) module"]
pub type TIMX_PSC = crate::Reg<u16, _TIMX_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_PSC;
#[doc = "`read()` method returns [timx_psc::R](timx_psc::R) reader structure"]
impl crate::Readable for TIMX_PSC {}
#[doc = "`write(|w| ..)` method takes [timx_psc::W](timx_psc::W) writer structure"]
impl crate::Writable for TIMX_PSC {}
#[doc = "TIM1/TIM8 prescaler"]
pub mod timx_psc;
#[doc = "TIM1/TIM8 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_arr](timx_arr) module"]
pub type TIMX_ARR = crate::Reg<u16, _TIMX_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_ARR;
#[doc = "`read()` method returns [timx_arr::R](timx_arr::R) reader structure"]
impl crate::Readable for TIMX_ARR {}
#[doc = "`write(|w| ..)` method takes [timx_arr::W](timx_arr::W) writer structure"]
impl crate::Writable for TIMX_ARR {}
#[doc = "TIM1/TIM8 auto-reload register"]
pub mod timx_arr;
#[doc = "TIM1/TIM8 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_rcr](timx_rcr) module"]
pub type TIMX_RCR = crate::Reg<u16, _TIMX_RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_RCR;
#[doc = "`read()` method returns [timx_rcr::R](timx_rcr::R) reader structure"]
impl crate::Readable for TIMX_RCR {}
#[doc = "`write(|w| ..)` method takes [timx_rcr::W](timx_rcr::W) writer structure"]
impl crate::Writable for TIMX_RCR {}
#[doc = "TIM1/TIM8 repetition counter register"]
pub mod timx_rcr;
#[doc = "TIM1/TIM8 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr1](timx_ccr1) module"]
pub type TIMX_CCR1 = crate::Reg<u16, _TIMX_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR1;
#[doc = "`read()` method returns [timx_ccr1::R](timx_ccr1::R) reader structure"]
impl crate::Readable for TIMX_CCR1 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr1::W](timx_ccr1::W) writer structure"]
impl crate::Writable for TIMX_CCR1 {}
#[doc = "TIM1/TIM8 capture/compare register 1"]
pub mod timx_ccr1;
#[doc = "TIM1/TIM8 capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr2](timx_ccr2) module"]
pub type TIMX_CCR2 = crate::Reg<u16, _TIMX_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR2;
#[doc = "`read()` method returns [timx_ccr2::R](timx_ccr2::R) reader structure"]
impl crate::Readable for TIMX_CCR2 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr2::W](timx_ccr2::W) writer structure"]
impl crate::Writable for TIMX_CCR2 {}
#[doc = "TIM1/TIM8 capture/compare register 2"]
pub mod timx_ccr2;
#[doc = "TIM1/TIM8 capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr3](timx_ccr3) module"]
pub type TIMX_CCR3 = crate::Reg<u16, _TIMX_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR3;
#[doc = "`read()` method returns [timx_ccr3::R](timx_ccr3::R) reader structure"]
impl crate::Readable for TIMX_CCR3 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr3::W](timx_ccr3::W) writer structure"]
impl crate::Writable for TIMX_CCR3 {}
#[doc = "TIM1/TIM8 capture/compare register 3"]
pub mod timx_ccr3;
#[doc = "TIM1/TIM8 capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr4](timx_ccr4) module"]
pub type TIMX_CCR4 = crate::Reg<u16, _TIMX_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR4;
#[doc = "`read()` method returns [timx_ccr4::R](timx_ccr4::R) reader structure"]
impl crate::Readable for TIMX_CCR4 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr4::W](timx_ccr4::W) writer structure"]
impl crate::Writable for TIMX_CCR4 {}
#[doc = "TIM1/TIM8 capture/compare register 4"]
pub mod timx_ccr4;
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_bdtr](timx_bdtr) module"]
pub type TIMX_BDTR = crate::Reg<u32, _TIMX_BDTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_BDTR;
#[doc = "`read()` method returns [timx_bdtr::R](timx_bdtr::R) reader structure"]
impl crate::Readable for TIMX_BDTR {}
#[doc = "`write(|w| ..)` method takes [timx_bdtr::W](timx_bdtr::W) writer structure"]
impl crate::Writable for TIMX_BDTR {}
#[doc = "As the bits BK2BID, BKBID, BK2DSRM, BKDSRM, BK2P, BK2E, BK2F\\[3:0\\], BKF\\[3:0\\], AOE, BKP, BKE, OSSI, OSSR and DTG\\[7:0\\]
can be write-locked depending on the LOCK configuration, it can be necessary to configure all of them during the first write access to the TIMx_BDTR register."]
pub mod timx_bdtr;
#[doc = "TIM1/TIM8 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dcr](timx_dcr) module"]
pub type TIMX_DCR = crate::Reg<u16, _TIMX_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_DCR;
#[doc = "`read()` method returns [timx_dcr::R](timx_dcr::R) reader structure"]
impl crate::Readable for TIMX_DCR {}
#[doc = "`write(|w| ..)` method takes [timx_dcr::W](timx_dcr::W) writer structure"]
impl crate::Writable for TIMX_DCR {}
#[doc = "TIM1/TIM8 DMA control register"]
pub mod timx_dcr;
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dmar](timx_dmar) module"]
pub type TIMX_DMAR = crate::Reg<u32, _TIMX_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_DMAR;
#[doc = "`read()` method returns [timx_dmar::R](timx_dmar::R) reader structure"]
impl crate::Readable for TIMX_DMAR {}
#[doc = "`write(|w| ..)` method takes [timx_dmar::W](timx_dmar::W) writer structure"]
impl crate::Writable for TIMX_DMAR {}
#[doc = "DMA address for full transfer"]
pub mod timx_dmar;
#[doc = "capture/compare mode register 3 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmr3](timx_ccmr3) module"]
pub type TIMX_CCMR3 = crate::Reg<u32, _TIMX_CCMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCMR3;
#[doc = "`read()` method returns [timx_ccmr3::R](timx_ccmr3::R) reader structure"]
impl crate::Readable for TIMX_CCMR3 {}
#[doc = "`write(|w| ..)` method takes [timx_ccmr3::W](timx_ccmr3::W) writer structure"]
impl crate::Writable for TIMX_CCMR3 {}
#[doc = "capture/compare mode register 3 (output mode)"]
pub mod timx_ccmr3;
#[doc = "TIM1/TIM8 capture/compare register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr5](timx_ccr5) module"]
pub type TIMX_CCR5 = crate::Reg<u32, _TIMX_CCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR5;
#[doc = "`read()` method returns [timx_ccr5::R](timx_ccr5::R) reader structure"]
impl crate::Readable for TIMX_CCR5 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr5::W](timx_ccr5::W) writer structure"]
impl crate::Writable for TIMX_CCR5 {}
#[doc = "TIM1/TIM8 capture/compare register 5"]
pub mod timx_ccr5;
#[doc = "TIM1/TIM8 capture/compare register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr6](timx_ccr6) module"]
pub type TIMX_CCR6 = crate::Reg<u16, _TIMX_CCR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR6;
#[doc = "`read()` method returns [timx_ccr6::R](timx_ccr6::R) reader structure"]
impl crate::Readable for TIMX_CCR6 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr6::W](timx_ccr6::W) writer structure"]
impl crate::Writable for TIMX_CCR6 {}
#[doc = "TIM1/TIM8 capture/compare register 6"]
pub mod timx_ccr6;
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_af1](timx_af1) module"]
pub type TIMX_AF1 = crate::Reg<u32, _TIMX_AF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_AF1;
#[doc = "`read()` method returns [timx_af1::R](timx_af1::R) reader structure"]
impl crate::Readable for TIMX_AF1 {}
#[doc = "`write(|w| ..)` method takes [timx_af1::W](timx_af1::W) writer structure"]
impl crate::Writable for TIMX_AF1 {}
#[doc = "DMA address for full transfer"]
pub mod timx_af1;
#[doc = "DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_af2](timx_af2) module"]
pub type TIMX_AF2 = crate::Reg<u32, _TIMX_AF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_AF2;
#[doc = "`read()` method returns [timx_af2::R](timx_af2::R) reader structure"]
impl crate::Readable for TIMX_AF2 {}
#[doc = "`write(|w| ..)` method takes [timx_af2::W](timx_af2::W) writer structure"]
impl crate::Writable for TIMX_AF2 {}
#[doc = "DMA address for full transfer"]
pub mod timx_af2;
#[doc = "TIM1 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_tisel](timx_tisel) module"]
pub type TIMX_TISEL = crate::Reg<u32, _TIMX_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_TISEL;
#[doc = "`read()` method returns [timx_tisel::R](timx_tisel::R) reader structure"]
impl crate::Readable for TIMX_TISEL {}
#[doc = "`write(|w| ..)` method takes [timx_tisel::W](timx_tisel::W) writer structure"]
impl crate::Writable for TIMX_TISEL {}
#[doc = "TIM1 timer input selection register"]
pub mod timx_tisel;
