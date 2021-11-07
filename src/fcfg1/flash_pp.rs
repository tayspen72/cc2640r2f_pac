#[doc = "Register `FLASH_PP` reader"]
pub struct R(crate::R<FLASH_PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_PP` writer"]
pub struct W(crate::W<FLASH_PP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_PP_SPEC>;
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
impl From<crate::W<FLASH_PP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_PP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUMP_SU` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct PUMP_SU_R(crate::FieldReader<u8, u8>);
impl PUMP_SU_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUMP_SU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUMP_SU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUMP_SU` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct PUMP_SU_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMP_SU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `MAX_PP` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct MAX_PP_R(crate::FieldReader<u16, u16>);
impl MAX_PP_R {
    pub(crate) fn new(bits: u16) -> Self {
        MAX_PP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_PP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_PP` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct MAX_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_PP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pump_su(&self) -> PUMP_SU_R {
        PUMP_SU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_pp(&self) -> MAX_PP_R {
        MAX_PP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pump_su(&mut self) -> PUMP_SU_W {
        PUMP_SU_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_pp(&mut self) -> MAX_PP_W {
        MAX_PP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_pp](index.html) module"]
pub struct FLASH_PP_SPEC;
impl crate::RegisterSpec for FLASH_PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_pp::R](R) reader structure"]
impl crate::Readable for FLASH_PP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_pp::W](W) writer structure"]
impl crate::Writable for FLASH_PP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_PP to value 0x14"]
impl crate::Resettable for FLASH_PP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x14
    }
}
