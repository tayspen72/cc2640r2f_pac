#[doc = "Register `DMASTAT` reader"]
pub struct R(crate::R<DMASTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMASTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMASTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMASTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMASTAT` writer"]
pub struct W(crate::W<DMASTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMASTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMASTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMASTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED18_R(crate::FieldReader<u16, u16>);
impl RESERVED18_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED18_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED18` writer - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED18_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 18)) | ((value as u32 & 0x3fff) << 18);
        self.w
    }
}
#[doc = "Field `PORT_ERR` reader - 17:17\\]
Reflects possible transfer errors on the AHB port."]
pub struct PORT_ERR_R(crate::FieldReader<bool, bool>);
impl PORT_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORT_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_ERR` writer - 17:17\\]
Reflects possible transfer errors on the AHB port."]
pub struct PORT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `RESERVED2` reader - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_R(crate::FieldReader<u16, u16>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 2)) | ((value as u32 & 0x7fff) << 2);
        self.w
    }
}
#[doc = "Field `CH1_ACTIVE` reader - 1:1\\]
This register field indicates if DMA channel 1 is active or not. 0: Not active 1: Active"]
pub struct CH1_ACTIVE_R(crate::FieldReader<bool, bool>);
impl CH1_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1_ACTIVE` writer - 1:1\\]
This register field indicates if DMA channel 1 is active or not. 0: Not active 1: Active"]
pub struct CH1_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CH0_ACTIVE` reader - 0:0\\]
This register field indicates if DMA channel 0 is active or not. 0: Not active 1: Active"]
pub struct CH0_ACTIVE_R(crate::FieldReader<bool, bool>);
impl CH0_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0_ACTIVE` writer - 0:0\\]
This register field indicates if DMA channel 0 is active or not. 0: Not active 1: Active"]
pub struct CH0_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
    #[doc = "Bit 17 - 17:17\\]
Reflects possible transfer errors on the AHB port."]
    #[inline(always)]
    pub fn port_err(&self) -> PORT_ERR_R {
        PORT_ERR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 2:16 - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bit 1 - 1:1\\]
This register field indicates if DMA channel 1 is active or not. 0: Not active 1: Active"]
    #[inline(always)]
    pub fn ch1_active(&self) -> CH1_ACTIVE_R {
        CH1_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
This register field indicates if DMA channel 0 is active or not. 0: Not active 1: Active"]
    #[inline(always)]
    pub fn ch0_active(&self) -> CH0_ACTIVE_R {
        CH0_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&mut self) -> RESERVED18_W {
        RESERVED18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Reflects possible transfer errors on the AHB port."]
    #[inline(always)]
    pub fn port_err(&mut self) -> PORT_ERR_W {
        PORT_ERR_W { w: self }
    }
    #[doc = "Bits 2:16 - 16:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
This register field indicates if DMA channel 1 is active or not. 0: Not active 1: Active"]
    #[inline(always)]
    pub fn ch1_active(&mut self) -> CH1_ACTIVE_W {
        CH1_ACTIVE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
This register field indicates if DMA channel 0 is active or not. 0: Not active 1: Active"]
    #[inline(always)]
    pub fn ch0_active(&mut self) -> CH0_ACTIVE_W {
        CH0_ACTIVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Controller Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastat](index.html) module"]
pub struct DMASTAT_SPEC;
impl crate::RegisterSpec for DMASTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmastat::R](R) reader structure"]
impl crate::Readable for DMASTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmastat::W](W) writer structure"]
impl crate::Writable for DMASTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMASTAT to value 0"]
impl crate::Resettable for DMASTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
