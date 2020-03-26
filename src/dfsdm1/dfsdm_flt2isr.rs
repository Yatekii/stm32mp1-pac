#[doc = "Reader of register DFSDM_FLT2ISR"]
pub type R = crate::R<u32, super::DFSDM_FLT2ISR>;
#[doc = "JEOCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOCF_A {
    #[doc = "0: No injected conversion has\r\n                  completed"]
    B_0X0 = 0,
    #[doc = "1: An injected conversion has completed\r\n                  and its data may be read"]
    B_0X1 = 1,
}
impl From<JEOCF_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JEOCF`"]
pub type JEOCF_R = crate::R<bool, JEOCF_A>;
impl JEOCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOCF_A {
        match self.bits {
            false => JEOCF_A::B_0X0,
            true => JEOCF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == JEOCF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == JEOCF_A::B_0X1
    }
}
#[doc = "REOCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REOCF_A {
    #[doc = "0: No regular conversion has\r\n                  completed"]
    B_0X0 = 0,
    #[doc = "1: A regular conversion has completed\r\n                  and its data may be read"]
    B_0X1 = 1,
}
impl From<REOCF_A> for bool {
    #[inline(always)]
    fn from(variant: REOCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REOCF`"]
pub type REOCF_R = crate::R<bool, REOCF_A>;
impl REOCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REOCF_A {
        match self.bits {
            false => REOCF_A::B_0X0,
            true => REOCF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REOCF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REOCF_A::B_0X1
    }
}
#[doc = "JOVRF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JOVRF_A {
    #[doc = "0: No injected conversion overrun has\r\n                  occurred"]
    B_0X0 = 0,
    #[doc = "1: An injected conversion overrun has\r\n                  occurred, which means that an injected conversion\r\n                  finished while JEOCF was already 1 . JDATAR is\r\n                  not affected by overruns"]
    B_0X1 = 1,
}
impl From<JOVRF_A> for bool {
    #[inline(always)]
    fn from(variant: JOVRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JOVRF`"]
pub type JOVRF_R = crate::R<bool, JOVRF_A>;
impl JOVRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JOVRF_A {
        match self.bits {
            false => JOVRF_A::B_0X0,
            true => JOVRF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == JOVRF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == JOVRF_A::B_0X1
    }
}
#[doc = "ROVRF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROVRF_A {
    #[doc = "0: No regular conversion overrun has\r\n                  occurred"]
    B_0X0 = 0,
    #[doc = "1: A regular conversion overrun has\r\n                  occurred, which means that a regular conversion\r\n                  finished while REOCF was already 1 . RDATAR is\r\n                  not affected by overruns"]
    B_0X1 = 1,
}
impl From<ROVRF_A> for bool {
    #[inline(always)]
    fn from(variant: ROVRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ROVRF`"]
pub type ROVRF_R = crate::R<bool, ROVRF_A>;
impl ROVRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVRF_A {
        match self.bits {
            false => ROVRF_A::B_0X0,
            true => ROVRF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ROVRF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ROVRF_A::B_0X1
    }
}
#[doc = "AWDF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWDF_A {
    #[doc = "0: No Analog watchdog event\r\n                  occurred"]
    B_0X0 = 0,
    #[doc = "1: The analog watchdog block detected\r\n                  voltage which crosses the value programmed in the\r\n                  DFSDM_FLTxAWLTR or DFSDM_FLTxAWHTR\r\n                  registers."]
    B_0X1 = 1,
}
impl From<AWDF_A> for bool {
    #[inline(always)]
    fn from(variant: AWDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWDF`"]
pub type AWDF_R = crate::R<bool, AWDF_A>;
impl AWDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWDF_A {
        match self.bits {
            false => AWDF_A::B_0X0,
            true => AWDF_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == AWDF_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == AWDF_A::B_0X1
    }
}
#[doc = "JCIP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JCIP_A {
    #[doc = "0: No request to convert the injected\r\n                  channel group (neither by software nor by\r\n                  trigger) has been issued"]
    B_0X0 = 0,
    #[doc = "1: The conversion of the injected\r\n                  channel group is in progress or a request for a\r\n                  injected conversion is pending, due either to 1\r\n                  being written to JSWSTART or to a trigger\r\n                  detection"]
    B_0X1 = 1,
}
impl From<JCIP_A> for bool {
    #[inline(always)]
    fn from(variant: JCIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JCIP`"]
pub type JCIP_R = crate::R<bool, JCIP_A>;
impl JCIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JCIP_A {
        match self.bits {
            false => JCIP_A::B_0X0,
            true => JCIP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == JCIP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == JCIP_A::B_0X1
    }
}
#[doc = "RCIP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCIP_A {
    #[doc = "0: No request to convert the regular\r\n                  channel has been issued"]
    B_0X0 = 0,
    #[doc = "1: The conversion of the regular\r\n                  channel is in progress or a request for a regular\r\n                  conversion is pending"]
    B_0X1 = 1,
}
impl From<RCIP_A> for bool {
    #[inline(always)]
    fn from(variant: RCIP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RCIP`"]
pub type RCIP_R = crate::R<bool, RCIP_A>;
impl RCIP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCIP_A {
        match self.bits {
            false => RCIP_A::B_0X0,
            true => RCIP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RCIP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RCIP_A::B_0X1
    }
}
#[doc = "Reader of field `CKABF`"]
pub type CKABF_R = crate::R<u8, u8>;
#[doc = "Reader of field `SCDF`"]
pub type SCDF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - JEOCF"]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - REOCF"]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - JOVRF"]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ROVRF"]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AWDF"]
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 13 - JCIP"]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RCIP"]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - CKABF"]
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCDF"]
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
