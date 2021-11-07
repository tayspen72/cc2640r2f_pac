#[doc = "Register `FLASH_E_P` reader"]
pub struct R(crate::R<FLASH_E_P_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_E_P_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_E_P_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_E_P_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_E_P` writer"]
pub struct W(crate::W<FLASH_E_P_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_E_P_SPEC>;
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
impl From<crate::W<FLASH_E_P_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_E_P_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSU` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct PSU_R(crate::FieldReader<u8, u8>);
impl PSU_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSU` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct PSU_W<'a> {
    w: &'a mut W,
}
impl<'a> PSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `ESU` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct ESU_R(crate::FieldReader<u8, u8>);
impl ESU_R {
    pub(crate) fn new(bits: u8) -> Self {
        ESU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESU` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct ESU_W<'a> {
    w: &'a mut W,
}
impl<'a> ESU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PVSU` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct PVSU_R(crate::FieldReader<u8, u8>);
impl PVSU_R {
    pub(crate) fn new(bits: u8) -> Self {
        PVSU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVSU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVSU` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct PVSU_W<'a> {
    w: &'a mut W,
}
impl<'a> PVSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `EVSU` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct EVSU_R(crate::FieldReader<u8, u8>);
impl EVSU_R {
    pub(crate) fn new(bits: u8) -> Self {
        EVSU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVSU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVSU` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct EVSU_W<'a> {
    w: &'a mut W,
}
impl<'a> EVSU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psu(&self) -> PSU_R {
        PSU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn esu(&self) -> ESU_R {
        ESU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvsu(&self) -> PVSU_R {
        PVSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn evsu(&self) -> EVSU_R {
        EVSU_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psu(&mut self) -> PSU_W {
        PSU_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn esu(&mut self) -> ESU_W {
        ESU_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvsu(&mut self) -> PVSU_W {
        PVSU_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn evsu(&mut self) -> EVSU_W {
        EVSU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_e_p](index.html) module"]
pub struct FLASH_E_P_SPEC;
impl crate::RegisterSpec for FLASH_E_P_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_e_p::R](R) reader structure"]
impl crate::Readable for FLASH_E_P_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_e_p::W](W) writer structure"]
impl crate::Writable for FLASH_E_P_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_E_P to value 0x1733_1a33"]
impl crate::Resettable for FLASH_E_P_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1733_1a33
    }
}
