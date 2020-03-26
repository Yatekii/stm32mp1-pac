#[doc = "Reader of register DMAMUX1_RGCFR"]
pub type R = crate::R<u32, super::DMAMUX1_RGCFR>;
#[doc = "Reader of field `COF0`"]
pub type COF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `COF1`"]
pub type COF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `COF2`"]
pub type COF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `COF3`"]
pub type COF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `COF4`"]
pub type COF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `COF5`"]
pub type COF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `COF6`"]
pub type COF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `COF7`"]
pub type COF7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - COF0"]
    #[inline(always)]
    pub fn cof0(&self) -> COF0_R {
        COF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - COF1"]
    #[inline(always)]
    pub fn cof1(&self) -> COF1_R {
        COF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - COF2"]
    #[inline(always)]
    pub fn cof2(&self) -> COF2_R {
        COF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - COF3"]
    #[inline(always)]
    pub fn cof3(&self) -> COF3_R {
        COF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - COF4"]
    #[inline(always)]
    pub fn cof4(&self) -> COF4_R {
        COF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - COF5"]
    #[inline(always)]
    pub fn cof5(&self) -> COF5_R {
        COF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - COF6"]
    #[inline(always)]
    pub fn cof6(&self) -> COF6_R {
        COF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - COF7"]
    #[inline(always)]
    pub fn cof7(&self) -> COF7_R {
        COF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
