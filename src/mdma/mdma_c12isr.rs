#[doc = "Reader of register MDMA_C12ISR"]
pub type R = crate::R<u32, super::MDMA_C12ISR>;
#[doc = "TEIF12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF12_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF12_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF12`"]
pub type TEIF12_R = crate::R<bool, TEIF12_A>;
impl TEIF12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF12_A {
        match self.bits {
            false => TEIF12_A::B_0X0,
            true => TEIF12_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF12_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF12_A::B_0X1
    }
}
#[doc = "CTCIF12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF12_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF12_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF12`"]
pub type CTCIF12_R = crate::R<bool, CTCIF12_A>;
impl CTCIF12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF12_A {
        match self.bits {
            false => CTCIF12_A::B_0X0,
            true => CTCIF12_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF12_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF12_A::B_0X1
    }
}
#[doc = "BRTIF12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF12_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF12_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF12`"]
pub type BRTIF12_R = crate::R<bool, BRTIF12_A>;
impl BRTIF12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF12_A {
        match self.bits {
            false => BRTIF12_A::B_0X0,
            true => BRTIF12_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF12_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF12_A::B_0X1
    }
}
#[doc = "BTIF12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF12_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF12_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF12`"]
pub type BTIF12_R = crate::R<bool, BTIF12_A>;
impl BTIF12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF12_A {
        match self.bits {
            false => BTIF12_A::B_0X0,
            true => BTIF12_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF12_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF12_A::B_0X1
    }
}
#[doc = "TCIF12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF12_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF12_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF12`"]
pub type TCIF12_R = crate::R<bool, TCIF12_A>;
impl TCIF12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF12_A {
        match self.bits {
            false => TCIF12_A::B_0X0,
            true => TCIF12_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF12_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF12_A::B_0X1
    }
}
#[doc = "CRQA12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA12_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA12_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA12`"]
pub type CRQA12_R = crate::R<bool, CRQA12_A>;
impl CRQA12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA12_A {
        match self.bits {
            false => CRQA12_A::B_0X0,
            true => CRQA12_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA12_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA12_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF12"]
    #[inline(always)]
    pub fn teif12(&self) -> TEIF12_R {
        TEIF12_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF12"]
    #[inline(always)]
    pub fn ctcif12(&self) -> CTCIF12_R {
        CTCIF12_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF12"]
    #[inline(always)]
    pub fn brtif12(&self) -> BRTIF12_R {
        BRTIF12_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF12"]
    #[inline(always)]
    pub fn btif12(&self) -> BTIF12_R {
        BTIF12_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF12"]
    #[inline(always)]
    pub fn tcif12(&self) -> TCIF12_R {
        TCIF12_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA12"]
    #[inline(always)]
    pub fn crqa12(&self) -> CRQA12_R {
        CRQA12_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
