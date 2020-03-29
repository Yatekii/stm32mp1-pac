#![doc = "Peripheral access API for STM32MP1_V0R3 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {
    fn FMC();
    fn DCMI();
    fn CRYP();
    fn HASH_RNG();
    fn IPCC_RX1();
    fn IPCC_TX1();
    fn HSEM0();
    fn SAI4();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 147] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FMC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: DCMI },
    Vector { _handler: CRYP },
    Vector { _handler: HASH_RNG },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: IPCC_RX1 },
    Vector { _handler: IPCC_TX1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: HSEM0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SAI4 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "48 - FMC global interrupt"]
    FMC = 48,
    #[doc = "78 - DCMI global interrupt"]
    DCMI = 78,
    #[doc = "79 - CRYP global interrupt"]
    CRYP = 79,
    #[doc = "80 - HASH and RNG"]
    HASH_RNG = 80,
    #[doc = "103 - IPCC RX1 occupied interrupt"]
    IPCC_RX1 = 103,
    #[doc = "104 - IPCC TX1 free interrupt"]
    IPCC_TX1 = 104,
    #[doc = "125 - HSEM global interrupt 1"]
    HSEM0 = 125,
    #[doc = "146 - SAI4 global interrupt"]
    SAI4 = 146,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "BSEC"]
pub struct BSEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BSEC {}
impl BSEC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bsec::RegisterBlock {
        0x5c00_5000 as *const _
    }
}
impl Deref for BSEC {
    type Target = bsec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BSEC::ptr() }
    }
}
#[doc = "BSEC"]
pub mod bsec;
#[doc = "DDRCTRL"]
pub struct DDRCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DDRCTRL {}
impl DDRCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ddrctrl::RegisterBlock {
        0x5a00_3000 as *const _
    }
}
impl Deref for DDRCTRL {
    type Target = ddrctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DDRCTRL::ptr() }
    }
}
#[doc = "DDRCTRL"]
pub mod ddrctrl;
#[doc = "DDRPHYC"]
pub struct DDRPHYC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DDRPHYC {}
impl DDRPHYC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ddrphyc::RegisterBlock {
        0x5a00_4000 as *const _
    }
}
impl Deref for DDRPHYC {
    type Target = ddrphyc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DDRPHYC::ptr() }
    }
}
#[doc = "DDRPHYC"]
pub mod ddrphyc;
#[doc = "DLYBQS"]
pub struct DLYBQS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DLYBQS {}
impl DLYBQS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dlybqs::RegisterBlock {
        0x5800_4000 as *const _
    }
}
impl Deref for DLYBQS {
    type Target = dlybqs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DLYBQS::ptr() }
    }
}
#[doc = "DLYBQS"]
pub mod dlybqs;
#[doc = "DLYBQS"]
pub struct DLYBSD1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DLYBSD1 {}
impl DLYBSD1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dlybqs::RegisterBlock {
        0x5800_6000 as *const _
    }
}
impl Deref for DLYBSD1 {
    type Target = dlybqs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DLYBSD1::ptr() }
    }
}
#[doc = "DLYBQS"]
pub struct DLYBSD2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DLYBSD2 {}
impl DLYBSD2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dlybqs::RegisterBlock {
        0x5800_8000 as *const _
    }
}
impl Deref for DLYBSD2 {
    type Target = dlybqs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DLYBSD2::ptr() }
    }
}
#[doc = "DLYBQS"]
pub struct DLYBSD3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DLYBSD3 {}
impl DLYBSD3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dlybqs::RegisterBlock {
        0x4800_5000 as *const _
    }
}
impl Deref for DLYBSD3 {
    type Target = dlybqs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DLYBSD3::ptr() }
    }
}
#[doc = "DFSDM1"]
pub struct DFSDM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DFSDM1 {}
impl DFSDM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dfsdm1::RegisterBlock {
        0x4400_d000 as *const _
    }
}
impl Deref for DFSDM1 {
    type Target = dfsdm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DFSDM1::ptr() }
    }
}
#[doc = "DFSDM1"]
pub mod dfsdm1;
#[doc = "DMAMUX1"]
pub struct DMAMUX1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAMUX1 {}
impl DMAMUX1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmamux1::RegisterBlock {
        0x4800_2000 as *const _
    }
}
impl Deref for DMAMUX1 {
    type Target = dmamux1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAMUX1::ptr() }
    }
}
#[doc = "DMAMUX1"]
pub mod dmamux1;
#[doc = "FDCAN1"]
pub struct FDCAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FDCAN1 {}
impl FDCAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fdcan1::RegisterBlock {
        0x4400_e000 as *const _
    }
}
impl Deref for FDCAN1 {
    type Target = fdcan1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FDCAN1::ptr() }
    }
}
#[doc = "FDCAN1"]
pub mod fdcan1;
#[doc = "FDCAN1"]
pub struct FDCAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FDCAN2 {}
impl FDCAN2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fdcan1::RegisterBlock {
        0x4400_f000 as *const _
    }
}
impl Deref for FDCAN2 {
    type Target = fdcan1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FDCAN2::ptr() }
    }
}
#[doc = "HDP"]
pub struct HDP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HDP {}
impl HDP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hdp::RegisterBlock {
        0x5002_a000 as *const _
    }
}
impl Deref for HDP {
    type Target = hdp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HDP::ptr() }
    }
}
#[doc = "HDP"]
pub mod hdp;
#[doc = "IWDG1"]
pub struct IWDG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG1 {}
impl IWDG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iwdg1::RegisterBlock {
        0x5c00_3000 as *const _
    }
}
impl Deref for IWDG1 {
    type Target = iwdg1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IWDG1::ptr() }
    }
}
#[doc = "IWDG1"]
pub mod iwdg1;
#[doc = "MDMA"]
pub struct MDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MDMA {}
impl MDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mdma::RegisterBlock {
        0x5800_0000 as *const _
    }
}
impl Deref for MDMA {
    type Target = mdma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MDMA::ptr() }
    }
}
#[doc = "MDMA"]
pub mod mdma;
#[doc = "QUADSPI"]
pub struct QUADSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QUADSPI {}
impl QUADSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const quadspi::RegisterBlock {
        0x5800_3000 as *const _
    }
}
impl Deref for QUADSPI {
    type Target = quadspi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QUADSPI::ptr() }
    }
}
#[doc = "QUADSPI"]
pub mod quadspi;
#[doc = "RCC"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcc::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "RCC"]
pub mod rcc;
#[doc = "RNG1"]
pub struct RNG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG1 {}
impl RNG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng1::RegisterBlock {
        0x5400_3000 as *const _
    }
}
impl Deref for RNG1 {
    type Target = rng1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG1::ptr() }
    }
}
#[doc = "RNG1"]
pub mod rng1;
#[doc = "RNG1"]
pub struct RNG2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG2 {}
impl RNG2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng1::RegisterBlock {
        0x4c00_3000 as *const _
    }
}
impl Deref for RNG2 {
    type Target = rng1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RNG2::ptr() }
    }
}
#[doc = "RTC register bank"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x5c00_4000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "RTC register bank"]
pub mod rtc;
#[doc = "STGEN"]
pub struct STGEN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STGEN {}
impl STGEN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const stgen::RegisterBlock {
        0x5c00_8000 as *const _
    }
}
impl Deref for STGEN {
    type Target = stgen::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*STGEN::ptr() }
    }
}
#[doc = "STGEN"]
pub mod stgen;
#[doc = "STGENR"]
pub struct STGENR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for STGENR {}
impl STGENR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const stgenr::RegisterBlock {
        0x5a00_5000 as *const _
    }
}
impl Deref for STGENR {
    type Target = stgenr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*STGENR::ptr() }
    }
}
#[doc = "STGENR"]
pub mod stgenr;
#[doc = "VREFBUF"]
pub struct VREFBUF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for VREFBUF {}
impl VREFBUF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const vrefbuf::RegisterBlock {
        0x5002_5000 as *const _
    }
}
impl Deref for VREFBUF {
    type Target = vrefbuf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*VREFBUF::ptr() }
    }
}
#[doc = "VREFBUF"]
pub mod vrefbuf;
#[doc = "PWR"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwr::RegisterBlock {
        0x5000_1000 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "PWR"]
