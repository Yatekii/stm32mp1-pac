#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RNG control register"]
    pub rng_cr: RNG_CR,
    #[doc = "0x04 - RNG status register"]
    pub rng_sr: RNG_SR,
    #[doc = "0x08 - The RNG_DR register is a read-only register that delivers a 32-bit random value when read. After being read this register delivers a new random value after 216 periods of AHB clock if the output FIFO is empty. The content of this register is valid when DRDY, even if RNGEN"]
    pub rng_dr: RNG_DR,
    _reserved3: [u8; 1000usize],
    #[doc = "0x3f4 - RNG Version Register"]
    pub rng_ver: RNG_VER,
    #[doc = "0x3f8 - RNG Identification"]
    pub rng_id: RNG_ID,
    #[doc = "0x3fc - RNG hardware magic ID"]
    pub rng_mid: RNG_MID,
}
#[doc = "RNG control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_cr](rng_cr) module"]
pub type RNG_CR = crate::Reg<u32, _RNG_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_CR;
#[doc = "`read()` method returns [rng_cr::R](rng_cr::R) reader structure"]
impl crate::Readable for RNG_CR {}
#[doc = "`write(|w| ..)` method takes [rng_cr::W](rng_cr::W) writer structure"]
impl crate::Writable for RNG_CR {}
#[doc = "RNG control register"]
pub mod rng_cr;
#[doc = "RNG status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_sr](rng_sr) module"]
pub type RNG_SR = crate::Reg<u32, _RNG_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_SR;
#[doc = "`read()` method returns [rng_sr::R](rng_sr::R) reader structure"]
impl crate::Readable for RNG_SR {}
#[doc = "`write(|w| ..)` method takes [rng_sr::W](rng_sr::W) writer structure"]
impl crate::Writable for RNG_SR {}
#[doc = "RNG status register"]
pub mod rng_sr;
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. After being read this register delivers a new random value after 216 periods of AHB clock if the output FIFO is empty. The content of this register is valid when DRDY, even if RNGEN\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_dr](rng_dr) module"]
pub type RNG_DR = crate::Reg<u32, _RNG_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_DR;
#[doc = "`read()` method returns [rng_dr::R](rng_dr::R) reader structure"]
impl crate::Readable for RNG_DR {}
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. After being read this register delivers a new random value after 216 periods of AHB clock if the output FIFO is empty. The content of this register is valid when DRDY, even if RNGEN"]
pub mod rng_dr;
#[doc = "RNG Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_ver](rng_ver) module"]
pub type RNG_VER = crate::Reg<u32, _RNG_VER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_VER;
#[doc = "`read()` method returns [rng_ver::R](rng_ver::R) reader structure"]
impl crate::Readable for RNG_VER {}
#[doc = "RNG Version Register"]
pub mod rng_ver;
#[doc = "RNG Identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_id](rng_id) module"]
pub type RNG_ID = crate::Reg<u32, _RNG_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_ID;
#[doc = "`read()` method returns [rng_id::R](rng_id::R) reader structure"]
impl crate::Readable for RNG_ID {}
#[doc = "RNG Identification"]
pub mod rng_id;
#[doc = "RNG hardware magic ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_mid](rng_mid) module"]
pub type RNG_MID = crate::Reg<u32, _RNG_MID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNG_MID;
#[doc = "`read()` method returns [rng_mid::R](rng_mid::R) reader structure"]
impl crate::Readable for RNG_MID {}
#[doc = "RNG hardware magic ID"]
pub mod rng_mid;
