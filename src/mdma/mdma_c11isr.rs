#[doc = "Reader of register MDMA_C11ISR"]
pub type R = crate::R<u32, super::MDMA_C11ISR>;
#[doc = "TEIF11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF11_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF11_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF11`"]
pub type TEIF11_R = crate::R<bool, TEIF11_A>;
impl TEIF11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF11_A {
        match self.bits {
            false => TEIF11_A::B_0X0,
            true => TEIF11_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF11_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF11_A::B_0X1
    }
}
#[doc = "CTCIF11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF11_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF11_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF11`"]
pub type CTCIF11_R = crate::R<bool, CTCIF11_A>;
impl CTCIF11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF11_A {
        match self.bits {
            false => CTCIF11_A::B_0X0,
            true => CTCIF11_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF11_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF11_A::B_0X1
    }
}
#[doc = "BRTIF11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF11_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF11_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF11`"]
pub type BRTIF11_R = crate::R<bool, BRTIF11_A>;
impl BRTIF11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF11_A {
        match self.bits {
            false => BRTIF11_A::B_0X0,
            true => BRTIF11_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF11_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF11_A::B_0X1
    }
}
#[doc = "BTIF11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF11_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF11_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF11`"]
pub type BTIF11_R = crate::R<bool, BTIF11_A>;
impl BTIF11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF11_A {
        match self.bits {
            false => BTIF11_A::B_0X0,
            true => BTIF11_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF11_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF11_A::B_0X1
    }
}
#[doc = "TCIF11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF11_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF11_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF11`"]
pub type TCIF11_R = crate::R<bool, TCIF11_A>;
impl TCIF11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF11_A {
        match self.bits {
            false => TCIF11_A::B_0X0,
            true => TCIF11_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF11_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF11_A::B_0X1
    }
}
#[doc = "CRQA11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA11_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA11_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA11`"]
pub type CRQA11_R = crate::R<bool, CRQA11_A>;
impl CRQA11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA11_A {
        match self.bits {
            false => CRQA11_A::B_0X0,
            true => CRQA11_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA11_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA11_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF11"]
    #[inline(always)]
    pub fn teif11(&self) -> TEIF11_R {
        TEIF11_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF11"]
    #[inline(always)]
    pub fn ctcif11(&self) -> CTCIF11_R {
        CTCIF11_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF11"]
    #[inline(always)]
    pub fn brtif11(&self) -> BRTIF11_R {
        BRTIF11_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF11"]
    #[inline(always)]
    pub fn btif11(&self) -> BTIF11_R {
        BTIF11_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF11"]
    #[inline(always)]
    pub fn tcif11(&self) -> TCIF11_R {
        TCIF11_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA11"]
    #[inline(always)]
    pub fn crqa11(&self) -> CRQA11_R {
        CRQA11_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