pub mod pwr;
#[doc = "WWDG1"]
pub struct WWDG1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG1 {}
impl WWDG1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdg1::RegisterBlock {
        0x4000_a000 as *const _
    }
}
impl Deref for WWDG1 {
    type Target = wwdg1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDG1::ptr() }
    }
}
#[doc = "WWDG1"]
pub mod wwdg1;
#[doc = "WWDG1"]
pub struct WWDG2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG2 {}
impl WWDG2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wwdg1::RegisterBlock {
        0x5a00_2000 as *const _
    }
}
impl Deref for WWDG2 {
    type Target = wwdg1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WWDG2::ptr() }
    }
}
#[doc = "GICD"]
pub struct GICD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GICD {}
impl GICD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gicd::RegisterBlock {
        0xa002_1000 as *const _
    }
}
impl Deref for GICD {
    type Target = gicd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GICD::ptr() }
    }
}
#[doc = "GICD"]
pub mod gicd;
#[doc = "AXIMC"]
pub struct AXIMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AXIMC {}
impl AXIMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aximc::RegisterBlock {
        0x5700_0000 as *const _
    }
}
impl Deref for AXIMC {
    type Target = aximc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AXIMC::ptr() }
    }
}
#[doc = "AXIMC"]
pub mod aximc;
#[doc = "TZC"]
pub struct TZC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TZC {}
impl TZC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tzc::RegisterBlock {
        0x5c00_6000 as *const _
    }
}
impl Deref for TZC {
    type Target = tzc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TZC::ptr() }
    }
}
#[doc = "TZC"]
pub mod tzc;
#[doc = "GICV"]
pub struct GICV {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GICV {}
impl GICV {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gicv::RegisterBlock {
        0xa002_6000 as *const _
    }
}
impl Deref for GICV {
    type Target = gicv::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GICV::ptr() }
    }
}
#[doc = "GICV"]
pub mod gicv;
#[doc = "GICH"]
pub struct GICH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GICH {}
impl GICH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gich::RegisterBlock {
        0xa002_4000 as *const _
    }
}
impl Deref for GICH {
    type Target = gich::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GICH::ptr() }
    }
}
#[doc = "GICH"]
pub mod gich;
#[doc = "GICC"]
pub struct GICC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GICC {}
impl GICC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gicc::RegisterBlock {
        0xa002_2000 as *const _
    }
}
impl Deref for GICC {
    type Target = gicc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GICC::ptr() }
    }
}
#[doc = "GICC"]
pub mod gicc;
#[doc = "ETZPC"]
pub struct ETZPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETZPC {}
impl ETZPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const etzpc::RegisterBlock {
        0x5c00_7000 as *const _
    }
}
impl Deref for ETZPC {
    type Target = etzpc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETZPC::ptr() }
    }
}
#[doc = "ETZPC"]
pub mod etzpc;
#[doc = "DDRPERFM"]
pub struct DDRPERFM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DDRPERFM {}
impl DDRPERFM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ddrperfm::RegisterBlock {
        0x5a00_7000 as *const _
    }
}
impl Deref for DDRPERFM {
    type Target = ddrperfm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DDRPERFM::ptr() }
    }
}
#[doc = "DDRPERFM"]
pub mod ddrperfm;
#[doc = "USBPHYC"]
pub struct USBPHYC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBPHYC {}
impl USBPHYC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbphyc::RegisterBlock {
        0x5a00_6000 as *const _
    }
}
impl Deref for USBPHYC {
    type Target = usbphyc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBPHYC::ptr() }
    }
}
#[doc = "USBPHYC"]
pub mod usbphyc;
#[doc = "TAMP"]
pub struct TAMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TAMP {}
impl TAMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tamp::RegisterBlock {
        0x5c00_a000 as *const _
    }
}
impl Deref for TAMP {
    type Target = tamp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TAMP::ptr() }
    }
}
#[doc = "TAMP"]
pub mod tamp;
#[doc = "Inter-integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4001_2000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub mod i2c1;
#[doc = "Inter-integrated circuit"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4001_3000 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct I2C3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C3 {}
impl I2C3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for I2C3 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C3::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct I2C4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C4 {}
impl I2C4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x5c00_2000 as *const _
    }
}
impl Deref for I2C4 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C4::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct I2C5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C5 {}
impl I2C5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4001_5000 as *const _
    }
}
impl Deref for I2C5 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C5::ptr() }
    }
}
#[doc = "Inter-integrated circuit"]
pub struct I2C6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C6 {}
impl I2C6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x5c00_9000 as *const _
    }
}
impl Deref for I2C6 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C6::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4400_4000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub mod spi1;
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_b000 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI4 {}
impl SPI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4400_5000 as *const _
    }
}
impl Deref for SPI4 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI4::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI5 {}
impl SPI5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4400_9000 as *const _
    }
}
impl Deref for SPI5 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI5::ptr() }
    }
}
#[doc = "Serial peripheral interface/Inter-IC sound"]
pub struct SPI6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI6 {}
impl SPI6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x5c00_1000 as *const _
    }
}
impl Deref for SPI6 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI6::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x5c00_0000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_e000 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4000_f000 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for UART4 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_1000 as *const _
    }
}
impl Deref for UART5 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART5::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART6 {}
impl USART6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4400_3000 as *const _
    }
}
impl Deref for USART6 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART6::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART7 {}
impl UART7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for UART7 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART7::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct UART8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART8 {}
impl UART8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4001_9000 as *const _
    }
}
impl Deref for UART8 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART8::ptr() }
    }
}
#[doc = "LCD-TFT Controller"]
pub struct LTDC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LTDC {}
impl LTDC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ltdc::RegisterBlock {
        0x5a00_1000 as *const _
    }
}
impl Deref for LTDC {
    type Target = ltdc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LTDC::ptr() }
    }
}
#[doc = "LCD-TFT Controller"]
pub mod ltdc;
#[doc = "DSI Host"]
pub struct DSI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSI {}
impl DSI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dsi::RegisterBlock {
        0x5a00_0000 as *const _
    }
}
impl Deref for DSI {
    type Target = dsi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DSI::ptr() }
    }
}
#[doc = "DSI Host"]
pub mod dsi;
#[doc = "ETH_MAC_MMC"]
pub struct ETH_MAC_MMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETH_MAC_MMC {}
impl ETH_MAC_MMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eth_mac_mmc::RegisterBlock {
        0x5800_a000 as *const _
    }
}
impl Deref for ETH_MAC_MMC {
    type Target = eth_mac_mmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETH_MAC_MMC::ptr() }
    }
}
#[doc = "ETH_MAC_MMC"]
pub mod eth_mac_mmc;
#[doc = "ETH_MTL"]
pub struct ETH_MTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETH_MTL {}
impl ETH_MTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eth_mtl::RegisterBlock {
        0x5800_ac00 as *const _
    }
}
impl Deref for ETH_MTL {
    type Target = eth_mtl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETH_MTL::ptr() }
    }
}
#[doc = "ETH_MTL"]
pub mod eth_mtl;
#[doc = "ETH_DMA"]
pub struct ETH_DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ETH_DMA {}
impl ETH_DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const eth_dma::RegisterBlock {
        0x5800_b000 as *const _
    }
}
impl Deref for ETH_DMA {
    type Target = eth_dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ETH_DMA::ptr() }
    }
}
#[doc = "ETH_DMA"]
pub mod eth_dma;
#[doc = "Cyclic redundancy check calculation unit"]
pub struct CRC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC1 {}
impl CRC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc1::RegisterBlock {
        0x5800_9000 as *const _
    }
}
impl Deref for CRC1 {
    type Target = crc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC1::ptr() }
    }
}
#[doc = "Cyclic redundancy check calculation unit"]
pub mod crc1;
#[doc = "Cyclic redundancy check calculation unit"]
pub struct CRC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC2 {}
impl CRC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc1::RegisterBlock {
        0x4c00_4000 as *const _
    }
}
impl Deref for CRC2 {
    type Target = crc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC2::ptr() }
    }
}
#[doc = "SDMMC1"]
pub struct SDMMC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC1 {}
impl SDMMC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc1::RegisterBlock {
        0x5800_5000 as *const _
    }
}
impl Deref for SDMMC1 {
    type Target = sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC1::ptr() }
    }
}
#[doc = "SDMMC1"]
pub mod sdmmc1;
#[doc = "SDMMC1"]
pub struct SDMMC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC2 {}
impl SDMMC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc1::RegisterBlock {
        0x5800_7000 as *const _
    }
}
impl Deref for SDMMC2 {
    type Target = sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC2::ptr() }
    }
}
#[doc = "SDMMC1"]
pub struct SDMMC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDMMC3 {}
impl SDMMC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdmmc1::RegisterBlock {
        0x4800_4000 as *const _
    }
}
impl Deref for SDMMC3 {
    type Target = sdmmc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDMMC3::ptr() }
    }
}
#[doc = "FMC"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x5800_2000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "FMC"]
pub mod fmc;
#[doc = "General-purpose I/Os"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_2000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub mod gpioa;
#[doc = "General-purpose I/Os"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_3000 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_4000 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_5000 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_6000 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_7000 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_8000 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOH {}
impl GPIOH {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_9000 as *const _
    }
}
impl Deref for GPIOH {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOH::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOI {}
impl GPIOI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_a000 as *const _
    }
}
impl Deref for GPIOI {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOI::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOJ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOJ {}
impl GPIOJ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_b000 as *const _
    }
}
impl Deref for GPIOJ {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOJ::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOK {}
impl GPIOK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5000_c000 as *const _
    }
}
impl Deref for GPIOK {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOK::ptr() }
    }
}
#[doc = "General-purpose I/Os"]
pub struct GPIOZ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOZ {}
impl GPIOZ {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x5400_4000 as *const _
    }
}
impl Deref for GPIOZ {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOZ::ptr() }
    }
}
#[doc = "Hash processor"]
pub struct HASH1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HASH1 {}
impl HASH1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hash1::RegisterBlock {
        0x5400_2000 as *const _
    }
}
impl Deref for HASH1 {
    type Target = hash1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HASH1::ptr() }
    }
}
#[doc = "Hash processor"]
pub mod hash1;
#[doc = "Hash processor"]
pub struct HASH2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HASH2 {}
impl HASH2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hash1::RegisterBlock {
        0x4c00_2000 as *const _
    }
}
impl Deref for HASH2 {
    type Target = hash1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HASH2::ptr() }
    }
}
#[doc = "Cryptographic processor"]
pub struct CRYP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYP1 {}
impl CRYP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cryp1::RegisterBlock {
        0x5400_1000 as *const _
    }
}
impl Deref for CRYP1 {
    type Target = cryp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYP1::ptr() }
    }
}
#[doc = "Cryptographic processor"]
pub mod cryp1;
#[doc = "Cryptographic processor"]
pub struct CRYP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRYP2 {}
impl CRYP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cryp1::RegisterBlock {
        0x4c00_5000 as *const _
    }
}
impl Deref for CRYP2 {
    type Target = cryp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRYP2::ptr() }
    }
}
#[doc = "TEMP"]
pub struct TEMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TEMP {}
impl TEMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const temp::RegisterBlock {
        0x5002_8000 as *const _
    }
}
impl Deref for TEMP {
    type Target = temp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TEMP::ptr() }
    }
}
#[doc = "TEMP"]
pub mod temp;
#[doc = "SAI"]
pub struct SAI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI1 {}
impl SAI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4400_a000 as *const _
    }
}
impl Deref for SAI1 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI1::ptr() }
    }
}
#[doc = "SAI"]
pub mod sai1;
#[doc = "SAI"]
pub struct SAI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI2 {}
impl SAI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4400_b000 as *const _
    }
}
impl Deref for SAI2 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI2::ptr() }
    }
}
#[doc = "SAI"]
pub struct SAI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI3 {}
impl SAI3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x4400_c000 as *const _
    }
}
impl Deref for SAI3 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI3::ptr() }
    }
}
#[doc = "SAI"]
pub struct SAI4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SAI4 {}
impl SAI4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sai1::RegisterBlock {
        0x5002_7000 as *const _
    }
}
impl Deref for SAI4 {
    type Target = sai1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SAI4::ptr() }
    }
}
#[doc = "SYSCFG"]
pub struct SYSCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCFG {}
impl SYSCFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscfg::RegisterBlock {
        0x5002_0000 as *const _
    }
}
impl Deref for SYSCFG {
    type Target = syscfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCFG::ptr() }
    }
}
#[doc = "SYSCFG"]
pub mod syscfg;
#[doc = "External interrupt/event controller"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x5000_d000 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "External interrupt/event controller"]
pub mod exti;
#[doc = "Digital camera interface"]
pub struct DCMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DCMI {}
impl DCMI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dcmi::RegisterBlock {
        0x4c00_6000 as *const _
    }
}
impl Deref for DCMI {
    type Target = dcmi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DCMI::ptr() }
    }
}
#[doc = "Digital camera interface"]
pub mod dcmi;
#[doc = "IPCC"]
pub struct IPCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IPCC {}
impl IPCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ipcc::RegisterBlock {
        0x4c00_1000 as *const _
    }
}
impl Deref for IPCC {
    type Target = ipcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*IPCC::ptr() }
    }
}
#[doc = "IPCC"]
pub mod ipcc;
#[doc = "HSEM"]
pub struct HSEM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSEM {}
impl HSEM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsem::RegisterBlock {
        0x4c00_0000 as *const _
    }
}
impl Deref for HSEM {
    type Target = hsem::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HSEM::ptr() }
    }
}
#[doc = "HSEM"]
pub mod hsem;
#[doc = "Analog to Digital Converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc1::RegisterBlock {
        0x4800_3000 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod adc1;
