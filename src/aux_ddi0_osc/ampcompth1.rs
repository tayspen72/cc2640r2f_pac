#[doc = "Register `AMPCOMPTH1` reader"]
pub struct R(crate::R<AMPCOMPTH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPCOMPTH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPCOMPTH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPCOMPTH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPCOMPTH1` writer"]
pub struct W(crate::W<AMPCOMPTH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPCOMPTH1_SPEC>;
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
impl From<crate::W<AMPCOMPTH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPCOMPTH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPARE24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE24_R(crate::FieldReader<u8, u8>);
impl SPARE24_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPARE24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE24_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `HPMRAMP3_LTH` reader - 23:18\\]
Internal. Only to be used through TI provided API."]
pub struct HPMRAMP3_LTH_R(crate::FieldReader<u8, u8>);
impl HPMRAMP3_LTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        HPMRAMP3_LTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPMRAMP3_LTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPMRAMP3_LTH` writer - 23:18\\]
Internal. Only to be used through TI provided API."]
pub struct HPMRAMP3_LTH_W<'a> {
    w: &'a mut W,
}
impl<'a> HPMRAMP3_LTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `SPARE16` reader - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE16_R(crate::FieldReader<u8, u8>);
impl SPARE16_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPARE16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE16` writer - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE16_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `HPMRAMP3_HTH` reader - 15:10\\]
Internal. Only to be used through TI provided API."]
pub struct HPMRAMP3_HTH_R(crate::FieldReader<u8, u8>);
impl HPMRAMP3_HTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        HPMRAMP3_HTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPMRAMP3_HTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPMRAMP3_HTH` writer - 15:10\\]
Internal. Only to be used through TI provided API."]
pub struct HPMRAMP3_HTH_W<'a> {
    w: &'a mut W,
}
impl<'a> HPMRAMP3_HTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | ((value as u32 & 0x3f) << 10);
        self.w
    }
}
#[doc = "Field `IBIASCAP_LPTOHP_OL_CNT` reader - 9:6\\]
Internal. Only to be used through TI provided API."]
pub struct IBIASCAP_LPTOHP_OL_CNT_R(crate::FieldReader<u8, u8>);
impl IBIASCAP_LPTOHP_OL_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        IBIASCAP_LPTOHP_OL_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBIASCAP_LPTOHP_OL_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBIASCAP_LPTOHP_OL_CNT` writer - 9:6\\]
Internal. Only to be used through TI provided API."]
pub struct IBIASCAP_LPTOHP_OL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIASCAP_LPTOHP_OL_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | ((value as u32 & 0x0f) << 6);
        self.w
    }
}
#[doc = "Field `HPMRAMP1_TH` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub struct HPMRAMP1_TH_R(crate::FieldReader<u8, u8>);
impl HPMRAMP1_TH_R {
    pub(crate) fn new(bits: u8) -> Self {
        HPMRAMP1_TH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPMRAMP1_TH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPMRAMP1_TH` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub struct HPMRAMP1_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> HPMRAMP1_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare24(&self) -> SPARE24_R {
        SPARE24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp3_lth(&self) -> HPMRAMP3_LTH_R {
        HPMRAMP3_LTH_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare16(&self) -> SPARE16_R {
        SPARE16_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp3_hth(&self) -> HPMRAMP3_HTH_R {
        HPMRAMP3_HTH_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_lptohp_ol_cnt(&self) -> IBIASCAP_LPTOHP_OL_CNT_R {
        IBIASCAP_LPTOHP_OL_CNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp1_th(&self) -> HPMRAMP1_TH_R {
        HPMRAMP1_TH_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare24(&mut self) -> SPARE24_W {
        SPARE24_W { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp3_lth(&mut self) -> HPMRAMP3_LTH_W {
        HPMRAMP3_LTH_W { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare16(&mut self) -> SPARE16_W {
        SPARE16_W { w: self }
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp3_hth(&mut self) -> HPMRAMP3_HTH_W {
        HPMRAMP3_HTH_W { w: self }
    }
    #[doc = "Bits 6:9 - 9:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_lptohp_ol_cnt(&mut self) -> IBIASCAP_LPTOHP_OL_CNT_W {
        IBIASCAP_LPTOHP_OL_CNT_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp1_th(&mut self) -> HPMRAMP1_TH_W {
        HPMRAMP1_TH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Amplitude Compensation Threshold 1 This register contains threshold values for amplitude compensation algorithm\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcompth1](index.html) module"]
pub struct AMPCOMPTH1_SPEC;
impl crate::RegisterSpec for AMPCOMPTH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ampcompth1::R](R) reader structure"]
impl crate::Readable for AMPCOMPTH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampcompth1::W](W) writer structure"]
impl crate::Writable for AMPCOMPTH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AMPCOMPTH1 to value 0"]
impl crate::Resettable for AMPCOMPTH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
