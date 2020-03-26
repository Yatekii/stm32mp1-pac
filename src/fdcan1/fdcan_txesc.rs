#[doc = "Reader of register FDCAN_TXESC"]
pub type R = crate::R<u32, super::FDCAN_TXESC>;
#[doc = "TBDS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TBDS_A {
    #[doc = "0: 8 byte data field"]
    B_0X0 = 0,
    #[doc = "1: 12 byte data field"]
    B_0X1 = 1,
    #[doc = "2: 16 byte data field"]
    B_0X2 = 2,
    #[doc = "3: 20 byte data field"]
    B_0X3 = 3,
    #[doc = "4: 24 byte data field"]
    B_0X4 = 4,
    #[doc = "5: 32 byte data field"]
    B_0X5 = 5,
    #[doc = "6: 48 byte data field"]
    B_0X6 = 6,
    #[doc = "7: 64 byte data field"]
    B_0X7 = 7,
}
impl From<TBDS_A> for u8 {
    #[inline(always)]
    fn from(variant: TBDS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TBDS`"]
pub type TBDS_R = crate::R<u8, TBDS_A>;
impl TBDS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBDS_A {
        match self.bits {
            0 => TBDS_A::B_0X0,
            1 => TBDS_A::B_0X1,
            2 => TBDS_A::B_0X2,
            3 => TBDS_A::B_0X3,
            4 => TBDS_A::B_0X4,
            5 => TBDS_A::B_0X5,
            6 => TBDS_A::B_0X6,
            7 => TBDS_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TBDS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TBDS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TBDS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TBDS_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == TBDS_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == TBDS_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == TBDS_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == TBDS_A::B_0X7
    }
}
impl R {
    #[doc = "Bits 0:2 - TBDS"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 0x07) as u8)
    }
}
