#[doc = "Reader of register FDCAN_TXBRP"]
pub type R = crate::R<u32, super::FDCAN_TXBRP>;
#[doc = "TRP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TRP_A {
    #[doc = "0: No transmission request\r\n                  pending"]
    B_0X0 = 0,
    #[doc = "1: Transmission request\r\n                  pending"]
    B_0X1 = 1,
}
impl From<TRP_A> for u32 {
    #[inline(always)]
    fn from(variant: TRP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRP`"]
pub type TRP_R = crate::R<u32, TRP_A>;
impl TRP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, TRP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRP_A::B_0X0),
            1 => Val(TRP_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TRP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TRP_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:31 - TRP"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
