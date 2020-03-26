#[doc = "Reader of register MDMA_C0ISR"]
pub type R = crate::R<u32, super::MDMA_C0ISR>;
#[doc = "TEIF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF0_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF0`"]
pub type TEIF0_R = crate::R<bool, TEIF0_A>;
impl TEIF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF0_A {
        match self.bits {
            false => TEIF0_A::B_0X0,
            true => TEIF0_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF0_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF0_A::B_0X1
    }
}
#[doc = "CTCIF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF0_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF0_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF0`"]
pub type CTCIF0_R = crate::R<bool, CTCIF0_A>;
impl CTCIF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF0_A {
        match self.bits {
            false => CTCIF0_A::B_0X0,
            true => CTCIF0_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF0_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF0_A::B_0X1
    }
}
#[doc = "BRTIF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF0_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF0_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF0`"]
pub type BRTIF0_R = crate::R<bool, BRTIF0_A>;
impl BRTIF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF0_A {
        match self.bits {
            false => BRTIF0_A::B_0X0,
            true => BRTIF0_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF0_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF0_A::B_0X1
    }
}
#[doc = "BTIF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF0_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF0_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF0`"]
pub type BTIF0_R = crate::R<bool, BTIF0_A>;
impl BTIF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF0_A {
        match self.bits {
            false => BTIF0_A::B_0X0,
            true => BTIF0_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF0_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF0_A::B_0X1
    }
}
#[doc = "TCIF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF0_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF0`"]
pub type TCIF0_R = crate::R<bool, TCIF0_A>;
impl TCIF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF0_A {
        match self.bits {
            false => TCIF0_A::B_0X0,
            true => TCIF0_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF0_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF0_A::B_0X1
    }
}
#[doc = "CRQA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA0_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA0_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA0`"]
pub type CRQA0_R = crate::R<bool, CRQA0_A>;
impl CRQA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA0_A {
        match self.bits {
            false => CRQA0_A::B_0X0,
            true => CRQA0_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA0_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA0_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF0"]
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF0"]
    #[inline(always)]
    pub fn ctcif0(&self) -> CTCIF0_R {
        CTCIF0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF0"]
    #[inline(always)]
    pub fn brtif0(&self) -> BRTIF0_R {
        BRTIF0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF0"]
    #[inline(always)]
    pub fn btif0(&self) -> BTIF0_R {
        BTIF0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF0"]
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA0"]
    #[inline(always)]
    pub fn crqa0(&self) -> CRQA0_R {
        CRQA0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
