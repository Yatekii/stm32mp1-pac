#[doc = "Reader of register MDMA_C10ISR"]
pub type R = crate::R<u32, super::MDMA_C10ISR>;
#[doc = "TEIF10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF10_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF10_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF10`"]
pub type TEIF10_R = crate::R<bool, TEIF10_A>;
impl TEIF10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF10_A {
        match self.bits {
            false => TEIF10_A::B_0X0,
            true => TEIF10_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF10_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF10_A::B_0X1
    }
}
#[doc = "CTCIF10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF10_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF10_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF10`"]
pub type CTCIF10_R = crate::R<bool, CTCIF10_A>;
impl CTCIF10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF10_A {
        match self.bits {
            false => CTCIF10_A::B_0X0,
            true => CTCIF10_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF10_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF10_A::B_0X1
    }
}
#[doc = "BRTIF10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF10_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF10_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF10`"]
pub type BRTIF10_R = crate::R<bool, BRTIF10_A>;
impl BRTIF10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF10_A {
        match self.bits {
            false => BRTIF10_A::B_0X0,
            true => BRTIF10_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF10_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF10_A::B_0X1
    }
}
#[doc = "BTIF10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF10_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF10_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF10`"]
pub type BTIF10_R = crate::R<bool, BTIF10_A>;
impl BTIF10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF10_A {
        match self.bits {
            false => BTIF10_A::B_0X0,
            true => BTIF10_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF10_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF10_A::B_0X1
    }
}
#[doc = "TCIF10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF10_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF10_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF10`"]
pub type TCIF10_R = crate::R<bool, TCIF10_A>;
impl TCIF10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF10_A {
        match self.bits {
            false => TCIF10_A::B_0X0,
            true => TCIF10_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF10_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF10_A::B_0X1
    }
}
#[doc = "CRQA10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA10_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA10_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA10`"]
pub type CRQA10_R = crate::R<bool, CRQA10_A>;
impl CRQA10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA10_A {
        match self.bits {
            false => CRQA10_A::B_0X0,
            true => CRQA10_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA10_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA10_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF10"]
    #[inline(always)]
    pub fn teif10(&self) -> TEIF10_R {
        TEIF10_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF10"]
    #[inline(always)]
    pub fn ctcif10(&self) -> CTCIF10_R {
        CTCIF10_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF10"]
    #[inline(always)]
    pub fn brtif10(&self) -> BRTIF10_R {
        BRTIF10_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF10"]
    #[inline(always)]
    pub fn btif10(&self) -> BTIF10_R {
        BTIF10_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF10"]
    #[inline(always)]
    pub fn tcif10(&self) -> TCIF10_R {
        TCIF10_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA10"]
    #[inline(always)]
    pub fn crqa10(&self) -> CRQA10_R {
        CRQA10_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
