#[doc = "Reader of register MDMA_C1ISR"]
pub type R = crate::R<u32, super::MDMA_C1ISR>;
#[doc = "TEIF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF1_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF1`"]
pub type TEIF1_R = crate::R<bool, TEIF1_A>;
impl TEIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF1_A {
        match self.bits {
            false => TEIF1_A::B_0X0,
            true => TEIF1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF1_A::B_0X1
    }
}
#[doc = "CTCIF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF1_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF1_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF1`"]
pub type CTCIF1_R = crate::R<bool, CTCIF1_A>;
impl CTCIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF1_A {
        match self.bits {
            false => CTCIF1_A::B_0X0,
            true => CTCIF1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF1_A::B_0X1
    }
}
#[doc = "BRTIF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF1_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF1_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF1`"]
pub type BRTIF1_R = crate::R<bool, BRTIF1_A>;
impl BRTIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF1_A {
        match self.bits {
            false => BRTIF1_A::B_0X0,
            true => BRTIF1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF1_A::B_0X1
    }
}
#[doc = "BTIF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF1_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF1_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF1`"]
pub type BTIF1_R = crate::R<bool, BTIF1_A>;
impl BTIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF1_A {
        match self.bits {
            false => BTIF1_A::B_0X0,
            true => BTIF1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF1_A::B_0X1
    }
}
#[doc = "TCIF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF1_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF1_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF1`"]
pub type TCIF1_R = crate::R<bool, TCIF1_A>;
impl TCIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF1_A {
        match self.bits {
            false => TCIF1_A::B_0X0,
            true => TCIF1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF1_A::B_0X1
    }
}
#[doc = "CRQA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA1_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA1_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA1`"]
pub type CRQA1_R = crate::R<bool, CRQA1_A>;
impl CRQA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA1_A {
        match self.bits {
            false => CRQA1_A::B_0X0,
            true => CRQA1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA1_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF1"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF1"]
    #[inline(always)]
    pub fn ctcif1(&self) -> CTCIF1_R {
        CTCIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF1"]
    #[inline(always)]
    pub fn brtif1(&self) -> BRTIF1_R {
        BRTIF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF1"]
    #[inline(always)]
    pub fn btif1(&self) -> BTIF1_R {
        BTIF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF1"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA1"]
    #[inline(always)]
    pub fn crqa1(&self) -> CRQA1_R {
        CRQA1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
