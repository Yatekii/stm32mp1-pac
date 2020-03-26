#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch0cfgr1: DFSDM_CH0CFGR1,
    #[doc = "0x04 - This register specifies the parameters used by channel y (y = 0..7)."]
    pub dfsdm_ch0cfgr2: DFSDM_CH0CFGR2,
    #[doc = "0x08 - Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
    pub dfsdm_ch0awscdr: DFSDM_CH0AWSCDR,
    #[doc = "0x0c - This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
    pub dfsdm_ch0wdatr: DFSDM_CH0WDATR,
    #[doc = "0x10 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch0datinr: DFSDM_CH0DATINR,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch1cfgr1: DFSDM_CH1CFGR1,
    #[doc = "0x24 - This register specifies the parameters used by channel y (y = 0..7)."]
    pub dfsdm_ch1cfgr2: DFSDM_CH1CFGR2,
    #[doc = "0x28 - Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
    pub dfsdm_ch1awscdr: DFSDM_CH1AWSCDR,
    #[doc = "0x2c - This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
    pub dfsdm_ch1wdatr: DFSDM_CH1WDATR,
    #[doc = "0x30 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch1datinr: DFSDM_CH1DATINR,
    _reserved10: [u8; 12usize],
    #[doc = "0x40 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch2cfgr1: DFSDM_CH2CFGR1,
    #[doc = "0x44 - This register specifies the parameters used by channel y (y = 0..7)."]
    pub dfsdm_ch2cfgr2: DFSDM_CH2CFGR2,
    #[doc = "0x48 - Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
    pub dfsdm_ch2awscdr: DFSDM_CH2AWSCDR,
    #[doc = "0x4c - This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
    pub dfsdm_ch2wdatr: DFSDM_CH2WDATR,
    #[doc = "0x50 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch2datinr: DFSDM_CH2DATINR,
    _reserved15: [u8; 12usize],
    #[doc = "0x60 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch3cfgr1: DFSDM_CH3CFGR1,
    #[doc = "0x64 - This register specifies the parameters used by channel y (y = 0..7)."]
    pub dfsdm_ch3cfgr2: DFSDM_CH3CFGR2,
    #[doc = "0x68 - Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
    pub dfsdm_ch3awscdr: DFSDM_CH3AWSCDR,
    #[doc = "0x6c - This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
    pub dfsdm_ch3wdatr: DFSDM_CH3WDATR,
    #[doc = "0x70 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch3datinr: DFSDM_CH3DATINR,
    _reserved20: [u8; 12usize],
    #[doc = "0x80 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch4cfgr1: DFSDM_CH4CFGR1,
    #[doc = "0x84 - This register specifies the parameters used by channel y (y = 0..7)."]
    pub dfsdm_ch4cfgr2: DFSDM_CH4CFGR2,
    #[doc = "0x88 - Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
    pub dfsdm_ch4awscdr: DFSDM_CH4AWSCDR,
    #[doc = "0x8c - This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
    pub dfsdm_ch4wdatr: DFSDM_CH4WDATR,
    #[doc = "0x90 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch4datinr: DFSDM_CH4DATINR,
    _reserved25: [u8; 12usize],
    #[doc = "0xa0 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch5cfgr1: DFSDM_CH5CFGR1,
    #[doc = "0xa4 - This register specifies the parameters used by channel y (y = 0..7)."]
    pub dfsdm_ch5cfgr2: DFSDM_CH5CFGR2,
    #[doc = "0xa8 - Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
    pub dfsdm_ch5awscdr: DFSDM_CH5AWSCDR,
    #[doc = "0xac - This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
    pub dfsdm_ch5wdatr: DFSDM_CH5WDATR,
    #[doc = "0xb0 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch5datinr: DFSDM_CH5DATINR,
    _reserved30: [u8; 12usize],
    #[doc = "0xc0 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch6cfgr1: DFSDM_CH6CFGR1,
    #[doc = "0xc4 - This register specifies the parameters used by channel y (y = 0..7)."]
    pub dfsdm_ch6cfgr2: DFSDM_CH6CFGR2,
    #[doc = "0xc8 - Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
    pub dfsdm_ch6awscdr: DFSDM_CH6AWSCDR,
    #[doc = "0xcc - This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
    pub dfsdm_ch6wdatr: DFSDM_CH6WDATR,
    #[doc = "0xd0 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch6datinr: DFSDM_CH6DATINR,
    _reserved35: [u8; 12usize],
    #[doc = "0xe0 - This register specifies the parameters used by channel y."]
    pub dfsdm_ch7cfgr1: DFSDM_CH7CFGR1,
    #[doc = "0xe4 - This register specifies the parameters used by channel y (y = 0..7)."]
    pub dfsdm_ch7cfgr2: DFSDM_CH7CFGR2,
    #[doc = "0xe8 - Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
    pub dfsdm_ch7awscdr: DFSDM_CH7AWSCDR,
    #[doc = "0xec - This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
    pub dfsdm_ch7wdatr: DFSDM_CH7WDATR,
    #[doc = "0xf0 - This register contains 16-bit input data to be processed by DFSDM filter module."]
    pub dfsdm_ch7datinr: DFSDM_CH7DATINR,
    _reserved40: [u8; 12usize],
    #[doc = "0x100 - DFSDM control register 1"]
    pub dfsdm_flt0cr1: DFSDM_FLT0CR1,
    #[doc = "0x104 - DFSDM control register 2"]
    pub dfsdm_flt0cr2: DFSDM_FLT0CR2,
    #[doc = "0x108 - DFSDM interrupt and status register"]
    pub dfsdm_flt0isr: DFSDM_FLT0ISR,
    #[doc = "0x10c - DFSDM interrupt flag clear register"]
    pub dfsdm_flt0icr: DFSDM_FLT0ICR,
    #[doc = "0x110 - DFSDM injected channel group selection register"]
    pub dfsdm_flt0jchgr: DFSDM_FLT0JCHGR,
    #[doc = "0x114 - DFSDM filter control register"]
    pub dfsdm_flt0fcr: DFSDM_FLT0FCR,
    #[doc = "0x118 - DFSDM data register for injected group"]
    pub dfsdm_flt0jdatar: DFSDM_FLT0JDATAR,
    #[doc = "0x11c - DFSDM data register for the regular channel"]
    pub dfsdm_flt0rdatar: DFSDM_FLT0RDATAR,
    #[doc = "0x120 - DFSDM analog watchdog high threshold register"]
    pub dfsdm_flt0awhtr: DFSDM_FLT0AWHTR,
    #[doc = "0x124 - DFSDM analog watchdog low threshold register"]
    pub dfsdm_flt0awltr: DFSDM_FLT0AWLTR,
    #[doc = "0x128 - DFSDM analog watchdog status register"]
    pub dfsdm_flt0awsr: DFSDM_FLT0AWSR,
    #[doc = "0x12c - DFSDM analog watchdog clear flag register"]
    pub dfsdm_flt0awcfr: DFSDM_FLT0AWCFR,
    #[doc = "0x130 - DFSDM Extremes detector maximum register"]
    pub dfsdm_flt0exmax: DFSDM_FLT0EXMAX,
    #[doc = "0x134 - DFSDM Extremes detector minimum register"]
    pub dfsdm_flt0exmin: DFSDM_FLT0EXMIN,
    #[doc = "0x138 - DFSDM conversion timer register"]
    pub dfsdm_flt0cnvtimr: DFSDM_FLT0CNVTIMR,
    _reserved55: [u8; 68usize],
    #[doc = "0x180 - DFSDM control register 1"]
    pub dfsdm_flt1cr1: DFSDM_FLT1CR1,
    #[doc = "0x184 - DFSDM control register 2"]
    pub dfsdm_flt1cr2: DFSDM_FLT1CR2,
    #[doc = "0x188 - DFSDM interrupt and status register"]
    pub dfsdm_flt1isr: DFSDM_FLT1ISR,
    #[doc = "0x18c - DFSDM interrupt flag clear register"]
    pub dfsdm_flt1icr: DFSDM_FLT1ICR,
    #[doc = "0x190 - DFSDM injected channel group selection register"]
    pub dfsdm_flt1jchgr: DFSDM_FLT1JCHGR,
    #[doc = "0x194 - DFSDM filter control register"]
    pub dfsdm_flt1fcr: DFSDM_FLT1FCR,
    #[doc = "0x198 - DFSDM data register for injected group"]
    pub dfsdm_flt1jdatar: DFSDM_FLT1JDATAR,
    #[doc = "0x19c - DFSDM data register for the regular channel"]
    pub dfsdm_flt1rdatar: DFSDM_FLT1RDATAR,
    #[doc = "0x1a0 - DFSDM analog watchdog high threshold register"]
    pub dfsdm_flt1awhtr: DFSDM_FLT1AWHTR,
    #[doc = "0x1a4 - DFSDM analog watchdog low threshold register"]
    pub dfsdm_flt1awltr: DFSDM_FLT1AWLTR,
    #[doc = "0x1a8 - DFSDM analog watchdog status register"]
    pub dfsdm_flt1awsr: DFSDM_FLT1AWSR,
    #[doc = "0x1ac - DFSDM analog watchdog clear flag register"]
    pub dfsdm_flt1awcfr: DFSDM_FLT1AWCFR,
    #[doc = "0x1b0 - DFSDM Extremes detector maximum register"]
    pub dfsdm_flt1exmax: DFSDM_FLT1EXMAX,
    #[doc = "0x1b4 - DFSDM Extremes detector minimum register"]
    pub dfsdm_flt1exmin: DFSDM_FLT1EXMIN,
    #[doc = "0x1b8 - DFSDM conversion timer register"]
    pub dfsdm_flt1cnvtimr: DFSDM_FLT1CNVTIMR,
    _reserved70: [u8; 68usize],
    #[doc = "0x200 - DFSDM control register 1"]
    pub dfsdm_flt2cr1: DFSDM_FLT2CR1,
    #[doc = "0x204 - DFSDM control register 2"]
    pub dfsdm_flt2cr2: DFSDM_FLT2CR2,
    #[doc = "0x208 - DFSDM interrupt and status register"]
    pub dfsdm_flt2isr: DFSDM_FLT2ISR,
    #[doc = "0x20c - DFSDM interrupt flag clear register"]
    pub dfsdm_flt2icr: DFSDM_FLT2ICR,
    #[doc = "0x210 - DFSDM injected channel group selection register"]
    pub dfsdm_flt2jchgr: DFSDM_FLT2JCHGR,
    #[doc = "0x214 - DFSDM filter control register"]
    pub dfsdm_flt2fcr: DFSDM_FLT2FCR,
    #[doc = "0x218 - DFSDM data register for injected group"]
    pub dfsdm_flt2jdatar: DFSDM_FLT2JDATAR,
    #[doc = "0x21c - DFSDM data register for the regular channel"]
    pub dfsdm_flt2rdatar: DFSDM_FLT2RDATAR,
    #[doc = "0x220 - DFSDM analog watchdog high threshold register"]
    pub dfsdm_flt2awhtr: DFSDM_FLT2AWHTR,
    #[doc = "0x224 - DFSDM analog watchdog low threshold register"]
    pub dfsdm_flt2awltr: DFSDM_FLT2AWLTR,
    #[doc = "0x228 - DFSDM analog watchdog status register"]
    pub dfsdm_flt2awsr: DFSDM_FLT2AWSR,
    #[doc = "0x22c - DFSDM analog watchdog clear flag register"]
    pub dfsdm_flt2awcfr: DFSDM_FLT2AWCFR,
    #[doc = "0x230 - DFSDM Extremes detector maximum register"]
    pub dfsdm_flt2exmax: DFSDM_FLT2EXMAX,
    #[doc = "0x234 - DFSDM Extremes detector minimum register"]
    pub dfsdm_flt2exmin: DFSDM_FLT2EXMIN,
    #[doc = "0x238 - DFSDM conversion timer register"]
    pub dfsdm_flt2cnvtimr: DFSDM_FLT2CNVTIMR,
    _reserved85: [u8; 68usize],
    #[doc = "0x280 - DFSDM control register 1"]
    pub dfsdm_flt3cr1: DFSDM_FLT3CR1,
    #[doc = "0x284 - DFSDM control register 2"]
    pub dfsdm_flt3cr2: DFSDM_FLT3CR2,
    #[doc = "0x288 - DFSDM interrupt and status register"]
    pub dfsdm_flt3isr: DFSDM_FLT3ISR,
    #[doc = "0x28c - DFSDM interrupt flag clear register"]
    pub dfsdm_flt3icr: DFSDM_FLT3ICR,
    #[doc = "0x290 - DFSDM injected channel group selection register"]
    pub dfsdm_flt3jchgr: DFSDM_FLT3JCHGR,
    #[doc = "0x294 - DFSDM filter control register"]
    pub dfsdm_flt3fcr: DFSDM_FLT3FCR,
    #[doc = "0x298 - DFSDM data register for injected group"]
    pub dfsdm_flt3jdatar: DFSDM_FLT3JDATAR,
    #[doc = "0x29c - DFSDM data register for the regular channel"]
    pub dfsdm_flt3rdatar: DFSDM_FLT3RDATAR,
    #[doc = "0x2a0 - DFSDM analog watchdog high threshold register"]
    pub dfsdm_flt3awhtr: DFSDM_FLT3AWHTR,
    #[doc = "0x2a4 - DFSDM analog watchdog low threshold register"]
    pub dfsdm_flt3awltr: DFSDM_FLT3AWLTR,
    #[doc = "0x2a8 - DFSDM analog watchdog status register"]
    pub dfsdm_flt3awsr: DFSDM_FLT3AWSR,
    #[doc = "0x2ac - DFSDM analog watchdog clear flag register"]
    pub dfsdm_flt3awcfr: DFSDM_FLT3AWCFR,
    #[doc = "0x2b0 - DFSDM Extremes detector maximum register"]
    pub dfsdm_flt3exmax: DFSDM_FLT3EXMAX,
    #[doc = "0x2b4 - DFSDM Extremes detector minimum register"]
    pub dfsdm_flt3exmin: DFSDM_FLT3EXMIN,
    #[doc = "0x2b8 - DFSDM conversion timer register"]
    pub dfsdm_flt3cnvtimr: DFSDM_FLT3CNVTIMR,
}
#[doc = "This register specifies the parameters used by channel y.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch0cfgr1](dfsdm_ch0cfgr1) module"]
pub type DFSDM_CH0CFGR1 = crate::Reg<u32, _DFSDM_CH0CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH0CFGR1;
#[doc = "`read()` method returns [dfsdm_ch0cfgr1::R](dfsdm_ch0cfgr1::R) reader structure"]
impl crate::Readable for DFSDM_CH0CFGR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch0cfgr1::W](dfsdm_ch0cfgr1::W) writer structure"]
impl crate::Writable for DFSDM_CH0CFGR1 {}
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch0cfgr1;
#[doc = "This register specifies the parameters used by channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch0cfgr2](dfsdm_ch0cfgr2) module"]
pub type DFSDM_CH0CFGR2 = crate::Reg<u32, _DFSDM_CH0CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH0CFGR2;
#[doc = "`read()` method returns [dfsdm_ch0cfgr2::R](dfsdm_ch0cfgr2::R) reader structure"]
impl crate::Readable for DFSDM_CH0CFGR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch0cfgr2::W](dfsdm_ch0cfgr2::W) writer structure"]
impl crate::Writable for DFSDM_CH0CFGR2 {}
#[doc = "This register specifies the parameters used by channel y (y = 0..7)."]
pub mod dfsdm_ch0cfgr2;
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch0awscdr](dfsdm_ch0awscdr) module"]
pub type DFSDM_CH0AWSCDR = crate::Reg<u32, _DFSDM_CH0AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH0AWSCDR;
#[doc = "`read()` method returns [dfsdm_ch0awscdr::R](dfsdm_ch0awscdr::R) reader structure"]
impl crate::Readable for DFSDM_CH0AWSCDR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch0awscdr::W](dfsdm_ch0awscdr::W) writer structure"]
impl crate::Writable for DFSDM_CH0AWSCDR {}
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
pub mod dfsdm_ch0awscdr;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch0wdatr](dfsdm_ch0wdatr) module"]
pub type DFSDM_CH0WDATR = crate::Reg<u32, _DFSDM_CH0WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH0WDATR;
#[doc = "`read()` method returns [dfsdm_ch0wdatr::R](dfsdm_ch0wdatr::R) reader structure"]
impl crate::Readable for DFSDM_CH0WDATR {}
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
pub mod dfsdm_ch0wdatr;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch0datinr](dfsdm_ch0datinr) module"]
pub type DFSDM_CH0DATINR = crate::Reg<u32, _DFSDM_CH0DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH0DATINR;
#[doc = "`read()` method returns [dfsdm_ch0datinr::R](dfsdm_ch0datinr::R) reader structure"]
impl crate::Readable for DFSDM_CH0DATINR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch0datinr::W](dfsdm_ch0datinr::W) writer structure"]
impl crate::Writable for DFSDM_CH0DATINR {}
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch0datinr;
#[doc = "This register specifies the parameters used by channel y.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch1cfgr1](dfsdm_ch1cfgr1) module"]
pub type DFSDM_CH1CFGR1 = crate::Reg<u32, _DFSDM_CH1CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH1CFGR1;
#[doc = "`read()` method returns [dfsdm_ch1cfgr1::R](dfsdm_ch1cfgr1::R) reader structure"]
impl crate::Readable for DFSDM_CH1CFGR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch1cfgr1::W](dfsdm_ch1cfgr1::W) writer structure"]
impl crate::Writable for DFSDM_CH1CFGR1 {}
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch1cfgr1;
#[doc = "This register specifies the parameters used by channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch1cfgr2](dfsdm_ch1cfgr2) module"]
pub type DFSDM_CH1CFGR2 = crate::Reg<u32, _DFSDM_CH1CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH1CFGR2;
#[doc = "`read()` method returns [dfsdm_ch1cfgr2::R](dfsdm_ch1cfgr2::R) reader structure"]
impl crate::Readable for DFSDM_CH1CFGR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch1cfgr2::W](dfsdm_ch1cfgr2::W) writer structure"]
impl crate::Writable for DFSDM_CH1CFGR2 {}
#[doc = "This register specifies the parameters used by channel y (y = 0..7)."]
pub mod dfsdm_ch1cfgr2;
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch1awscdr](dfsdm_ch1awscdr) module"]
pub type DFSDM_CH1AWSCDR = crate::Reg<u32, _DFSDM_CH1AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH1AWSCDR;
#[doc = "`read()` method returns [dfsdm_ch1awscdr::R](dfsdm_ch1awscdr::R) reader structure"]
impl crate::Readable for DFSDM_CH1AWSCDR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch1awscdr::W](dfsdm_ch1awscdr::W) writer structure"]
impl crate::Writable for DFSDM_CH1AWSCDR {}
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
pub mod dfsdm_ch1awscdr;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch1wdatr](dfsdm_ch1wdatr) module"]
pub type DFSDM_CH1WDATR = crate::Reg<u32, _DFSDM_CH1WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH1WDATR;
#[doc = "`read()` method returns [dfsdm_ch1wdatr::R](dfsdm_ch1wdatr::R) reader structure"]
impl crate::Readable for DFSDM_CH1WDATR {}
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
pub mod dfsdm_ch1wdatr;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch1datinr](dfsdm_ch1datinr) module"]
pub type DFSDM_CH1DATINR = crate::Reg<u32, _DFSDM_CH1DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH1DATINR;
#[doc = "`read()` method returns [dfsdm_ch1datinr::R](dfsdm_ch1datinr::R) reader structure"]
impl crate::Readable for DFSDM_CH1DATINR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch1datinr::W](dfsdm_ch1datinr::W) writer structure"]
impl crate::Writable for DFSDM_CH1DATINR {}
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch1datinr;
#[doc = "This register specifies the parameters used by channel y.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch2cfgr1](dfsdm_ch2cfgr1) module"]
pub type DFSDM_CH2CFGR1 = crate::Reg<u32, _DFSDM_CH2CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH2CFGR1;
#[doc = "`read()` method returns [dfsdm_ch2cfgr1::R](dfsdm_ch2cfgr1::R) reader structure"]
impl crate::Readable for DFSDM_CH2CFGR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch2cfgr1::W](dfsdm_ch2cfgr1::W) writer structure"]
impl crate::Writable for DFSDM_CH2CFGR1 {}
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch2cfgr1;
#[doc = "This register specifies the parameters used by channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch2cfgr2](dfsdm_ch2cfgr2) module"]
pub type DFSDM_CH2CFGR2 = crate::Reg<u32, _DFSDM_CH2CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH2CFGR2;
#[doc = "`read()` method returns [dfsdm_ch2cfgr2::R](dfsdm_ch2cfgr2::R) reader structure"]
impl crate::Readable for DFSDM_CH2CFGR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch2cfgr2::W](dfsdm_ch2cfgr2::W) writer structure"]
impl crate::Writable for DFSDM_CH2CFGR2 {}
#[doc = "This register specifies the parameters used by channel y (y = 0..7)."]
pub mod dfsdm_ch2cfgr2;
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch2awscdr](dfsdm_ch2awscdr) module"]
pub type DFSDM_CH2AWSCDR = crate::Reg<u32, _DFSDM_CH2AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH2AWSCDR;
#[doc = "`read()` method returns [dfsdm_ch2awscdr::R](dfsdm_ch2awscdr::R) reader structure"]
impl crate::Readable for DFSDM_CH2AWSCDR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch2awscdr::W](dfsdm_ch2awscdr::W) writer structure"]
impl crate::Writable for DFSDM_CH2AWSCDR {}
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
pub mod dfsdm_ch2awscdr;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch2wdatr](dfsdm_ch2wdatr) module"]
pub type DFSDM_CH2WDATR = crate::Reg<u32, _DFSDM_CH2WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH2WDATR;
#[doc = "`read()` method returns [dfsdm_ch2wdatr::R](dfsdm_ch2wdatr::R) reader structure"]
impl crate::Readable for DFSDM_CH2WDATR {}
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
pub mod dfsdm_ch2wdatr;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch2datinr](dfsdm_ch2datinr) module"]
pub type DFSDM_CH2DATINR = crate::Reg<u32, _DFSDM_CH2DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH2DATINR;
#[doc = "`read()` method returns [dfsdm_ch2datinr::R](dfsdm_ch2datinr::R) reader structure"]
impl crate::Readable for DFSDM_CH2DATINR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch2datinr::W](dfsdm_ch2datinr::W) writer structure"]
impl crate::Writable for DFSDM_CH2DATINR {}
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch2datinr;
#[doc = "This register specifies the parameters used by channel y.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch3cfgr1](dfsdm_ch3cfgr1) module"]
pub type DFSDM_CH3CFGR1 = crate::Reg<u32, _DFSDM_CH3CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH3CFGR1;
#[doc = "`read()` method returns [dfsdm_ch3cfgr1::R](dfsdm_ch3cfgr1::R) reader structure"]
impl crate::Readable for DFSDM_CH3CFGR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch3cfgr1::W](dfsdm_ch3cfgr1::W) writer structure"]
impl crate::Writable for DFSDM_CH3CFGR1 {}
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch3cfgr1;
#[doc = "This register specifies the parameters used by channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch3cfgr2](dfsdm_ch3cfgr2) module"]
pub type DFSDM_CH3CFGR2 = crate::Reg<u32, _DFSDM_CH3CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH3CFGR2;
#[doc = "`read()` method returns [dfsdm_ch3cfgr2::R](dfsdm_ch3cfgr2::R) reader structure"]
impl crate::Readable for DFSDM_CH3CFGR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch3cfgr2::W](dfsdm_ch3cfgr2::W) writer structure"]
impl crate::Writable for DFSDM_CH3CFGR2 {}
#[doc = "This register specifies the parameters used by channel y (y = 0..7)."]
pub mod dfsdm_ch3cfgr2;
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch3awscdr](dfsdm_ch3awscdr) module"]
pub type DFSDM_CH3AWSCDR = crate::Reg<u32, _DFSDM_CH3AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH3AWSCDR;
#[doc = "`read()` method returns [dfsdm_ch3awscdr::R](dfsdm_ch3awscdr::R) reader structure"]
impl crate::Readable for DFSDM_CH3AWSCDR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch3awscdr::W](dfsdm_ch3awscdr::W) writer structure"]
impl crate::Writable for DFSDM_CH3AWSCDR {}
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
pub mod dfsdm_ch3awscdr;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch3wdatr](dfsdm_ch3wdatr) module"]
pub type DFSDM_CH3WDATR = crate::Reg<u32, _DFSDM_CH3WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH3WDATR;
#[doc = "`read()` method returns [dfsdm_ch3wdatr::R](dfsdm_ch3wdatr::R) reader structure"]
impl crate::Readable for DFSDM_CH3WDATR {}
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
pub mod dfsdm_ch3wdatr;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch3datinr](dfsdm_ch3datinr) module"]
pub type DFSDM_CH3DATINR = crate::Reg<u32, _DFSDM_CH3DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH3DATINR;
#[doc = "`read()` method returns [dfsdm_ch3datinr::R](dfsdm_ch3datinr::R) reader structure"]
impl crate::Readable for DFSDM_CH3DATINR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch3datinr::W](dfsdm_ch3datinr::W) writer structure"]
impl crate::Writable for DFSDM_CH3DATINR {}
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch3datinr;
#[doc = "This register specifies the parameters used by channel y.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch4cfgr1](dfsdm_ch4cfgr1) module"]
pub type DFSDM_CH4CFGR1 = crate::Reg<u32, _DFSDM_CH4CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH4CFGR1;
#[doc = "`read()` method returns [dfsdm_ch4cfgr1::R](dfsdm_ch4cfgr1::R) reader structure"]
impl crate::Readable for DFSDM_CH4CFGR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch4cfgr1::W](dfsdm_ch4cfgr1::W) writer structure"]
impl crate::Writable for DFSDM_CH4CFGR1 {}
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch4cfgr1;
#[doc = "This register specifies the parameters used by channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch4cfgr2](dfsdm_ch4cfgr2) module"]
pub type DFSDM_CH4CFGR2 = crate::Reg<u32, _DFSDM_CH4CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH4CFGR2;
#[doc = "`read()` method returns [dfsdm_ch4cfgr2::R](dfsdm_ch4cfgr2::R) reader structure"]
impl crate::Readable for DFSDM_CH4CFGR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch4cfgr2::W](dfsdm_ch4cfgr2::W) writer structure"]
impl crate::Writable for DFSDM_CH4CFGR2 {}
#[doc = "This register specifies the parameters used by channel y (y = 0..7)."]
pub mod dfsdm_ch4cfgr2;
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch4awscdr](dfsdm_ch4awscdr) module"]
pub type DFSDM_CH4AWSCDR = crate::Reg<u32, _DFSDM_CH4AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH4AWSCDR;
#[doc = "`read()` method returns [dfsdm_ch4awscdr::R](dfsdm_ch4awscdr::R) reader structure"]
impl crate::Readable for DFSDM_CH4AWSCDR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch4awscdr::W](dfsdm_ch4awscdr::W) writer structure"]
impl crate::Writable for DFSDM_CH4AWSCDR {}
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
pub mod dfsdm_ch4awscdr;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch4wdatr](dfsdm_ch4wdatr) module"]
pub type DFSDM_CH4WDATR = crate::Reg<u32, _DFSDM_CH4WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH4WDATR;
#[doc = "`read()` method returns [dfsdm_ch4wdatr::R](dfsdm_ch4wdatr::R) reader structure"]
impl crate::Readable for DFSDM_CH4WDATR {}
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
pub mod dfsdm_ch4wdatr;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch4datinr](dfsdm_ch4datinr) module"]
pub type DFSDM_CH4DATINR = crate::Reg<u32, _DFSDM_CH4DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH4DATINR;
#[doc = "`read()` method returns [dfsdm_ch4datinr::R](dfsdm_ch4datinr::R) reader structure"]
impl crate::Readable for DFSDM_CH4DATINR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch4datinr::W](dfsdm_ch4datinr::W) writer structure"]
impl crate::Writable for DFSDM_CH4DATINR {}
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch4datinr;
#[doc = "This register specifies the parameters used by channel y.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch5cfgr1](dfsdm_ch5cfgr1) module"]
pub type DFSDM_CH5CFGR1 = crate::Reg<u32, _DFSDM_CH5CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH5CFGR1;
#[doc = "`read()` method returns [dfsdm_ch5cfgr1::R](dfsdm_ch5cfgr1::R) reader structure"]
impl crate::Readable for DFSDM_CH5CFGR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch5cfgr1::W](dfsdm_ch5cfgr1::W) writer structure"]
impl crate::Writable for DFSDM_CH5CFGR1 {}
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch5cfgr1;
#[doc = "This register specifies the parameters used by channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch5cfgr2](dfsdm_ch5cfgr2) module"]
pub type DFSDM_CH5CFGR2 = crate::Reg<u32, _DFSDM_CH5CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH5CFGR2;
#[doc = "`read()` method returns [dfsdm_ch5cfgr2::R](dfsdm_ch5cfgr2::R) reader structure"]
impl crate::Readable for DFSDM_CH5CFGR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch5cfgr2::W](dfsdm_ch5cfgr2::W) writer structure"]
impl crate::Writable for DFSDM_CH5CFGR2 {}
#[doc = "This register specifies the parameters used by channel y (y = 0..7)."]
pub mod dfsdm_ch5cfgr2;
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch5awscdr](dfsdm_ch5awscdr) module"]
pub type DFSDM_CH5AWSCDR = crate::Reg<u32, _DFSDM_CH5AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH5AWSCDR;
#[doc = "`read()` method returns [dfsdm_ch5awscdr::R](dfsdm_ch5awscdr::R) reader structure"]
impl crate::Readable for DFSDM_CH5AWSCDR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch5awscdr::W](dfsdm_ch5awscdr::W) writer structure"]
impl crate::Writable for DFSDM_CH5AWSCDR {}
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
pub mod dfsdm_ch5awscdr;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch5wdatr](dfsdm_ch5wdatr) module"]
pub type DFSDM_CH5WDATR = crate::Reg<u32, _DFSDM_CH5WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH5WDATR;
#[doc = "`read()` method returns [dfsdm_ch5wdatr::R](dfsdm_ch5wdatr::R) reader structure"]
impl crate::Readable for DFSDM_CH5WDATR {}
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
pub mod dfsdm_ch5wdatr;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch5datinr](dfsdm_ch5datinr) module"]
pub type DFSDM_CH5DATINR = crate::Reg<u32, _DFSDM_CH5DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH5DATINR;
#[doc = "`read()` method returns [dfsdm_ch5datinr::R](dfsdm_ch5datinr::R) reader structure"]
impl crate::Readable for DFSDM_CH5DATINR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch5datinr::W](dfsdm_ch5datinr::W) writer structure"]
impl crate::Writable for DFSDM_CH5DATINR {}
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch5datinr;
#[doc = "This register specifies the parameters used by channel y.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch6cfgr1](dfsdm_ch6cfgr1) module"]
pub type DFSDM_CH6CFGR1 = crate::Reg<u32, _DFSDM_CH6CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH6CFGR1;
#[doc = "`read()` method returns [dfsdm_ch6cfgr1::R](dfsdm_ch6cfgr1::R) reader structure"]
impl crate::Readable for DFSDM_CH6CFGR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch6cfgr1::W](dfsdm_ch6cfgr1::W) writer structure"]
impl crate::Writable for DFSDM_CH6CFGR1 {}
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch6cfgr1;
#[doc = "This register specifies the parameters used by channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch6cfgr2](dfsdm_ch6cfgr2) module"]
pub type DFSDM_CH6CFGR2 = crate::Reg<u32, _DFSDM_CH6CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH6CFGR2;
#[doc = "`read()` method returns [dfsdm_ch6cfgr2::R](dfsdm_ch6cfgr2::R) reader structure"]
impl crate::Readable for DFSDM_CH6CFGR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch6cfgr2::W](dfsdm_ch6cfgr2::W) writer structure"]
impl crate::Writable for DFSDM_CH6CFGR2 {}
#[doc = "This register specifies the parameters used by channel y (y = 0..7)."]
pub mod dfsdm_ch6cfgr2;
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch6awscdr](dfsdm_ch6awscdr) module"]
pub type DFSDM_CH6AWSCDR = crate::Reg<u32, _DFSDM_CH6AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH6AWSCDR;
#[doc = "`read()` method returns [dfsdm_ch6awscdr::R](dfsdm_ch6awscdr::R) reader structure"]
impl crate::Readable for DFSDM_CH6AWSCDR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch6awscdr::W](dfsdm_ch6awscdr::W) writer structure"]
impl crate::Writable for DFSDM_CH6AWSCDR {}
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
pub mod dfsdm_ch6awscdr;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch6wdatr](dfsdm_ch6wdatr) module"]
pub type DFSDM_CH6WDATR = crate::Reg<u32, _DFSDM_CH6WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH6WDATR;
#[doc = "`read()` method returns [dfsdm_ch6wdatr::R](dfsdm_ch6wdatr::R) reader structure"]
impl crate::Readable for DFSDM_CH6WDATR {}
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
pub mod dfsdm_ch6wdatr;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch6datinr](dfsdm_ch6datinr) module"]
pub type DFSDM_CH6DATINR = crate::Reg<u32, _DFSDM_CH6DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH6DATINR;
#[doc = "`read()` method returns [dfsdm_ch6datinr::R](dfsdm_ch6datinr::R) reader structure"]
impl crate::Readable for DFSDM_CH6DATINR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch6datinr::W](dfsdm_ch6datinr::W) writer structure"]
impl crate::Writable for DFSDM_CH6DATINR {}
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch6datinr;
#[doc = "This register specifies the parameters used by channel y.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch7cfgr1](dfsdm_ch7cfgr1) module"]
pub type DFSDM_CH7CFGR1 = crate::Reg<u32, _DFSDM_CH7CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH7CFGR1;
#[doc = "`read()` method returns [dfsdm_ch7cfgr1::R](dfsdm_ch7cfgr1::R) reader structure"]
impl crate::Readable for DFSDM_CH7CFGR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch7cfgr1::W](dfsdm_ch7cfgr1::W) writer structure"]
impl crate::Writable for DFSDM_CH7CFGR1 {}
#[doc = "This register specifies the parameters used by channel y."]
pub mod dfsdm_ch7cfgr1;
#[doc = "This register specifies the parameters used by channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch7cfgr2](dfsdm_ch7cfgr2) module"]
pub type DFSDM_CH7CFGR2 = crate::Reg<u32, _DFSDM_CH7CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH7CFGR2;
#[doc = "`read()` method returns [dfsdm_ch7cfgr2::R](dfsdm_ch7cfgr2::R) reader structure"]
impl crate::Readable for DFSDM_CH7CFGR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch7cfgr2::W](dfsdm_ch7cfgr2::W) writer structure"]
impl crate::Writable for DFSDM_CH7CFGR2 {}
#[doc = "This register specifies the parameters used by channel y (y = 0..7)."]
pub mod dfsdm_ch7cfgr2;
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch7awscdr](dfsdm_ch7awscdr) module"]
pub type DFSDM_CH7AWSCDR = crate::Reg<u32, _DFSDM_CH7AWSCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH7AWSCDR;
#[doc = "`read()` method returns [dfsdm_ch7awscdr::R](dfsdm_ch7awscdr::R) reader structure"]
impl crate::Readable for DFSDM_CH7AWSCDR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch7awscdr::W](dfsdm_ch7awscdr::W) writer structure"]
impl crate::Writable for DFSDM_CH7AWSCDR {}
#[doc = "Short-circuit detector and analog watchdog settings for channel y (y = 0..7)"]
pub mod dfsdm_ch7awscdr;
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch7wdatr](dfsdm_ch7wdatr) module"]
pub type DFSDM_CH7WDATR = crate::Reg<u32, _DFSDM_CH7WDATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH7WDATR;
#[doc = "`read()` method returns [dfsdm_ch7wdatr::R](dfsdm_ch7wdatr::R) reader structure"]
impl crate::Readable for DFSDM_CH7WDATR {}
#[doc = "This register contains the data resulting from the analog watchdog filter associated to the input channel y (y = 0..7)."]
pub mod dfsdm_ch7wdatr;
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_ch7datinr](dfsdm_ch7datinr) module"]
pub type DFSDM_CH7DATINR = crate::Reg<u32, _DFSDM_CH7DATINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_CH7DATINR;
#[doc = "`read()` method returns [dfsdm_ch7datinr::R](dfsdm_ch7datinr::R) reader structure"]
impl crate::Readable for DFSDM_CH7DATINR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_ch7datinr::W](dfsdm_ch7datinr::W) writer structure"]
impl crate::Writable for DFSDM_CH7DATINR {}
#[doc = "This register contains 16-bit input data to be processed by DFSDM filter module."]
pub mod dfsdm_ch7datinr;
#[doc = "DFSDM control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0cr1](dfsdm_flt0cr1) module"]
pub type DFSDM_FLT0CR1 = crate::Reg<u32, _DFSDM_FLT0CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0CR1;
#[doc = "`read()` method returns [dfsdm_flt0cr1::R](dfsdm_flt0cr1::R) reader structure"]
impl crate::Readable for DFSDM_FLT0CR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt0cr1::W](dfsdm_flt0cr1::W) writer structure"]
impl crate::Writable for DFSDM_FLT0CR1 {}
#[doc = "DFSDM control register 1"]
pub mod dfsdm_flt0cr1;
#[doc = "DFSDM control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0cr2](dfsdm_flt0cr2) module"]
pub type DFSDM_FLT0CR2 = crate::Reg<u32, _DFSDM_FLT0CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0CR2;
#[doc = "`read()` method returns [dfsdm_flt0cr2::R](dfsdm_flt0cr2::R) reader structure"]
impl crate::Readable for DFSDM_FLT0CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt0cr2::W](dfsdm_flt0cr2::W) writer structure"]
impl crate::Writable for DFSDM_FLT0CR2 {}
#[doc = "DFSDM control register 2"]
pub mod dfsdm_flt0cr2;
#[doc = "DFSDM interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0isr](dfsdm_flt0isr) module"]
pub type DFSDM_FLT0ISR = crate::Reg<u32, _DFSDM_FLT0ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0ISR;
#[doc = "`read()` method returns [dfsdm_flt0isr::R](dfsdm_flt0isr::R) reader structure"]
impl crate::Readable for DFSDM_FLT0ISR {}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm_flt0isr;
#[doc = "DFSDM interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0icr](dfsdm_flt0icr) module"]
pub type DFSDM_FLT0ICR = crate::Reg<u32, _DFSDM_FLT0ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0ICR;
#[doc = "`read()` method returns [dfsdm_flt0icr::R](dfsdm_flt0icr::R) reader structure"]
impl crate::Readable for DFSDM_FLT0ICR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt0icr::W](dfsdm_flt0icr::W) writer structure"]
impl crate::Writable for DFSDM_FLT0ICR {}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm_flt0icr;
#[doc = "DFSDM injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0jchgr](dfsdm_flt0jchgr) module"]
pub type DFSDM_FLT0JCHGR = crate::Reg<u32, _DFSDM_FLT0JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0JCHGR;
#[doc = "`read()` method returns [dfsdm_flt0jchgr::R](dfsdm_flt0jchgr::R) reader structure"]
impl crate::Readable for DFSDM_FLT0JCHGR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt0jchgr::W](dfsdm_flt0jchgr::W) writer structure"]
impl crate::Writable for DFSDM_FLT0JCHGR {}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm_flt0jchgr;
#[doc = "DFSDM filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0fcr](dfsdm_flt0fcr) module"]
pub type DFSDM_FLT0FCR = crate::Reg<u32, _DFSDM_FLT0FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0FCR;
#[doc = "`read()` method returns [dfsdm_flt0fcr::R](dfsdm_flt0fcr::R) reader structure"]
impl crate::Readable for DFSDM_FLT0FCR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt0fcr::W](dfsdm_flt0fcr::W) writer structure"]
impl crate::Writable for DFSDM_FLT0FCR {}
#[doc = "DFSDM filter control register"]
pub mod dfsdm_flt0fcr;
#[doc = "DFSDM data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0jdatar](dfsdm_flt0jdatar) module"]
pub type DFSDM_FLT0JDATAR = crate::Reg<u32, _DFSDM_FLT0JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0JDATAR;
#[doc = "`read()` method returns [dfsdm_flt0jdatar::R](dfsdm_flt0jdatar::R) reader structure"]
impl crate::Readable for DFSDM_FLT0JDATAR {}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm_flt0jdatar;
#[doc = "DFSDM data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0rdatar](dfsdm_flt0rdatar) module"]
pub type DFSDM_FLT0RDATAR = crate::Reg<u32, _DFSDM_FLT0RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0RDATAR;
#[doc = "`read()` method returns [dfsdm_flt0rdatar::R](dfsdm_flt0rdatar::R) reader structure"]
impl crate::Readable for DFSDM_FLT0RDATAR {}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm_flt0rdatar;
#[doc = "DFSDM analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0awhtr](dfsdm_flt0awhtr) module"]
pub type DFSDM_FLT0AWHTR = crate::Reg<u32, _DFSDM_FLT0AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0AWHTR;
#[doc = "`read()` method returns [dfsdm_flt0awhtr::R](dfsdm_flt0awhtr::R) reader structure"]
impl crate::Readable for DFSDM_FLT0AWHTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt0awhtr::W](dfsdm_flt0awhtr::W) writer structure"]
impl crate::Writable for DFSDM_FLT0AWHTR {}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm_flt0awhtr;
#[doc = "DFSDM analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0awltr](dfsdm_flt0awltr) module"]
pub type DFSDM_FLT0AWLTR = crate::Reg<u32, _DFSDM_FLT0AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0AWLTR;
#[doc = "`read()` method returns [dfsdm_flt0awltr::R](dfsdm_flt0awltr::R) reader structure"]
impl crate::Readable for DFSDM_FLT0AWLTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt0awltr::W](dfsdm_flt0awltr::W) writer structure"]
impl crate::Writable for DFSDM_FLT0AWLTR {}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm_flt0awltr;
#[doc = "DFSDM analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0awsr](dfsdm_flt0awsr) module"]
pub type DFSDM_FLT0AWSR = crate::Reg<u32, _DFSDM_FLT0AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0AWSR;
#[doc = "`read()` method returns [dfsdm_flt0awsr::R](dfsdm_flt0awsr::R) reader structure"]
impl crate::Readable for DFSDM_FLT0AWSR {}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm_flt0awsr;
#[doc = "DFSDM analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0awcfr](dfsdm_flt0awcfr) module"]
pub type DFSDM_FLT0AWCFR = crate::Reg<u32, _DFSDM_FLT0AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0AWCFR;
#[doc = "`read()` method returns [dfsdm_flt0awcfr::R](dfsdm_flt0awcfr::R) reader structure"]
impl crate::Readable for DFSDM_FLT0AWCFR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt0awcfr::W](dfsdm_flt0awcfr::W) writer structure"]
impl crate::Writable for DFSDM_FLT0AWCFR {}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm_flt0awcfr;
#[doc = "DFSDM Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0exmax](dfsdm_flt0exmax) module"]
pub type DFSDM_FLT0EXMAX = crate::Reg<u32, _DFSDM_FLT0EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0EXMAX;
#[doc = "`read()` method returns [dfsdm_flt0exmax::R](dfsdm_flt0exmax::R) reader structure"]
impl crate::Readable for DFSDM_FLT0EXMAX {}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm_flt0exmax;
#[doc = "DFSDM Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0exmin](dfsdm_flt0exmin) module"]
pub type DFSDM_FLT0EXMIN = crate::Reg<u32, _DFSDM_FLT0EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0EXMIN;
#[doc = "`read()` method returns [dfsdm_flt0exmin::R](dfsdm_flt0exmin::R) reader structure"]
impl crate::Readable for DFSDM_FLT0EXMIN {}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm_flt0exmin;
#[doc = "DFSDM conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0cnvtimr](dfsdm_flt0cnvtimr) module"]
pub type DFSDM_FLT0CNVTIMR = crate::Reg<u32, _DFSDM_FLT0CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT0CNVTIMR;
#[doc = "`read()` method returns [dfsdm_flt0cnvtimr::R](dfsdm_flt0cnvtimr::R) reader structure"]
impl crate::Readable for DFSDM_FLT0CNVTIMR {}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm_flt0cnvtimr;
#[doc = "DFSDM control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1cr1](dfsdm_flt1cr1) module"]
pub type DFSDM_FLT1CR1 = crate::Reg<u32, _DFSDM_FLT1CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1CR1;
#[doc = "`read()` method returns [dfsdm_flt1cr1::R](dfsdm_flt1cr1::R) reader structure"]
impl crate::Readable for DFSDM_FLT1CR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt1cr1::W](dfsdm_flt1cr1::W) writer structure"]
impl crate::Writable for DFSDM_FLT1CR1 {}
#[doc = "DFSDM control register 1"]
pub mod dfsdm_flt1cr1;
#[doc = "DFSDM control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1cr2](dfsdm_flt1cr2) module"]
pub type DFSDM_FLT1CR2 = crate::Reg<u32, _DFSDM_FLT1CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1CR2;
#[doc = "`read()` method returns [dfsdm_flt1cr2::R](dfsdm_flt1cr2::R) reader structure"]
impl crate::Readable for DFSDM_FLT1CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt1cr2::W](dfsdm_flt1cr2::W) writer structure"]
impl crate::Writable for DFSDM_FLT1CR2 {}
#[doc = "DFSDM control register 2"]
pub mod dfsdm_flt1cr2;
#[doc = "DFSDM interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1isr](dfsdm_flt1isr) module"]
pub type DFSDM_FLT1ISR = crate::Reg<u32, _DFSDM_FLT1ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1ISR;
#[doc = "`read()` method returns [dfsdm_flt1isr::R](dfsdm_flt1isr::R) reader structure"]
impl crate::Readable for DFSDM_FLT1ISR {}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm_flt1isr;
#[doc = "DFSDM interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1icr](dfsdm_flt1icr) module"]
pub type DFSDM_FLT1ICR = crate::Reg<u32, _DFSDM_FLT1ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1ICR;
#[doc = "`read()` method returns [dfsdm_flt1icr::R](dfsdm_flt1icr::R) reader structure"]
impl crate::Readable for DFSDM_FLT1ICR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt1icr::W](dfsdm_flt1icr::W) writer structure"]
impl crate::Writable for DFSDM_FLT1ICR {}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm_flt1icr;
#[doc = "DFSDM injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1jchgr](dfsdm_flt1jchgr) module"]
pub type DFSDM_FLT1JCHGR = crate::Reg<u32, _DFSDM_FLT1JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1JCHGR;
#[doc = "`read()` method returns [dfsdm_flt1jchgr::R](dfsdm_flt1jchgr::R) reader structure"]
impl crate::Readable for DFSDM_FLT1JCHGR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt1jchgr::W](dfsdm_flt1jchgr::W) writer structure"]
impl crate::Writable for DFSDM_FLT1JCHGR {}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm_flt1jchgr;
#[doc = "DFSDM filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1fcr](dfsdm_flt1fcr) module"]
pub type DFSDM_FLT1FCR = crate::Reg<u32, _DFSDM_FLT1FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1FCR;
#[doc = "`read()` method returns [dfsdm_flt1fcr::R](dfsdm_flt1fcr::R) reader structure"]
impl crate::Readable for DFSDM_FLT1FCR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt1fcr::W](dfsdm_flt1fcr::W) writer structure"]
impl crate::Writable for DFSDM_FLT1FCR {}
#[doc = "DFSDM filter control register"]
pub mod dfsdm_flt1fcr;
#[doc = "DFSDM data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1jdatar](dfsdm_flt1jdatar) module"]
pub type DFSDM_FLT1JDATAR = crate::Reg<u32, _DFSDM_FLT1JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1JDATAR;
#[doc = "`read()` method returns [dfsdm_flt1jdatar::R](dfsdm_flt1jdatar::R) reader structure"]
impl crate::Readable for DFSDM_FLT1JDATAR {}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm_flt1jdatar;
#[doc = "DFSDM data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1rdatar](dfsdm_flt1rdatar) module"]
pub type DFSDM_FLT1RDATAR = crate::Reg<u32, _DFSDM_FLT1RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1RDATAR;
#[doc = "`read()` method returns [dfsdm_flt1rdatar::R](dfsdm_flt1rdatar::R) reader structure"]
impl crate::Readable for DFSDM_FLT1RDATAR {}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm_flt1rdatar;
#[doc = "DFSDM analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1awhtr](dfsdm_flt1awhtr) module"]
pub type DFSDM_FLT1AWHTR = crate::Reg<u32, _DFSDM_FLT1AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1AWHTR;
#[doc = "`read()` method returns [dfsdm_flt1awhtr::R](dfsdm_flt1awhtr::R) reader structure"]
impl crate::Readable for DFSDM_FLT1AWHTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt1awhtr::W](dfsdm_flt1awhtr::W) writer structure"]
impl crate::Writable for DFSDM_FLT1AWHTR {}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm_flt1awhtr;
#[doc = "DFSDM analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1awltr](dfsdm_flt1awltr) module"]
pub type DFSDM_FLT1AWLTR = crate::Reg<u32, _DFSDM_FLT1AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1AWLTR;
#[doc = "`read()` method returns [dfsdm_flt1awltr::R](dfsdm_flt1awltr::R) reader structure"]
impl crate::Readable for DFSDM_FLT1AWLTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt1awltr::W](dfsdm_flt1awltr::W) writer structure"]
impl crate::Writable for DFSDM_FLT1AWLTR {}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm_flt1awltr;
#[doc = "DFSDM analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1awsr](dfsdm_flt1awsr) module"]
pub type DFSDM_FLT1AWSR = crate::Reg<u32, _DFSDM_FLT1AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1AWSR;
#[doc = "`read()` method returns [dfsdm_flt1awsr::R](dfsdm_flt1awsr::R) reader structure"]
impl crate::Readable for DFSDM_FLT1AWSR {}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm_flt1awsr;
#[doc = "DFSDM analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1awcfr](dfsdm_flt1awcfr) module"]
pub type DFSDM_FLT1AWCFR = crate::Reg<u32, _DFSDM_FLT1AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1AWCFR;
#[doc = "`read()` method returns [dfsdm_flt1awcfr::R](dfsdm_flt1awcfr::R) reader structure"]
impl crate::Readable for DFSDM_FLT1AWCFR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt1awcfr::W](dfsdm_flt1awcfr::W) writer structure"]
impl crate::Writable for DFSDM_FLT1AWCFR {}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm_flt1awcfr;
#[doc = "DFSDM Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1exmax](dfsdm_flt1exmax) module"]
pub type DFSDM_FLT1EXMAX = crate::Reg<u32, _DFSDM_FLT1EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1EXMAX;
#[doc = "`read()` method returns [dfsdm_flt1exmax::R](dfsdm_flt1exmax::R) reader structure"]
impl crate::Readable for DFSDM_FLT1EXMAX {}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm_flt1exmax;
#[doc = "DFSDM Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1exmin](dfsdm_flt1exmin) module"]
pub type DFSDM_FLT1EXMIN = crate::Reg<u32, _DFSDM_FLT1EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1EXMIN;
#[doc = "`read()` method returns [dfsdm_flt1exmin::R](dfsdm_flt1exmin::R) reader structure"]
impl crate::Readable for DFSDM_FLT1EXMIN {}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm_flt1exmin;
#[doc = "DFSDM conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1cnvtimr](dfsdm_flt1cnvtimr) module"]
pub type DFSDM_FLT1CNVTIMR = crate::Reg<u32, _DFSDM_FLT1CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT1CNVTIMR;
#[doc = "`read()` method returns [dfsdm_flt1cnvtimr::R](dfsdm_flt1cnvtimr::R) reader structure"]
impl crate::Readable for DFSDM_FLT1CNVTIMR {}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm_flt1cnvtimr;
#[doc = "DFSDM control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2cr1](dfsdm_flt2cr1) module"]
pub type DFSDM_FLT2CR1 = crate::Reg<u32, _DFSDM_FLT2CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2CR1;
#[doc = "`read()` method returns [dfsdm_flt2cr1::R](dfsdm_flt2cr1::R) reader structure"]
impl crate::Readable for DFSDM_FLT2CR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt2cr1::W](dfsdm_flt2cr1::W) writer structure"]
impl crate::Writable for DFSDM_FLT2CR1 {}
#[doc = "DFSDM control register 1"]
pub mod dfsdm_flt2cr1;
#[doc = "DFSDM control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2cr2](dfsdm_flt2cr2) module"]
pub type DFSDM_FLT2CR2 = crate::Reg<u32, _DFSDM_FLT2CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2CR2;
#[doc = "`read()` method returns [dfsdm_flt2cr2::R](dfsdm_flt2cr2::R) reader structure"]
impl crate::Readable for DFSDM_FLT2CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt2cr2::W](dfsdm_flt2cr2::W) writer structure"]
impl crate::Writable for DFSDM_FLT2CR2 {}
#[doc = "DFSDM control register 2"]
pub mod dfsdm_flt2cr2;
#[doc = "DFSDM interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2isr](dfsdm_flt2isr) module"]
pub type DFSDM_FLT2ISR = crate::Reg<u32, _DFSDM_FLT2ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2ISR;
#[doc = "`read()` method returns [dfsdm_flt2isr::R](dfsdm_flt2isr::R) reader structure"]
impl crate::Readable for DFSDM_FLT2ISR {}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm_flt2isr;
#[doc = "DFSDM interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2icr](dfsdm_flt2icr) module"]
pub type DFSDM_FLT2ICR = crate::Reg<u32, _DFSDM_FLT2ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2ICR;
#[doc = "`read()` method returns [dfsdm_flt2icr::R](dfsdm_flt2icr::R) reader structure"]
impl crate::Readable for DFSDM_FLT2ICR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt2icr::W](dfsdm_flt2icr::W) writer structure"]
impl crate::Writable for DFSDM_FLT2ICR {}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm_flt2icr;
#[doc = "DFSDM injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2jchgr](dfsdm_flt2jchgr) module"]
pub type DFSDM_FLT2JCHGR = crate::Reg<u32, _DFSDM_FLT2JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2JCHGR;
#[doc = "`read()` method returns [dfsdm_flt2jchgr::R](dfsdm_flt2jchgr::R) reader structure"]
impl crate::Readable for DFSDM_FLT2JCHGR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt2jchgr::W](dfsdm_flt2jchgr::W) writer structure"]
impl crate::Writable for DFSDM_FLT2JCHGR {}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm_flt2jchgr;
#[doc = "DFSDM filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2fcr](dfsdm_flt2fcr) module"]
pub type DFSDM_FLT2FCR = crate::Reg<u32, _DFSDM_FLT2FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2FCR;
#[doc = "`read()` method returns [dfsdm_flt2fcr::R](dfsdm_flt2fcr::R) reader structure"]
impl crate::Readable for DFSDM_FLT2FCR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt2fcr::W](dfsdm_flt2fcr::W) writer structure"]
impl crate::Writable for DFSDM_FLT2FCR {}
#[doc = "DFSDM filter control register"]
pub mod dfsdm_flt2fcr;
#[doc = "DFSDM data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2jdatar](dfsdm_flt2jdatar) module"]
pub type DFSDM_FLT2JDATAR = crate::Reg<u32, _DFSDM_FLT2JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2JDATAR;
#[doc = "`read()` method returns [dfsdm_flt2jdatar::R](dfsdm_flt2jdatar::R) reader structure"]
impl crate::Readable for DFSDM_FLT2JDATAR {}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm_flt2jdatar;
#[doc = "DFSDM data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2rdatar](dfsdm_flt2rdatar) module"]
pub type DFSDM_FLT2RDATAR = crate::Reg<u32, _DFSDM_FLT2RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2RDATAR;
#[doc = "`read()` method returns [dfsdm_flt2rdatar::R](dfsdm_flt2rdatar::R) reader structure"]
impl crate::Readable for DFSDM_FLT2RDATAR {}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm_flt2rdatar;
#[doc = "DFSDM analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2awhtr](dfsdm_flt2awhtr) module"]
pub type DFSDM_FLT2AWHTR = crate::Reg<u32, _DFSDM_FLT2AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2AWHTR;
#[doc = "`read()` method returns [dfsdm_flt2awhtr::R](dfsdm_flt2awhtr::R) reader structure"]
impl crate::Readable for DFSDM_FLT2AWHTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt2awhtr::W](dfsdm_flt2awhtr::W) writer structure"]
impl crate::Writable for DFSDM_FLT2AWHTR {}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm_flt2awhtr;
#[doc = "DFSDM analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2awltr](dfsdm_flt2awltr) module"]
pub type DFSDM_FLT2AWLTR = crate::Reg<u32, _DFSDM_FLT2AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2AWLTR;
#[doc = "`read()` method returns [dfsdm_flt2awltr::R](dfsdm_flt2awltr::R) reader structure"]
impl crate::Readable for DFSDM_FLT2AWLTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt2awltr::W](dfsdm_flt2awltr::W) writer structure"]
impl crate::Writable for DFSDM_FLT2AWLTR {}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm_flt2awltr;
#[doc = "DFSDM analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2awsr](dfsdm_flt2awsr) module"]
pub type DFSDM_FLT2AWSR = crate::Reg<u32, _DFSDM_FLT2AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2AWSR;
#[doc = "`read()` method returns [dfsdm_flt2awsr::R](dfsdm_flt2awsr::R) reader structure"]
impl crate::Readable for DFSDM_FLT2AWSR {}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm_flt2awsr;
#[doc = "DFSDM analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2awcfr](dfsdm_flt2awcfr) module"]
pub type DFSDM_FLT2AWCFR = crate::Reg<u32, _DFSDM_FLT2AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2AWCFR;
#[doc = "`read()` method returns [dfsdm_flt2awcfr::R](dfsdm_flt2awcfr::R) reader structure"]
impl crate::Readable for DFSDM_FLT2AWCFR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt2awcfr::W](dfsdm_flt2awcfr::W) writer structure"]
impl crate::Writable for DFSDM_FLT2AWCFR {}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm_flt2awcfr;
#[doc = "DFSDM Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2exmax](dfsdm_flt2exmax) module"]
pub type DFSDM_FLT2EXMAX = crate::Reg<u32, _DFSDM_FLT2EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2EXMAX;
#[doc = "`read()` method returns [dfsdm_flt2exmax::R](dfsdm_flt2exmax::R) reader structure"]
impl crate::Readable for DFSDM_FLT2EXMAX {}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm_flt2exmax;
#[doc = "DFSDM Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2exmin](dfsdm_flt2exmin) module"]
pub type DFSDM_FLT2EXMIN = crate::Reg<u32, _DFSDM_FLT2EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2EXMIN;
#[doc = "`read()` method returns [dfsdm_flt2exmin::R](dfsdm_flt2exmin::R) reader structure"]
impl crate::Readable for DFSDM_FLT2EXMIN {}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm_flt2exmin;
#[doc = "DFSDM conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt2cnvtimr](dfsdm_flt2cnvtimr) module"]
pub type DFSDM_FLT2CNVTIMR = crate::Reg<u32, _DFSDM_FLT2CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT2CNVTIMR;
#[doc = "`read()` method returns [dfsdm_flt2cnvtimr::R](dfsdm_flt2cnvtimr::R) reader structure"]
impl crate::Readable for DFSDM_FLT2CNVTIMR {}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm_flt2cnvtimr;
#[doc = "DFSDM control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3cr1](dfsdm_flt3cr1) module"]
pub type DFSDM_FLT3CR1 = crate::Reg<u32, _DFSDM_FLT3CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3CR1;
#[doc = "`read()` method returns [dfsdm_flt3cr1::R](dfsdm_flt3cr1::R) reader structure"]
impl crate::Readable for DFSDM_FLT3CR1 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt3cr1::W](dfsdm_flt3cr1::W) writer structure"]
impl crate::Writable for DFSDM_FLT3CR1 {}
#[doc = "DFSDM control register 1"]
pub mod dfsdm_flt3cr1;
#[doc = "DFSDM control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3cr2](dfsdm_flt3cr2) module"]
pub type DFSDM_FLT3CR2 = crate::Reg<u32, _DFSDM_FLT3CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3CR2;
#[doc = "`read()` method returns [dfsdm_flt3cr2::R](dfsdm_flt3cr2::R) reader structure"]
impl crate::Readable for DFSDM_FLT3CR2 {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt3cr2::W](dfsdm_flt3cr2::W) writer structure"]
impl crate::Writable for DFSDM_FLT3CR2 {}
#[doc = "DFSDM control register 2"]
pub mod dfsdm_flt3cr2;
#[doc = "DFSDM interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3isr](dfsdm_flt3isr) module"]
pub type DFSDM_FLT3ISR = crate::Reg<u32, _DFSDM_FLT3ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3ISR;
#[doc = "`read()` method returns [dfsdm_flt3isr::R](dfsdm_flt3isr::R) reader structure"]
impl crate::Readable for DFSDM_FLT3ISR {}
#[doc = "DFSDM interrupt and status register"]
pub mod dfsdm_flt3isr;
#[doc = "DFSDM interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3icr](dfsdm_flt3icr) module"]
pub type DFSDM_FLT3ICR = crate::Reg<u32, _DFSDM_FLT3ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3ICR;
#[doc = "`read()` method returns [dfsdm_flt3icr::R](dfsdm_flt3icr::R) reader structure"]
impl crate::Readable for DFSDM_FLT3ICR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt3icr::W](dfsdm_flt3icr::W) writer structure"]
impl crate::Writable for DFSDM_FLT3ICR {}
#[doc = "DFSDM interrupt flag clear register"]
pub mod dfsdm_flt3icr;
#[doc = "DFSDM injected channel group selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3jchgr](dfsdm_flt3jchgr) module"]
pub type DFSDM_FLT3JCHGR = crate::Reg<u32, _DFSDM_FLT3JCHGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3JCHGR;
#[doc = "`read()` method returns [dfsdm_flt3jchgr::R](dfsdm_flt3jchgr::R) reader structure"]
impl crate::Readable for DFSDM_FLT3JCHGR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt3jchgr::W](dfsdm_flt3jchgr::W) writer structure"]
impl crate::Writable for DFSDM_FLT3JCHGR {}
#[doc = "DFSDM injected channel group selection register"]
pub mod dfsdm_flt3jchgr;
#[doc = "DFSDM filter control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3fcr](dfsdm_flt3fcr) module"]
pub type DFSDM_FLT3FCR = crate::Reg<u32, _DFSDM_FLT3FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3FCR;
#[doc = "`read()` method returns [dfsdm_flt3fcr::R](dfsdm_flt3fcr::R) reader structure"]
impl crate::Readable for DFSDM_FLT3FCR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt3fcr::W](dfsdm_flt3fcr::W) writer structure"]
impl crate::Writable for DFSDM_FLT3FCR {}
#[doc = "DFSDM filter control register"]
pub mod dfsdm_flt3fcr;
#[doc = "DFSDM data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3jdatar](dfsdm_flt3jdatar) module"]
pub type DFSDM_FLT3JDATAR = crate::Reg<u32, _DFSDM_FLT3JDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3JDATAR;
#[doc = "`read()` method returns [dfsdm_flt3jdatar::R](dfsdm_flt3jdatar::R) reader structure"]
impl crate::Readable for DFSDM_FLT3JDATAR {}
#[doc = "DFSDM data register for injected group"]
pub mod dfsdm_flt3jdatar;
#[doc = "DFSDM data register for the regular channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3rdatar](dfsdm_flt3rdatar) module"]
pub type DFSDM_FLT3RDATAR = crate::Reg<u32, _DFSDM_FLT3RDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3RDATAR;
#[doc = "`read()` method returns [dfsdm_flt3rdatar::R](dfsdm_flt3rdatar::R) reader structure"]
impl crate::Readable for DFSDM_FLT3RDATAR {}
#[doc = "DFSDM data register for the regular channel"]
pub mod dfsdm_flt3rdatar;
#[doc = "DFSDM analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3awhtr](dfsdm_flt3awhtr) module"]
pub type DFSDM_FLT3AWHTR = crate::Reg<u32, _DFSDM_FLT3AWHTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3AWHTR;
#[doc = "`read()` method returns [dfsdm_flt3awhtr::R](dfsdm_flt3awhtr::R) reader structure"]
impl crate::Readable for DFSDM_FLT3AWHTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt3awhtr::W](dfsdm_flt3awhtr::W) writer structure"]
impl crate::Writable for DFSDM_FLT3AWHTR {}
#[doc = "DFSDM analog watchdog high threshold register"]
pub mod dfsdm_flt3awhtr;
#[doc = "DFSDM analog watchdog low threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3awltr](dfsdm_flt3awltr) module"]
pub type DFSDM_FLT3AWLTR = crate::Reg<u32, _DFSDM_FLT3AWLTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3AWLTR;
#[doc = "`read()` method returns [dfsdm_flt3awltr::R](dfsdm_flt3awltr::R) reader structure"]
impl crate::Readable for DFSDM_FLT3AWLTR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt3awltr::W](dfsdm_flt3awltr::W) writer structure"]
impl crate::Writable for DFSDM_FLT3AWLTR {}
#[doc = "DFSDM analog watchdog low threshold register"]
pub mod dfsdm_flt3awltr;
#[doc = "DFSDM analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3awsr](dfsdm_flt3awsr) module"]
pub type DFSDM_FLT3AWSR = crate::Reg<u32, _DFSDM_FLT3AWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3AWSR;
#[doc = "`read()` method returns [dfsdm_flt3awsr::R](dfsdm_flt3awsr::R) reader structure"]
impl crate::Readable for DFSDM_FLT3AWSR {}
#[doc = "DFSDM analog watchdog status register"]
pub mod dfsdm_flt3awsr;
#[doc = "DFSDM analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3awcfr](dfsdm_flt3awcfr) module"]
pub type DFSDM_FLT3AWCFR = crate::Reg<u32, _DFSDM_FLT3AWCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3AWCFR;
#[doc = "`read()` method returns [dfsdm_flt3awcfr::R](dfsdm_flt3awcfr::R) reader structure"]
impl crate::Readable for DFSDM_FLT3AWCFR {}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt3awcfr::W](dfsdm_flt3awcfr::W) writer structure"]
impl crate::Writable for DFSDM_FLT3AWCFR {}
#[doc = "DFSDM analog watchdog clear flag register"]
pub mod dfsdm_flt3awcfr;
#[doc = "DFSDM Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3exmax](dfsdm_flt3exmax) module"]
pub type DFSDM_FLT3EXMAX = crate::Reg<u32, _DFSDM_FLT3EXMAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3EXMAX;
#[doc = "`read()` method returns [dfsdm_flt3exmax::R](dfsdm_flt3exmax::R) reader structure"]
impl crate::Readable for DFSDM_FLT3EXMAX {}
#[doc = "DFSDM Extremes detector maximum register"]
pub mod dfsdm_flt3exmax;
#[doc = "DFSDM Extremes detector minimum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3exmin](dfsdm_flt3exmin) module"]
pub type DFSDM_FLT3EXMIN = crate::Reg<u32, _DFSDM_FLT3EXMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3EXMIN;
#[doc = "`read()` method returns [dfsdm_flt3exmin::R](dfsdm_flt3exmin::R) reader structure"]
impl crate::Readable for DFSDM_FLT3EXMIN {}
#[doc = "DFSDM Extremes detector minimum register"]
pub mod dfsdm_flt3exmin;
#[doc = "DFSDM conversion timer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt3cnvtimr](dfsdm_flt3cnvtimr) module"]
pub type DFSDM_FLT3CNVTIMR = crate::Reg<u32, _DFSDM_FLT3CNVTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSDM_FLT3CNVTIMR;
#[doc = "`read()` method returns [dfsdm_flt3cnvtimr::R](dfsdm_flt3cnvtimr::R) reader structure"]
impl crate::Readable for DFSDM_FLT3CNVTIMR {}
#[doc = "DFSDM conversion timer register"]
pub mod dfsdm_flt3cnvtimr;
