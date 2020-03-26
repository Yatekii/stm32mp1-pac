#[doc = "Reader of register MDMA_C13ISR"]
pub type R = crate::R<u32, super::MDMA_C13ISR>;
#[doc = "TEIF13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF13_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF13_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF13`"]
pub type TEIF13_R = crate::R<bool, TEIF13_A>;
impl TEIF13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF13_A {
        match self.bits {
            false => TEIF13_A::B_0X0,
            true => TEIF13_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF13_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF13_A::B_0X1
    }
}
#[doc = "CTCIF13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF13_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF13_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF13`"]
pub type CTCIF13_R = crate::R<bool, CTCIF13_A>;
impl CTCIF13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF13_A {
        match self.bits {
            false => CTCIF13_A::B_0X0,
            true => CTCIF13_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF13_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF13_A::B_0X1
    }
}
#[doc = "BRTIF13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF13_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF13_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF13`"]
pub type BRTIF13_R = crate::R<bool, BRTIF13_A>;
impl BRTIF13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF13_A {
        match self.bits {
            false => BRTIF13_A::B_0X0,
            true => BRTIF13_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF13_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF13_A::B_0X1
    }
}
#[doc = "BTIF13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF13_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF13_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF13`"]
pub type BTIF13_R = crate::R<bool, BTIF13_A>;
impl BTIF13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF13_A {
        match self.bits {
            false => BTIF13_A::B_0X0,
            true => BTIF13_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF13_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF13_A::B_0X1
    }
}
#[doc = "TCIF13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF13_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF13_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF13`"]
pub type TCIF13_R = crate::R<bool, TCIF13_A>;
impl TCIF13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF13_A {
        match self.bits {
            false => TCIF13_A::B_0X0,
            true => TCIF13_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF13_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF13_A::B_0X1
    }
}
#[doc = "CRQA13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA13_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA13_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA13`"]
pub type CRQA13_R = crate::R<bool, CRQA13_A>;
impl CRQA13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA13_A {
        match self.bits {
            false => CRQA13_A::B_0X0,
            true => CRQA13_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA13_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA13_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF13"]
    #[inline(always)]
    pub fn teif13(&self) -> TEIF13_R {
        TEIF13_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF13"]
    #[inline(always)]
    pub fn ctcif13(&self) -> CTCIF13_R {
        CTCIF13_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF13"]
    #[inline(always)]
    pub fn brtif13(&self) -> BRTIF13_R {
        BRTIF13_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF13"]
    #[inline(always)]
    pub fn btif13(&self) -> BTIF13_R {
        BTIF13_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF13"]
    #[inline(always)]
    pub fn tcif13(&self) -> TCIF13_R {
        TCIF13_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA13"]
    #[inline(always)]
    pub fn crqa13(&self) -> CRQA13_R {
        CRQA13_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
