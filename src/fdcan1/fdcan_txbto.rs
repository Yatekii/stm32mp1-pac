#[doc = "Reader of register FDCAN_TXBTO"]
pub type R = crate::R<u32, super::FDCAN_TXBTO>;
#[doc = "TO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TO_A {
    #[doc = "0: No transmission\r\n                  occurred"]
    B_0X0 = 0,
    #[doc = "1: Transmission occurred"]
    B_0X1 = 1,
}
impl From<TO_A> for u32 {
    #[inline(always)]
    fn from(variant: TO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TO`"]
pub type TO_R = crate::R<u32, TO_A>;
impl TO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, TO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TO_A::B_0X0),
            1 => Val(TO_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TO_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TO_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:31 - TO"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
