#[doc = "Reader of register FDCAN_ECR"]
pub type R = crate::R<u32, super::FDCAN_ECR>;
#[doc = "Reader of field `TEC`"]
pub type TEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `TREC`"]
pub type TREC_R = crate::R<u8, u8>;
#[doc = "RP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RP_A {
    #[doc = "0: The Receive Error Counter is below\r\n                  the error passive level of 128"]
    B_0X0 = 0,
    #[doc = "1: The Receive Error Counter has\r\n                  reached the error passive level of\r\n                  128"]
    B_0X1 = 1,
}
impl From<RP_A> for bool {
    #[inline(always)]
    fn from(variant: RP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RP`"]
pub type RP_R = crate::R<bool, RP_A>;
impl RP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RP_A {
        match self.bits {
            false => RP_A::B_0X0,
            true => RP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RP_A::B_0X1
    }
}
#[doc = "Reader of field `CEL`"]
pub type CEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - TREC"]
    #[inline(always)]
    pub fn trec(&self) -> TREC_R {
        TREC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - RP"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - CEL"]
    #[inline(always)]
    pub fn cel(&self) -> CEL_R {
        CEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
