#[doc = "Register `FLASH_EH_SEQ` reader"]
pub struct R(crate::R<FLASH_EH_SEQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_EH_SEQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_EH_SEQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_EH_SEQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_EH_SEQ` writer"]
pub struct W(crate::W<FLASH_EH_SEQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_EH_SEQ_SPEC>;
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
impl From<crate::W<FLASH_EH_SEQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_EH_SEQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EH` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct EH_R(crate::FieldReader<u8, u8>);
impl EH_R {
    pub(crate) fn new(bits: u8) -> Self {
        EH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EH` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct EH_W<'a> {
    w: &'a mut W,
}
impl<'a> EH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `SEQ` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct SEQ_R(crate::FieldReader<u8, u8>);
impl SEQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQ` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct SEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `VSTAT` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct VSTAT_R(crate::FieldReader<u8, u8>);
impl VSTAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        VSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSTAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSTAT` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct VSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> VSTAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `SM_FREQUENCY` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct SM_FREQUENCY_R(crate::FieldReader<u16, u16>);
impl SM_FREQUENCY_R {
    pub(crate) fn new(bits: u16) -> Self {
        SM_FREQUENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM_FREQUENCY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM_FREQUENCY` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub struct SM_FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_FREQUENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eh(&self) -> EH_R {
        EH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn seq(&self) -> SEQ_R {
        SEQ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vstat(&self) -> VSTAT_R {
        VSTAT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sm_frequency(&self) -> SM_FREQUENCY_R {
        SM_FREQUENCY_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eh(&mut self) -> EH_W {
        EH_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn seq(&mut self) -> SEQ_W {
        SEQ_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vstat(&mut self) -> VSTAT_W {
        VSTAT_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sm_frequency(&mut self) -> SM_FREQUENCY_W {
        SM_FREQUENCY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_eh_seq](index.html) module"]
pub struct FLASH_EH_SEQ_SPEC;
impl crate::RegisterSpec for FLASH_EH_SEQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_eh_seq::R](R) reader structure"]
impl crate::Readable for FLASH_EH_SEQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_eh_seq::W](W) writer structure"]
impl crate::Writable for FLASH_EH_SEQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_EH_SEQ to value 0x0200_f000"]
impl crate::Resettable for FLASH_EH_SEQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_f000
    }
}