#[doc = "Analog to Digital Converter"]
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC2 {}
impl ADC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc2::RegisterBlock {
        0x4800_3100 as *const _
    }
}
impl Deref for ADC2 {
    type Target = adc2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC2::ptr() }
    }
}
#[doc = "Analog to Digital Converter"]
pub mod adc2;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC12_COMMON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC12_COMMON {}
impl ADC12_COMMON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc12_common::RegisterBlock {
        0x4800_3300 as *const _
    }
}
impl Deref for ADC12_COMMON {
    type Target = adc12_common::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC12_COMMON::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc12_common;
#[doc = "DMA controller"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4800_0000 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma1;
#[doc = "DMA controller"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma1::RegisterBlock {
        0x4800_1000 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "CCU registers"]
pub struct CCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCU {}
impl CCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccu::RegisterBlock {
        0x4401_0000 as *const _
    }
}
impl Deref for CCU {
    type Target = ccu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCU::ptr() }
    }
}
#[doc = "CCU registers"]
pub mod ccu;
#[doc = "DAC1"]
pub struct DAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC1 {}
impl DAC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac1::RegisterBlock {
        0x4001_7000 as *const _
    }
}
impl Deref for DAC1 {
    type Target = dac1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC1::ptr() }
    }
}
#[doc = "DAC1"]
pub mod dac1;
#[doc = "TIM1"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4400_0000 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "TIM1"]
pub mod tim1;
#[doc = "TIM1"]
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim1::RegisterBlock {
        0x4400_1000 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM8::ptr() }
    }
}
#[doc = "CEC"]
pub struct CEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CEC {}
impl CEC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cec::RegisterBlock {
        0x4001_6000 as *const _
    }
}
impl Deref for CEC {
    type Target = cec::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CEC::ptr() }
    }
}
#[doc = "CEC"]
pub mod cec;
#[doc = "MDIOS"]
pub struct MDIOS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MDIOS {}
impl MDIOS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mdios::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for MDIOS {
    type Target = mdios::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MDIOS::ptr() }
    }
}
#[doc = "MDIOS"]
pub mod mdios;
#[doc = "SPDIFRX"]
pub struct SPDIFRX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPDIFRX {}
impl SPDIFRX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spdifrx::RegisterBlock {
        0x4000_d000 as *const _
    }
}
impl Deref for SPDIFRX {
    type Target = spdifrx::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPDIFRX::ptr() }
    }
}
#[doc = "SPDIFRX"]
pub mod spdifrx;
#[doc = "TIM6"]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM6::ptr() }
    }
}
#[doc = "TIM6"]
pub mod tim6;
#[doc = "TIM6"]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim6::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM7::ptr() }
    }
}
#[doc = "TIM2"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "TIM2"]
pub mod tim2;
#[doc = "TIM2"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_1000 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "TIM2"]
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_2000 as *const _
    }
}
impl Deref for TIM4 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM4::ptr() }
    }
}
#[doc = "TIM2"]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim2::RegisterBlock {
        0x4000_3000 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM5::ptr() }
    }
}
#[doc = "TIM12"]
pub struct TIM12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM12 {}
impl TIM12 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim12::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for TIM12 {
    type Target = tim12::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM12::ptr() }
    }
}
#[doc = "TIM12"]
pub mod tim12;
#[doc = "TIM15"]
pub struct TIM15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM15 {}
impl TIM15 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim15::RegisterBlock {
        0x4400_6000 as *const _
    }
}
impl Deref for TIM15 {
    type Target = tim15::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM15::ptr() }
    }
}
#[doc = "TIM15"]
pub mod tim15;
#[doc = "TIM16"]
pub struct TIM16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM16 {}
impl TIM16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        0x4400_7000 as *const _
    }
}
impl Deref for TIM16 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM16::ptr() }
    }
}
#[doc = "TIM16"]
pub mod tim16;
#[doc = "TIM16"]
pub struct TIM17 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM17 {}
impl TIM17 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim16::RegisterBlock {
        0x4400_8000 as *const _
    }
}
impl Deref for TIM17 {
    type Target = tim16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM17::ptr() }
    }
}
#[doc = "LPTIM1"]
pub struct LPTIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM1 {}
impl LPTIM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x4000_9000 as *const _
    }
}
impl Deref for LPTIM1 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM1::ptr() }
    }
}
#[doc = "LPTIM1"]
pub mod lptim1;
#[doc = "LPTIM1"]
pub struct LPTIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM2 {}
impl LPTIM2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x5002_1000 as *const _
    }
}
impl Deref for LPTIM2 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM2::ptr() }
    }
}
#[doc = "LPTIM1"]
pub struct LPTIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM3 {}
impl LPTIM3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x5002_2000 as *const _
    }
}
impl Deref for LPTIM3 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM3::ptr() }
    }
}
#[doc = "LPTIM1"]
pub struct LPTIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM4 {}
impl LPTIM4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x5002_3000 as *const _
    }
}
impl Deref for LPTIM4 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM4::ptr() }
    }
}
#[doc = "LPTIM1"]
pub struct LPTIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTIM5 {}
impl LPTIM5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptim1::RegisterBlock {
        0x5002_4000 as *const _
    }
}
impl Deref for LPTIM5 {
    type Target = lptim1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTIM5::ptr() }
    }
}
#[doc = "TIM13"]
pub struct TIM13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM13 {}
impl TIM13 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim13::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for TIM13 {
    type Target = tim13::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM13::ptr() }
    }
}
#[doc = "TIM13"]
pub mod tim13;
#[doc = "TIM13"]
pub struct TIM14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM14 {}
impl TIM14 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tim13::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for TIM14 {
    type Target = tim13::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIM14::ptr() }
    }
}
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "BSEC"]
    pub BSEC: BSEC,
    #[doc = "DDRCTRL"]
    pub DDRCTRL: DDRCTRL,
    #[doc = "DDRPHYC"]
    pub DDRPHYC: DDRPHYC,
    #[doc = "DLYBQS"]
    pub DLYBQS: DLYBQS,
    #[doc = "DLYBSD1"]
    pub DLYBSD1: DLYBSD1,
    #[doc = "DLYBSD2"]
    pub DLYBSD2: DLYBSD2,
    #[doc = "DLYBSD3"]
    pub DLYBSD3: DLYBSD3,
    #[doc = "DFSDM1"]
    pub DFSDM1: DFSDM1,
    #[doc = "DMAMUX1"]
    pub DMAMUX1: DMAMUX1,
    #[doc = "FDCAN1"]
    pub FDCAN1: FDCAN1,
    #[doc = "FDCAN2"]
    pub FDCAN2: FDCAN2,
    #[doc = "HDP"]
    pub HDP: HDP,
    #[doc = "IWDG1"]
    pub IWDG1: IWDG1,
    #[doc = "MDMA"]
    pub MDMA: MDMA,
    #[doc = "QUADSPI"]
    pub QUADSPI: QUADSPI,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "RNG1"]
    pub RNG1: RNG1,
    #[doc = "RNG2"]
    pub RNG2: RNG2,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "STGEN"]
    pub STGEN: STGEN,
    #[doc = "STGENR"]
    pub STGENR: STGENR,
    #[doc = "VREFBUF"]
    pub VREFBUF: VREFBUF,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "WWDG1"]
    pub WWDG1: WWDG1,
    #[doc = "WWDG2"]
    pub WWDG2: WWDG2,
    #[doc = "GICD"]
    pub GICD: GICD,
    #[doc = "AXIMC"]
    pub AXIMC: AXIMC,
    #[doc = "TZC"]
    pub TZC: TZC,
    #[doc = "GICV"]
    pub GICV: GICV,
    #[doc = "GICH"]
    pub GICH: GICH,
    #[doc = "GICC"]
    pub GICC: GICC,
    #[doc = "ETZPC"]
    pub ETZPC: ETZPC,
    #[doc = "DDRPERFM"]
    pub DDRPERFM: DDRPERFM,
    #[doc = "USBPHYC"]
    pub USBPHYC: USBPHYC,
    #[doc = "TAMP"]
    pub TAMP: TAMP,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2C3"]
    pub I2C3: I2C3,
    #[doc = "I2C4"]
    pub I2C4: I2C4,
    #[doc = "I2C5"]
    pub I2C5: I2C5,
    #[doc = "I2C6"]
    pub I2C6: I2C6,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "SPI4"]
    pub SPI4: SPI4,
    #[doc = "SPI5"]
    pub SPI5: SPI5,
    #[doc = "SPI6"]
    pub SPI6: SPI6,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "USART6"]
    pub USART6: USART6,
    #[doc = "UART7"]
    pub UART7: UART7,
    #[doc = "UART8"]
    pub UART8: UART8,
    #[doc = "LTDC"]
    pub LTDC: LTDC,
    #[doc = "DSI"]
    pub DSI: DSI,
    #[doc = "ETH_MAC_MMC"]
    pub ETH_MAC_MMC: ETH_MAC_MMC,
    #[doc = "ETH_MTL"]
    pub ETH_MTL: ETH_MTL,
    #[doc = "ETH_DMA"]
    pub ETH_DMA: ETH_DMA,
    #[doc = "CRC1"]
    pub CRC1: CRC1,
    #[doc = "CRC2"]
    pub CRC2: CRC2,
    #[doc = "SDMMC1"]
    pub SDMMC1: SDMMC1,
    #[doc = "SDMMC2"]
    pub SDMMC2: SDMMC2,
    #[doc = "SDMMC3"]
    pub SDMMC3: SDMMC3,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOG"]
    pub GPIOG: GPIOG,
    #[doc = "GPIOH"]
    pub GPIOH: GPIOH,
    #[doc = "GPIOI"]
    pub GPIOI: GPIOI,
    #[doc = "GPIOJ"]
    pub GPIOJ: GPIOJ,
    #[doc = "GPIOK"]
    pub GPIOK: GPIOK,
    #[doc = "GPIOZ"]
    pub GPIOZ: GPIOZ,
    #[doc = "HASH1"]
    pub HASH1: HASH1,
    #[doc = "HASH2"]
    pub HASH2: HASH2,
    #[doc = "CRYP1"]
    pub CRYP1: CRYP1,
    #[doc = "CRYP2"]
    pub CRYP2: CRYP2,
    #[doc = "TEMP"]
    pub TEMP: TEMP,
    #[doc = "SAI1"]
    pub SAI1: SAI1,
    #[doc = "SAI2"]
    pub SAI2: SAI2,
    #[doc = "SAI3"]
    pub SAI3: SAI3,
    #[doc = "SAI4"]
    pub SAI4: SAI4,
    #[doc = "SYSCFG"]
    pub SYSCFG: SYSCFG,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "DCMI"]
    pub DCMI: DCMI,
    #[doc = "IPCC"]
    pub IPCC: IPCC,
    #[doc = "HSEM"]
    pub HSEM: HSEM,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ADC2"]
    pub ADC2: ADC2,
    #[doc = "ADC12_COMMON"]
    pub ADC12_COMMON: ADC12_COMMON,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "CCU"]
    pub CCU: CCU,
    #[doc = "DAC1"]
    pub DAC1: DAC1,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "CEC"]
    pub CEC: CEC,
    #[doc = "MDIOS"]
    pub MDIOS: MDIOS,
    #[doc = "SPDIFRX"]
    pub SPDIFRX: SPDIFRX,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM4"]
    pub TIM4: TIM4,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "TIM12"]
    pub TIM12: TIM12,
    #[doc = "TIM15"]
    pub TIM15: TIM15,
    #[doc = "TIM16"]
    pub TIM16: TIM16,
    #[doc = "TIM17"]
    pub TIM17: TIM17,
    #[doc = "LPTIM1"]
    pub LPTIM1: LPTIM1,
    #[doc = "LPTIM2"]
    pub LPTIM2: LPTIM2,
    #[doc = "LPTIM3"]
    pub LPTIM3: LPTIM3,
    #[doc = "LPTIM4"]
    pub LPTIM4: LPTIM4,
    #[doc = "LPTIM5"]
    pub LPTIM5: LPTIM5,
    #[doc = "TIM13"]
    pub TIM13: TIM13,
    #[doc = "TIM14"]
    pub TIM14: TIM14,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            BSEC: BSEC {
                _marker: PhantomData,
            },
            DDRCTRL: DDRCTRL {
                _marker: PhantomData,
            },
            DDRPHYC: DDRPHYC {
                _marker: PhantomData,
            },
            DLYBQS: DLYBQS {
                _marker: PhantomData,
            },
            DLYBSD1: DLYBSD1 {
                _marker: PhantomData,
            },
            DLYBSD2: DLYBSD2 {
                _marker: PhantomData,
            },
            DLYBSD3: DLYBSD3 {
                _marker: PhantomData,
            },
            DFSDM1: DFSDM1 {
                _marker: PhantomData,
            },
            DMAMUX1: DMAMUX1 {
                _marker: PhantomData,
            },
            FDCAN1: FDCAN1 {
                _marker: PhantomData,
            },
            FDCAN2: FDCAN2 {
                _marker: PhantomData,
            },
            HDP: HDP {
                _marker: PhantomData,
            },
            IWDG1: IWDG1 {
                _marker: PhantomData,
            },
            MDMA: MDMA {
                _marker: PhantomData,
            },
            QUADSPI: QUADSPI {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            RNG1: RNG1 {
                _marker: PhantomData,
            },
            RNG2: RNG2 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            STGEN: STGEN {
                _marker: PhantomData,
            },
            STGENR: STGENR {
                _marker: PhantomData,
            },
            VREFBUF: VREFBUF {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            WWDG1: WWDG1 {
                _marker: PhantomData,
            },
            WWDG2: WWDG2 {
                _marker: PhantomData,
            },
            GICD: GICD {
                _marker: PhantomData,
            },
            AXIMC: AXIMC {
                _marker: PhantomData,
            },
            TZC: TZC {
                _marker: PhantomData,
            },
            GICV: GICV {
                _marker: PhantomData,
            },
            GICH: GICH {
                _marker: PhantomData,
            },
            GICC: GICC {
                _marker: PhantomData,
            },
            ETZPC: ETZPC {
                _marker: PhantomData,
            },
            DDRPERFM: DDRPERFM {
                _marker: PhantomData,
            },
            USBPHYC: USBPHYC {
                _marker: PhantomData,
            },
            TAMP: TAMP {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2C3: I2C3 {
                _marker: PhantomData,
            },
            I2C4: I2C4 {
                _marker: PhantomData,
            },
            I2C5: I2C5 {
                _marker: PhantomData,
            },
            I2C6: I2C6 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            SPI4: SPI4 {
                _marker: PhantomData,
            },
            SPI5: SPI5 {
                _marker: PhantomData,
            },
            SPI6: SPI6 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            USART6: USART6 {
                _marker: PhantomData,
            },
            UART7: UART7 {
                _marker: PhantomData,
            },
            UART8: UART8 {
                _marker: PhantomData,
            },
            LTDC: LTDC {
                _marker: PhantomData,
            },
            DSI: DSI {
                _marker: PhantomData,
            },
            ETH_MAC_MMC: ETH_MAC_MMC {
                _marker: PhantomData,
            },
            ETH_MTL: ETH_MTL {
                _marker: PhantomData,
            },
            ETH_DMA: ETH_DMA {
                _marker: PhantomData,
            },
            CRC1: CRC1 {
                _marker: PhantomData,
            },
            CRC2: CRC2 {
                _marker: PhantomData,
            },
            SDMMC1: SDMMC1 {
                _marker: PhantomData,
            },
            SDMMC2: SDMMC2 {
                _marker: PhantomData,
            },
            SDMMC3: SDMMC3 {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            GPIOH: GPIOH {
                _marker: PhantomData,
            },
            GPIOI: GPIOI {
                _marker: PhantomData,
            },
            GPIOJ: GPIOJ {
                _marker: PhantomData,
            },
            GPIOK: GPIOK {
                _marker: PhantomData,
            },
            GPIOZ: GPIOZ {
                _marker: PhantomData,
            },
            HASH1: HASH1 {
                _marker: PhantomData,
            },
            HASH2: HASH2 {
                _marker: PhantomData,
            },
            CRYP1: CRYP1 {
                _marker: PhantomData,
            },
            CRYP2: CRYP2 {
                _marker: PhantomData,
            },
            TEMP: TEMP {
                _marker: PhantomData,
            },
            SAI1: SAI1 {
                _marker: PhantomData,
            },
            SAI2: SAI2 {
                _marker: PhantomData,
            },
            SAI3: SAI3 {
                _marker: PhantomData,
            },
            SAI4: SAI4 {
                _marker: PhantomData,
            },
            SYSCFG: SYSCFG {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            DCMI: DCMI {
                _marker: PhantomData,
            },
            IPCC: IPCC {
                _marker: PhantomData,
            },
            HSEM: HSEM {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            ADC12_COMMON: ADC12_COMMON {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            CCU: CCU {
                _marker: PhantomData,
            },
            DAC1: DAC1 {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            CEC: CEC {
                _marker: PhantomData,
            },
            MDIOS: MDIOS {
                _marker: PhantomData,
            },
            SPDIFRX: SPDIFRX {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM4: TIM4 {
                _marker: PhantomData,
            },
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            TIM12: TIM12 {
                _marker: PhantomData,
            },
            TIM15: TIM15 {
                _marker: PhantomData,
            },
            TIM16: TIM16 {
                _marker: PhantomData,
            },
            TIM17: TIM17 {
                _marker: PhantomData,
            },
            LPTIM1: LPTIM1 {
                _marker: PhantomData,
            },
            LPTIM2: LPTIM2 {
                _marker: PhantomData,
            },
            LPTIM3: LPTIM3 {
                _marker: PhantomData,
            },
            LPTIM4: LPTIM4 {
                _marker: PhantomData,
            },
            LPTIM5: LPTIM5 {
                _marker: PhantomData,
            },
            TIM13: TIM13 {
                _marker: PhantomData,
            },
            TIM14: TIM14 {
                _marker: PhantomData,
            },
        }
    }
}
