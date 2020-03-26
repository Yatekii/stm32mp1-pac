#[doc = "Reader of register FDCAN_RXESC"]
pub type R = crate::R<u32, super::FDCAN_RXESC>;
#[doc = "F0DS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum F0DS_A {
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
impl From<F0DS_A> for u8 {
    #[inline(always)]
    fn from(variant: F0DS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `F0DS`"]
pub type F0DS_R = crate::R<u8, F0DS_A>;
impl F0DS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F0DS_A {
        match self.bits {
            0 => F0DS_A::B_0X0,
            1 => F0DS_A::B_0X1,
            2 => F0DS_A::B_0X2,
            3 => F0DS_A::B_0X3,
            4 => F0DS_A::B_0X4,
            5 => F0DS_A::B_0X5,
            6 => F0DS_A::B_0X6,
            7 => F0DS_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == F0DS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == F0DS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == F0DS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == F0DS_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == F0DS_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == F0DS_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == F0DS_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == F0DS_A::B_0X7
    }
}
#[doc = "F1DS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum F1DS_A {
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
impl From<F1DS_A> for u8 {
    #[inline(always)]
    fn from(variant: F1DS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `F1DS`"]
pub type F1DS_R = crate::R<u8, F1DS_A>;
impl F1DS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F1DS_A {
        match self.bits {
            0 => F1DS_A::B_0X0,
            1 => F1DS_A::B_0X1,
            2 => F1DS_A::B_0X2,
            3 => F1DS_A::B_0X3,
            4 => F1DS_A::B_0X4,
            5 => F1DS_A::B_0X5,
            6 => F1DS_A::B_0X6,
            7 => F1DS_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == F1DS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == F1DS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == F1DS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == F1DS_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == F1DS_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == F1DS_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == F1DS_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == F1DS_A::B_0X7
    }
}
#[doc = "Reader of field `RBDS`"]
pub type RBDS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - F0DS"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - F1DS"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - RBDS"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
