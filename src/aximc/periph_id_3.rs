#[doc = "Reader of register PERIPH_ID_3"]
pub type R = crate::R<u32, super::PERIPH_ID_3>;
#[doc = "Reader of field `CUST_MOD_NUM`"]
pub type CUST_MOD_NUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `REV_AND`"]
pub type REV_AND_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - customer modification"]
    #[inline(always)]
    pub fn cust_mod_num(&self) -> CUST_MOD_NUM_R {
        CUST_MOD_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - customer version"]
    #[inline(always)]
    pub fn rev_and(&self) -> REV_AND_R {
        REV_AND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}