#[doc = "Reader of register RCC_OCRDYR"]
pub type R = crate::R<u32, super::RCC_OCRDYR>;
#[doc = "HSIRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIRDY_A {
    #[doc = "0: HSI clock is not ready (default\r\n                  after reset)"]
    B_0X0 = 0,
    #[doc = "1: HSI clock is ready"]
    B_0X1 = 1,
}
impl From<HSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSIRDY`"]
pub type HSIRDY_R = crate::R<bool, HSIRDY_A>;
impl HSIRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIRDY_A {
        match self.bits {
            false => HSIRDY_A::B_0X0,
            true => HSIRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIRDY_A::B_0X1
    }
}
#[doc = "HSIDIVRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSIDIVRDY_A {
    #[doc = "0: the new division ratio is not yet\r\n                  propagated to hsi_ck (hsi_ker_ck) (default after\r\n                  reset)"]
    B_0X0 = 0,
    #[doc = "1: the hsi_ck (hsi_ker_ck) clock\r\n                  frequency reflects the new HSIDIV\r\n                  value"]
    B_0X1 = 1,
}
impl From<HSIDIVRDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIDIVRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSIDIVRDY`"]
pub type HSIDIVRDY_R = crate::R<bool, HSIDIVRDY_A>;
impl HSIDIVRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSIDIVRDY_A {
        match self.bits {
            false => HSIDIVRDY_A::B_0X0,
            true => HSIDIVRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSIDIVRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSIDIVRDY_A::B_0X1
    }
}
#[doc = "CSIRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIRDY_A {
    #[doc = "0: CSI clock is not ready (default\r\n                  after reset)"]
    B_0X0 = 0,
    #[doc = "1: CSI clock is ready"]
    B_0X1 = 1,
}
impl From<CSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CSIRDY`"]
pub type CSIRDY_R = crate::R<bool, CSIRDY_A>;
impl CSIRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSIRDY_A {
        match self.bits {
            false => CSIRDY_A::B_0X0,
            true => CSIRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CSIRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CSIRDY_A::B_0X1
    }
}
#[doc = "HSERDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSERDY_A {
    #[doc = "0: HSE clock is not ready (default\r\n                  after reset)"]
    B_0X0 = 0,
    #[doc = "1: HSE clock is ready"]
    B_0X1 = 1,
}
impl From<HSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HSERDY`"]
pub type HSERDY_R = crate::R<bool, HSERDY_A>;
impl HSERDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSERDY_A {
        match self.bits {
            false => HSERDY_A::B_0X0,
            true => HSERDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HSERDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HSERDY_A::B_0X1
    }
}
#[doc = "MPUCKRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPUCKRDY_A {
    #[doc = "0: mpuss_ck clock is not available\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: mpuss_ck clock is\r\n                  available"]
    B_0X1 = 1,
}
impl From<MPUCKRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MPUCKRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPUCKRDY`"]
pub type MPUCKRDY_R = crate::R<bool, MPUCKRDY_A>;
impl MPUCKRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPUCKRDY_A {
        match self.bits {
            false => MPUCKRDY_A::B_0X0,
            true => MPUCKRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MPUCKRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MPUCKRDY_A::B_0X1
    }
}
#[doc = "AXICKRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AXICKRDY_A {
    #[doc = "0: axiss_ck clock is not available\r\n                  (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: axiss_ck clock is\r\n                  available"]
    B_0X1 = 1,
}
impl From<AXICKRDY_A> for bool {
    #[inline(always)]
    fn from(variant: AXICKRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AXICKRDY`"]
pub type AXICKRDY_R = crate::R<bool, AXICKRDY_A>;
impl AXICKRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXICKRDY_A {
        match self.bits {
            false => AXICKRDY_A::B_0X0,
            true => AXICKRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AXICKRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AXICKRDY_A::B_0X1
    }
}
#[doc = "CKREST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKREST_A {
    #[doc = "0: The clock restore process is not\r\n                  on-going (default after reset)"]
    B_0X0 = 0,
    #[doc = "1: The clock restore process is\r\n                  on-going"]
    B_0X1 = 1,
}
impl From<CKREST_A> for bool {
    #[inline(always)]
    fn from(variant: CKREST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CKREST`"]
pub type CKREST_R = crate::R<bool, CKREST_A>;
impl CKREST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKREST_A {
        match self.bits {
            false => CKREST_A::B_0X0,
            true => CKREST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CKREST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CKREST_A::B_0X1
    }
}
impl R {
    #[doc = "Bit 0 - HSIRDY"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSIDIVRDY"]
    #[inline(always)]
    pub fn hsidivrdy(&self) -> HSIDIVRDY_R {
        HSIDIVRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSIRDY"]
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HSERDY"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 23 - MPUCKRDY"]
    #[inline(always)]
    pub fn mpuckrdy(&self) -> MPUCKRDY_R {
        MPUCKRDY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - AXICKRDY"]
    #[inline(always)]
    pub fn axickrdy(&self) -> AXICKRDY_R {
        AXICKRDY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CKREST"]
    #[inline(always)]
    pub fn ckrest(&self) -> CKREST_R {
        CKREST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
