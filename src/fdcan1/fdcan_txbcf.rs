#[doc = "Reader of register FDCAN_TXBCF"]
pub type R = crate::R<u32, super::FDCAN_TXBCF>;
#[doc = "CF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CF_A {
    #[doc = "0: No transmit buffer\r\n                  cancellation"]
    B_0X0 = 0,
    #[doc = "1: Transmit buffer cancellation\r\n                  finished"]
    B_0X1 = 1,
}
impl From<CF_A> for u32 {
    #[inline(always)]
    fn from(variant: CF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CF`"]
pub type CF_R = crate::R<u32, CF_A>;
impl CF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CF_A::B_0X0),
            1 => Val(CF_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CF_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:31 - CF"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
