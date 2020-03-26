#[doc = "Reader of register MDMA_C5ISR"]
pub type R = crate::R<u32, super::MDMA_C5ISR>;
#[doc = "TEIF5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF5_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF5_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF5`"]
pub type TEIF5_R = crate::R<bool, TEIF5_A>;
impl TEIF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF5_A {
        match self.bits {
            false => TEIF5_A::B_0X0,
            true => TEIF5_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF5_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF5_A::B_0X1
    }
}
#[doc = "CTCIF5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF5_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF5_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF5`"]
pub type CTCIF5_R = crate::R<bool, CTCIF5_A>;
impl CTCIF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF5_A {
        match self.bits {
            false => CTCIF5_A::B_0X0,
            true => CTCIF5_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF5_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF5_A::B_0X1
    }
}
#[doc = "BRTIF5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF5_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF5_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF5`"]
pub type BRTIF5_R = crate::R<bool, BRTIF5_A>;
impl BRTIF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF5_A {
        match self.bits {
            false => BRTIF5_A::B_0X0,
            true => BRTIF5_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF5_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF5_A::B_0X1
    }
}
#[doc = "BTIF5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF5_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF5_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF5`"]
pub type BTIF5_R = crate::R<bool, BTIF5_A>;
impl BTIF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF5_A {
        match self.bits {
            false => BTIF5_A::B_0X0,
            true => BTIF5_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF5_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF5_A::B_0X1
    }
}
#[doc = "TCIF5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF5_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF5_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF5`"]
pub type TCIF5_R = crate::R<bool, TCIF5_A>;
impl TCIF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF5_A {
        match self.bits {
            false => TCIF5_A::B_0X0,
            true => TCIF5_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF5_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF5_A::B_0X1
    }
}
#[doc = "CRQA5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA5_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA5_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA5`"]
pub type CRQA5_R = crate::R<bool, CRQA5_A>;
impl CRQA5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA5_A {
        match self.bits {
            false => CRQA5_A::B_0X0,
            true => CRQA5_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA5_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA5_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF5"]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF5"]
    #[inline(always)]
    pub fn ctcif5(&self) -> CTCIF5_R {
        CTCIF5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF5"]
    #[inline(always)]
    pub fn brtif5(&self) -> BRTIF5_R {
        BRTIF5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF5"]
    #[inline(always)]
    pub fn btif5(&self) -> BTIF5_R {
        BTIF5_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF5"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA5"]
    #[inline(always)]
    pub fn crqa5(&self) -> CRQA5_R {
        CRQA5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
