#[doc = "Register `FLASH_V` reader"]
pub struct R(crate::R<FLASH_V_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_V_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_V_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_V_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_V` writer"]
pub struct W(crate::W<FLASH_V_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_V_SPEC>;
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
impl From<crate::W<FLASH_V_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_V_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VSL_P` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct VSL_P_R(crate::FieldReader<u8, u8>);
impl VSL_P_R {
    pub(crate) fn new(bits: u8) -> Self {
        VSL_P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSL_P_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSL_P` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct VSL_P_W<'a> {
    w: &'a mut W,
}
impl<'a> VSL_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `VWL_P` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct VWL_P_R(crate::FieldReader<u8, u8>);
impl VWL_P_R {
    pub(crate) fn new(bits: u8) -> Self {
        VWL_P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VWL_P_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VWL_P` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct VWL_P_W<'a> {
    w: &'a mut W,
}
impl<'a> VWL_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `V_READ` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct V_READ_R(crate::FieldReader<u8, u8>);
impl V_READ_R {
    pub(crate) fn new(bits: u8) -> Self {
        V_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V_READ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `V_READ` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct V_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> V_READ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vsl_p(&self) -> VSL_P_R {
        VSL_P_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwl_p(&self) -> VWL_P_R {
        VWL_P_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v_read(&self) -> V_READ_R {
        V_READ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vsl_p(&mut self) -> VSL_P_W {
        VSL_P_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwl_p(&mut self) -> VWL_P_W {
        VWL_P_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v_read(&mut self) -> V_READ_W {
        V_READ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_v](index.html) module"]
pub struct FLASH_V_SPEC;
impl crate::RegisterSpec for FLASH_V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_v::R](R) reader structure"]
impl crate::Readable for FLASH_V_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_v::W](W) writer structure"]
impl crate::Writable for FLASH_V_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_V to value 0"]
impl crate::Resettable for FLASH_V_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
