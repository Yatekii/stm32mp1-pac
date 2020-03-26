#[doc = "Reader of register MDMA_C9ISR"]
pub type R = crate::R<u32, super::MDMA_C9ISR>;
#[doc = "TEIF9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF9_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF9_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF9`"]
pub type TEIF9_R = crate::R<bool, TEIF9_A>;
impl TEIF9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF9_A {
        match self.bits {
            false => TEIF9_A::B_0X0,
            true => TEIF9_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF9_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF9_A::B_0X1
    }
}
#[doc = "CTCIF9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF9_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF9_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF9`"]
pub type CTCIF9_R = crate::R<bool, CTCIF9_A>;
impl CTCIF9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF9_A {
        match self.bits {
            false => CTCIF9_A::B_0X0,
            true => CTCIF9_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF9_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF9_A::B_0X1
    }
}
#[doc = "BRTIF9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF9_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF9_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF9`"]
pub type BRTIF9_R = crate::R<bool, BRTIF9_A>;
impl BRTIF9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF9_A {
        match self.bits {
            false => BRTIF9_A::B_0X0,
            true => BRTIF9_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF9_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF9_A::B_0X1
    }
}
#[doc = "BTIF9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF9_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF9_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF9`"]
pub type BTIF9_R = crate::R<bool, BTIF9_A>;
impl BTIF9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF9_A {
        match self.bits {
            false => BTIF9_A::B_0X0,
            true => BTIF9_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF9_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF9_A::B_0X1
    }
}
#[doc = "TCIF9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF9_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF9_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF9`"]
pub type TCIF9_R = crate::R<bool, TCIF9_A>;
impl TCIF9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF9_A {
        match self.bits {
            false => TCIF9_A::B_0X0,
            true => TCIF9_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF9_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF9_A::B_0X1
    }
}
#[doc = "CRQA9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA9_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA9_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA9`"]
pub type CRQA9_R = crate::R<bool, CRQA9_A>;
impl CRQA9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA9_A {
        match self.bits {
            false => CRQA9_A::B_0X0,
            true => CRQA9_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA9_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA9_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF9"]
    #[inline(always)]
    pub fn teif9(&self) -> TEIF9_R {
        TEIF9_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF9"]
    #[inline(always)]
    pub fn ctcif9(&self) -> CTCIF9_R {
        CTCIF9_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF9"]
    #[inline(always)]
    pub fn brtif9(&self) -> BRTIF9_R {
        BRTIF9_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF9"]
    #[inline(always)]
    pub fn btif9(&self) -> BTIF9_R {
        BTIF9_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF9"]
    #[inline(always)]
    pub fn tcif9(&self) -> TCIF9_R {
        TCIF9_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA9"]
    #[inline(always)]
    pub fn crqa9(&self) -> CRQA9_R {
        CRQA9_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
