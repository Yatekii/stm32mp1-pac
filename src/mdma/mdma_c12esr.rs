#[doc = "Reader of register MDMA_C12ESR"]
pub type R = crate::R<u32, super::MDMA_C12ESR>;
#[doc = "Reader of field `TEA`"]
pub type TEA_R = crate::R<u8, u8>;
#[doc = "TED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TED_A {
    #[doc = "0: The last transfer error on the\r\n                  channel was a related to a read\r\n                  access."]
    B_0X0 = 0,
    #[doc = "1: The last transfer error on the\r\n                  channel was a related to a write\r\n                  access."]
    B_0X1 = 1,
}
impl From<TED_A> for bool {
    #[inline(always)]
    fn from(variant: TED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TED`"]
pub type TED_R = crate::R<bool, TED_A>;
impl TED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TED_A {
        match self.bits {
            false => TED_A::B_0X0,
            true => TED_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TED_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TED_A::B_0X1
    }
}
#[doc = "TELD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TELD_A {
    #[doc = "0: No link data read access\r\n                  error."]
    B_0X0 = 0,
    #[doc = "1: The last transfer error on the\r\n                  channel was a related to a read of the Link Data\r\n                  structure."]
    B_0X1 = 1,
}
impl From<TELD_A> for bool {
    #[inline(always)]
    fn from(variant: TELD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TELD`"]
pub type TELD_R = crate::R<bool, TELD_A>;
impl TELD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TELD_A {
        match self.bits {
            false => TELD_A::B_0X0,
            true => TELD_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TELD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TELD_A::B_0X1
    }
}
#[doc = "TEMD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMD_A {
    #[doc = "0: No mask write access\r\n                  error."]
    B_0X0 = 0,
    #[doc = "1: The last transfer error on the\r\n                  channel was a related to a write of the Mask\r\n                  Data."]
    B_0X1 = 1,
}
impl From<TEMD_A> for bool {
    #[inline(always)]
    fn from(variant: TEMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TEMD`"]
pub type TEMD_R = crate::R<bool, TEMD_A>;
impl TEMD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEMD_A {
        match self.bits {
            false => TEMD_A::B_0X0,
            true => TEMD_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEMD_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEMD_A::B_0X1
    }
}
#[doc = "ASE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASE_A {
    #[doc = "0: No address/size error."]
    B_0X0 = 0,
    #[doc = "1: Programmed address is not coherent\r\n                  with the data size."]
    B_0X1 = 1,
}
impl From<ASE_A> for bool {
    #[inline(always)]
    fn from(variant: ASE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASE`"]
pub type ASE_R = crate::R<bool, ASE_A>;
impl ASE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASE_A {
        match self.bits {
            false => ASE_A::B_0X0,
            true => ASE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ASE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ASE_A::B_0X1
    }
}
#[doc = "BSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSE_A {
    #[doc = "0: No block size error."]
    B_0X0 = 0,
    #[doc = "1: Programmed block size is not an\r\n                  integer multiple of the data size."]
    B_0X1 = 1,
}
impl From<BSE_A> for bool {
    #[inline(always)]
    fn from(variant: BSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BSE`"]
pub type BSE_R = crate::R<bool, BSE_A>;
impl BSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSE_A {
        match self.bits {
            false => BSE_A::B_0X0,
            true => BSE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BSE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BSE_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:6 - TEA"]
    #[inline(always)]
    pub fn tea(&self) -> TEA_R {
        TEA_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - TED"]
    #[inline(always)]
    pub fn ted(&self) -> TED_R {
        TED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TELD"]
    #[inline(always)]
    pub fn teld(&self) -> TELD_R {
        TELD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TEMD"]
    #[inline(always)]
    pub fn temd(&self) -> TEMD_R {
        TEMD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ASE"]
    #[inline(always)]
    pub fn ase(&self) -> ASE_R {
        ASE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BSE"]
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
