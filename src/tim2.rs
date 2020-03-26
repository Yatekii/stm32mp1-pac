#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIMx control register 1"]
    pub timx_cr1: TIMX_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIMx control register 2"]
    pub timx_cr2: TIMX_CR2,
    _reserved2: [u8; 2usize],
    #[doc = "0x08 - slave mode control register"]
    pub timx_smcr: TIMX_SMCR,
    #[doc = "0x0c - TIMx DMA/Interrupt enable register"]
    pub timx_dier: TIMX_DIER,
    _reserved4: [u8; 2usize],
    #[doc = "0x10 - TIMx status register"]
    pub timx_sr: TIMX_SR,
    _reserved5: [u8; 2usize],
    #[doc = "0x14 - event generation register"]
    pub timx_egr: TIMX_EGR,
    _reserved_6_timx_ccmr1: [u8; 4usize],
    _reserved_7_timx_ccmr2: [u8; 4usize],
    #[doc = "0x20 - TIMx capture/compare enable register"]
    pub timx_ccer: TIMX_CCER,
    _reserved9: [u8; 2usize],
    #[doc = "0x24 - counter"]
    pub timx_cnt: TIMX_CNT,
    #[doc = "0x28 - TIMx prescaler"]
    pub timx_psc: TIMX_PSC,
    _reserved11: [u8; 2usize],
    #[doc = "0x2c - auto-reload register"]
    pub timx_arr: TIMX_ARR,
    _reserved12: [u8; 4usize],
    #[doc = "0x34 - capture/compare register 1"]
    pub timx_ccr1: TIMX_CCR1,
    #[doc = "0x38 - capture/compare register 2"]
    pub timx_ccr2: TIMX_CCR2,
    #[doc = "0x3c - capture/compare register 3"]
    pub timx_ccr3: TIMX_CCR3,
    #[doc = "0x40 - capture/compare register 4"]
    pub timx_ccr4: TIMX_CCR4,
    _reserved16: [u8; 4usize],
    #[doc = "0x48 - TIMx DMA control register"]
    pub timx_dcr: TIMX_DCR,
    _reserved17: [u8; 2usize],
    #[doc = "0x4c - TIMx DMA address for full transfer"]
    pub timx_dmar: TIMX_DMAR,
    _reserved18: [u8; 18usize],
    #[doc = "0x60 - TIM alternate function option register 1"]
    pub timx_af1: TIMX_AF1,
    _reserved19: [u8; 4usize],
    #[doc = "0x68 - TIMx timer input selection register"]
    pub timx_tisel: TIMX_TISEL,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub fn timx_ccmr1_input(&self) -> &TIMX_CCMR1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const TIMX_CCMR1_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
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
    #[doc = "0x1c - capture/compare mode register 2 (input mode)"]
    #[inline(always)]
    pub fn timx_ccmr2_input(&self) -> &TIMX_CCMR2_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const TIMX_CCMR2_INPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (input mode)"]
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
#[doc = "TIMx control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cr1](timx_cr1) module"]
pub type TIMX_CR1 = crate::Reg<u16, _TIMX_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CR1;
#[doc = "`read()` method returns [timx_cr1::R](timx_cr1::R) reader structure"]
impl crate::Readable for TIMX_CR1 {}
#[doc = "`write(|w| ..)` method takes [timx_cr1::W](timx_cr1::W) writer structure"]
impl crate::Writable for TIMX_CR1 {}
#[doc = "TIMx control register 1"]
pub mod timx_cr1;
#[doc = "TIMx control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cr2](timx_cr2) module"]
pub type TIMX_CR2 = crate::Reg<u16, _TIMX_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CR2;
#[doc = "`read()` method returns [timx_cr2::R](timx_cr2::R) reader structure"]
impl crate::Readable for TIMX_CR2 {}
#[doc = "`write(|w| ..)` method takes [timx_cr2::W](timx_cr2::W) writer structure"]
impl crate::Writable for TIMX_CR2 {}
#[doc = "TIMx control register 2"]
pub mod timx_cr2;
#[doc = "slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_smcr](timx_smcr) module"]
pub type TIMX_SMCR = crate::Reg<u32, _TIMX_SMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_SMCR;
#[doc = "`read()` method returns [timx_smcr::R](timx_smcr::R) reader structure"]
impl crate::Readable for TIMX_SMCR {}
#[doc = "`write(|w| ..)` method takes [timx_smcr::W](timx_smcr::W) writer structure"]
impl crate::Writable for TIMX_SMCR {}
#[doc = "slave mode control register"]
pub mod timx_smcr;
#[doc = "TIMx DMA/Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dier](timx_dier) module"]
pub type TIMX_DIER = crate::Reg<u16, _TIMX_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_DIER;
#[doc = "`read()` method returns [timx_dier::R](timx_dier::R) reader structure"]
impl crate::Readable for TIMX_DIER {}
#[doc = "`write(|w| ..)` method takes [timx_dier::W](timx_dier::W) writer structure"]
impl crate::Writable for TIMX_DIER {}
#[doc = "TIMx DMA/Interrupt enable register"]
pub mod timx_dier;
#[doc = "TIMx status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_sr](timx_sr) module"]
pub type TIMX_SR = crate::Reg<u16, _TIMX_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_SR;
#[doc = "`read()` method returns [timx_sr::R](timx_sr::R) reader structure"]
impl crate::Readable for TIMX_SR {}
#[doc = "`write(|w| ..)` method takes [timx_sr::W](timx_sr::W) writer structure"]
impl crate::Writable for TIMX_SR {}
#[doc = "TIMx status register"]
pub mod timx_sr;
#[doc = "event generation register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_egr](timx_egr) module"]
pub type TIMX_EGR = crate::Reg<u32, _TIMX_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_EGR;
#[doc = "`write(|w| ..)` method takes [timx_egr::W](timx_egr::W) writer structure"]
impl crate::Writable for TIMX_EGR {}
#[doc = "event generation register"]
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
#[doc = "capture/compare mode register 1 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmr1_input](timx_ccmr1_input) module"]
pub type TIMX_CCMR1_INPUT = crate::Reg<u32, _TIMX_CCMR1_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCMR1_INPUT;
#[doc = "`read()` method returns [timx_ccmr1_input::R](timx_ccmr1_input::R) reader structure"]
impl crate::Readable for TIMX_CCMR1_INPUT {}
#[doc = "`write(|w| ..)` method takes [timx_ccmr1_input::W](timx_ccmr1_input::W) writer structure"]
impl crate::Writable for TIMX_CCMR1_INPUT {}
#[doc = "capture/compare mode register 1 (input mode)"]
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
#[doc = "capture/compare mode register 2 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccmr2_input](timx_ccmr2_input) module"]
pub type TIMX_CCMR2_INPUT = crate::Reg<u32, _TIMX_CCMR2_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCMR2_INPUT;
#[doc = "`read()` method returns [timx_ccmr2_input::R](timx_ccmr2_input::R) reader structure"]
impl crate::Readable for TIMX_CCMR2_INPUT {}
#[doc = "`write(|w| ..)` method takes [timx_ccmr2_input::W](timx_ccmr2_input::W) writer structure"]
impl crate::Writable for TIMX_CCMR2_INPUT {}
#[doc = "capture/compare mode register 2 (input mode)"]
pub mod timx_ccmr2_input;
#[doc = "TIMx capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccer](timx_ccer) module"]
pub type TIMX_CCER = crate::Reg<u16, _TIMX_CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCER;
#[doc = "`read()` method returns [timx_ccer::R](timx_ccer::R) reader structure"]
impl crate::Readable for TIMX_CCER {}
#[doc = "`write(|w| ..)` method takes [timx_ccer::W](timx_ccer::W) writer structure"]
impl crate::Writable for TIMX_CCER {}
#[doc = "TIMx capture/compare enable register"]
pub mod timx_ccer;
#[doc = "counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cnt](timx_cnt) module"]
pub type TIMX_CNT = crate::Reg<u32, _TIMX_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CNT;
#[doc = "`read()` method returns [timx_cnt::R](timx_cnt::R) reader structure"]
impl crate::Readable for TIMX_CNT {}
#[doc = "`write(|w| ..)` method takes [timx_cnt::W](timx_cnt::W) writer structure"]
impl crate::Writable for TIMX_CNT {}
#[doc = "counter"]
pub mod timx_cnt;
#[doc = "TIMx prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_psc](timx_psc) module"]
pub type TIMX_PSC = crate::Reg<u16, _TIMX_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_PSC;
#[doc = "`read()` method returns [timx_psc::R](timx_psc::R) reader structure"]
impl crate::Readable for TIMX_PSC {}
#[doc = "`write(|w| ..)` method takes [timx_psc::W](timx_psc::W) writer structure"]
impl crate::Writable for TIMX_PSC {}
#[doc = "TIMx prescaler"]
pub mod timx_psc;
#[doc = "auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_arr](timx_arr) module"]
pub type TIMX_ARR = crate::Reg<u32, _TIMX_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_ARR;
#[doc = "`read()` method returns [timx_arr::R](timx_arr::R) reader structure"]
impl crate::Readable for TIMX_ARR {}
#[doc = "`write(|w| ..)` method takes [timx_arr::W](timx_arr::W) writer structure"]
impl crate::Writable for TIMX_ARR {}
#[doc = "auto-reload register"]
pub mod timx_arr;
#[doc = "capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr1](timx_ccr1) module"]
pub type TIMX_CCR1 = crate::Reg<u32, _TIMX_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR1;
#[doc = "`read()` method returns [timx_ccr1::R](timx_ccr1::R) reader structure"]
impl crate::Readable for TIMX_CCR1 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr1::W](timx_ccr1::W) writer structure"]
impl crate::Writable for TIMX_CCR1 {}
#[doc = "capture/compare register 1"]
pub mod timx_ccr1;
#[doc = "capture/compare register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr2](timx_ccr2) module"]
pub type TIMX_CCR2 = crate::Reg<u32, _TIMX_CCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR2;
#[doc = "`read()` method returns [timx_ccr2::R](timx_ccr2::R) reader structure"]
impl crate::Readable for TIMX_CCR2 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr2::W](timx_ccr2::W) writer structure"]
impl crate::Writable for TIMX_CCR2 {}
#[doc = "capture/compare register 2"]
pub mod timx_ccr2;
#[doc = "capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr3](timx_ccr3) module"]
pub type TIMX_CCR3 = crate::Reg<u32, _TIMX_CCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR3;
#[doc = "`read()` method returns [timx_ccr3::R](timx_ccr3::R) reader structure"]
impl crate::Readable for TIMX_CCR3 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr3::W](timx_ccr3::W) writer structure"]
impl crate::Writable for TIMX_CCR3 {}
#[doc = "capture/compare register 3"]
pub mod timx_ccr3;
#[doc = "capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr4](timx_ccr4) module"]
pub type TIMX_CCR4 = crate::Reg<u32, _TIMX_CCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR4;
#[doc = "`read()` method returns [timx_ccr4::R](timx_ccr4::R) reader structure"]
impl crate::Readable for TIMX_CCR4 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr4::W](timx_ccr4::W) writer structure"]
impl crate::Writable for TIMX_CCR4 {}
#[doc = "capture/compare register 4"]
pub mod timx_ccr4;
#[doc = "TIMx DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dcr](timx_dcr) module"]
pub type TIMX_DCR = crate::Reg<u16, _TIMX_DCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_DCR;
#[doc = "`read()` method returns [timx_dcr::R](timx_dcr::R) reader structure"]
impl crate::Readable for TIMX_DCR {}
#[doc = "`write(|w| ..)` method takes [timx_dcr::W](timx_dcr::W) writer structure"]
impl crate::Writable for TIMX_DCR {}
#[doc = "TIMx DMA control register"]
pub mod timx_dcr;
#[doc = "TIMx DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dmar](timx_dmar) module"]
pub type TIMX_DMAR = crate::Reg<u16, _TIMX_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_DMAR;
#[doc = "`read()` method returns [timx_dmar::R](timx_dmar::R) reader structure"]
impl crate::Readable for TIMX_DMAR {}
#[doc = "`write(|w| ..)` method takes [timx_dmar::W](timx_dmar::W) writer structure"]
impl crate::Writable for TIMX_DMAR {}
#[doc = "TIMx DMA address for full transfer"]
pub mod timx_dmar;
#[doc = "TIM alternate function option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_af1](timx_af1) module"]
pub type TIMX_AF1 = crate::Reg<u32, _TIMX_AF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_AF1;
#[doc = "`read()` method returns [timx_af1::R](timx_af1::R) reader structure"]
impl crate::Readable for TIMX_AF1 {}
#[doc = "`write(|w| ..)` method takes [timx_af1::W](timx_af1::W) writer structure"]
impl crate::Writable for TIMX_AF1 {}
#[doc = "TIM alternate function option register 1"]
pub mod timx_af1;
#[doc = "TIMx timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_tisel](timx_tisel) module"]
pub type TIMX_TISEL = crate::Reg<u32, _TIMX_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_TISEL;
#[doc = "`read()` method returns [timx_tisel::R](timx_tisel::R) reader structure"]
impl crate::Readable for TIMX_TISEL {}
#[doc = "`write(|w| ..)` method takes [timx_tisel::W](timx_tisel::W) writer structure"]
impl crate::Writable for TIMX_TISEL {}
#[doc = "TIMx timer input selection register"]
pub mod timx_tisel;
