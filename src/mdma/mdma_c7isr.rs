#[doc = "Reader of register MDMA_C7ISR"]
pub type R = crate::R<u32, super::MDMA_C7ISR>;
#[doc = "TEIF7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF7_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF7_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF7`"]
pub type TEIF7_R = crate::R<bool, TEIF7_A>;
impl TEIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF7_A {
        match self.bits {
            false => TEIF7_A::B_0X0,
            true => TEIF7_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF7_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF7_A::B_0X1
    }
}
#[doc = "CTCIF7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF7_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF7_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF7`"]
pub type CTCIF7_R = crate::R<bool, CTCIF7_A>;
impl CTCIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF7_A {
        match self.bits {
            false => CTCIF7_A::B_0X0,
            true => CTCIF7_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF7_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF7_A::B_0X1
    }
}
#[doc = "BRTIF7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF7_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF7_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF7`"]
pub type BRTIF7_R = crate::R<bool, BRTIF7_A>;
impl BRTIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF7_A {
        match self.bits {
            false => BRTIF7_A::B_0X0,
            true => BRTIF7_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF7_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF7_A::B_0X1
    }
}
#[doc = "BTIF7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF7_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF7_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF7`"]
pub type BTIF7_R = crate::R<bool, BTIF7_A>;
impl BTIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF7_A {
        match self.bits {
            false => BTIF7_A::B_0X0,
            true => BTIF7_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF7_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF7_A::B_0X1
    }
}
#[doc = "TCIF7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF7_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF7_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF7`"]
pub type TCIF7_R = crate::R<bool, TCIF7_A>;
impl TCIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF7_A {
        match self.bits {
            false => TCIF7_A::B_0X0,
            true => TCIF7_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF7_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF7_A::B_0X1
    }
}
#[doc = "CRQA7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA7_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA7_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA7`"]
pub type CRQA7_R = crate::R<bool, CRQA7_A>;
impl CRQA7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA7_A {
        match self.bits {
            false => CRQA7_A::B_0X0,
            true => CRQA7_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA7_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA7_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF7"]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF7"]
    #[inline(always)]
    pub fn ctcif7(&self) -> CTCIF7_R {
        CTCIF7_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF7"]
    #[inline(always)]
    pub fn brtif7(&self) -> BRTIF7_R {
        BRTIF7_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF7"]
    #[inline(always)]
    pub fn btif7(&self) -> BTIF7_R {
        BTIF7_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF7"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA7"]
    #[inline(always)]
    pub fn crqa7(&self) -> CRQA7_R {
        CRQA7_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
