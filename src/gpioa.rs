#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub gpiox_moder: GPIOX_MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub gpiox_otyper: GPIOX_OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub gpiox_ospeedr: GPIOX_OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub gpiox_pupdr: GPIOX_PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub gpiox_idr: GPIOX_IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub gpiox_odr: GPIOX_ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub gpiox_bsrr: GPIOX_BSRR,
    #[doc = "0x1c - This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
    pub gpiox_lckr: GPIOX_LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub gpiox_afrl: GPIOX_AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub gpiox_afrh: GPIOX_AFRH,
    _reserved10: [u8; 972usize],
    #[doc = "0x3f4 - GPIO version register"]
    pub gpiox_verr: GPIOX_VERR,
    #[doc = "0x3f8 - GPIO identification register"]
    pub gpiox_ipidr: GPIOX_IPIDR,
    #[doc = "0x3fc - GPIO size identification register"]
    pub gpiox_sidr: GPIOX_SIDR,
}
#[doc = "GPIO port mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_moder](gpiox_moder) module"]
pub type GPIOX_MODER = crate::Reg<u32, _GPIOX_MODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_MODER;
#[doc = "`read()` method returns [gpiox_moder::R](gpiox_moder::R) reader structure"]
impl crate::Readable for GPIOX_MODER {}
#[doc = "`write(|w| ..)` method takes [gpiox_moder::W](gpiox_moder::W) writer structure"]
impl crate::Writable for GPIOX_MODER {}
#[doc = "GPIO port mode register"]
pub mod gpiox_moder;
#[doc = "GPIO port output type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_otyper](gpiox_otyper) module"]
pub type GPIOX_OTYPER = crate::Reg<u32, _GPIOX_OTYPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_OTYPER;
#[doc = "`read()` method returns [gpiox_otyper::R](gpiox_otyper::R) reader structure"]
impl crate::Readable for GPIOX_OTYPER {}
#[doc = "`write(|w| ..)` method takes [gpiox_otyper::W](gpiox_otyper::W) writer structure"]
impl crate::Writable for GPIOX_OTYPER {}
#[doc = "GPIO port output type register"]
pub mod gpiox_otyper;
#[doc = "GPIO port output speed register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_ospeedr](gpiox_ospeedr) module"]
pub type GPIOX_OSPEEDR = crate::Reg<u32, _GPIOX_OSPEEDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_OSPEEDR;
#[doc = "`read()` method returns [gpiox_ospeedr::R](gpiox_ospeedr::R) reader structure"]
impl crate::Readable for GPIOX_OSPEEDR {}
#[doc = "`write(|w| ..)` method takes [gpiox_ospeedr::W](gpiox_ospeedr::W) writer structure"]
impl crate::Writable for GPIOX_OSPEEDR {}
#[doc = "GPIO port output speed register"]
pub mod gpiox_ospeedr;
#[doc = "GPIO port pull-up/pull-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_pupdr](gpiox_pupdr) module"]
pub type GPIOX_PUPDR = crate::Reg<u32, _GPIOX_PUPDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_PUPDR;
#[doc = "`read()` method returns [gpiox_pupdr::R](gpiox_pupdr::R) reader structure"]
impl crate::Readable for GPIOX_PUPDR {}
#[doc = "`write(|w| ..)` method takes [gpiox_pupdr::W](gpiox_pupdr::W) writer structure"]
impl crate::Writable for GPIOX_PUPDR {}
#[doc = "GPIO port pull-up/pull-down register"]
pub mod gpiox_pupdr;
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_idr](gpiox_idr) module"]
pub type GPIOX_IDR = crate::Reg<u32, _GPIOX_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_IDR;
#[doc = "`read()` method returns [gpiox_idr::R](gpiox_idr::R) reader structure"]
impl crate::Readable for GPIOX_IDR {}
#[doc = "GPIO port input data register"]
pub mod gpiox_idr;
#[doc = "GPIO port output data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_odr](gpiox_odr) module"]
pub type GPIOX_ODR = crate::Reg<u32, _GPIOX_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_ODR;
#[doc = "`read()` method returns [gpiox_odr::R](gpiox_odr::R) reader structure"]
impl crate::Readable for GPIOX_ODR {}
#[doc = "`write(|w| ..)` method takes [gpiox_odr::W](gpiox_odr::W) writer structure"]
impl crate::Writable for GPIOX_ODR {}
#[doc = "GPIO port output data register"]
pub mod gpiox_odr;
#[doc = "GPIO port bit set/reset register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_bsrr](gpiox_bsrr) module"]
pub type GPIOX_BSRR = crate::Reg<u32, _GPIOX_BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_BSRR;
#[doc = "`write(|w| ..)` method takes [gpiox_bsrr::W](gpiox_bsrr::W) writer structure"]
impl crate::Writable for GPIOX_BSRR {}
#[doc = "GPIO port bit set/reset register"]
pub mod gpiox_bsrr;
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_lckr](gpiox_lckr) module"]
pub type GPIOX_LCKR = crate::Reg<u32, _GPIOX_LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_LCKR;
#[doc = "`read()` method returns [gpiox_lckr::R](gpiox_lckr::R) reader structure"]
impl crate::Readable for GPIOX_LCKR {}
#[doc = "`write(|w| ..)` method takes [gpiox_lckr::W](gpiox_lckr::W) writer structure"]
impl crate::Writable for GPIOX_LCKR {}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers)."]
pub mod gpiox_lckr;
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_afrl](gpiox_afrl) module"]
pub type GPIOX_AFRL = crate::Reg<u32, _GPIOX_AFRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_AFRL;
#[doc = "`read()` method returns [gpiox_afrl::R](gpiox_afrl::R) reader structure"]
impl crate::Readable for GPIOX_AFRL {}
#[doc = "`write(|w| ..)` method takes [gpiox_afrl::W](gpiox_afrl::W) writer structure"]
impl crate::Writable for GPIOX_AFRL {}
#[doc = "GPIO alternate function low register"]
pub mod gpiox_afrl;
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_afrh](gpiox_afrh) module"]
pub type GPIOX_AFRH = crate::Reg<u32, _GPIOX_AFRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_AFRH;
#[doc = "`read()` method returns [gpiox_afrh::R](gpiox_afrh::R) reader structure"]
impl crate::Readable for GPIOX_AFRH {}
#[doc = "`write(|w| ..)` method takes [gpiox_afrh::W](gpiox_afrh::W) writer structure"]
impl crate::Writable for GPIOX_AFRH {}
#[doc = "GPIO alternate function high register"]
pub mod gpiox_afrh;
#[doc = "GPIO version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_verr](gpiox_verr) module"]
pub type GPIOX_VERR = crate::Reg<u32, _GPIOX_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_VERR;
#[doc = "`read()` method returns [gpiox_verr::R](gpiox_verr::R) reader structure"]
impl crate::Readable for GPIOX_VERR {}
#[doc = "GPIO version register"]
pub mod gpiox_verr;
#[doc = "GPIO identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_ipidr](gpiox_ipidr) module"]
pub type GPIOX_IPIDR = crate::Reg<u32, _GPIOX_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_IPIDR;
#[doc = "`read()` method returns [gpiox_ipidr::R](gpiox_ipidr::R) reader structure"]
impl crate::Readable for GPIOX_IPIDR {}
#[doc = "GPIO identification register"]
pub mod gpiox_ipidr;
#[doc = "GPIO size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiox_sidr](gpiox_sidr) module"]
pub type GPIOX_SIDR = crate::Reg<u32, _GPIOX_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOX_SIDR;
#[doc = "`read()` method returns [gpiox_sidr::R](gpiox_sidr::R) reader structure"]
impl crate::Readable for GPIOX_SIDR {}
#[doc = "GPIO size identification register"]
pub mod gpiox_sidr;
