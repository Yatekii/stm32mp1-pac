#[doc = "Reader of register FDCAN_RXF1S"]
pub type R = crate::R<u32, super::FDCAN_RXF1S>;
#[doc = "Reader of field `F1FL`"]
pub type F1FL_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1GI`"]
pub type F1GI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1PI`"]
pub type F1PI_R = crate::R<u8, u8>;
#[doc = "F1F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F1F_A {
    #[doc = "0: Rx FIFO 1 not full"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 1 full"]
    B_0X1 = 1,
}
impl From<F1F_A> for bool {
    #[inline(always)]
    fn from(variant: F1F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `F1F`"]
pub type F1F_R = crate::R<bool, F1F_A>;
impl F1F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F1F_A {
        match self.bits {
            false => F1F_A::B_0X0,
            true => F1F_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == F1F_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == F1F_A::B_0X1
    }
}
#[doc = "RF1L\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RF1L_A {
    #[doc = "0: No Rx FIFO 1 message\r\n                  lost"]
    B_0X0 = 0,
    #[doc = "1: Rx FIFO 1 message lost, also set\r\n                  after write attempt to Rx FIFO 1 of size\r\n                  zero."]
    B_0X1 = 1,
}
impl From<RF1L_A> for bool {
    #[inline(always)]
    fn from(variant: RF1L_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RF1L`"]
pub type RF1L_R = crate::R<bool, RF1L_A>;
impl RF1L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF1L_A {
        match self.bits {
            false => RF1L_A::B_0X0,
            true => RF1L_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1L_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1L_A::B_0X1
    }
}
#[doc = "Reader of field `DMS`"]
pub type DMS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - F1FL"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - F1GI"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - F1PI"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - F1F"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RF1L"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - DMS"]
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
