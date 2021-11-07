#[doc = "Register `RCOSC_HF_TEMPCOMP` reader"]
pub struct R(crate::R<RCOSC_HF_TEMPCOMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCOSC_HF_TEMPCOMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCOSC_HF_TEMPCOMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCOSC_HF_TEMPCOMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCOSC_HF_TEMPCOMP` writer"]
pub struct W(crate::W<RCOSC_HF_TEMPCOMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCOSC_HF_TEMPCOMP_SPEC>;
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
impl From<crate::W<RCOSC_HF_TEMPCOMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCOSC_HF_TEMPCOMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINE_RESISTOR` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct FINE_RESISTOR_R(crate::FieldReader<u8, u8>);
impl FINE_RESISTOR_R {
    pub(crate) fn new(bits: u8) -> Self {
        FINE_RESISTOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINE_RESISTOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINE_RESISTOR` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct FINE_RESISTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> FINE_RESISTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `CTRIM` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct CTRIM_R(crate::FieldReader<u8, u8>);
impl CTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRIM` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct CTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CTRIMFRACT_QUAD` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct CTRIMFRACT_QUAD_R(crate::FieldReader<u8, u8>);
impl CTRIMFRACT_QUAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRIMFRACT_QUAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRIMFRACT_QUAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRIMFRACT_QUAD` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct CTRIMFRACT_QUAD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRIMFRACT_QUAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CTRIMFRACT_SLOPE` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct CTRIMFRACT_SLOPE_R(crate::FieldReader<u8, u8>);
impl CTRIMFRACT_SLOPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRIMFRACT_SLOPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRIMFRACT_SLOPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRIMFRACT_SLOPE` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct CTRIMFRACT_SLOPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRIMFRACT_SLOPE_W<'a> {
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
    pub fn fine_resistor(&self) -> FINE_RESISTOR_R {
        FINE_RESISTOR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrim(&self) -> CTRIM_R {
        CTRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_quad(&self) -> CTRIMFRACT_QUAD_R {
        CTRIMFRACT_QUAD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_slope(&self) -> CTRIMFRACT_SLOPE_R {
        CTRIMFRACT_SLOPE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fine_resistor(&mut self) -> FINE_RESISTOR_W {
        FINE_RESISTOR_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrim(&mut self) -> CTRIM_W {
        CTRIM_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_quad(&mut self) -> CTRIMFRACT_QUAD_W {
        CTRIMFRACT_QUAD_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrimfract_slope(&mut self) -> CTRIMFRACT_SLOPE_W {
        CTRIMFRACT_SLOPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcosc_hf_tempcomp](index.html) module"]
pub struct RCOSC_HF_TEMPCOMP_SPEC;
impl crate::RegisterSpec for RCOSC_HF_TEMPCOMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcosc_hf_tempcomp::R](R) reader structure"]
impl crate::Readable for RCOSC_HF_TEMPCOMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcosc_hf_tempcomp::W](W) writer structure"]
impl crate::Writable for RCOSC_HF_TEMPCOMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCOSC_HF_TEMPCOMP to value 0x03"]
impl crate::Resettable for RCOSC_HF_TEMPCOMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
