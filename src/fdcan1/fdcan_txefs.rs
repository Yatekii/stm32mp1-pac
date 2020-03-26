#[doc = "Reader of register FDCAN_TXEFS"]
pub type R = crate::R<u32, super::FDCAN_TXEFS>;
#[doc = "Reader of field `EFFL`"]
pub type EFFL_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFGI`"]
pub type EFGI_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFPI`"]
pub type EFPI_R = crate::R<u8, u8>;
#[doc = "EFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EFF_A {
    #[doc = "0: Tx Event FIFO not full"]
    B_0X0 = 0,
    #[doc = "1: Tx Event FIFO full"]
    B_0X1 = 1,
}
impl From<EFF_A> for bool {
    #[inline(always)]
    fn from(variant: EFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EFF`"]
pub type EFF_R = crate::R<bool, EFF_A>;
impl EFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFF_A {
        match self.bits {
            false => EFF_A::B_0X0,
            true => EFF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EFF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EFF_A::B_0X1
    }
}
#[doc = "Reader of field `TEFL`"]
pub type TEFL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:5 - EFFL"]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - EFGI"]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - EFPI"]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - EFF"]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - TEFL"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
