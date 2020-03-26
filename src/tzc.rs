#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TZC configuration register"]
    pub build_config: BUILD_CONFIG,
    #[doc = "0x04 - TZC action register"]
    pub action: ACTION,
    #[doc = "0x08 - TZC gate keeper register"]
    pub gate_keeper: GATE_KEEPER,
    #[doc = "0x0c - TZC speculation control register"]
    pub speculation_ctrl: SPECULATION_CTRL,
    #[doc = "0x10 - TZC interrupt status register"]
    pub int_status: INT_STATUS,
    #[doc = "0x14 - TZC interrupt clear register"]
    pub int_clear: INT_CLEAR,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - TZC fail address low register0"]
    pub fail_address_low0: FAIL_ADDRESS_LOW0,
    _reserved7: [u8; 4usize],
    #[doc = "0x28 - TZC fail control register 0"]
    pub fail_control0: FAIL_CONTROL0,
    #[doc = "0x2c - TZC fail ID register 0"]
    pub fail_id0: FAIL_ID0,
    #[doc = "0x30 - TZC fail address low register1"]
    pub fail_address_low1: FAIL_ADDRESS_LOW1,
    _reserved10: [u8; 4usize],
    #[doc = "0x38 - TZC fail control register 1"]
    pub fail_control1: FAIL_CONTROL1,
    #[doc = "0x3c - TZC fail ID register 1"]
    pub fail_id1: FAIL_ID1,
    _reserved12: [u8; 192usize],
    #[doc = "0x100 - TZC region 0 base address low register"]
    pub region_base_low0: REGION_BASE_LOW0,
    _reserved13: [u8; 4usize],
    #[doc = "0x108 - TZC region 0 top address low register"]
    pub region_top_low0: REGION_TOP_LOW0,
    _reserved14: [u8; 4usize],
    #[doc = "0x110 - TZC region 0 attribute register"]
    pub region_attribute0: REGION_ATTRIBUTE0,
    #[doc = "0x114 - TZC region 0 ID access register"]
    pub region_id_access0: REGION_ID_ACCESS0,
    _reserved16: [u8; 8usize],
    #[doc = "0x120 - TZC region 1 ID access register"]
    pub region_base_low1: REGION_BASE_LOW1,
    _reserved17: [u8; 4usize],
    #[doc = "0x128 - TZC regions 1 top address low register"]
    pub region_top_low1: REGION_TOP_LOW1,
    _reserved18: [u8; 4usize],
    #[doc = "0x130 - TZC region 2 attribute register"]
    pub region_attribute1: REGION_ATTRIBUTE1,
    #[doc = "0x134 - TZC region 1 ID access register"]
    pub region_id_access1: REGION_ID_ACCESS1,
    _reserved20: [u8; 8usize],
    #[doc = "0x140 - TZC region 2 ID access register"]
    pub region_base_low2: REGION_BASE_LOW2,
    _reserved21: [u8; 4usize],
    #[doc = "0x148 - TZC regions 2 top address low register"]
    pub region_top_low2: REGION_TOP_LOW2,
    _reserved22: [u8; 4usize],
    #[doc = "0x150 - TZC region 2 attribute register"]
    pub region_attribute2: REGION_ATTRIBUTE2,
    #[doc = "0x154 - TZC region 2 ID access register"]
    pub region_id_access2: REGION_ID_ACCESS2,
    _reserved24: [u8; 8usize],
    #[doc = "0x160 - TZC region 3 ID access register"]
    pub region_base_low3: REGION_BASE_LOW3,
    _reserved25: [u8; 4usize],
    #[doc = "0x168 - TZC regions 3 top address low register"]
    pub region_top_low3: REGION_TOP_LOW3,
    _reserved26: [u8; 4usize],
    #[doc = "0x170 - TZC region 3 attribute register"]
    pub region_attribute3: REGION_ATTRIBUTE3,
    #[doc = "0x174 - TZC region 3 ID access register"]
    pub region_id_access3: REGION_ID_ACCESS3,
    _reserved28: [u8; 8usize],
    #[doc = "0x180 - TZC region 4 ID access register"]
    pub region_base_low4: REGION_BASE_LOW4,
    _reserved29: [u8; 4usize],
    #[doc = "0x188 - TZC regions 4 top address low register"]
    pub region_top_low4: REGION_TOP_LOW4,
    _reserved30: [u8; 4usize],
    #[doc = "0x190 - TZC region 4 attribute register"]
    pub region_attribute4: REGION_ATTRIBUTE4,
    #[doc = "0x194 - TZC region 4 ID access register"]
    pub region_id_access4: REGION_ID_ACCESS4,
    _reserved32: [u8; 8usize],
    #[doc = "0x1a0 - TZC region 5 ID access register"]
    pub region_base_low5: REGION_BASE_LOW5,
    _reserved33: [u8; 4usize],
    #[doc = "0x1a8 - TZC regions 5 top address low register"]
    pub region_top_low5: REGION_TOP_LOW5,
    _reserved34: [u8; 4usize],
    #[doc = "0x1b0 - TZC region 5 attribute register"]
    pub region_attribute5: REGION_ATTRIBUTE5,
    #[doc = "0x1b4 - TZC region 5 ID access register"]
    pub region_id_access5: REGION_ID_ACCESS5,
    _reserved36: [u8; 8usize],
    #[doc = "0x1c0 - TZC region 6 ID access register"]
    pub region_base_low6: REGION_BASE_LOW6,
    _reserved37: [u8; 4usize],
    #[doc = "0x1c8 - TZC regions 6 top address low register"]
    pub region_top_low6: REGION_TOP_LOW6,
    _reserved38: [u8; 4usize],
    #[doc = "0x1d0 - TZC region 6 attribute register"]
    pub region_attribute6: REGION_ATTRIBUTE6,
    #[doc = "0x1d4 - TZC region 6 ID access register"]
    pub region_id_access6: REGION_ID_ACCESS6,
    _reserved40: [u8; 8usize],
    #[doc = "0x1e0 - TZC region 7 ID access register"]
    pub region_base_low7: REGION_BASE_LOW7,
    _reserved41: [u8; 4usize],
    #[doc = "0x1e8 - TZC regions 7 top address low register"]
    pub region_top_low7: REGION_TOP_LOW7,
    _reserved42: [u8; 4usize],
    #[doc = "0x1f0 - TZC region 7 attribute register"]
    pub region_attribute7: REGION_ATTRIBUTE7,
    #[doc = "0x1f4 - TZC region 7 ID access register"]
    pub region_id_access7: REGION_ID_ACCESS7,
    _reserved44: [u8; 8usize],
    #[doc = "0x200 - TZC region 8 ID access register"]
    pub region_base_low8: REGION_BASE_LOW8,
    _reserved45: [u8; 4usize],
    #[doc = "0x208 - TZC regions 8 top address low register"]
    pub region_top_low8: REGION_TOP_LOW8,
    _reserved46: [u8; 4usize],
    #[doc = "0x210 - TZC region 8 attribute register"]
    pub region_attribute8: REGION_ATTRIBUTE8,
    #[doc = "0x214 - TZC region 8 ID access register"]
    pub region_id_access8: REGION_ID_ACCESS8,
    _reserved48: [u8; 3512usize],
    #[doc = "0xfd0 - TZC peripheral ID 4 register"]
    pub pid4: PID4,
    #[doc = "0xfd4 - TZC peripheral ID 5 register"]
    pub pid5: PID5,
    #[doc = "0xfd8 - TZC peripheral ID 6 register"]
    pub pid6: PID6,
    #[doc = "0xfdc - TZC peripheral ID 7 register"]
    pub pid7: PID7,
    #[doc = "0xfe0 - TZC peripheral ID 0 register"]
    pub pid0: PID0,
    #[doc = "0xfe4 - TZC peripheral ID 1 register"]
    pub pid1: PID1,
    #[doc = "0xfe8 - TZC peripheral ID 2 register"]
    pub pid2: PID2,
    #[doc = "0xfec - TZC peripheral ID 3 register"]
    pub pid3: PID3,
    #[doc = "0xff0 - TZC component ID 0 register"]
    pub cid0: CID0,
    #[doc = "0xff4 - TZC component ID 1 register"]
    pub cid1: CID1,
    #[doc = "0xff8 - TZC component ID 2 register"]
    pub cid2: CID2,
    #[doc = "0xffc - TZC component ID 3 register"]
    pub cid3: CID3,
}
#[doc = "TZC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [build_config](build_config) module"]
pub type BUILD_CONFIG = crate::Reg<u32, _BUILD_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUILD_CONFIG;
#[doc = "`read()` method returns [build_config::R](build_config::R) reader structure"]
impl crate::Readable for BUILD_CONFIG {}
#[doc = "`write(|w| ..)` method takes [build_config::W](build_config::W) writer structure"]
impl crate::Writable for BUILD_CONFIG {}
#[doc = "TZC configuration register"]
pub mod build_config;
#[doc = "TZC action register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [action](action) module"]
pub type ACTION = crate::Reg<u32, _ACTION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTION;
#[doc = "`read()` method returns [action::R](action::R) reader structure"]
impl crate::Readable for ACTION {}
#[doc = "`write(|w| ..)` method takes [action::W](action::W) writer structure"]
impl crate::Writable for ACTION {}
#[doc = "TZC action register"]
pub mod action;
#[doc = "TZC gate keeper register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gate_keeper](gate_keeper) module"]
pub type GATE_KEEPER = crate::Reg<u32, _GATE_KEEPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GATE_KEEPER;
#[doc = "`read()` method returns [gate_keeper::R](gate_keeper::R) reader structure"]
impl crate::Readable for GATE_KEEPER {}
#[doc = "`write(|w| ..)` method takes [gate_keeper::W](gate_keeper::W) writer structure"]
impl crate::Writable for GATE_KEEPER {}
#[doc = "TZC gate keeper register"]
pub mod gate_keeper;
#[doc = "TZC speculation control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [speculation_ctrl](speculation_ctrl) module"]
pub type SPECULATION_CTRL = crate::Reg<u32, _SPECULATION_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECULATION_CTRL;
#[doc = "`read()` method returns [speculation_ctrl::R](speculation_ctrl::R) reader structure"]
impl crate::Readable for SPECULATION_CTRL {}
#[doc = "`write(|w| ..)` method takes [speculation_ctrl::W](speculation_ctrl::W) writer structure"]
impl crate::Writable for SPECULATION_CTRL {}
#[doc = "TZC speculation control register"]
pub mod speculation_ctrl;
#[doc = "TZC interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status](int_status) module"]
pub type INT_STATUS = crate::Reg<u32, _INT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STATUS;
#[doc = "`read()` method returns [int_status::R](int_status::R) reader structure"]
impl crate::Readable for INT_STATUS {}
#[doc = "TZC interrupt status register"]
pub mod int_status;
#[doc = "TZC interrupt clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clear](int_clear) module"]
pub type INT_CLEAR = crate::Reg<u32, _INT_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLEAR;
#[doc = "`write(|w| ..)` method takes [int_clear::W](int_clear::W) writer structure"]
impl crate::Writable for INT_CLEAR {}
#[doc = "TZC interrupt clear register"]
pub mod int_clear;
#[doc = "TZC fail address low register0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fail_address_low0](fail_address_low0) module"]
pub type FAIL_ADDRESS_LOW0 = crate::Reg<u32, _FAIL_ADDRESS_LOW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAIL_ADDRESS_LOW0;
#[doc = "`read()` method returns [fail_address_low0::R](fail_address_low0::R) reader structure"]
impl crate::Readable for FAIL_ADDRESS_LOW0 {}
#[doc = "TZC fail address low register0"]
pub mod fail_address_low0;
#[doc = "TZC fail address low register1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fail_address_low1](fail_address_low1) module"]
pub type FAIL_ADDRESS_LOW1 = crate::Reg<u32, _FAIL_ADDRESS_LOW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAIL_ADDRESS_LOW1;
#[doc = "`read()` method returns [fail_address_low1::R](fail_address_low1::R) reader structure"]
impl crate::Readable for FAIL_ADDRESS_LOW1 {}
#[doc = "TZC fail address low register1"]
pub mod fail_address_low1;
#[doc = "TZC fail control register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fail_control0](fail_control0) module"]
pub type FAIL_CONTROL0 = crate::Reg<u32, _FAIL_CONTROL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAIL_CONTROL0;
#[doc = "`read()` method returns [fail_control0::R](fail_control0::R) reader structure"]
impl crate::Readable for FAIL_CONTROL0 {}
#[doc = "TZC fail control register 0"]
pub mod fail_control0;
#[doc = "TZC fail control register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fail_control1](fail_control1) module"]
pub type FAIL_CONTROL1 = crate::Reg<u32, _FAIL_CONTROL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAIL_CONTROL1;
#[doc = "`read()` method returns [fail_control1::R](fail_control1::R) reader structure"]
impl crate::Readable for FAIL_CONTROL1 {}
#[doc = "TZC fail control register 1"]
pub mod fail_control1;
#[doc = "TZC fail ID register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fail_id0](fail_id0) module"]
pub type FAIL_ID0 = crate::Reg<u32, _FAIL_ID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAIL_ID0;
#[doc = "`read()` method returns [fail_id0::R](fail_id0::R) reader structure"]
impl crate::Readable for FAIL_ID0 {}
#[doc = "TZC fail ID register 0"]
pub mod fail_id0;
#[doc = "TZC fail ID register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fail_id1](fail_id1) module"]
pub type FAIL_ID1 = crate::Reg<u32, _FAIL_ID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAIL_ID1;
#[doc = "`read()` method returns [fail_id1::R](fail_id1::R) reader structure"]
impl crate::Readable for FAIL_ID1 {}
#[doc = "TZC fail ID register 1"]
pub mod fail_id1;
#[doc = "TZC region 0 base address low register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_base_low0](region_base_low0) module"]
pub type REGION_BASE_LOW0 = crate::Reg<u32, _REGION_BASE_LOW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_BASE_LOW0;
#[doc = "`read()` method returns [region_base_low0::R](region_base_low0::R) reader structure"]
impl crate::Readable for REGION_BASE_LOW0 {}
#[doc = "TZC region 0 base address low register"]
pub mod region_base_low0;
#[doc = "TZC region 0 top address low register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_top_low0](region_top_low0) module"]
pub type REGION_TOP_LOW0 = crate::Reg<u32, _REGION_TOP_LOW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_TOP_LOW0;
#[doc = "`read()` method returns [region_top_low0::R](region_top_low0::R) reader structure"]
impl crate::Readable for REGION_TOP_LOW0 {}
#[doc = "TZC region 0 top address low register"]
pub mod region_top_low0;
#[doc = "TZC region 0 attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_attribute0](region_attribute0) module"]
pub type REGION_ATTRIBUTE0 = crate::Reg<u32, _REGION_ATTRIBUTE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ATTRIBUTE0;
#[doc = "`read()` method returns [region_attribute0::R](region_attribute0::R) reader structure"]
impl crate::Readable for REGION_ATTRIBUTE0 {}
#[doc = "`write(|w| ..)` method takes [region_attribute0::W](region_attribute0::W) writer structure"]
impl crate::Writable for REGION_ATTRIBUTE0 {}
#[doc = "TZC region 0 attribute register"]
pub mod region_attribute0;
#[doc = "TZC region 0 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_id_access0](region_id_access0) module"]
pub type REGION_ID_ACCESS0 = crate::Reg<u32, _REGION_ID_ACCESS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ID_ACCESS0;
#[doc = "`read()` method returns [region_id_access0::R](region_id_access0::R) reader structure"]
impl crate::Readable for REGION_ID_ACCESS0 {}
#[doc = "`write(|w| ..)` method takes [region_id_access0::W](region_id_access0::W) writer structure"]
impl crate::Writable for REGION_ID_ACCESS0 {}
#[doc = "TZC region 0 ID access register"]
pub mod region_id_access0;
#[doc = "TZC region 1 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_id_access1](region_id_access1) module"]
pub type REGION_ID_ACCESS1 = crate::Reg<u32, _REGION_ID_ACCESS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ID_ACCESS1;
#[doc = "`read()` method returns [region_id_access1::R](region_id_access1::R) reader structure"]
impl crate::Readable for REGION_ID_ACCESS1 {}
#[doc = "`write(|w| ..)` method takes [region_id_access1::W](region_id_access1::W) writer structure"]
impl crate::Writable for REGION_ID_ACCESS1 {}
#[doc = "TZC region 1 ID access register"]
pub mod region_id_access1;
#[doc = "TZC region 2 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_id_access2](region_id_access2) module"]
pub type REGION_ID_ACCESS2 = crate::Reg<u32, _REGION_ID_ACCESS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ID_ACCESS2;
#[doc = "`read()` method returns [region_id_access2::R](region_id_access2::R) reader structure"]
impl crate::Readable for REGION_ID_ACCESS2 {}
#[doc = "`write(|w| ..)` method takes [region_id_access2::W](region_id_access2::W) writer structure"]
impl crate::Writable for REGION_ID_ACCESS2 {}
#[doc = "TZC region 2 ID access register"]
pub mod region_id_access2;
#[doc = "TZC region 3 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_id_access3](region_id_access3) module"]
pub type REGION_ID_ACCESS3 = crate::Reg<u32, _REGION_ID_ACCESS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ID_ACCESS3;
#[doc = "`read()` method returns [region_id_access3::R](region_id_access3::R) reader structure"]
impl crate::Readable for REGION_ID_ACCESS3 {}
#[doc = "`write(|w| ..)` method takes [region_id_access3::W](region_id_access3::W) writer structure"]
impl crate::Writable for REGION_ID_ACCESS3 {}
#[doc = "TZC region 3 ID access register"]
pub mod region_id_access3;
#[doc = "TZC region 4 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_id_access4](region_id_access4) module"]
pub type REGION_ID_ACCESS4 = crate::Reg<u32, _REGION_ID_ACCESS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ID_ACCESS4;
#[doc = "`read()` method returns [region_id_access4::R](region_id_access4::R) reader structure"]
impl crate::Readable for REGION_ID_ACCESS4 {}
#[doc = "`write(|w| ..)` method takes [region_id_access4::W](region_id_access4::W) writer structure"]
impl crate::Writable for REGION_ID_ACCESS4 {}
#[doc = "TZC region 4 ID access register"]
pub mod region_id_access4;
#[doc = "TZC region 5 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_id_access5](region_id_access5) module"]
pub type REGION_ID_ACCESS5 = crate::Reg<u32, _REGION_ID_ACCESS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ID_ACCESS5;
#[doc = "`read()` method returns [region_id_access5::R](region_id_access5::R) reader structure"]
impl crate::Readable for REGION_ID_ACCESS5 {}
#[doc = "`write(|w| ..)` method takes [region_id_access5::W](region_id_access5::W) writer structure"]
impl crate::Writable for REGION_ID_ACCESS5 {}
#[doc = "TZC region 5 ID access register"]
pub mod region_id_access5;
#[doc = "TZC region 6 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_id_access6](region_id_access6) module"]
pub type REGION_ID_ACCESS6 = crate::Reg<u32, _REGION_ID_ACCESS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ID_ACCESS6;
#[doc = "`read()` method returns [region_id_access6::R](region_id_access6::R) reader structure"]
impl crate::Readable for REGION_ID_ACCESS6 {}
#[doc = "`write(|w| ..)` method takes [region_id_access6::W](region_id_access6::W) writer structure"]
impl crate::Writable for REGION_ID_ACCESS6 {}
#[doc = "TZC region 6 ID access register"]
pub mod region_id_access6;
#[doc = "TZC region 7 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_id_access7](region_id_access7) module"]
pub type REGION_ID_ACCESS7 = crate::Reg<u32, _REGION_ID_ACCESS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ID_ACCESS7;
#[doc = "`read()` method returns [region_id_access7::R](region_id_access7::R) reader structure"]
impl crate::Readable for REGION_ID_ACCESS7 {}
#[doc = "`write(|w| ..)` method takes [region_id_access7::W](region_id_access7::W) writer structure"]
impl crate::Writable for REGION_ID_ACCESS7 {}
#[doc = "TZC region 7 ID access register"]
pub mod region_id_access7;
#[doc = "TZC region 8 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_id_access8](region_id_access8) module"]
pub type REGION_ID_ACCESS8 = crate::Reg<u32, _REGION_ID_ACCESS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ID_ACCESS8;
#[doc = "`read()` method returns [region_id_access8::R](region_id_access8::R) reader structure"]
impl crate::Readable for REGION_ID_ACCESS8 {}
#[doc = "`write(|w| ..)` method takes [region_id_access8::W](region_id_access8::W) writer structure"]
impl crate::Writable for REGION_ID_ACCESS8 {}
#[doc = "TZC region 8 ID access register"]
pub mod region_id_access8;
#[doc = "TZC region 1 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_base_low1](region_base_low1) module"]
pub type REGION_BASE_LOW1 = crate::Reg<u32, _REGION_BASE_LOW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_BASE_LOW1;
#[doc = "`read()` method returns [region_base_low1::R](region_base_low1::R) reader structure"]
impl crate::Readable for REGION_BASE_LOW1 {}
#[doc = "`write(|w| ..)` method takes [region_base_low1::W](region_base_low1::W) writer structure"]
impl crate::Writable for REGION_BASE_LOW1 {}
#[doc = "TZC region 1 ID access register"]
pub mod region_base_low1;
#[doc = "TZC region 2 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_base_low2](region_base_low2) module"]
pub type REGION_BASE_LOW2 = crate::Reg<u32, _REGION_BASE_LOW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_BASE_LOW2;
#[doc = "`read()` method returns [region_base_low2::R](region_base_low2::R) reader structure"]
impl crate::Readable for REGION_BASE_LOW2 {}
#[doc = "`write(|w| ..)` method takes [region_base_low2::W](region_base_low2::W) writer structure"]
impl crate::Writable for REGION_BASE_LOW2 {}
#[doc = "TZC region 2 ID access register"]
pub mod region_base_low2;
#[doc = "TZC region 3 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_base_low3](region_base_low3) module"]
pub type REGION_BASE_LOW3 = crate::Reg<u32, _REGION_BASE_LOW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_BASE_LOW3;
#[doc = "`read()` method returns [region_base_low3::R](region_base_low3::R) reader structure"]
impl crate::Readable for REGION_BASE_LOW3 {}
#[doc = "`write(|w| ..)` method takes [region_base_low3::W](region_base_low3::W) writer structure"]
impl crate::Writable for REGION_BASE_LOW3 {}
#[doc = "TZC region 3 ID access register"]
pub mod region_base_low3;
#[doc = "TZC region 4 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_base_low4](region_base_low4) module"]
pub type REGION_BASE_LOW4 = crate::Reg<u32, _REGION_BASE_LOW4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_BASE_LOW4;
#[doc = "`read()` method returns [region_base_low4::R](region_base_low4::R) reader structure"]
impl crate::Readable for REGION_BASE_LOW4 {}
#[doc = "`write(|w| ..)` method takes [region_base_low4::W](region_base_low4::W) writer structure"]
impl crate::Writable for REGION_BASE_LOW4 {}
#[doc = "TZC region 4 ID access register"]
pub mod region_base_low4;
#[doc = "TZC region 5 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_base_low5](region_base_low5) module"]
pub type REGION_BASE_LOW5 = crate::Reg<u32, _REGION_BASE_LOW5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_BASE_LOW5;
#[doc = "`read()` method returns [region_base_low5::R](region_base_low5::R) reader structure"]
impl crate::Readable for REGION_BASE_LOW5 {}
#[doc = "`write(|w| ..)` method takes [region_base_low5::W](region_base_low5::W) writer structure"]
impl crate::Writable for REGION_BASE_LOW5 {}
#[doc = "TZC region 5 ID access register"]
pub mod region_base_low5;
#[doc = "TZC region 6 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_base_low6](region_base_low6) module"]
pub type REGION_BASE_LOW6 = crate::Reg<u32, _REGION_BASE_LOW6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_BASE_LOW6;
#[doc = "`read()` method returns [region_base_low6::R](region_base_low6::R) reader structure"]
impl crate::Readable for REGION_BASE_LOW6 {}
#[doc = "`write(|w| ..)` method takes [region_base_low6::W](region_base_low6::W) writer structure"]
impl crate::Writable for REGION_BASE_LOW6 {}
#[doc = "TZC region 6 ID access register"]
pub mod region_base_low6;
#[doc = "TZC region 7 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_base_low7](region_base_low7) module"]
pub type REGION_BASE_LOW7 = crate::Reg<u32, _REGION_BASE_LOW7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_BASE_LOW7;
#[doc = "`read()` method returns [region_base_low7::R](region_base_low7::R) reader structure"]
impl crate::Readable for REGION_BASE_LOW7 {}
#[doc = "`write(|w| ..)` method takes [region_base_low7::W](region_base_low7::W) writer structure"]
impl crate::Writable for REGION_BASE_LOW7 {}
#[doc = "TZC region 7 ID access register"]
pub mod region_base_low7;
#[doc = "TZC region 8 ID access register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_base_low8](region_base_low8) module"]
pub type REGION_BASE_LOW8 = crate::Reg<u32, _REGION_BASE_LOW8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_BASE_LOW8;
#[doc = "`read()` method returns [region_base_low8::R](region_base_low8::R) reader structure"]
impl crate::Readable for REGION_BASE_LOW8 {}
#[doc = "`write(|w| ..)` method takes [region_base_low8::W](region_base_low8::W) writer structure"]
impl crate::Writable for REGION_BASE_LOW8 {}
#[doc = "TZC region 8 ID access register"]
pub mod region_base_low8;
#[doc = "TZC regions 1 top address low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_top_low1](region_top_low1) module"]
pub type REGION_TOP_LOW1 = crate::Reg<u32, _REGION_TOP_LOW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_TOP_LOW1;
#[doc = "`read()` method returns [region_top_low1::R](region_top_low1::R) reader structure"]
impl crate::Readable for REGION_TOP_LOW1 {}
#[doc = "`write(|w| ..)` method takes [region_top_low1::W](region_top_low1::W) writer structure"]
impl crate::Writable for REGION_TOP_LOW1 {}
#[doc = "TZC regions 1 top address low register"]
pub mod region_top_low1;
#[doc = "TZC regions 2 top address low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_top_low2](region_top_low2) module"]
pub type REGION_TOP_LOW2 = crate::Reg<u32, _REGION_TOP_LOW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_TOP_LOW2;
#[doc = "`read()` method returns [region_top_low2::R](region_top_low2::R) reader structure"]
impl crate::Readable for REGION_TOP_LOW2 {}
#[doc = "`write(|w| ..)` method takes [region_top_low2::W](region_top_low2::W) writer structure"]
impl crate::Writable for REGION_TOP_LOW2 {}
#[doc = "TZC regions 2 top address low register"]
pub mod region_top_low2;
#[doc = "TZC regions 3 top address low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_top_low3](region_top_low3) module"]
pub type REGION_TOP_LOW3 = crate::Reg<u32, _REGION_TOP_LOW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_TOP_LOW3;
#[doc = "`read()` method returns [region_top_low3::R](region_top_low3::R) reader structure"]
impl crate::Readable for REGION_TOP_LOW3 {}
#[doc = "`write(|w| ..)` method takes [region_top_low3::W](region_top_low3::W) writer structure"]
impl crate::Writable for REGION_TOP_LOW3 {}
#[doc = "TZC regions 3 top address low register"]
pub mod region_top_low3;
#[doc = "TZC regions 4 top address low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_top_low4](region_top_low4) module"]
pub type REGION_TOP_LOW4 = crate::Reg<u32, _REGION_TOP_LOW4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_TOP_LOW4;
#[doc = "`read()` method returns [region_top_low4::R](region_top_low4::R) reader structure"]
impl crate::Readable for REGION_TOP_LOW4 {}
#[doc = "`write(|w| ..)` method takes [region_top_low4::W](region_top_low4::W) writer structure"]
impl crate::Writable for REGION_TOP_LOW4 {}
#[doc = "TZC regions 4 top address low register"]
pub mod region_top_low4;
#[doc = "TZC regions 5 top address low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_top_low5](region_top_low5) module"]
pub type REGION_TOP_LOW5 = crate::Reg<u32, _REGION_TOP_LOW5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_TOP_LOW5;
#[doc = "`read()` method returns [region_top_low5::R](region_top_low5::R) reader structure"]
impl crate::Readable for REGION_TOP_LOW5 {}
#[doc = "`write(|w| ..)` method takes [region_top_low5::W](region_top_low5::W) writer structure"]
impl crate::Writable for REGION_TOP_LOW5 {}
#[doc = "TZC regions 5 top address low register"]
pub mod region_top_low5;
#[doc = "TZC regions 6 top address low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_top_low6](region_top_low6) module"]
pub type REGION_TOP_LOW6 = crate::Reg<u32, _REGION_TOP_LOW6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_TOP_LOW6;
#[doc = "`read()` method returns [region_top_low6::R](region_top_low6::R) reader structure"]
impl crate::Readable for REGION_TOP_LOW6 {}
#[doc = "`write(|w| ..)` method takes [region_top_low6::W](region_top_low6::W) writer structure"]
impl crate::Writable for REGION_TOP_LOW6 {}
#[doc = "TZC regions 6 top address low register"]
pub mod region_top_low6;
#[doc = "TZC regions 7 top address low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_top_low7](region_top_low7) module"]
pub type REGION_TOP_LOW7 = crate::Reg<u32, _REGION_TOP_LOW7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_TOP_LOW7;
#[doc = "`read()` method returns [region_top_low7::R](region_top_low7::R) reader structure"]
impl crate::Readable for REGION_TOP_LOW7 {}
#[doc = "`write(|w| ..)` method takes [region_top_low7::W](region_top_low7::W) writer structure"]
impl crate::Writable for REGION_TOP_LOW7 {}
#[doc = "TZC regions 7 top address low register"]
pub mod region_top_low7;
#[doc = "TZC regions 8 top address low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_top_low8](region_top_low8) module"]
pub type REGION_TOP_LOW8 = crate::Reg<u32, _REGION_TOP_LOW8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_TOP_LOW8;
#[doc = "`read()` method returns [region_top_low8::R](region_top_low8::R) reader structure"]
impl crate::Readable for REGION_TOP_LOW8 {}
#[doc = "`write(|w| ..)` method takes [region_top_low8::W](region_top_low8::W) writer structure"]
impl crate::Writable for REGION_TOP_LOW8 {}
#[doc = "TZC regions 8 top address low register"]
pub mod region_top_low8;
#[doc = "TZC region 2 attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_attribute1](region_attribute1) module"]
pub type REGION_ATTRIBUTE1 = crate::Reg<u32, _REGION_ATTRIBUTE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ATTRIBUTE1;
#[doc = "`read()` method returns [region_attribute1::R](region_attribute1::R) reader structure"]
impl crate::Readable for REGION_ATTRIBUTE1 {}
#[doc = "`write(|w| ..)` method takes [region_attribute1::W](region_attribute1::W) writer structure"]
impl crate::Writable for REGION_ATTRIBUTE1 {}
#[doc = "TZC region 2 attribute register"]
pub mod region_attribute1;
#[doc = "TZC region 2 attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_attribute2](region_attribute2) module"]
pub type REGION_ATTRIBUTE2 = crate::Reg<u32, _REGION_ATTRIBUTE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ATTRIBUTE2;
#[doc = "`read()` method returns [region_attribute2::R](region_attribute2::R) reader structure"]
impl crate::Readable for REGION_ATTRIBUTE2 {}
#[doc = "`write(|w| ..)` method takes [region_attribute2::W](region_attribute2::W) writer structure"]
impl crate::Writable for REGION_ATTRIBUTE2 {}
#[doc = "TZC region 2 attribute register"]
pub mod region_attribute2;
#[doc = "TZC region 3 attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_attribute3](region_attribute3) module"]
pub type REGION_ATTRIBUTE3 = crate::Reg<u32, _REGION_ATTRIBUTE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ATTRIBUTE3;
#[doc = "`read()` method returns [region_attribute3::R](region_attribute3::R) reader structure"]
impl crate::Readable for REGION_ATTRIBUTE3 {}
#[doc = "`write(|w| ..)` method takes [region_attribute3::W](region_attribute3::W) writer structure"]
impl crate::Writable for REGION_ATTRIBUTE3 {}
#[doc = "TZC region 3 attribute register"]
pub mod region_attribute3;
#[doc = "TZC region 4 attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_attribute4](region_attribute4) module"]
pub type REGION_ATTRIBUTE4 = crate::Reg<u32, _REGION_ATTRIBUTE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ATTRIBUTE4;
#[doc = "`read()` method returns [region_attribute4::R](region_attribute4::R) reader structure"]
impl crate::Readable for REGION_ATTRIBUTE4 {}
#[doc = "`write(|w| ..)` method takes [region_attribute4::W](region_attribute4::W) writer structure"]
impl crate::Writable for REGION_ATTRIBUTE4 {}
#[doc = "TZC region 4 attribute register"]
pub mod region_attribute4;
#[doc = "TZC region 5 attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_attribute5](region_attribute5) module"]
pub type REGION_ATTRIBUTE5 = crate::Reg<u32, _REGION_ATTRIBUTE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ATTRIBUTE5;
#[doc = "`read()` method returns [region_attribute5::R](region_attribute5::R) reader structure"]
impl crate::Readable for REGION_ATTRIBUTE5 {}
#[doc = "`write(|w| ..)` method takes [region_attribute5::W](region_attribute5::W) writer structure"]
impl crate::Writable for REGION_ATTRIBUTE5 {}
#[doc = "TZC region 5 attribute register"]
pub mod region_attribute5;
#[doc = "TZC region 6 attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_attribute6](region_attribute6) module"]
pub type REGION_ATTRIBUTE6 = crate::Reg<u32, _REGION_ATTRIBUTE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ATTRIBUTE6;
#[doc = "`read()` method returns [region_attribute6::R](region_attribute6::R) reader structure"]
impl crate::Readable for REGION_ATTRIBUTE6 {}
#[doc = "`write(|w| ..)` method takes [region_attribute6::W](region_attribute6::W) writer structure"]
impl crate::Writable for REGION_ATTRIBUTE6 {}
#[doc = "TZC region 6 attribute register"]
pub mod region_attribute6;
#[doc = "TZC region 7 attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_attribute7](region_attribute7) module"]
pub type REGION_ATTRIBUTE7 = crate::Reg<u32, _REGION_ATTRIBUTE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ATTRIBUTE7;
#[doc = "`read()` method returns [region_attribute7::R](region_attribute7::R) reader structure"]
impl crate::Readable for REGION_ATTRIBUTE7 {}
#[doc = "`write(|w| ..)` method takes [region_attribute7::W](region_attribute7::W) writer structure"]
impl crate::Writable for REGION_ATTRIBUTE7 {}
#[doc = "TZC region 7 attribute register"]
pub mod region_attribute7;
#[doc = "TZC region 8 attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region_attribute8](region_attribute8) module"]
pub type REGION_ATTRIBUTE8 = crate::Reg<u32, _REGION_ATTRIBUTE8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REGION_ATTRIBUTE8;
#[doc = "`read()` method returns [region_attribute8::R](region_attribute8::R) reader structure"]
impl crate::Readable for REGION_ATTRIBUTE8 {}
#[doc = "`write(|w| ..)` method takes [region_attribute8::W](region_attribute8::W) writer structure"]
impl crate::Writable for REGION_ATTRIBUTE8 {}
#[doc = "TZC region 8 attribute register"]
pub mod region_attribute8;
#[doc = "TZC peripheral ID 4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid4](pid4) module"]
pub type PID4 = crate::Reg<u32, _PID4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID4;
#[doc = "`read()` method returns [pid4::R](pid4::R) reader structure"]
impl crate::Readable for PID4 {}
#[doc = "TZC peripheral ID 4 register"]
pub mod pid4;
#[doc = "TZC peripheral ID 5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid5](pid5) module"]
pub type PID5 = crate::Reg<u32, _PID5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID5;
#[doc = "`read()` method returns [pid5::R](pid5::R) reader structure"]
impl crate::Readable for PID5 {}
#[doc = "TZC peripheral ID 5 register"]
pub mod pid5;
#[doc = "TZC peripheral ID 6 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid6](pid6) module"]
pub type PID6 = crate::Reg<u32, _PID6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID6;
#[doc = "`read()` method returns [pid6::R](pid6::R) reader structure"]
impl crate::Readable for PID6 {}
#[doc = "TZC peripheral ID 6 register"]
pub mod pid6;
#[doc = "TZC peripheral ID 7 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid7](pid7) module"]
pub type PID7 = crate::Reg<u32, _PID7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID7;
#[doc = "`read()` method returns [pid7::R](pid7::R) reader structure"]
impl crate::Readable for PID7 {}
#[doc = "TZC peripheral ID 7 register"]
pub mod pid7;
#[doc = "TZC peripheral ID 0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid0](pid0) module"]
pub type PID0 = crate::Reg<u32, _PID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID0;
#[doc = "`read()` method returns [pid0::R](pid0::R) reader structure"]
impl crate::Readable for PID0 {}
#[doc = "TZC peripheral ID 0 register"]
pub mod pid0;
#[doc = "TZC peripheral ID 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid1](pid1) module"]
pub type PID1 = crate::Reg<u32, _PID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID1;
#[doc = "`read()` method returns [pid1::R](pid1::R) reader structure"]
impl crate::Readable for PID1 {}
#[doc = "TZC peripheral ID 1 register"]
pub mod pid1;
#[doc = "TZC peripheral ID 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid2](pid2) module"]
pub type PID2 = crate::Reg<u32, _PID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID2;
#[doc = "`read()` method returns [pid2::R](pid2::R) reader structure"]
impl crate::Readable for PID2 {}
#[doc = "TZC peripheral ID 2 register"]
pub mod pid2;
#[doc = "TZC peripheral ID 3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid3](pid3) module"]
pub type PID3 = crate::Reg<u32, _PID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PID3;
#[doc = "`read()` method returns [pid3::R](pid3::R) reader structure"]
impl crate::Readable for PID3 {}
#[doc = "TZC peripheral ID 3 register"]
pub mod pid3;
#[doc = "TZC component ID 0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid0](cid0) module"]
pub type CID0 = crate::Reg<u32, _CID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID0;
#[doc = "`read()` method returns [cid0::R](cid0::R) reader structure"]
impl crate::Readable for CID0 {}
#[doc = "TZC component ID 0 register"]
pub mod cid0;
#[doc = "TZC component ID 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid1](cid1) module"]
pub type CID1 = crate::Reg<u32, _CID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID1;
#[doc = "`read()` method returns [cid1::R](cid1::R) reader structure"]
impl crate::Readable for CID1 {}
#[doc = "TZC component ID 1 register"]
pub mod cid1;
#[doc = "TZC component ID 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid2](cid2) module"]
pub type CID2 = crate::Reg<u32, _CID2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID2;
#[doc = "`read()` method returns [cid2::R](cid2::R) reader structure"]
impl crate::Readable for CID2 {}
#[doc = "TZC component ID 2 register"]
pub mod cid2;
#[doc = "TZC component ID 3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid3](cid3) module"]
pub type CID3 = crate::Reg<u32, _CID3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CID3;
#[doc = "`read()` method returns [cid3::R](cid3::R) reader structure"]
impl crate::Readable for CID3 {}
#[doc = "TZC component ID 3 register"]
pub mod cid3;
