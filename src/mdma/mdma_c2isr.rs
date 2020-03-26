#[doc = "Reader of register MDMA_C2ISR"]
pub type R = crate::R<u32, super::MDMA_C2ISR>;
#[doc = "TEIF2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF2_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF2_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF2`"]
pub type TEIF2_R = crate::R<bool, TEIF2_A>;
impl TEIF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF2_A {
        match self.bits {
            false => TEIF2_A::B_0X0,
            true => TEIF2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF2_A::B_0X1
    }
}
#[doc = "CTCIF2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF2_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF2_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF2`"]
pub type CTCIF2_R = crate::R<bool, CTCIF2_A>;
impl CTCIF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF2_A {
        match self.bits {
            false => CTCIF2_A::B_0X0,
            true => CTCIF2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF2_A::B_0X1
    }
}
#[doc = "BRTIF2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF2_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF2_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF2`"]
pub type BRTIF2_R = crate::R<bool, BRTIF2_A>;
impl BRTIF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF2_A {
        match self.bits {
            false => BRTIF2_A::B_0X0,
            true => BRTIF2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF2_A::B_0X1
    }
}
#[doc = "BTIF2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF2_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF2_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF2`"]
pub type BTIF2_R = crate::R<bool, BTIF2_A>;
impl BTIF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF2_A {
        match self.bits {
            false => BTIF2_A::B_0X0,
            true => BTIF2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF2_A::B_0X1
    }
}
#[doc = "TCIF2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF2_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF2_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF2`"]
pub type TCIF2_R = crate::R<bool, TCIF2_A>;
impl TCIF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF2_A {
        match self.bits {
            false => TCIF2_A::B_0X0,
            true => TCIF2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF2_A::B_0X1
    }
}
#[doc = "CRQA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA2_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA2_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA2`"]
pub type CRQA2_R = crate::R<bool, CRQA2_A>;
impl CRQA2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA2_A {
        match self.bits {
            false => CRQA2_A::B_0X0,
            true => CRQA2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA2_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF2"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF2"]
    #[inline(always)]
    pub fn ctcif2(&self) -> CTCIF2_R {
        CTCIF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF2"]
    #[inline(always)]
    pub fn brtif2(&self) -> BRTIF2_R {
        BRTIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF2"]
    #[inline(always)]
    pub fn btif2(&self) -> BTIF2_R {
        BTIF2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF2"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA2"]
    #[inline(always)]
    pub fn crqa2(&self) -> CRQA2_R {
        CRQA2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
