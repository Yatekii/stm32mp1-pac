#[doc = "Reader of register MDMA_C24ISR"]
pub type R = crate::R<u32, super::MDMA_C24ISR>;
#[doc = "TEIF15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF15_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF15_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF15`"]
pub type TEIF15_R = crate::R<bool, TEIF15_A>;
impl TEIF15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF15_A {
        match self.bits {
            false => TEIF15_A::B_0X0,
            true => TEIF15_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF15_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF15_A::B_0X1
    }
}
#[doc = "CTCIF15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF15_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF15_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF15`"]
pub type CTCIF15_R = crate::R<bool, CTCIF15_A>;
impl CTCIF15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF15_A {
        match self.bits {
            false => CTCIF15_A::B_0X0,
            true => CTCIF15_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF15_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF15_A::B_0X1
    }
}
#[doc = "BRTIF15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF15_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF15_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF15`"]
pub type BRTIF15_R = crate::R<bool, BRTIF15_A>;
impl BRTIF15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF15_A {
        match self.bits {
            false => BRTIF15_A::B_0X0,
            true => BRTIF15_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF15_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF15_A::B_0X1
    }
}
#[doc = "BTIF15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF15_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF15_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF15`"]
pub type BTIF15_R = crate::R<bool, BTIF15_A>;
impl BTIF15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF15_A {
        match self.bits {
            false => BTIF15_A::B_0X0,
            true => BTIF15_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF15_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF15_A::B_0X1
    }
}
#[doc = "TCIF15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF15_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF15_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF15`"]
pub type TCIF15_R = crate::R<bool, TCIF15_A>;
impl TCIF15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF15_A {
        match self.bits {
            false => TCIF15_A::B_0X0,
            true => TCIF15_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF15_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF15_A::B_0X1
    }
}
#[doc = "CRQA15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA15_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA15_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA15`"]
pub type CRQA15_R = crate::R<bool, CRQA15_A>;
impl CRQA15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA15_A {
        match self.bits {
            false => CRQA15_A::B_0X0,
            true => CRQA15_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA15_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA15_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF15"]
    #[inline(always)]
    pub fn teif15(&self) -> TEIF15_R {
        TEIF15_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF15"]
    #[inline(always)]
    pub fn ctcif15(&self) -> CTCIF15_R {
        CTCIF15_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF15"]
    #[inline(always)]
    pub fn brtif15(&self) -> BRTIF15_R {
        BRTIF15_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF15"]
    #[inline(always)]
    pub fn btif15(&self) -> BTIF15_R {
        BTIF15_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF15"]
    #[inline(always)]
    pub fn tcif15(&self) -> TCIF15_R {
        TCIF15_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA15"]
    #[inline(always)]
    pub fn crqa15(&self) -> CRQA15_R {
        CRQA15_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
