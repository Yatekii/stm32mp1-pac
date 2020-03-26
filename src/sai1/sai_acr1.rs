#[doc = "Reader of register SAI_ACR1"]
pub type R = crate::R<u32, super::SAI_ACR1>;
#[doc = "Writer for register SAI_ACR1"]
pub type W = crate::W<u32, super::SAI_ACR1>;
#[doc = "Register SAI_ACR1 `reset()`'s with value 0x40"]
impl crate::ResetValue for super::SAI_ACR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PRTCFG`"]
pub type PRTCFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRTCFG`"]
pub struct PRTCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRTCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DS`"]
pub type DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS`"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `LSBFIRST`"]
pub type LSBFIRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSBFIRST`"]
pub struct LSBFIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBFIRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CKSTR`"]
pub type CKSTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKSTR`"]
pub struct CKSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SYNCEN`"]
pub type SYNCEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNCEN`"]
pub struct SYNCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `MONO`"]
pub type MONO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONO`"]
pub struct MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `OUTDRIV`"]
pub type OUTDRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OUTDRIV`"]
pub struct OUTDRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTDRIV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SAIXEN`"]
pub type SAIXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAIXEN`"]
pub struct SAIXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SAIXEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `NOMCK`"]
pub type NOMCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOMCK`"]
pub struct NOMCK_W<'a> {
    w: &'a mut W,
}
impl<'a> NOMCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `MCKDIV`"]
pub type MCKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCKDIV`"]
pub struct MCKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
#[doc = "Reader of field `OSR`"]
pub type OSR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSR`"]
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `MCKEN`"]
pub type MCKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCKEN`"]
pub struct MCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SAIx audio block mode immediately"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Protocol configuration. These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - Data size. These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\]
bits, DS\\[1:0\\]
are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC97 audio protocol since AC97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Clock strobing edge. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - Synchronization enable. These bits are set and cleared by software. They must be configured when the audio sub-block is disabled. Note: The audio sub-block should be configured as asynchronous when SPDIF mode is enabled."]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Mono mode. This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to Section: Mono/stereo mode for more details."]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Output drive. This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
    #[inline(always)]
    pub fn outdriv(&self) -> OUTDRIV_R {
        OUTDRIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Audio block enable where x is A or B. This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command will not be taken into account. This bit allows to control the state of SAIx audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When SAIx block is configured in master mode, the clock must be present on the input of SAIx before setting SAIXEN bit."]
    #[inline(always)]
    pub fn saixen(&self) -> SAIXEN_R {
        SAIXEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DMA enable. This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\]
bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nomck(&self) -> NOMCK_R {
        NOMCK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:25 - Master clock divider. These bits are set and cleared by software. These bits are meaningless when the audio block operates in slave mode. They have to be configured when the audio block is disabled. Others: the master clock frequency is calculated accordingly to the following formula:"]
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - Oversampling ratio for master clock"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - MCKEN"]
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SAIx audio block mode immediately"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Protocol configuration. These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn prtcfg(&mut self) -> PRTCFG_W {
        PRTCFG_W { w: self }
    }
    #[doc = "Bits 5:7 - Data size. These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\]
bits, DS\\[1:0\\]
are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    #[doc = "Bit 8 - Least significant bit first. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC97 audio protocol since AC97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W {
        LSBFIRST_W { w: self }
    }
    #[doc = "Bit 9 - Clock strobing edge. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
    #[inline(always)]
    pub fn ckstr(&mut self) -> CKSTR_W {
        CKSTR_W { w: self }
    }
    #[doc = "Bits 10:11 - Synchronization enable. These bits are set and cleared by software. They must be configured when the audio sub-block is disabled. Note: The audio sub-block should be configured as asynchronous when SPDIF mode is enabled."]
    #[inline(always)]
    pub fn syncen(&mut self) -> SYNCEN_W {
        SYNCEN_W { w: self }
    }
    #[doc = "Bit 12 - Mono mode. This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to Section: Mono/stereo mode for more details."]
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W {
        MONO_W { w: self }
    }
    #[doc = "Bit 13 - Output drive. This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
    #[inline(always)]
    pub fn outdriv(&mut self) -> OUTDRIV_W {
        OUTDRIV_W { w: self }
    }
    #[doc = "Bit 16 - Audio block enable where x is A or B. This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command will not be taken into account. This bit allows to control the state of SAIx audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When SAIx block is configured in master mode, the clock must be present on the input of SAIx before setting SAIXEN bit."]
    #[inline(always)]
    pub fn saixen(&mut self) -> SAIXEN_W {
        SAIXEN_W { w: self }
    }
    #[doc = "Bit 17 - DMA enable. This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\]
bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nomck(&mut self) -> NOMCK_W {
        NOMCK_W { w: self }
    }
    #[doc = "Bits 20:25 - Master clock divider. These bits are set and cleared by software. These bits are meaningless when the audio block operates in slave mode. They have to be configured when the audio block is disabled. Others: the master clock frequency is calculated accordingly to the following formula:"]
    #[inline(always)]
    pub fn mckdiv(&mut self) -> MCKDIV_W {
        MCKDIV_W { w: self }
    }
    #[doc = "Bit 26 - Oversampling ratio for master clock"]
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
    #[doc = "Bit 27 - MCKEN"]
    #[inline(always)]
    pub fn mcken(&mut self) -> MCKEN_W {
        MCKEN_W { w: self }
    }
}
