#[doc = "Reader of register MDMA_C3ISR"]
pub type R = crate::R<u32, super::MDMA_C3ISR>;
#[doc = "TEIF3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF3_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF3_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF3`"]
pub type TEIF3_R = crate::R<bool, TEIF3_A>;
impl TEIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF3_A {
        match self.bits {
            false => TEIF3_A::B_0X0,
            true => TEIF3_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF3_A::B_0X1
    }
}
#[doc = "CTCIF3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF3_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF3_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF3`"]
pub type CTCIF3_R = crate::R<bool, CTCIF3_A>;
impl CTCIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF3_A {
        match self.bits {
            false => CTCIF3_A::B_0X0,
            true => CTCIF3_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF3_A::B_0X1
    }
}
#[doc = "BRTIF3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF3_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF3_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF3`"]
pub type BRTIF3_R = crate::R<bool, BRTIF3_A>;
impl BRTIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF3_A {
        match self.bits {
            false => BRTIF3_A::B_0X0,
            true => BRTIF3_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF3_A::B_0X1
    }
}
#[doc = "BTIF3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF3_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF3_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF3`"]
pub type BTIF3_R = crate::R<bool, BTIF3_A>;
impl BTIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF3_A {
        match self.bits {
            false => BTIF3_A::B_0X0,
            true => BTIF3_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF3_A::B_0X1
    }
}
#[doc = "TCIF3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF3_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF3_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF3`"]
pub type TCIF3_R = crate::R<bool, TCIF3_A>;
impl TCIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF3_A {
        match self.bits {
            false => TCIF3_A::B_0X0,
            true => TCIF3_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF3_A::B_0X1
    }
}
#[doc = "CRQA3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA3_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA3_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA3`"]
pub type CRQA3_R = crate::R<bool, CRQA3_A>;
impl CRQA3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA3_A {
        match self.bits {
            false => CRQA3_A::B_0X0,
            true => CRQA3_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA3_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF3"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF3"]
    #[inline(always)]
    pub fn ctcif3(&self) -> CTCIF3_R {
        CTCIF3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF3"]
    #[inline(always)]
    pub fn brtif3(&self) -> BRTIF3_R {
        BRTIF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF3"]
    #[inline(always)]
    pub fn btif3(&self) -> BTIF3_R {
        BTIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF3"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA3"]
    #[inline(always)]
    pub fn crqa3(&self) -> CRQA3_R {
        CRQA3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
