#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 8144usize],
    #[doc = "0x1fd0 - AXIMC peripheral ID4 register"]
    pub periph_id_4: PERIPH_ID_4,
    #[doc = "0x1fd4 - AXIMC peripheral ID5 register"]
    pub periph_id_5: PERIPH_ID_5,
    #[doc = "0x1fd8 - AXIMC peripheral ID6 register"]
    pub periph_id_6: PERIPH_ID_6,
    _reserved3: [u8; 4usize],
    #[doc = "0x1fe0 - AXIMC peripheral ID0 register"]
    pub periph_id_0: PERIPH_ID_0,
    #[doc = "0x1fe4 - AXIMC peripheral ID1 register"]
    pub periph_id_1: PERIPH_ID_1,
    #[doc = "0x1fe8 - AXIMC peripheral ID2 register"]
    pub periph_id_2: PERIPH_ID_2,
    #[doc = "0x1fec - AXIMC peripheral ID3 register"]
    pub periph_id_3: PERIPH_ID_3,
    #[doc = "0x1ff0 - AXIMC component ID0 register"]
    pub comp_id_0: COMP_ID_0,
    #[doc = "0x1ff4 - AXIMC component ID1 register"]
    pub comp_id_1: COMP_ID_1,
    #[doc = "0x1ff8 - AXIMC component ID2 register"]
    pub comp_id_2: COMP_ID_2,
    #[doc = "0x1ffc - AXIMC component ID3 register"]
    pub comp_id_3: COMP_ID_3,
    _reserved11: [u8; 122312usize],
    #[doc = "0x1fdc8 - AXIMC peripheral ID7 register"]
    pub periph_id_7: PERIPH_ID_7,
    _reserved12: [u8; 139864usize],
    #[doc = "0x42024 - AXIMC master 0 packing functionality register"]
    pub m0_fn_mod2: M0_FN_MOD2,
    #[doc = "0x42028 - AXIMC master 0 AHB conversion override functionality register"]
    pub m0_fn_mod_ahb: M0_FN_MOD_AHB,
    _reserved14: [u8; 212usize],
    #[doc = "0x42100 - AXIMC master 0 read priority register"]
    pub m0_read_qos: M0_READ_QOS,
    #[doc = "0x42104 - AXIMC master 0 write priority register"]
    pub m0_write_qos: M0_WRITE_QOS,
    #[doc = "0x42108 - AXIMC master 0 issuing capability override functionality register"]
    pub m0_fn_mod: M0_FN_MOD,
    _reserved17: [u8; 3864usize],
    #[doc = "0x43024 - AXIMC master 1 packing functionality register"]
    pub m1_fn_mod2: M1_FN_MOD2,
    #[doc = "0x43028 - AXIMC master 1 AHB conversion override functionality register"]
    pub m1_fn_mod_ahb: M1_FN_MOD_AHB,
    _reserved19: [u8; 212usize],
    #[doc = "0x43100 - AXIMC master 1 read priority register"]
    pub m1_read_qos: M1_READ_QOS,
    #[doc = "0x43104 - AXIMC master 1 write priority register"]
    pub m1_write_qos: M1_WRITE_QOS,
    #[doc = "0x43108 - AXIMC master 1 issuing capability override functionality register"]
    pub m1_fn_mod: M1_FN_MOD,
    _reserved22: [u8; 3864usize],
    #[doc = "0x44024 - AXIMC master 2 packing functionality register"]
    pub m2_fn_mod2: M2_FN_MOD2,
    #[doc = "0x44028 - AXIMC master 2 AHB conversion override functionality register"]
    pub m2_fn_mod_ahb: M2_FN_MOD_AHB,
    _reserved24: [u8; 212usize],
    #[doc = "0x44100 - AXIMC master 2 read priority register"]
    pub m2_read_qos: M2_READ_QOS,
    #[doc = "0x44104 - AXIMC master 2 write priority register"]
    pub m2_write_qos: M2_WRITE_QOS,
    #[doc = "0x44108 - AXIMC master 2 issuing capability override functionality register"]
    pub m2_fn_mod: M2_FN_MOD,
    _reserved27: [u8; 3864usize],
    #[doc = "0x45024 - AXIMC master 5 packing functionality register"]
    pub m5_fn_mod2: M5_FN_MOD2,
    #[doc = "0x45028 - AXIMC master 5 AHB conversion override functionality register"]
    pub m5_fn_mod_ahb: M5_FN_MOD_AHB,
    _reserved29: [u8; 212usize],
    #[doc = "0x45100 - AXIMC master 5 read priority register"]
    pub m5_read_qos: M5_READ_QOS,
    #[doc = "0x45104 - AXIMC master 5 write priority register"]
    pub m5_write_qos: M5_WRITE_QOS,
    #[doc = "0x45108 - AXIMC master 5 issuing capability override functionality register"]
    pub m5_fn_mod: M5_FN_MOD,
    _reserved32: [u8; 4084usize],
    #[doc = "0x46100 - AXIMC master 3 read priority register"]
    pub m3_read_qos: M3_READ_QOS,
    #[doc = "0x46104 - AXIMC master 3 write priority register"]
    pub m3_write_qos: M3_WRITE_QOS,
    #[doc = "0x46108 - AXIMC master 3 packing functionality register"]
    pub m3_fn_mod: M3_FN_MOD,
    _reserved35: [u8; 4084usize],
    #[doc = "0x47100 - AXIMC master 7 read priority register"]
    pub m7_read_qos: M7_READ_QOS,
    #[doc = "0x47104 - AXIMC master 7 write priority register"]
    pub m7_write_qos: M7_WRITE_QOS,
    #[doc = "0x47108 - AXIMC master 7 issuing capability override functionality register"]
    pub m7_fn_mod: M7_FN_MOD,
    _reserved38: [u8; 4084usize],
    #[doc = "0x48100 - AXIMC master 8 read priority register"]
    pub m8_read_qos: M8_READ_QOS,
    #[doc = "0x48104 - AXIMC master 8 write priority register"]
    pub m8_write_qos: M8_WRITE_QOS,
    #[doc = "0x48108 - AXIMC master 8 issuing capability override functionality register"]
    pub m8_fn_mod: M8_FN_MOD,
    _reserved41: [u8; 7968usize],
    #[doc = "0x4a02c - AXIMC long burst capability inhibition register"]
    pub fn_mod_lb: FN_MOD_LB,
    _reserved42: [u8; 208usize],
    #[doc = "0x4a100 - AXIMC master 4 read priority register"]
    pub m4_read_qos: M4_READ_QOS,
    #[doc = "0x4a104 - AXIMC master 4 write priority register"]
    pub m4_write_qos: M4_WRITE_QOS,
    #[doc = "0x4a108 - AXIMC master 4 packing functionality register"]
    pub m4_fn_mod: M4_FN_MOD,
    _reserved45: [u8; 4084usize],
    #[doc = "0x4b100 - AXIMC master 9 read priority register"]
    pub m9_read_qos: M9_READ_QOS,
    #[doc = "0x4b104 - AXIMC master 9 write priority register"]
    pub m9_write_qos: M9_WRITE_QOS,
    #[doc = "0x4b108 - AXIMC master 9 issuing capability override functionality register"]
    pub m9_fn_mod: M9_FN_MOD,
    _reserved48: [u8; 4084usize],
    #[doc = "0x4c100 - AXIMC master 10 read priority register"]
    pub m10_read_qos: M10_READ_QOS,
    #[doc = "0x4c104 - AXIMC master 10 write priority register"]
    pub m10_write_qos: M10_WRITE_QOS,
    #[doc = "0x4c108 - AXIMC master 10 issuing capability override functionality register"]
    pub m10_fn_mod: M10_FN_MOD,
    _reserved51: [u8; 3868usize],
    #[doc = "0x4d028 - AXIMC master 6 AHB conversion override functionality register"]
    pub m6_fn_mod_ahb: M6_FN_MOD_AHB,
    _reserved52: [u8; 212usize],
    #[doc = "0x4d100 - AXIMC master 6 read priority register"]
    pub m6_read_qos: M6_READ_QOS,
    #[doc = "0x4d104 - AXIMC master 6 write priority register"]
    pub m6_write_qos: M6_WRITE_QOS,
    #[doc = "0x4d108 - AXIMC master 6 issuing capability override functionality register"]
    pub m6_fn_mod: M6_FN_MOD,
}
#[doc = "AXIMC peripheral ID4 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_4](periph_id_4) module"]
pub type PERIPH_ID_4 = crate::Reg<u32, _PERIPH_ID_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_4;
#[doc = "`read()` method returns [periph_id_4::R](periph_id_4::R) reader structure"]
impl crate::Readable for PERIPH_ID_4 {}
#[doc = "AXIMC peripheral ID4 register"]
pub mod periph_id_4;
#[doc = "AXIMC peripheral ID5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_5](periph_id_5) module"]
pub type PERIPH_ID_5 = crate::Reg<u32, _PERIPH_ID_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_5;
#[doc = "`read()` method returns [periph_id_5::R](periph_id_5::R) reader structure"]
impl crate::Readable for PERIPH_ID_5 {}
#[doc = "AXIMC peripheral ID5 register"]
pub mod periph_id_5;
#[doc = "AXIMC peripheral ID6 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_6](periph_id_6) module"]
pub type PERIPH_ID_6 = crate::Reg<u32, _PERIPH_ID_6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_6;
#[doc = "`read()` method returns [periph_id_6::R](periph_id_6::R) reader structure"]
impl crate::Readable for PERIPH_ID_6 {}
#[doc = "AXIMC peripheral ID6 register"]
pub mod periph_id_6;
#[doc = "AXIMC peripheral ID7 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_7](periph_id_7) module"]
pub type PERIPH_ID_7 = crate::Reg<u32, _PERIPH_ID_7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_7;
#[doc = "`read()` method returns [periph_id_7::R](periph_id_7::R) reader structure"]
impl crate::Readable for PERIPH_ID_7 {}
#[doc = "AXIMC peripheral ID7 register"]
pub mod periph_id_7;
#[doc = "AXIMC peripheral ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_0](periph_id_0) module"]
pub type PERIPH_ID_0 = crate::Reg<u32, _PERIPH_ID_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_0;
#[doc = "`read()` method returns [periph_id_0::R](periph_id_0::R) reader structure"]
impl crate::Readable for PERIPH_ID_0 {}
#[doc = "AXIMC peripheral ID0 register"]
pub mod periph_id_0;
#[doc = "AXIMC peripheral ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_1](periph_id_1) module"]
pub type PERIPH_ID_1 = crate::Reg<u32, _PERIPH_ID_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_1;
#[doc = "`read()` method returns [periph_id_1::R](periph_id_1::R) reader structure"]
impl crate::Readable for PERIPH_ID_1 {}
#[doc = "AXIMC peripheral ID1 register"]
pub mod periph_id_1;
#[doc = "AXIMC peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_2](periph_id_2) module"]
pub type PERIPH_ID_2 = crate::Reg<u32, _PERIPH_ID_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_2;
#[doc = "`read()` method returns [periph_id_2::R](periph_id_2::R) reader structure"]
impl crate::Readable for PERIPH_ID_2 {}
#[doc = "AXIMC peripheral ID2 register"]
pub mod periph_id_2;
#[doc = "AXIMC peripheral ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periph_id_3](periph_id_3) module"]
pub type PERIPH_ID_3 = crate::Reg<u32, _PERIPH_ID_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPH_ID_3;
#[doc = "`read()` method returns [periph_id_3::R](periph_id_3::R) reader structure"]
impl crate::Readable for PERIPH_ID_3 {}
#[doc = "AXIMC peripheral ID3 register"]
pub mod periph_id_3;
#[doc = "AXIMC component ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_id_0](comp_id_0) module"]
pub type COMP_ID_0 = crate::Reg<u32, _COMP_ID_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_ID_0;
#[doc = "`read()` method returns [comp_id_0::R](comp_id_0::R) reader structure"]
impl crate::Readable for COMP_ID_0 {}
#[doc = "AXIMC component ID0 register"]
pub mod comp_id_0;
#[doc = "AXIMC component ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_id_1](comp_id_1) module"]
pub type COMP_ID_1 = crate::Reg<u32, _COMP_ID_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_ID_1;
#[doc = "`read()` method returns [comp_id_1::R](comp_id_1::R) reader structure"]
impl crate::Readable for COMP_ID_1 {}
#[doc = "AXIMC component ID1 register"]
pub mod comp_id_1;
#[doc = "AXIMC component ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_id_2](comp_id_2) module"]
pub type COMP_ID_2 = crate::Reg<u32, _COMP_ID_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_ID_2;
#[doc = "`read()` method returns [comp_id_2::R](comp_id_2::R) reader structure"]
impl crate::Readable for COMP_ID_2 {}
#[doc = "AXIMC component ID2 register"]
pub mod comp_id_2;
#[doc = "AXIMC component ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_id_3](comp_id_3) module"]
pub type COMP_ID_3 = crate::Reg<u32, _COMP_ID_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_ID_3;
#[doc = "`read()` method returns [comp_id_3::R](comp_id_3::R) reader structure"]
impl crate::Readable for COMP_ID_3 {}
#[doc = "AXIMC component ID3 register"]
pub mod comp_id_3;
#[doc = "AXIMC master 0 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m0_fn_mod2](m0_fn_mod2) module"]
pub type M0_FN_MOD2 = crate::Reg<u32, _M0_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M0_FN_MOD2;
#[doc = "`read()` method returns [m0_fn_mod2::R](m0_fn_mod2::R) reader structure"]
impl crate::Readable for M0_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [m0_fn_mod2::W](m0_fn_mod2::W) writer structure"]
impl crate::Writable for M0_FN_MOD2 {}
#[doc = "AXIMC master 0 packing functionality register"]
pub mod m0_fn_mod2;
#[doc = "AXIMC master 1 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1_fn_mod2](m1_fn_mod2) module"]
pub type M1_FN_MOD2 = crate::Reg<u32, _M1_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1_FN_MOD2;
#[doc = "`read()` method returns [m1_fn_mod2::R](m1_fn_mod2::R) reader structure"]
impl crate::Readable for M1_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [m1_fn_mod2::W](m1_fn_mod2::W) writer structure"]
impl crate::Writable for M1_FN_MOD2 {}
#[doc = "AXIMC master 1 packing functionality register"]
pub mod m1_fn_mod2;
#[doc = "AXIMC master 2 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2_fn_mod2](m2_fn_mod2) module"]
pub type M2_FN_MOD2 = crate::Reg<u32, _M2_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M2_FN_MOD2;
#[doc = "`read()` method returns [m2_fn_mod2::R](m2_fn_mod2::R) reader structure"]
impl crate::Readable for M2_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [m2_fn_mod2::W](m2_fn_mod2::W) writer structure"]
impl crate::Writable for M2_FN_MOD2 {}
#[doc = "AXIMC master 2 packing functionality register"]
pub mod m2_fn_mod2;
#[doc = "AXIMC master 0 AHB conversion override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m0_fn_mod_ahb](m0_fn_mod_ahb) module"]
pub type M0_FN_MOD_AHB = crate::Reg<u32, _M0_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M0_FN_MOD_AHB;
#[doc = "`read()` method returns [m0_fn_mod_ahb::R](m0_fn_mod_ahb::R) reader structure"]
impl crate::Readable for M0_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [m0_fn_mod_ahb::W](m0_fn_mod_ahb::W) writer structure"]
impl crate::Writable for M0_FN_MOD_AHB {}
#[doc = "AXIMC master 0 AHB conversion override functionality register"]
pub mod m0_fn_mod_ahb;
#[doc = "AXIMC master 1 AHB conversion override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1_fn_mod_ahb](m1_fn_mod_ahb) module"]
pub type M1_FN_MOD_AHB = crate::Reg<u32, _M1_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1_FN_MOD_AHB;
#[doc = "`read()` method returns [m1_fn_mod_ahb::R](m1_fn_mod_ahb::R) reader structure"]
impl crate::Readable for M1_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [m1_fn_mod_ahb::W](m1_fn_mod_ahb::W) writer structure"]
impl crate::Writable for M1_FN_MOD_AHB {}
#[doc = "AXIMC master 1 AHB conversion override functionality register"]
pub mod m1_fn_mod_ahb;
#[doc = "AXIMC master 2 AHB conversion override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2_fn_mod_ahb](m2_fn_mod_ahb) module"]
pub type M2_FN_MOD_AHB = crate::Reg<u32, _M2_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M2_FN_MOD_AHB;
#[doc = "`read()` method returns [m2_fn_mod_ahb::R](m2_fn_mod_ahb::R) reader structure"]
impl crate::Readable for M2_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [m2_fn_mod_ahb::W](m2_fn_mod_ahb::W) writer structure"]
impl crate::Writable for M2_FN_MOD_AHB {}
#[doc = "AXIMC master 2 AHB conversion override functionality register"]
pub mod m2_fn_mod_ahb;
#[doc = "AXIMC master 0 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m0_read_qos](m0_read_qos) module"]
pub type M0_READ_QOS = crate::Reg<u32, _M0_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M0_READ_QOS;
#[doc = "`read()` method returns [m0_read_qos::R](m0_read_qos::R) reader structure"]
impl crate::Readable for M0_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [m0_read_qos::W](m0_read_qos::W) writer structure"]
impl crate::Writable for M0_READ_QOS {}
#[doc = "AXIMC master 0 read priority register"]
pub mod m0_read_qos;
#[doc = "AXIMC master 1 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1_read_qos](m1_read_qos) module"]
pub type M1_READ_QOS = crate::Reg<u32, _M1_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1_READ_QOS;
#[doc = "`read()` method returns [m1_read_qos::R](m1_read_qos::R) reader structure"]
impl crate::Readable for M1_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [m1_read_qos::W](m1_read_qos::W) writer structure"]
impl crate::Writable for M1_READ_QOS {}
#[doc = "AXIMC master 1 read priority register"]
pub mod m1_read_qos;
#[doc = "AXIMC master 2 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2_read_qos](m2_read_qos) module"]
pub type M2_READ_QOS = crate::Reg<u32, _M2_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M2_READ_QOS;
#[doc = "`read()` method returns [m2_read_qos::R](m2_read_qos::R) reader structure"]
impl crate::Readable for M2_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [m2_read_qos::W](m2_read_qos::W) writer structure"]
impl crate::Writable for M2_READ_QOS {}
#[doc = "AXIMC master 2 read priority register"]
pub mod m2_read_qos;
#[doc = "AXIMC master 0 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m0_write_qos](m0_write_qos) module"]
pub type M0_WRITE_QOS = crate::Reg<u32, _M0_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M0_WRITE_QOS;
#[doc = "`read()` method returns [m0_write_qos::R](m0_write_qos::R) reader structure"]
impl crate::Readable for M0_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [m0_write_qos::W](m0_write_qos::W) writer structure"]
impl crate::Writable for M0_WRITE_QOS {}
#[doc = "AXIMC master 0 write priority register"]
pub mod m0_write_qos;
#[doc = "AXIMC master 1 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1_write_qos](m1_write_qos) module"]
pub type M1_WRITE_QOS = crate::Reg<u32, _M1_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1_WRITE_QOS;
#[doc = "`read()` method returns [m1_write_qos::R](m1_write_qos::R) reader structure"]
impl crate::Readable for M1_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [m1_write_qos::W](m1_write_qos::W) writer structure"]
impl crate::Writable for M1_WRITE_QOS {}
#[doc = "AXIMC master 1 write priority register"]
pub mod m1_write_qos;
#[doc = "AXIMC master 2 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2_write_qos](m2_write_qos) module"]
pub type M2_WRITE_QOS = crate::Reg<u32, _M2_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M2_WRITE_QOS;
#[doc = "`read()` method returns [m2_write_qos::R](m2_write_qos::R) reader structure"]
impl crate::Readable for M2_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [m2_write_qos::W](m2_write_qos::W) writer structure"]
impl crate::Writable for M2_WRITE_QOS {}
#[doc = "AXIMC master 2 write priority register"]
pub mod m2_write_qos;
#[doc = "AXIMC master 0 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m0_fn_mod](m0_fn_mod) module"]
pub type M0_FN_MOD = crate::Reg<u32, _M0_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M0_FN_MOD;
#[doc = "`read()` method returns [m0_fn_mod::R](m0_fn_mod::R) reader structure"]
impl crate::Readable for M0_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [m0_fn_mod::W](m0_fn_mod::W) writer structure"]
impl crate::Writable for M0_FN_MOD {}
#[doc = "AXIMC master 0 issuing capability override functionality register"]
pub mod m0_fn_mod;
#[doc = "AXIMC master 1 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m1_fn_mod](m1_fn_mod) module"]
pub type M1_FN_MOD = crate::Reg<u32, _M1_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M1_FN_MOD;
#[doc = "`read()` method returns [m1_fn_mod::R](m1_fn_mod::R) reader structure"]
impl crate::Readable for M1_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [m1_fn_mod::W](m1_fn_mod::W) writer structure"]
impl crate::Writable for M1_FN_MOD {}
#[doc = "AXIMC master 1 issuing capability override functionality register"]
pub mod m1_fn_mod;
#[doc = "AXIMC master 2 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2_fn_mod](m2_fn_mod) module"]
pub type M2_FN_MOD = crate::Reg<u32, _M2_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M2_FN_MOD;
#[doc = "`read()` method returns [m2_fn_mod::R](m2_fn_mod::R) reader structure"]
impl crate::Readable for M2_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [m2_fn_mod::W](m2_fn_mod::W) writer structure"]
impl crate::Writable for M2_FN_MOD {}
#[doc = "AXIMC master 2 issuing capability override functionality register"]
pub mod m2_fn_mod;
#[doc = "AXIMC master 5 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5_fn_mod2](m5_fn_mod2) module"]
pub type M5_FN_MOD2 = crate::Reg<u32, _M5_FN_MOD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M5_FN_MOD2;
#[doc = "`read()` method returns [m5_fn_mod2::R](m5_fn_mod2::R) reader structure"]
impl crate::Readable for M5_FN_MOD2 {}
#[doc = "`write(|w| ..)` method takes [m5_fn_mod2::W](m5_fn_mod2::W) writer structure"]
impl crate::Writable for M5_FN_MOD2 {}
#[doc = "AXIMC master 5 packing functionality register"]
pub mod m5_fn_mod2;
#[doc = "AXIMC master 5 AHB conversion override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5_fn_mod_ahb](m5_fn_mod_ahb) module"]
pub type M5_FN_MOD_AHB = crate::Reg<u32, _M5_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M5_FN_MOD_AHB;
#[doc = "`read()` method returns [m5_fn_mod_ahb::R](m5_fn_mod_ahb::R) reader structure"]
impl crate::Readable for M5_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [m5_fn_mod_ahb::W](m5_fn_mod_ahb::W) writer structure"]
impl crate::Writable for M5_FN_MOD_AHB {}
#[doc = "AXIMC master 5 AHB conversion override functionality register"]
pub mod m5_fn_mod_ahb;
#[doc = "AXIMC master 6 AHB conversion override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m6_fn_mod_ahb](m6_fn_mod_ahb) module"]
pub type M6_FN_MOD_AHB = crate::Reg<u32, _M6_FN_MOD_AHB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M6_FN_MOD_AHB;
#[doc = "`read()` method returns [m6_fn_mod_ahb::R](m6_fn_mod_ahb::R) reader structure"]
impl crate::Readable for M6_FN_MOD_AHB {}
#[doc = "`write(|w| ..)` method takes [m6_fn_mod_ahb::W](m6_fn_mod_ahb::W) writer structure"]
impl crate::Writable for M6_FN_MOD_AHB {}
#[doc = "AXIMC master 6 AHB conversion override functionality register"]
pub mod m6_fn_mod_ahb;
#[doc = "AXIMC master 5 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5_read_qos](m5_read_qos) module"]
pub type M5_READ_QOS = crate::Reg<u32, _M5_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M5_READ_QOS;
#[doc = "`read()` method returns [m5_read_qos::R](m5_read_qos::R) reader structure"]
impl crate::Readable for M5_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [m5_read_qos::W](m5_read_qos::W) writer structure"]
impl crate::Writable for M5_READ_QOS {}
#[doc = "AXIMC master 5 read priority register"]
pub mod m5_read_qos;
#[doc = "AXIMC master 6 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m6_read_qos](m6_read_qos) module"]
pub type M6_READ_QOS = crate::Reg<u32, _M6_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M6_READ_QOS;
#[doc = "`read()` method returns [m6_read_qos::R](m6_read_qos::R) reader structure"]
impl crate::Readable for M6_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [m6_read_qos::W](m6_read_qos::W) writer structure"]
impl crate::Writable for M6_READ_QOS {}
#[doc = "AXIMC master 6 read priority register"]
pub mod m6_read_qos;
#[doc = "AXIMC master 5 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5_write_qos](m5_write_qos) module"]
pub type M5_WRITE_QOS = crate::Reg<u32, _M5_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M5_WRITE_QOS;
#[doc = "`read()` method returns [m5_write_qos::R](m5_write_qos::R) reader structure"]
impl crate::Readable for M5_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [m5_write_qos::W](m5_write_qos::W) writer structure"]
impl crate::Writable for M5_WRITE_QOS {}
#[doc = "AXIMC master 5 write priority register"]
pub mod m5_write_qos;
#[doc = "AXIMC master 6 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m6_write_qos](m6_write_qos) module"]
pub type M6_WRITE_QOS = crate::Reg<u32, _M6_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M6_WRITE_QOS;
#[doc = "`read()` method returns [m6_write_qos::R](m6_write_qos::R) reader structure"]
impl crate::Readable for M6_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [m6_write_qos::W](m6_write_qos::W) writer structure"]
impl crate::Writable for M6_WRITE_QOS {}
#[doc = "AXIMC master 6 write priority register"]
pub mod m6_write_qos;
#[doc = "AXIMC master 5 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5_fn_mod](m5_fn_mod) module"]
pub type M5_FN_MOD = crate::Reg<u32, _M5_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M5_FN_MOD;
#[doc = "`read()` method returns [m5_fn_mod::R](m5_fn_mod::R) reader structure"]
impl crate::Readable for M5_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [m5_fn_mod::W](m5_fn_mod::W) writer structure"]
impl crate::Writable for M5_FN_MOD {}
#[doc = "AXIMC master 5 issuing capability override functionality register"]
pub mod m5_fn_mod;
#[doc = "AXIMC master 6 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m6_fn_mod](m6_fn_mod) module"]
pub type M6_FN_MOD = crate::Reg<u32, _M6_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M6_FN_MOD;
#[doc = "`read()` method returns [m6_fn_mod::R](m6_fn_mod::R) reader structure"]
impl crate::Readable for M6_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [m6_fn_mod::W](m6_fn_mod::W) writer structure"]
impl crate::Writable for M6_FN_MOD {}
#[doc = "AXIMC master 6 issuing capability override functionality register"]
pub mod m6_fn_mod;
#[doc = "AXIMC master 3 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3_read_qos](m3_read_qos) module"]
pub type M3_READ_QOS = crate::Reg<u32, _M3_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M3_READ_QOS;
#[doc = "`read()` method returns [m3_read_qos::R](m3_read_qos::R) reader structure"]
impl crate::Readable for M3_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [m3_read_qos::W](m3_read_qos::W) writer structure"]
impl crate::Writable for M3_READ_QOS {}
#[doc = "AXIMC master 3 read priority register"]
pub mod m3_read_qos;
#[doc = "AXIMC master 4 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_read_qos](m4_read_qos) module"]
pub type M4_READ_QOS = crate::Reg<u32, _M4_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M4_READ_QOS;
#[doc = "`read()` method returns [m4_read_qos::R](m4_read_qos::R) reader structure"]
impl crate::Readable for M4_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [m4_read_qos::W](m4_read_qos::W) writer structure"]
impl crate::Writable for M4_READ_QOS {}
#[doc = "AXIMC master 4 read priority register"]
pub mod m4_read_qos;
#[doc = "AXIMC master 3 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3_write_qos](m3_write_qos) module"]
pub type M3_WRITE_QOS = crate::Reg<u32, _M3_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M3_WRITE_QOS;
#[doc = "`read()` method returns [m3_write_qos::R](m3_write_qos::R) reader structure"]
impl crate::Readable for M3_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [m3_write_qos::W](m3_write_qos::W) writer structure"]
impl crate::Writable for M3_WRITE_QOS {}
#[doc = "AXIMC master 3 write priority register"]
pub mod m3_write_qos;
#[doc = "AXIMC master 4 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_write_qos](m4_write_qos) module"]
pub type M4_WRITE_QOS = crate::Reg<u32, _M4_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M4_WRITE_QOS;
#[doc = "`read()` method returns [m4_write_qos::R](m4_write_qos::R) reader structure"]
impl crate::Readable for M4_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [m4_write_qos::W](m4_write_qos::W) writer structure"]
impl crate::Writable for M4_WRITE_QOS {}
#[doc = "AXIMC master 4 write priority register"]
pub mod m4_write_qos;
#[doc = "AXIMC master 3 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3_fn_mod](m3_fn_mod) module"]
pub type M3_FN_MOD = crate::Reg<u32, _M3_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M3_FN_MOD;
#[doc = "`read()` method returns [m3_fn_mod::R](m3_fn_mod::R) reader structure"]
impl crate::Readable for M3_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [m3_fn_mod::W](m3_fn_mod::W) writer structure"]
impl crate::Writable for M3_FN_MOD {}
#[doc = "AXIMC master 3 packing functionality register"]
pub mod m3_fn_mod;
#[doc = "AXIMC master 4 packing functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_fn_mod](m4_fn_mod) module"]
pub type M4_FN_MOD = crate::Reg<u32, _M4_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M4_FN_MOD;
#[doc = "`read()` method returns [m4_fn_mod::R](m4_fn_mod::R) reader structure"]
impl crate::Readable for M4_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [m4_fn_mod::W](m4_fn_mod::W) writer structure"]
impl crate::Writable for M4_FN_MOD {}
#[doc = "AXIMC master 4 packing functionality register"]
pub mod m4_fn_mod;
#[doc = "AXIMC master 7 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m7_read_qos](m7_read_qos) module"]
pub type M7_READ_QOS = crate::Reg<u32, _M7_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M7_READ_QOS;
#[doc = "`read()` method returns [m7_read_qos::R](m7_read_qos::R) reader structure"]
impl crate::Readable for M7_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [m7_read_qos::W](m7_read_qos::W) writer structure"]
impl crate::Writable for M7_READ_QOS {}
#[doc = "AXIMC master 7 read priority register"]
pub mod m7_read_qos;
#[doc = "AXIMC master 8 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m8_read_qos](m8_read_qos) module"]
pub type M8_READ_QOS = crate::Reg<u32, _M8_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M8_READ_QOS;
#[doc = "`read()` method returns [m8_read_qos::R](m8_read_qos::R) reader structure"]
impl crate::Readable for M8_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [m8_read_qos::W](m8_read_qos::W) writer structure"]
impl crate::Writable for M8_READ_QOS {}
#[doc = "AXIMC master 8 read priority register"]
pub mod m8_read_qos;
#[doc = "AXIMC master 7 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m7_write_qos](m7_write_qos) module"]
pub type M7_WRITE_QOS = crate::Reg<u32, _M7_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M7_WRITE_QOS;
#[doc = "`read()` method returns [m7_write_qos::R](m7_write_qos::R) reader structure"]
impl crate::Readable for M7_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [m7_write_qos::W](m7_write_qos::W) writer structure"]
impl crate::Writable for M7_WRITE_QOS {}
#[doc = "AXIMC master 7 write priority register"]
pub mod m7_write_qos;
#[doc = "AXIMC master 8 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m8_write_qos](m8_write_qos) module"]
pub type M8_WRITE_QOS = crate::Reg<u32, _M8_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M8_WRITE_QOS;
#[doc = "`read()` method returns [m8_write_qos::R](m8_write_qos::R) reader structure"]
impl crate::Readable for M8_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [m8_write_qos::W](m8_write_qos::W) writer structure"]
impl crate::Writable for M8_WRITE_QOS {}
#[doc = "AXIMC master 8 write priority register"]
pub mod m8_write_qos;
#[doc = "AXIMC master 7 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m7_fn_mod](m7_fn_mod) module"]
pub type M7_FN_MOD = crate::Reg<u32, _M7_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M7_FN_MOD;
#[doc = "`read()` method returns [m7_fn_mod::R](m7_fn_mod::R) reader structure"]
impl crate::Readable for M7_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [m7_fn_mod::W](m7_fn_mod::W) writer structure"]
impl crate::Writable for M7_FN_MOD {}
#[doc = "AXIMC master 7 issuing capability override functionality register"]
pub mod m7_fn_mod;
#[doc = "AXIMC master 8 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m8_fn_mod](m8_fn_mod) module"]
pub type M8_FN_MOD = crate::Reg<u32, _M8_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M8_FN_MOD;
#[doc = "`read()` method returns [m8_fn_mod::R](m8_fn_mod::R) reader structure"]
impl crate::Readable for M8_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [m8_fn_mod::W](m8_fn_mod::W) writer structure"]
impl crate::Writable for M8_FN_MOD {}
#[doc = "AXIMC master 8 issuing capability override functionality register"]
pub mod m8_fn_mod;
#[doc = "AXIMC long burst capability inhibition register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fn_mod_lb](fn_mod_lb) module"]
pub type FN_MOD_LB = crate::Reg<u32, _FN_MOD_LB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FN_MOD_LB;
#[doc = "`read()` method returns [fn_mod_lb::R](fn_mod_lb::R) reader structure"]
impl crate::Readable for FN_MOD_LB {}
#[doc = "`write(|w| ..)` method takes [fn_mod_lb::W](fn_mod_lb::W) writer structure"]
impl crate::Writable for FN_MOD_LB {}
#[doc = "AXIMC long burst capability inhibition register"]
pub mod fn_mod_lb;
#[doc = "AXIMC master 9 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m9_read_qos](m9_read_qos) module"]
pub type M9_READ_QOS = crate::Reg<u32, _M9_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M9_READ_QOS;
#[doc = "`read()` method returns [m9_read_qos::R](m9_read_qos::R) reader structure"]
impl crate::Readable for M9_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [m9_read_qos::W](m9_read_qos::W) writer structure"]
impl crate::Writable for M9_READ_QOS {}
#[doc = "AXIMC master 9 read priority register"]
pub mod m9_read_qos;
#[doc = "AXIMC master 10 read priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m10_read_qos](m10_read_qos) module"]
pub type M10_READ_QOS = crate::Reg<u32, _M10_READ_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M10_READ_QOS;
#[doc = "`read()` method returns [m10_read_qos::R](m10_read_qos::R) reader structure"]
impl crate::Readable for M10_READ_QOS {}
#[doc = "`write(|w| ..)` method takes [m10_read_qos::W](m10_read_qos::W) writer structure"]
impl crate::Writable for M10_READ_QOS {}
#[doc = "AXIMC master 10 read priority register"]
pub mod m10_read_qos;
#[doc = "AXIMC master 9 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m9_write_qos](m9_write_qos) module"]
pub type M9_WRITE_QOS = crate::Reg<u32, _M9_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M9_WRITE_QOS;
#[doc = "`read()` method returns [m9_write_qos::R](m9_write_qos::R) reader structure"]
impl crate::Readable for M9_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [m9_write_qos::W](m9_write_qos::W) writer structure"]
impl crate::Writable for M9_WRITE_QOS {}
#[doc = "AXIMC master 9 write priority register"]
pub mod m9_write_qos;
#[doc = "AXIMC master 10 write priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m10_write_qos](m10_write_qos) module"]
pub type M10_WRITE_QOS = crate::Reg<u32, _M10_WRITE_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M10_WRITE_QOS;
#[doc = "`read()` method returns [m10_write_qos::R](m10_write_qos::R) reader structure"]
impl crate::Readable for M10_WRITE_QOS {}
#[doc = "`write(|w| ..)` method takes [m10_write_qos::W](m10_write_qos::W) writer structure"]
impl crate::Writable for M10_WRITE_QOS {}
#[doc = "AXIMC master 10 write priority register"]
pub mod m10_write_qos;
#[doc = "AXIMC master 9 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m9_fn_mod](m9_fn_mod) module"]
pub type M9_FN_MOD = crate::Reg<u32, _M9_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M9_FN_MOD;
#[doc = "`read()` method returns [m9_fn_mod::R](m9_fn_mod::R) reader structure"]
impl crate::Readable for M9_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [m9_fn_mod::W](m9_fn_mod::W) writer structure"]
impl crate::Writable for M9_FN_MOD {}
#[doc = "AXIMC master 9 issuing capability override functionality register"]
pub mod m9_fn_mod;
#[doc = "AXIMC master 10 issuing capability override functionality register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m10_fn_mod](m10_fn_mod) module"]
pub type M10_FN_MOD = crate::Reg<u32, _M10_FN_MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _M10_FN_MOD;
#[doc = "`read()` method returns [m10_fn_mod::R](m10_fn_mod::R) reader structure"]
impl crate::Readable for M10_FN_MOD {}
#[doc = "`write(|w| ..)` method takes [m10_fn_mod::W](m10_fn_mod::W) writer structure"]
impl crate::Writable for M10_FN_MOD {}
#[doc = "AXIMC master 10 issuing capability override functionality register"]
pub mod m10_fn_mod;
