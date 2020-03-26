#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TIM6/TIM7 control register 1"]
    pub timx_cr1: TIMX_CR1,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - TIM6/TIM7 control register 2"]
    pub timx_cr2: TIMX_CR2,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - TIM6/TIM7 DMA/Interrupt enable register"]
    pub timx_dier: TIMX_DIER,
    #[doc = "0x10 - TIM6/TIM7 status register"]
    pub timx_sr: TIMX_SR,
    #[doc = "0x14 - TIM6/TIM7 event generation register"]
    pub timx_egr: TIMX_EGR,
    _reserved5: [u8; 12usize],
    #[doc = "0x24 - TIM6/TIM7 counter"]
    pub timx_cnt: TIMX_CNT,
    #[doc = "0x28 - prescaler"]
    pub timx_psc: TIMX_PSC,
    #[doc = "0x2c - auto-reload register"]
    pub timx_arr: TIMX_ARR,
}
#[doc = "TIM6/TIM7 control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cr1](timx_cr1) module"]
pub type TIMX_CR1 = crate::Reg<u16, _TIMX_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CR1;
#[doc = "`read()` method returns [timx_cr1::R](timx_cr1::R) reader structure"]
impl crate::Readable for TIMX_CR1 {}
#[doc = "`write(|w| ..)` method takes [timx_cr1::W](timx_cr1::W) writer structure"]
impl crate::Writable for TIMX_CR1 {}
#[doc = "TIM6/TIM7 control register 1"]
pub mod timx_cr1;
#[doc = "TIM6/TIM7 control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cr2](timx_cr2) module"]
pub type TIMX_CR2 = crate::Reg<u32, _TIMX_CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CR2;
#[doc = "`read()` method returns [timx_cr2::R](timx_cr2::R) reader structure"]
impl crate::Readable for TIMX_CR2 {}
#[doc = "`write(|w| ..)` method takes [timx_cr2::W](timx_cr2::W) writer structure"]
impl crate::Writable for TIMX_CR2 {}
#[doc = "TIM6/TIM7 control register 2"]
pub mod timx_cr2;
#[doc = "TIM6/TIM7 DMA/Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dier](timx_dier) module"]
pub type TIMX_DIER = crate::Reg<u32, _TIMX_DIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_DIER;
#[doc = "`read()` method returns [timx_dier::R](timx_dier::R) reader structure"]
impl crate::Readable for TIMX_DIER {}
#[doc = "`write(|w| ..)` method takes [timx_dier::W](timx_dier::W) writer structure"]
impl crate::Writable for TIMX_DIER {}
#[doc = "TIM6/TIM7 DMA/Interrupt enable register"]
pub mod timx_dier;
#[doc = "TIM6/TIM7 status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_sr](timx_sr) module"]
pub type TIMX_SR = crate::Reg<u32, _TIMX_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_SR;
#[doc = "`read()` method returns [timx_sr::R](timx_sr::R) reader structure"]
impl crate::Readable for TIMX_SR {}
#[doc = "`write(|w| ..)` method takes [timx_sr::W](timx_sr::W) writer structure"]
impl crate::Writable for TIMX_SR {}
#[doc = "TIM6/TIM7 status register"]
pub mod timx_sr;
#[doc = "TIM6/TIM7 event generation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_egr](timx_egr) module"]
pub type TIMX_EGR = crate::Reg<u32, _TIMX_EGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_EGR;
#[doc = "`read()` method returns [timx_egr::R](timx_egr::R) reader structure"]
impl crate::Readable for TIMX_EGR {}
#[doc = "`write(|w| ..)` method takes [timx_egr::W](timx_egr::W) writer structure"]
impl crate::Writable for TIMX_EGR {}
#[doc = "TIM6/TIM7 event generation register"]
pub mod timx_egr;
#[doc = "TIM6/TIM7 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_cnt](timx_cnt) module"]
pub type TIMX_CNT = crate::Reg<u32, _TIMX_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_CNT;
#[doc = "`read()` method returns [timx_cnt::R](timx_cnt::R) reader structure"]
impl crate::Readable for TIMX_CNT {}
#[doc = "`write(|w| ..)` method takes [timx_cnt::W](timx_cnt::W) writer structure"]
impl crate::Writable for TIMX_CNT {}
#[doc = "TIM6/TIM7 counter"]
pub mod timx_cnt;
#[doc = "prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_psc](timx_psc) module"]
pub type TIMX_PSC = crate::Reg<u32, _TIMX_PSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMX_PSC;
#[doc = "`read()` method returns [timx_psc::R](timx_psc::R) reader structure"]
impl crate::Readable for TIMX_PSC {}
#[doc = "`write(|w| ..)` method takes [timx_psc::W](timx_psc::W) writer structure"]
impl crate::Writable for TIMX_PSC {}
#[doc = "prescaler"]
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
