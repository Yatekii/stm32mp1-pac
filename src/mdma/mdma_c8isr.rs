#[doc = "Reader of register MDMA_C8ISR"]
pub type R = crate::R<u32, super::MDMA_C8ISR>;
#[doc = "TEIF8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEIF8_A {
    #[doc = "0: No transfer error on stream\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: A transfer error occurred on stream\r\n                  x"]
    B_0X1 = 1,
}
impl From<TEIF8_A> for bool {
    #[inline(always)]
    fn from(variant: TEIF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEIF8`"]
pub type TEIF8_R = crate::R<bool, TEIF8_A>;
impl TEIF8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIF8_A {
        match self.bits {
            false => TEIF8_A::B_0X0,
            true => TEIF8_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIF8_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIF8_A::B_0X1
    }
}
#[doc = "CTCIF8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCIF8_A {
    #[doc = "0: No channel transfer complete event\r\n                  on channel x"]
    B_0X0 = 0,
    #[doc = "1: A channel transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<CTCIF8_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTCIF8`"]
pub type CTCIF8_R = crate::R<bool, CTCIF8_A>;
impl CTCIF8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCIF8_A {
        match self.bits {
            false => CTCIF8_A::B_0X0,
            true => CTCIF8_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTCIF8_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTCIF8_A::B_0X1
    }
}
#[doc = "BRTIF8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRTIF8_A {
    #[doc = "0: No block repeat transfer complete\r\n                  event on channel x"]
    B_0X0 = 0,
    #[doc = "1: A block repeat transfer complete\r\n                  event occurred on channel x"]
    B_0X1 = 1,
}
impl From<BRTIF8_A> for bool {
    #[inline(always)]
    fn from(variant: BRTIF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BRTIF8`"]
pub type BRTIF8_R = crate::R<bool, BRTIF8_A>;
impl BRTIF8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRTIF8_A {
        match self.bits {
            false => BRTIF8_A::B_0X0,
            true => BRTIF8_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BRTIF8_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BRTIF8_A::B_0X1
    }
}
#[doc = "BTIF8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BTIF8_A {
    #[doc = "0: No block transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A block transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<BTIF8_A> for bool {
    #[inline(always)]
    fn from(variant: BTIF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BTIF8`"]
pub type BTIF8_R = crate::R<bool, BTIF8_A>;
impl BTIF8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTIF8_A {
        match self.bits {
            false => BTIF8_A::B_0X0,
            true => BTIF8_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BTIF8_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BTIF8_A::B_0X1
    }
}
#[doc = "TCIF8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIF8_A {
    #[doc = "0: No buffer transfer complete event on\r\n                  channel x"]
    B_0X0 = 0,
    #[doc = "1: A buffer transfer complete event\r\n                  occurred on channel x"]
    B_0X1 = 1,
}
impl From<TCIF8_A> for bool {
    #[inline(always)]
    fn from(variant: TCIF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TCIF8`"]
pub type TCIF8_R = crate::R<bool, TCIF8_A>;
impl TCIF8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIF8_A {
        match self.bits {
            false => TCIF8_A::B_0X0,
            true => TCIF8_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIF8_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIF8_A::B_0X1
    }
}
#[doc = "CRQA8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRQA8_A {
    #[doc = "0: The MDMA transfer RQ is inactive for\r\n                  channel x."]
    B_0X0 = 0,
    #[doc = "1: The MDMA transfer RQ is active for\r\n                  channel x"]
    B_0X1 = 1,
}
impl From<CRQA8_A> for bool {
    #[inline(always)]
    fn from(variant: CRQA8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRQA8`"]
pub type CRQA8_R = crate::R<bool, CRQA8_A>;
impl CRQA8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRQA8_A {
        match self.bits {
            false => CRQA8_A::B_0X0,
            true => CRQA8_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRQA8_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRQA8_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - TEIF8"]
    #[inline(always)]
    pub fn teif8(&self) -> TEIF8_R {
        TEIF8_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTCIF8"]
    #[inline(always)]
    pub fn ctcif8(&self) -> CTCIF8_R {
        CTCIF8_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRTIF8"]
    #[inline(always)]
    pub fn brtif8(&self) -> BRTIF8_R {
        BRTIF8_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BTIF8"]
    #[inline(always)]
    pub fn btif8(&self) -> BTIF8_R {
        BTIF8_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCIF8"]
    #[inline(always)]
    pub fn tcif8(&self) -> TCIF8_R {
        TCIF8_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CRQA8"]
    #[inline(always)]
    pub fn crqa8(&self) -> CRQA8_R {
        CRQA8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
