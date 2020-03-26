#[doc = "Reader of register DDRPHYC_DCUSR1"]
pub type R = crate::R<u32, super::DDRPHYC_DCUSR1>;
#[doc = "Reader of field `RDCNT`"]
pub type RDCNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `FLCND`"]
pub type FLCND_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPCNT`"]
pub type LPCNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - RDCNT"]
    #[inline(always)]
    pub fn rdcnt(&self) -> RDCNT_R {
        RDCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - FLCND"]
    #[inline(always)]
    pub fn flcnd(&self) -> FLCND_R {
        FLCND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - LPCNT"]
    #[inline(always)]
    pub fn lpcnt(&self) -> LPCNT_R {
        LPCNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
