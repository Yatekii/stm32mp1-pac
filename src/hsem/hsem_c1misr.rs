#[doc = "Reader of register HSEM_C1MISR"]
pub type R = crate::R<u32, super::HSEM_C1MISR>;
#[doc = "Reader of field `MISF0`"]
pub type MISF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF1`"]
pub type MISF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF2`"]
pub type MISF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF3`"]
pub type MISF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF4`"]
pub type MISF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF5`"]
pub type MISF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF6`"]
pub type MISF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF7`"]
pub type MISF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF8`"]
pub type MISF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF9`"]
pub type MISF9_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF10`"]
pub type MISF10_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF11`"]
pub type MISF11_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF12`"]
pub type MISF12_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF13`"]
pub type MISF13_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF14`"]
pub type MISF14_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF15`"]
pub type MISF15_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF16`"]
pub type MISF16_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF17`"]
pub type MISF17_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF18`"]
pub type MISF18_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF19`"]
pub type MISF19_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF20`"]
pub type MISF20_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF21`"]
pub type MISF21_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF22`"]
pub type MISF22_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF23`"]
pub type MISF23_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF24`"]
pub type MISF24_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF25`"]
pub type MISF25_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF26`"]
pub type MISF26_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF27`"]
pub type MISF27_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF28`"]
pub type MISF28_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF29`"]
pub type MISF29_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF30`"]
pub type MISF30_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISF31`"]
pub type MISF31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf0(&self) -> MISF0_R {
        MISF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf1(&self) -> MISF1_R {
        MISF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf2(&self) -> MISF2_R {
        MISF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf3(&self) -> MISF3_R {
        MISF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf4(&self) -> MISF4_R {
        MISF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf5(&self) -> MISF5_R {
        MISF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf6(&self) -> MISF6_R {
        MISF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf7(&self) -> MISF7_R {
        MISF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf8(&self) -> MISF8_R {
        MISF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf9(&self) -> MISF9_R {
        MISF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf10(&self) -> MISF10_R {
        MISF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf11(&self) -> MISF11_R {
        MISF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf12(&self) -> MISF12_R {
        MISF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf13(&self) -> MISF13_R {
        MISF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf14(&self) -> MISF14_R {
        MISF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf15(&self) -> MISF15_R {
        MISF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf16(&self) -> MISF16_R {
        MISF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf17(&self) -> MISF17_R {
        MISF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf18(&self) -> MISF18_R {
        MISF18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf19(&self) -> MISF19_R {
        MISF19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf20(&self) -> MISF20_R {
        MISF20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf21(&self) -> MISF21_R {
        MISF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf22(&self) -> MISF22_R {
        MISF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf23(&self) -> MISF23_R {
        MISF23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf24(&self) -> MISF24_R {
        MISF24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf25(&self) -> MISF25_R {
        MISF25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf26(&self) -> MISF26_R {
        MISF26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf27(&self) -> MISF27_R {
        MISF27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf28(&self) -> MISF28_R {
        MISF28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf29(&self) -> MISF29_R {
        MISF29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf30(&self) -> MISF30_R {
        MISF30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf31(&self) -> MISF31_R {
        MISF31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
