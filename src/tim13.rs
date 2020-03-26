#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIMx control register 1"]
    pub timx_cr1: TIMX_CR1,
    _reserved1: [u8; 10usize],
    #[doc = "0x0c - TIMx DMA/interrupt enable register"]
    pub timx_dier: TIMX_DIER,
    _reserved2: [u8; 2usize],
    #[doc = "0x10 - TIMx status register"]
    pub timx_sr: TIMX_SR,
    _reserved3: [u8; 2usize],
    #[doc = "0x14 - event generation register"]
    pub timx_egr: TIMX_EGR,
    _reserved_4_timx_ccmr1: [u8; 4usize],
    _reserved5: [u8; 4usize],
    #[doc = "0x20 - TIMx capture/compare enable register"]
    pub timx_ccer: TIMX_CCER,
    _reserved6: [u8; 2usize],
    #[doc = "0x24 - TIMx counter"]
    pub timx_cnt: TIMX_CNT,
    #[doc = "0x28 - TIMx prescaler"]
    pub timx_psc: TIMX_PSC,
    _reserved8: [u8; 2usize],
    #[doc = "0x2c - TIMx auto-reload register"]
    pub timx_arr: TIMX_ARR,
    _reserved9: [u8; 6usize],
    #[doc = "0x34 - TIMx capture/compare register 1"]
    pub timx_ccr1: TIMX_CCR1,
    _reserved10: [u8; 50usize],
    #[doc = "0x68 - TIMx input selection register"]
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
#[doc = "TIMx DMA/interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dier](timx_dier) module"]
pub type TIMX_DIER = crate::Reg<u16, _TIMX_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_DIER;
#[doc = "`read()` method returns [timx_dier::R](timx_dier::R) reader structure"]
impl crate::Readable for TIMX_DIER {}
#[doc = "`write(|w| ..)` method takes [timx_dier::W](timx_dier::W) writer structure"]
impl crate::Writable for TIMX_DIER {}
#[doc = "TIMx DMA/interrupt enable register"]
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
#[doc = "TIMx counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cnt](timx_cnt) module"]
pub type TIMX_CNT = crate::Reg<u32, _TIMX_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CNT;
#[doc = "`read()` method returns [timx_cnt::R](timx_cnt::R) reader structure"]
impl crate::Readable for TIMX_CNT {}
#[doc = "`write(|w| ..)` method takes [timx_cnt::W](timx_cnt::W) writer structure"]
impl crate::Writable for TIMX_CNT {}
#[doc = "TIMx counter"]
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
#[doc = "TIMx auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_arr](timx_arr) module"]
pub type TIMX_ARR = crate::Reg<u16, _TIMX_ARR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_ARR;
#[doc = "`read()` method returns [timx_arr::R](timx_arr::R) reader structure"]
impl crate::Readable for TIMX_ARR {}
#[doc = "`write(|w| ..)` method takes [timx_arr::W](timx_arr::W) writer structure"]
impl crate::Writable for TIMX_ARR {}
#[doc = "TIMx auto-reload register"]
pub mod timx_arr;
#[doc = "TIMx capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_ccr1](timx_ccr1) module"]
pub type TIMX_CCR1 = crate::Reg<u16, _TIMX_CCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CCR1;
#[doc = "`read()` method returns [timx_ccr1::R](timx_ccr1::R) reader structure"]
impl crate::Readable for TIMX_CCR1 {}
#[doc = "`write(|w| ..)` method takes [timx_ccr1::W](timx_ccr1::W) writer structure"]
impl crate::Writable for TIMX_CCR1 {}
#[doc = "TIMx capture/compare register 1"]
pub mod timx_ccr1;
#[doc = "TIMx input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_tisel](timx_tisel) module"]
pub type TIMX_TISEL = crate::Reg<u32, _TIMX_TISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_TISEL;
#[doc = "`read()` method returns [timx_tisel::R](timx_tisel::R) reader structure"]
impl crate::Readable for TIMX_TISEL {}
#[doc = "`write(|w| ..)` method takes [timx_tisel::W](timx_tisel::W) writer structure"]
impl crate::Writable for TIMX_TISEL {}
#[doc = "TIMx input selection register"]
pub mod timx_tisel;
