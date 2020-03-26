#[doc = "Reader of register FAIL_CONTROL0"]
pub type R = crate::R<u32, super::FAIL_CONTROL0>;
#[doc = "Reader of field `PRIVILEGE`"]
pub type PRIVILEGE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NON_SECURE`"]
pub type NON_SECURE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIRECTION`"]
pub type DIRECTION_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 20 - Privilege access failure"]
    #[inline(always)]
    pub fn privilege(&self) -> PRIVILEGE_R {
        PRIVILEGE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Non-secure access failure"]
    #[inline(always)]
    pub fn non_secure(&self) -> NON_SECURE_R {
        NON_SECURE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Access failure direction"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
