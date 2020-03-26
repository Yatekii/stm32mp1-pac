#[doc = "Reader of register HSEM_CR"]
pub type R = crate::R<u32, super::HSEM_CR>;
#[doc = "Writer for register HSEM_CR"]
pub type W = crate::W<u32, super::HSEM_CR>;
#[doc = "Register HSEM_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSEM_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COREID`"]
pub type COREID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COREID`"]
pub struct COREID_W<'a> {
    w: &'a mut W,
}
impl<'a> COREID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - MasterID of semaphores to be cleared"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Semaphore clear Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:11 - MasterID of semaphores to be cleared"]
    #[inline(always)]
    pub fn coreid(&mut self) -> COREID_W {
        COREID_W { w: self }
    }
    #[doc = "Bits 16:31 - Semaphore clear Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
