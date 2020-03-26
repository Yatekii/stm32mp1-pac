#[doc = "Reader of register MDMA_C6ISR"]
pub type R = crate::R<u32, super::MDMA_C6ISR>;
#[doc = "TEIF6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF6_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF6_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF6`"]
pub type TEIF6_R = crate::R<bool, TEIF6_A>;
impl TEIF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF6_A {
        match self.bits {
            false => TEIF6_A::B_0X0,
            true => TEIF6_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF6_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF6_A::B_0X1
    }
}
#[doc = "CTCIF6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF6_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF6_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF6`"]
pub type CTCIF6_R = crate::R<bool, CTCIF6_A>;
impl CTCIF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF6_A {
        match self.bits {
            false => CTCIF6_A::B_0X0,
            true => CTCIF6_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF6_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF6_A::B_0X1
    }
}
#[doc = "BRTIF6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF6_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF6_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF6`"]
pub type BRTIF6_R = crate::R<bool, BRTIF6_A>;
impl BRTIF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF6_A {
        match self.bits {
            false => BRTIF6_A::B_0X0,
            true => BRTIF6_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF6_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF6_A::B_0X1
    }
}
#[doc = "BTIF6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF6_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF6_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF6`"]
pub type BTIF6_R = crate::R<bool, BTIF6_A>;
impl BTIF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF6_A {
        match self.bits {
            false => BTIF6_A::B_0X0,
            true => BTIF6_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF6_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF6_A::B_0X1
    }
}
#[doc = "TCIF6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF6_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF6_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF6`"]
pub type TCIF6_R = crate::R<bool, TCIF6_A>;
impl TCIF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF6_A {
        match self.bits {
            false => TCIF6_A::B_0X0,
            true => TCIF6_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF6_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF6_A::B_0X1
    }
}
#[doc = "CRQA6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA6_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA6_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA6`"]
pub type CRQA6_R = crate::R<bool, CRQA6_A>;
impl CRQA6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA6_A {
        match self.bits {
            false => CRQA6_A::B_0X0,
            true => CRQA6_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA6_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA6_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF6"]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF6"]
    #[inline(always)]
    pub fn ctcif6(&self) -> CTCIF6_R {
        CTCIF6_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF6"]
    #[inline(always)]
    pub fn brtif6(&self) -> BRTIF6_R {
        BRTIF6_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF6"]
    #[inline(always)]
    pub fn btif6(&self) -> BTIF6_R {
        BTIF6_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF6"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA6"]
    #[inline(always)]
    pub fn crqa6(&self) -> CRQA6_R {
        CRQA6_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
