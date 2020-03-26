#[doc = "Reader of register MDMA_SGISR0"]
pub type R = crate::R<u32, super::MDMA_SGISR0>;
#[doc = "GIF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF0_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF0_A> for bool {
    #[inline(always)]
    fn from(variant: GIF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF0`"]
pub type GIF0_R = crate::R<bool, GIF0_A>;
impl GIF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF0_A {
        match self.bits {
            false => GIF0_A::B_0X0,
            true => GIF0_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF0_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF0_A::B_0X1
    }
}
#[doc = "GIF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF1_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF1_A> for bool {
    #[inline(always)]
    fn from(variant: GIF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF1`"]
pub type GIF1_R = crate::R<bool, GIF1_A>;
impl GIF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF1_A {
        match self.bits {
            false => GIF1_A::B_0X0,
            true => GIF1_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF1_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF1_A::B_0X1
    }
}
#[doc = "GIF2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF2_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF2_A> for bool {
    #[inline(always)]
    fn from(variant: GIF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF2`"]
pub type GIF2_R = crate::R<bool, GIF2_A>;
impl GIF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF2_A {
        match self.bits {
            false => GIF2_A::B_0X0,
            true => GIF2_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF2_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF2_A::B_0X1
    }
}
#[doc = "GIF3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF3_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF3_A> for bool {
    #[inline(always)]
    fn from(variant: GIF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF3`"]
pub type GIF3_R = crate::R<bool, GIF3_A>;
impl GIF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF3_A {
        match self.bits {
            false => GIF3_A::B_0X0,
            true => GIF3_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF3_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF3_A::B_0X1
    }
}
#[doc = "GIF4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF4_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF4_A> for bool {
    #[inline(always)]
    fn from(variant: GIF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF4`"]
pub type GIF4_R = crate::R<bool, GIF4_A>;
impl GIF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF4_A {
        match self.bits {
            false => GIF4_A::B_0X0,
            true => GIF4_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF4_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF4_A::B_0X1
    }
}
#[doc = "GIF5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF5_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF5_A> for bool {
    #[inline(always)]
    fn from(variant: GIF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF5`"]
pub type GIF5_R = crate::R<bool, GIF5_A>;
impl GIF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF5_A {
        match self.bits {
            false => GIF5_A::B_0X0,
            true => GIF5_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF5_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF5_A::B_0X1
    }
}
#[doc = "GIF6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF6_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF6_A> for bool {
    #[inline(always)]
    fn from(variant: GIF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF6`"]
pub type GIF6_R = crate::R<bool, GIF6_A>;
impl GIF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF6_A {
        match self.bits {
            false => GIF6_A::B_0X0,
            true => GIF6_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF6_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF6_A::B_0X1
    }
}
#[doc = "GIF7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF7_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF7_A> for bool {
    #[inline(always)]
    fn from(variant: GIF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF7`"]
pub type GIF7_R = crate::R<bool, GIF7_A>;
impl GIF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF7_A {
        match self.bits {
            false => GIF7_A::B_0X0,
            true => GIF7_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF7_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF7_A::B_0X1
    }
}
#[doc = "GIF8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF8_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF8_A> for bool {
    #[inline(always)]
    fn from(variant: GIF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF8`"]
pub type GIF8_R = crate::R<bool, GIF8_A>;
impl GIF8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF8_A {
        match self.bits {
            false => GIF8_A::B_0X0,
            true => GIF8_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF8_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF8_A::B_0X1
    }
}
#[doc = "GIF9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF9_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF9_A> for bool {
    #[inline(always)]
    fn from(variant: GIF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF9`"]
pub type GIF9_R = crate::R<bool, GIF9_A>;
impl GIF9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF9_A {
        match self.bits {
            false => GIF9_A::B_0X0,
            true => GIF9_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF9_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF9_A::B_0X1
    }
}
#[doc = "GIF10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF10_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF10_A> for bool {
    #[inline(always)]
    fn from(variant: GIF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF10`"]
pub type GIF10_R = crate::R<bool, GIF10_A>;
impl GIF10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF10_A {
        match self.bits {
            false => GIF10_A::B_0X0,
            true => GIF10_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF10_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF10_A::B_0X1
    }
}
#[doc = "GIF11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF11_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF11_A> for bool {
    #[inline(always)]
    fn from(variant: GIF11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF11`"]
pub type GIF11_R = crate::R<bool, GIF11_A>;
impl GIF11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF11_A {
        match self.bits {
            false => GIF11_A::B_0X0,
            true => GIF11_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF11_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF11_A::B_0X1
    }
}
#[doc = "GIF12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF12_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF12_A> for bool {
    #[inline(always)]
    fn from(variant: GIF12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF12`"]
pub type GIF12_R = crate::R<bool, GIF12_A>;
impl GIF12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF12_A {
        match self.bits {
            false => GIF12_A::B_0X0,
            true => GIF12_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF12_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF12_A::B_0X1
    }
}
#[doc = "GIF13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF13_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF13_A> for bool {
    #[inline(always)]
    fn from(variant: GIF13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF13`"]
pub type GIF13_R = crate::R<bool, GIF13_A>;
impl GIF13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF13_A {
        match self.bits {
            false => GIF13_A::B_0X0,
            true => GIF13_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF13_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF13_A::B_0X1
    }
}
#[doc = "GIF14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF14_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF14_A> for bool {
    #[inline(always)]
    fn from(variant: GIF14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF14`"]
pub type GIF14_R = crate::R<bool, GIF14_A>;
impl GIF14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF14_A {
        match self.bits {
            false => GIF14_A::B_0X0,
            true => GIF14_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF14_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF14_A::B_0X1
    }
}
#[doc = "GIF15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIF15_A {
    #[doc = "0: No interrupt generated by channel\r\n                  x"]
    B_0X0 = 0,
    #[doc = "1: Interrupt generated by channel\r\n                  x"]
    B_0X1 = 1,
}
impl From<GIF15_A> for bool {
    #[inline(always)]
    fn from(variant: GIF15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GIF15`"]
pub type GIF15_R = crate::R<bool, GIF15_A>;
impl GIF15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GIF15_A {
        match self.bits {
            false => GIF15_A::B_0X0,
            true => GIF15_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GIF15_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GIF15_A::B_0X1
    }
}
#[doc = "Reader of field `GIF16`"]
pub type GIF16_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF17`"]
pub type GIF17_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF18`"]
pub type GIF18_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF19`"]
pub type GIF19_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF20`"]
pub type GIF20_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF21`"]
pub type GIF21_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF22`"]
pub type GIF22_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF23`"]
pub type GIF23_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF24`"]
pub type GIF24_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF25`"]
pub type GIF25_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF26`"]
pub type GIF26_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF27`"]
pub type GIF27_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF28`"]
pub type GIF28_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF29`"]
pub type GIF29_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF30`"]
pub type GIF30_R = crate::R<bool, bool>;
#[doc = "Reader of field `GIF31`"]
pub type GIF31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - GIF0"]
    #[inline(always)]
    pub fn gif0(&self) -> GIF0_R {
        GIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GIF1"]
    #[inline(always)]
    pub fn gif1(&self) -> GIF1_R {
        GIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GIF2"]
    #[inline(always)]
    pub fn gif2(&self) -> GIF2_R {
        GIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GIF3"]
    #[inline(always)]
    pub fn gif3(&self) -> GIF3_R {
        GIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GIF4"]
    #[inline(always)]
    pub fn gif4(&self) -> GIF4_R {
        GIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GIF5"]
    #[inline(always)]
    pub fn gif5(&self) -> GIF5_R {
        GIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GIF6"]
    #[inline(always)]
    pub fn gif6(&self) -> GIF6_R {
        GIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GIF7"]
    #[inline(always)]
    pub fn gif7(&self) -> GIF7_R {
        GIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GIF8"]
    #[inline(always)]
    pub fn gif8(&self) -> GIF8_R {
        GIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GIF9"]
    #[inline(always)]
    pub fn gif9(&self) -> GIF9_R {
        GIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GIF10"]
    #[inline(always)]
    pub fn gif10(&self) -> GIF10_R {
        GIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GIF11"]
    #[inline(always)]
    pub fn gif11(&self) -> GIF11_R {
        GIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - GIF12"]
    #[inline(always)]
    pub fn gif12(&self) -> GIF12_R {
        GIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - GIF13"]
    #[inline(always)]
    pub fn gif13(&self) -> GIF13_R {
        GIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - GIF14"]
    #[inline(always)]
    pub fn gif14(&self) -> GIF14_R {
        GIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GIF15"]
    #[inline(always)]
    pub fn gif15(&self) -> GIF15_R {
        GIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GIF16"]
    #[inline(always)]
    pub fn gif16(&self) -> GIF16_R {
        GIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - GIF17"]
    #[inline(always)]
    pub fn gif17(&self) -> GIF17_R {
        GIF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - GIF18"]
    #[inline(always)]
    pub fn gif18(&self) -> GIF18_R {
        GIF18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - GIF19"]
    #[inline(always)]
    pub fn gif19(&self) -> GIF19_R {
        GIF19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - GIF20"]
    #[inline(always)]
    pub fn gif20(&self) -> GIF20_R {
        GIF20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - GIF21"]
    #[inline(always)]
    pub fn gif21(&self) -> GIF21_R {
        GIF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - GIF22"]
    #[inline(always)]
    pub fn gif22(&self) -> GIF22_R {
        GIF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GIF23"]
    #[inline(always)]
    pub fn gif23(&self) -> GIF23_R {
        GIF23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - GIF24"]
    #[inline(always)]
    pub fn gif24(&self) -> GIF24_R {
        GIF24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - GIF25"]
    #[inline(always)]
    pub fn gif25(&self) -> GIF25_R {
        GIF25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - GIF26"]
    #[inline(always)]
    pub fn gif26(&self) -> GIF26_R {
        GIF26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - GIF27"]
    #[inline(always)]
    pub fn gif27(&self) -> GIF27_R {
        GIF27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - GIF28"]
    #[inline(always)]
    pub fn gif28(&self) -> GIF28_R {
        GIF28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - GIF29"]
    #[inline(always)]
    pub fn gif29(&self) -> GIF29_R {
        GIF29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - GIF30"]
    #[inline(always)]
    pub fn gif30(&self) -> GIF30_R {
        GIF30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - GIF31"]
    #[inline(always)]
    pub fn gif31(&self) -> GIF31_R {
        GIF31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
