#[doc = "Reader of register FDCAN_TSCC"]
pub type R = crate::R<u32, super::FDCAN_TSCC>;
#[doc = "Writer for register FDCAN_TSCC"]
pub type W = crate::W<u32, super::FDCAN_TSCC>;
#[doc = "Register FDCAN_TSCC `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TSCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TSS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSS_A {
    #[doc = "0: Timestamp counter value always\r\n                  0x0000"]
    B_0X0 = 0,
    #[doc = "1: Timestamp counter value incremented\r\n                  according to TCP"]
    B_0X1 = 1,
    #[doc = "2: External timestamp counter from TIM3\r\n                  value used (tim3_cnt\\[0:15\\])"]
    B_0X2 = 2,
    #[doc = "3: Same as 00 ."]
    B_0X3 = 3,
}
impl From<TSS_A> for u8 {
    #[inline(always)]
    fn from(variant: TSS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSS`"]
pub type TSS_R = crate::R<u8, TSS_A>;
impl TSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSS_A {
        match self.bits {
            0 => TSS_A::B_0X0,
            1 => TSS_A::B_0X1,
            2 => TSS_A::B_0X2,
            3 => TSS_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSS_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TSS_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TSS_A::B_0X3
    }
}
#[doc = "Write proxy for field `TSS`"]
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TSS_A::B_0X0)
    }
    #[doc = "Timestamp counter value incremented according to TCP"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TSS_A::B_0X1)
    }
    #[doc = "External timestamp counter from TIM3 value used (tim3_cnt\\[0:15\\])"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(TSS_A::B_0X2)
    }
    #[doc = "Same as 00 ."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(TSS_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TCP`"]
pub type TCP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCP`"]
pub struct TCP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - TCP"]
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TSS"]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
    }
    #[doc = "Bits 16:19 - TCP"]
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W {
        TCP_W { w: self }
    }
}
