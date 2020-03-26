#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - SPI configuration register 1"]
    pub cfg1: CFG1,
    #[doc = "0x0c - SPI configuration register 2"]
    pub cfg2: CFG2,
    #[doc = "0x10 - SPI/I2S interrupt enable register"]
    pub ier: IER,
    #[doc = "0x14 - status register"]
    pub sr: SR,
    #[doc = "0x18 - SPI/I2S interrupt/status flags clear register"]
    pub ifcr: IFCR,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - SPI/I2S transmit data register"]
    pub txdr: TXDR,
    _reserved8: [u8; 12usize],
    #[doc = "0x30 - SPI/I2S receive data register"]
    pub rxdr: RXDR,
    _reserved9: [u8; 12usize],
    #[doc = "0x40 - SPI polynomial register"]
    pub crcpoly: CRCPOLY,
    #[doc = "0x44 - SPI transmitter CRC register"]
    pub txcrc: TXCRC,
    #[doc = "0x48 - SPI receiver CRC register"]
    pub rxcrc: RXCRC,
    #[doc = "0x4c - SPI underrun data register"]
    pub udrdr: UDRDR,
    #[doc = "0x50 - SPI/I2S configuration register"]
    pub i2scgfr: I2SCGFR,
    _reserved14: [u8; 924usize],
    #[doc = "0x3f0 - SPI/I2S hardware configuration register"]
    pub hwcfgr: HWCFGR,
    #[doc = "0x3f4 - EXTI IP Version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - EXTI Identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - EXTI Size ID register"]
    pub sidr: SIDR,
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "control register 1"]
pub mod cr1;
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SPI configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "SPI configuration register 1"]
pub mod cfg1;
#[doc = "SPI configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](cfg2) module"]
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
#[doc = "`read()` method returns [cfg2::R](cfg2::R) reader structure"]
impl crate::Readable for CFG2 {}
#[doc = "`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure"]
impl crate::Writable for CFG2 {}
#[doc = "SPI configuration register 2"]
pub mod cfg2;
#[doc = "SPI/I2S interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "SPI/I2S interrupt enable register"]
pub mod ier;
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "status register"]
pub mod sr;
#[doc = "SPI/I2S interrupt/status flags clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](ifcr) module"]
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
#[doc = "`read()` method returns [ifcr::R](ifcr::R) reader structure"]
impl crate::Readable for IFCR {}
#[doc = "`write(|w| ..)` method takes [ifcr::W](ifcr::W) writer structure"]
impl crate::Writable for IFCR {}
#[doc = "SPI/I2S interrupt/status flags clear register"]
pub mod ifcr;
#[doc = "SPI/I2S transmit data register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdr](txdr) module"]
pub type TXDR = crate::Reg<u32, _TXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDR;
#[doc = "`write(|w| ..)` method takes [txdr::W](txdr::W) writer structure"]
impl crate::Writable for TXDR {}
#[doc = "SPI/I2S transmit data register"]
pub mod txdr;
#[doc = "SPI/I2S receive data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdr](rxdr) module"]
pub type RXDR = crate::Reg<u32, _RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDR;
#[doc = "`read()` method returns [rxdr::R](rxdr::R) reader structure"]
impl crate::Readable for RXDR {}
#[doc = "SPI/I2S receive data register"]
pub mod rxdr;
#[doc = "SPI polynomial register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcpoly](crcpoly) module"]
pub type CRCPOLY = crate::Reg<u32, _CRCPOLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRCPOLY;
#[doc = "`read()` method returns [crcpoly::R](crcpoly::R) reader structure"]
impl crate::Readable for CRCPOLY {}
#[doc = "`write(|w| ..)` method takes [crcpoly::W](crcpoly::W) writer structure"]
impl crate::Writable for CRCPOLY {}
#[doc = "SPI polynomial register"]
pub mod crcpoly;
#[doc = "SPI transmitter CRC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcrc](txcrc) module"]
pub type TXCRC = crate::Reg<u32, _TXCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCRC;
#[doc = "`read()` method returns [txcrc::R](txcrc::R) reader structure"]
impl crate::Readable for TXCRC {}
#[doc = "`write(|w| ..)` method takes [txcrc::W](txcrc::W) writer structure"]
impl crate::Writable for TXCRC {}
#[doc = "SPI transmitter CRC register"]
pub mod txcrc;
#[doc = "SPI receiver CRC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcrc](rxcrc) module"]
pub type RXCRC = crate::Reg<u32, _RXCRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCRC;
#[doc = "`read()` method returns [rxcrc::R](rxcrc::R) reader structure"]
impl crate::Readable for RXCRC {}
#[doc = "`write(|w| ..)` method takes [rxcrc::W](rxcrc::W) writer structure"]
impl crate::Writable for RXCRC {}
#[doc = "SPI receiver CRC register"]
pub mod rxcrc;
#[doc = "SPI underrun data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udrdr](udrdr) module"]
pub type UDRDR = crate::Reg<u32, _UDRDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDRDR;
#[doc = "`read()` method returns [udrdr::R](udrdr::R) reader structure"]
impl crate::Readable for UDRDR {}
#[doc = "`write(|w| ..)` method takes [udrdr::W](udrdr::W) writer structure"]
impl crate::Writable for UDRDR {}
#[doc = "SPI underrun data register"]
pub mod udrdr;
#[doc = "SPI/I2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2scgfr](i2scgfr) module"]
pub type I2SCGFR = crate::Reg<u32, _I2SCGFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SCGFR;
#[doc = "`read()` method returns [i2scgfr::R](i2scgfr::R) reader structure"]
impl crate::Readable for I2SCGFR {}
#[doc = "`write(|w| ..)` method takes [i2scgfr::W](i2scgfr::W) writer structure"]
impl crate::Writable for I2SCGFR {}
#[doc = "SPI/I2S configuration register"]
pub mod i2scgfr;
#[doc = "SPI/I2S hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr](hwcfgr) module"]
pub type HWCFGR = crate::Reg<u32, _HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR;
#[doc = "`read()` method returns [hwcfgr::R](hwcfgr::R) reader structure"]
impl crate::Readable for HWCFGR {}
#[doc = "SPI/I2S hardware configuration register"]
pub mod hwcfgr;
#[doc = "EXTI IP Version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verr](verr) module"]
pub type VERR = crate::Reg<u32, _VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERR;
#[doc = "`read()` method returns [verr::R](verr::R) reader structure"]
impl crate::Readable for VERR {}
#[doc = "EXTI IP Version register"]
pub mod verr;
#[doc = "EXTI Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipidr](ipidr) module"]
pub type IPIDR = crate::Reg<u32, _IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPIDR;
#[doc = "`read()` method returns [ipidr::R](ipidr::R) reader structure"]
impl crate::Readable for IPIDR {}
#[doc = "EXTI Identification register"]
pub mod ipidr;
#[doc = "EXTI Size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidr](sidr) module"]
pub type SIDR = crate::Reg<u32, _SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIDR;
#[doc = "`read()` method returns [sidr::R](sidr::R) reader structure"]
impl crate::Readable for SIDR {}
#[doc = "EXTI Size ID register"]
pub mod sidr;
