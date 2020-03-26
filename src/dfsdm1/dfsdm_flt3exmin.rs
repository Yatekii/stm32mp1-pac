#[doc = "Reader of register DFSDM_FLT3EXMIN"]
pub type R = crate::R<u32, super::DFSDM_FLT3EXMIN>;
#[doc = "Reader of field `EXMINCH`"]
pub type EXMINCH_R = crate::R<u8, u8>;
#[doc = "Reader of field `EXMIN`"]
pub type EXMIN_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - EXMINCH"]
    #[inline(always)]
    pub fn exminch(&self) -> EXMINCH_R {
        EXMINCH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - EXMIN"]
    #[inline(always)]
    pub fn exmin(&self) -> EXMIN_R {
        EXMIN_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
