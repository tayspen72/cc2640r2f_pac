#[doc = "Register `SOC_ADC_ABS_GAIN` reader"]
pub struct R(crate::R<SOC_ADC_ABS_GAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_ADC_ABS_GAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_ADC_ABS_GAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_ADC_ABS_GAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOC_ADC_ABS_GAIN` writer"]
pub struct W(crate::W<SOC_ADC_ABS_GAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOC_ADC_ABS_GAIN_SPEC>;
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
impl From<crate::W<SOC_ADC_ABS_GAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOC_ADC_ABS_GAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_R(crate::FieldReader<u16, u16>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `SOC_ADC_ABS_GAIN_TEMP1` reader - 15:0\\]
SOC_ADC gain in absolute reference mode at temperature 1 (30C). Calculated in production test.."]
pub struct SOC_ADC_ABS_GAIN_TEMP1_R(crate::FieldReader<u16, u16>);
impl SOC_ADC_ABS_GAIN_TEMP1_R {
    pub(crate) fn new(bits: u16) -> Self {
        SOC_ADC_ABS_GAIN_TEMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOC_ADC_ABS_GAIN_TEMP1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOC_ADC_ABS_GAIN_TEMP1` writer - 15:0\\]
SOC_ADC gain in absolute reference mode at temperature 1 (30C). Calculated in production test.."]
pub struct SOC_ADC_ABS_GAIN_TEMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_ADC_ABS_GAIN_TEMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
SOC_ADC gain in absolute reference mode at temperature 1 (30C). Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_abs_gain_temp1(&self) -> SOC_ADC_ABS_GAIN_TEMP1_R {
        SOC_ADC_ABS_GAIN_TEMP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
SOC_ADC gain in absolute reference mode at temperature 1 (30C). Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_abs_gain_temp1(&mut self) -> SOC_ADC_ABS_GAIN_TEMP1_W {
        SOC_ADC_ABS_GAIN_TEMP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX_ADC Gain in Absolute Reference Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_abs_gain](index.html) module"]
pub struct SOC_ADC_ABS_GAIN_SPEC;
impl crate::RegisterSpec for SOC_ADC_ABS_GAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_adc_abs_gain::R](R) reader structure"]
impl crate::Readable for SOC_ADC_ABS_GAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soc_adc_abs_gain::W](W) writer structure"]
impl crate::Writable for SOC_ADC_ABS_GAIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOC_ADC_ABS_GAIN to value 0"]
impl crate::Resettable for SOC_ADC_ABS_GAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
