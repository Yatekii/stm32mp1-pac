#[doc = "Reader of register FDCAN_HPMS"]
pub type R = crate::R<u32, super::FDCAN_HPMS>;
#[doc = "Reader of field `BIDX`"]
pub type BIDX_R = crate::R<u8, u8>;
#[doc = "MSI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MSI_A {
    #[doc = "0: No FIFO selected"]
    B_0X0 = 0,
    #[doc = "1: FIFO overrun"]
    B_0X1 = 1,
    #[doc = "2: Message stored in FIFO\r\n                  0"]
    B_0X2 = 2,
    #[doc = "3: Message stored in FIFO\r\n                  1"]
    B_0X3 = 3,
}
impl From<MSI_A> for u8 {
    #[inline(always)]
    fn from(variant: MSI_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MSI`"]
pub type MSI_R = crate::R<u8, MSI_A>;
impl MSI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSI_A {
        match self.bits {
            0 => MSI_A::B_0X0,
            1 => MSI_A::B_0X1,
            2 => MSI_A::B_0X2,
            3 => MSI_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSI_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSI_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MSI_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == MSI_A::B_0X3
    }
}
#[doc = "Reader of field `FIDX`"]
pub type FIDX_R = crate::R<u8, u8>;
#[doc = "FLST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLST_A {
    #[doc = "0: Standard Filter List"]
    B_0X0 = 0,
    #[doc = "1: Extended Filter List"]
    B_0X1 = 1,
}
impl From<FLST_A> for bool {
    #[inline(always)]
    fn from(variant: FLST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLST`"]
pub type FLST_R = crate::R<bool, FLST_A>;
impl FLST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLST_A {
        match self.bits {
            false => FLST_A::B_0X0,
            true => FLST_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FLST_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FLST_A::B_0X1
    }
}
impl R {
    #[doc = "Bits 0:5 - BIDX"]
    #[inline(always)]
    pub fn bidx(&self) -> BIDX_R {
        BIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - MSI"]
    #[inline(always)]
    pub fn msi(&self) -> MSI_R {
        MSI_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:14 - FIDX"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - FLST"]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
