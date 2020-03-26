#[doc = "Reader of register MDMA_C4ISR"]
pub type R = crate::R<u32, super::MDMA_C4ISR>;
#[doc = "TEIF4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF4_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF4_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF4`"]
pub type TEIF4_R = crate::R<bool, TEIF4_A>;
impl TEIF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF4_A {
        match self.bits {
            false => TEIF4_A::B_0X0,
            true => TEIF4_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF4_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF4_A::B_0X1
    }
}
#[doc = "CTCIF4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF4_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF4_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF4`"]
pub type CTCIF4_R = crate::R<bool, CTCIF4_A>;
impl CTCIF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF4_A {
        match self.bits {
            false => CTCIF4_A::B_0X0,
            true => CTCIF4_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF4_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF4_A::B_0X1
    }
}
#[doc = "BRTIF4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF4_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF4_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF4`"]
pub type BRTIF4_R = crate::R<bool, BRTIF4_A>;
impl BRTIF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF4_A {
        match self.bits {
            false => BRTIF4_A::B_0X0,
            true => BRTIF4_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF4_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF4_A::B_0X1
    }
}
#[doc = "BTIF4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF4_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF4_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF4`"]
pub type BTIF4_R = crate::R<bool, BTIF4_A>;
impl BTIF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF4_A {
        match self.bits {
            false => BTIF4_A::B_0X0,
            true => BTIF4_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF4_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF4_A::B_0X1
    }
}
#[doc = "TCIF4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF4_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF4_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF4`"]
pub type TCIF4_R = crate::R<bool, TCIF4_A>;
impl TCIF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF4_A {
        match self.bits {
            false => TCIF4_A::B_0X0,
            true => TCIF4_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF4_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF4_A::B_0X1
    }
}
#[doc = "CRQA4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA4_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA4_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA4`"]
pub type CRQA4_R = crate::R<bool, CRQA4_A>;
impl CRQA4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA4_A {
        match self.bits {
            false => CRQA4_A::B_0X0,
            true => CRQA4_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA4_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA4_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF4"]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF4"]
    #[inline(always)]
    pub fn ctcif4(&self) -> CTCIF4_R {
        CTCIF4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF4"]
    #[inline(always)]
    pub fn brtif4(&self) -> BRTIF4_R {
        BRTIF4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF4"]
    #[inline(always)]
    pub fn btif4(&self) -> BTIF4_R {
        BTIF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF4"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA4"]
    #[inline(always)]
    pub fn crqa4(&self) -> CRQA4_R {
        CRQA4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
