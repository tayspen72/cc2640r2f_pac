#[doc = "Register `STAT2` reader"]
pub struct R(crate::R<STAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT2` writer"]
pub struct W(crate::W<STAT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT2_SPEC>;
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
impl From<crate::W<STAT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_DCBIAS` reader - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
pub struct ADC_DCBIAS_R(crate::FieldReader<u8, u8>);
impl ADC_DCBIAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_DCBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DCBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_DCBIAS` writer - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
pub struct ADC_DCBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DCBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
#[doc = "Field `HPM_RAMP1_THMET` reader - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
pub struct HPM_RAMP1_THMET_R(crate::FieldReader<bool, bool>);
impl HPM_RAMP1_THMET_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPM_RAMP1_THMET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPM_RAMP1_THMET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPM_RAMP1_THMET` writer - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
pub struct HPM_RAMP1_THMET_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_RAMP1_THMET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `HPM_RAMP2_THMET` reader - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
pub struct HPM_RAMP2_THMET_R(crate::FieldReader<bool, bool>);
impl HPM_RAMP2_THMET_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPM_RAMP2_THMET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPM_RAMP2_THMET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPM_RAMP2_THMET` writer - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
pub struct HPM_RAMP2_THMET_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_RAMP2_THMET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `HPM_RAMP3_THMET` reader - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
pub struct HPM_RAMP3_THMET_R(crate::FieldReader<bool, bool>);
impl HPM_RAMP3_THMET_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPM_RAMP3_THMET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPM_RAMP3_THMET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPM_RAMP3_THMET` writer - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
pub struct HPM_RAMP3_THMET_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_RAMP3_THMET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `RESERVED16` reader - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_R(crate::FieldReader<u8, u8>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `RAMPSTATE` reader - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
pub struct RAMPSTATE_R(crate::FieldReader<u8, u8>);
impl RAMPSTATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RAMPSTATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMPSTATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMPSTATE` writer - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
pub struct RAMPSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPSTATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `RESERVED4` reader - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_R(crate::FieldReader<u8, u8>);
impl RESERVED4_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | ((value as u32 & 0xff) << 4);
        self.w
    }
}
#[doc = "Field `AMPCOMP_REQ` reader - 3:3\\]
ampcomp_req"]
pub struct AMPCOMP_REQ_R(crate::FieldReader<bool, bool>);
impl AMPCOMP_REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMPCOMP_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMPCOMP_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMPCOMP_REQ` writer - 3:3\\]
ampcomp_req"]
pub struct AMPCOMP_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `XOSC_HF_AMPGOOD` reader - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
pub struct XOSC_HF_AMPGOOD_R(crate::FieldReader<bool, bool>);
impl XOSC_HF_AMPGOOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        XOSC_HF_AMPGOOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_HF_AMPGOOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_HF_AMPGOOD` writer - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
pub struct XOSC_HF_AMPGOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_AMPGOOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `XOSC_HF_FREQGOOD` reader - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
pub struct XOSC_HF_FREQGOOD_R(crate::FieldReader<bool, bool>);
impl XOSC_HF_FREQGOOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        XOSC_HF_FREQGOOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_HF_FREQGOOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_HF_FREQGOOD` writer - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
pub struct XOSC_HF_FREQGOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_FREQGOOD_W<'a> {
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
#[doc = "Field `XOSC_HF_RF_FREQGOOD` reader - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
pub struct XOSC_HF_RF_FREQGOOD_R(crate::FieldReader<bool, bool>);
impl XOSC_HF_RF_FREQGOOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        XOSC_HF_RF_FREQGOOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_HF_RF_FREQGOOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_HF_RF_FREQGOOD` writer - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
pub struct XOSC_HF_RF_FREQGOOD_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_RF_FREQGOOD_W<'a> {
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
    #[doc = "Bits 26:31 - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline(always)]
    pub fn adc_dcbias(&self) -> ADC_DCBIAS_R {
        ADC_DCBIAS_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
    #[inline(always)]
    pub fn hpm_ramp1_thmet(&self) -> HPM_RAMP1_THMET_R {
        HPM_RAMP1_THMET_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
    #[inline(always)]
    pub fn hpm_ramp2_thmet(&self) -> HPM_RAMP2_THMET_R {
        HPM_RAMP2_THMET_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
    #[inline(always)]
    pub fn hpm_ramp3_thmet(&self) -> HPM_RAMP3_THMET_R {
        HPM_RAMP3_THMET_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline(always)]
    pub fn rampstate(&self) -> RAMPSTATE_R {
        RAMPSTATE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
ampcomp_req"]
    #[inline(always)]
    pub fn ampcomp_req(&self) -> AMPCOMP_REQ_R {
        AMPCOMP_REQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
    #[inline(always)]
    pub fn xosc_hf_ampgood(&self) -> XOSC_HF_AMPGOOD_R {
        XOSC_HF_AMPGOOD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
    #[inline(always)]
    pub fn xosc_hf_freqgood(&self) -> XOSC_HF_FREQGOOD_R {
        XOSC_HF_FREQGOOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline(always)]
    pub fn xosc_hf_rf_freqgood(&self) -> XOSC_HF_RF_FREQGOOD_R {
        XOSC_HF_RF_FREQGOOD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 26:31 - 31:26\\]
DC Bias read by RADC during SAR mode The value is an unsigned integer. It is used for debug only."]
    #[inline(always)]
    pub fn adc_dcbias(&mut self) -> ADC_DCBIAS_W {
        ADC_DCBIAS_W { w: self }
    }
    #[doc = "Bit 25 - 25:25\\]
Indication of threshold is met for hpm_ramp1"]
    #[inline(always)]
    pub fn hpm_ramp1_thmet(&mut self) -> HPM_RAMP1_THMET_W {
        HPM_RAMP1_THMET_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
Indication of threshold is met for hpm_ramp2"]
    #[inline(always)]
    pub fn hpm_ramp2_thmet(&mut self) -> HPM_RAMP2_THMET_W {
        HPM_RAMP2_THMET_W { w: self }
    }
    #[doc = "Bit 23 - 23:23\\]
Indication of threshold is met for hpm_ramp3"]
    #[inline(always)]
    pub fn hpm_ramp3_thmet(&mut self) -> HPM_RAMP3_THMET_W {
        HPM_RAMP3_THMET_W { w: self }
    }
    #[doc = "Bits 16:22 - 22:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
xosc_hf amplitude compensation FSM This is identical to STAT1.RAMPSTATE. See that description for encoding."]
    #[inline(always)]
    pub fn rampstate(&mut self) -> RAMPSTATE_W {
        RAMPSTATE_W { w: self }
    }
    #[doc = "Bits 4:11 - 11:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
ampcomp_req"]
    #[inline(always)]
    pub fn ampcomp_req(&mut self) -> AMPCOMP_REQ_W {
        AMPCOMP_REQ_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
amplitude of xosc_hf is within the required threshold (set by DDI). Not used for anything just for debug/status"]
    #[inline(always)]
    pub fn xosc_hf_ampgood(&mut self) -> XOSC_HF_AMPGOOD_W {
        XOSC_HF_AMPGOOD_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
frequency of xosc_hf is good to use for the digital clocks"]
    #[inline(always)]
    pub fn xosc_hf_freqgood(&mut self) -> XOSC_HF_FREQGOOD_W {
        XOSC_HF_FREQGOOD_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
frequency of xosc_hf is within +/- 20 ppm and xosc_hf is good for radio operations. Used for SW to start synthesizer."]
    #[inline(always)]
    pub fn xosc_hf_rf_freqgood(&mut self) -> XOSC_HF_RF_FREQGOOD_W {
        XOSC_HF_RF_FREQGOOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status 2 This register contains status signals from AMPCOMP FSM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat2](index.html) module"]
pub struct STAT2_SPEC;
impl crate::RegisterSpec for STAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat2::R](R) reader structure"]
impl crate::Readable for STAT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat2::W](W) writer structure"]
impl crate::Writable for STAT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STAT2 to value 0"]
impl crate::Resettable for STAT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
