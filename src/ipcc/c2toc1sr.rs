#[doc = "Reader of register C2TOC1SR"]
pub type R = crate::R<u32, super::C2TOC1SR>;
#[doc = "Reader of field `CH1F`"]
pub type CH1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH2F`"]
pub type CH2F_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CH1F"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH2F"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
