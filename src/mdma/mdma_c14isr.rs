#[doc = "Reader of register MDMA_C14ISR"]
pub type R = crate::R<u32, super::MDMA_C14ISR>;
#[doc = "TEIF14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF14_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF14_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF14`"]
pub type TEIF14_R = crate::R<bool, TEIF14_A>;
impl TEIF14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF14_A {
        match self.bits {
            false => TEIF14_A::B_0X0,
            true => TEIF14_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF14_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF14_A::B_0X1
    }
}
#[doc = "CTCIF14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF14_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF14_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF14`"]
pub type CTCIF14_R = crate::R<bool, CTCIF14_A>;
impl CTCIF14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF14_A {
        match self.bits {
            false => CTCIF14_A::B_0X0,
            true => CTCIF14_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF14_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF14_A::B_0X1
    }
}
#[doc = "BRTIF14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF14_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF14_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF14`"]
pub type BRTIF14_R = crate::R<bool, BRTIF14_A>;
impl BRTIF14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF14_A {
        match self.bits {
            false => BRTIF14_A::B_0X0,
            true => BRTIF14_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF14_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF14_A::B_0X1
    }
}
#[doc = "BTIF14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF14_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF14_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF14`"]
pub type BTIF14_R = crate::R<bool, BTIF14_A>;
impl BTIF14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF14_A {
        match self.bits {
            false => BTIF14_A::B_0X0,
            true => BTIF14_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF14_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF14_A::B_0X1
    }
}
#[doc = "TCIF14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF14_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF14_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF14`"]
pub type TCIF14_R = crate::R<bool, TCIF14_A>;
impl TCIF14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF14_A {
        match self.bits {
            false => TCIF14_A::B_0X0,
            true => TCIF14_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF14_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF14_A::B_0X1
    }
}
#[doc = "CRQA14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA14_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA14_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA14`"]
pub type CRQA14_R = crate::R<bool, CRQA14_A>;
impl CRQA14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA14_A {
        match self.bits {
            false => CRQA14_A::B_0X0,
            true => CRQA14_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA14_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA14_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF14"]
    #[inline(always)]
    pub fn teif14(&self) -> TEIF14_R {
        TEIF14_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF14"]
    #[inline(always)]
    pub fn ctcif14(&self) -> CTCIF14_R {
        CTCIF14_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF14"]
    #[inline(always)]
    pub fn brtif14(&self) -> BRTIF14_R {
        BRTIF14_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF14"]
    #[inline(always)]
    pub fn btif14(&self) -> BTIF14_R {
        BTIF14_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF14"]
    #[inline(always)]
    pub fn tcif14(&self) -> TCIF14_R {
        TCIF14_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA14"]
    #[inline(always)]
    pub fn crqa14(&self) -> CRQA14_R {
        CRQA14_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
