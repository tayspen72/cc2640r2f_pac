#[doc = "Register `AMPCOMPTH2` reader"]
pub struct R(crate::R<AMPCOMPTH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPCOMPTH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPCOMPTH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPCOMPTH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPCOMPTH2` writer"]
pub struct W(crate::W<AMPCOMPTH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPCOMPTH2_SPEC>;
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
impl From<crate::W<AMPCOMPTH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPCOMPTH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPMUPDATE_LTH` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub struct LPMUPDATE_LTH_R(crate::FieldReader<u8, u8>);
impl LPMUPDATE_LTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMUPDATE_LTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMUPDATE_LTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMUPDATE_LTH` writer - 31:26\\]
Internal. Only to be used through TI provided API."]
pub struct LPMUPDATE_LTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMUPDATE_LTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
#[doc = "Field `SPARE24` reader - 25:24\\]
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
#[doc = "Field `SPARE24` writer - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE24_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `LPMUPDATE_HTH` reader - 23:18\\]
Internal. Only to be used through TI provided API."]
pub struct LPMUPDATE_HTH_R(crate::FieldReader<u8, u8>);
impl LPMUPDATE_HTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMUPDATE_HTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPMUPDATE_HTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMUPDATE_HTH` writer - 23:18\\]
Internal. Only to be used through TI provided API."]
pub struct LPMUPDATE_HTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMUPDATE_HTH_W<'a> {
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
#[doc = "Field `ADC_COMP_AMPTH_LPM` reader - 15:10\\]
Internal. Only to be used through TI provided API."]
pub struct ADC_COMP_AMPTH_LPM_R(crate::FieldReader<u8, u8>);
impl ADC_COMP_AMPTH_LPM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_COMP_AMPTH_LPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_COMP_AMPTH_LPM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_COMP_AMPTH_LPM` writer - 15:10\\]
Internal. Only to be used through TI provided API."]
pub struct ADC_COMP_AMPTH_LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_COMP_AMPTH_LPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | ((value as u32 & 0x3f) << 10);
        self.w
    }
}
#[doc = "Field `SPARE8` reader - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE8_R(crate::FieldReader<u8, u8>);
impl SPARE8_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPARE8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE8` writer - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE8_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `ADC_COMP_AMPTH_HPM` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub struct ADC_COMP_AMPTH_HPM_R(crate::FieldReader<u8, u8>);
impl ADC_COMP_AMPTH_HPM_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_COMP_AMPTH_HPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_COMP_AMPTH_HPM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_COMP_AMPTH_HPM` writer - 7:2\\]
Internal. Only to be used through TI provided API."]
pub struct ADC_COMP_AMPTH_HPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_COMP_AMPTH_HPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u32 & 0x3f) << 2);
        self.w
    }
}
#[doc = "Field `SPARE0` reader - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE0_R(crate::FieldReader<u8, u8>);
impl SPARE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPARE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE0` writer - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct SPARE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_lth(&self) -> LPMUPDATE_LTH_R {
        LPMUPDATE_LTH_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare24(&self) -> SPARE24_R {
        SPARE24_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_hth(&self) -> LPMUPDATE_HTH_R {
        LPMUPDATE_HTH_R::new(((self.bits >> 18) & 0x3f) as u8)
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
    pub fn adc_comp_ampth_lpm(&self) -> ADC_COMP_AMPTH_LPM_R {
        ADC_COMP_AMPTH_LPM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare8(&self) -> SPARE8_R {
        SPARE8_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&self) -> ADC_COMP_AMPTH_HPM_R {
        ADC_COMP_AMPTH_HPM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare0(&self) -> SPARE0_R {
        SPARE0_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_lth(&mut self) -> LPMUPDATE_LTH_W {
        LPMUPDATE_LTH_W { w: self }
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare24(&mut self) -> SPARE24_W {
        SPARE24_W { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_hth(&mut self) -> LPMUPDATE_HTH_W {
        LPMUPDATE_HTH_W { w: self }
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
    pub fn adc_comp_ampth_lpm(&mut self) -> ADC_COMP_AMPTH_LPM_W {
        ADC_COMP_AMPTH_LPM_W { w: self }
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare8(&mut self) -> SPARE8_W {
        SPARE8_W { w: self }
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&mut self) -> ADC_COMP_AMPTH_HPM_W {
        ADC_COMP_AMPTH_HPM_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare0(&mut self) -> SPARE0_W {
        SPARE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcompth2](index.html) module"]
pub struct AMPCOMPTH2_SPEC;
impl crate::RegisterSpec for AMPCOMPTH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ampcompth2::R](R) reader structure"]
impl crate::Readable for AMPCOMPTH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampcompth2::W](W) writer structure"]
impl crate::Writable for AMPCOMPTH2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AMPCOMPTH2 to value 0"]
impl crate::Resettable for AMPCOMPTH2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
