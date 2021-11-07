#[doc = "Register `SOC_ADC_OFFSET_INT` reader"]
pub struct R(crate::R<SOC_ADC_OFFSET_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_ADC_OFFSET_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_ADC_OFFSET_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_ADC_OFFSET_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOC_ADC_OFFSET_INT` writer"]
pub struct W(crate::W<SOC_ADC_OFFSET_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOC_ADC_OFFSET_INT_SPEC>;
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
impl From<crate::W<SOC_ADC_OFFSET_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOC_ADC_OFFSET_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED24_R(crate::FieldReader<u8, u8>);
impl RESERVED24_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `SOC_ADC_REL_OFFSET_TEMP1` reader - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub struct SOC_ADC_REL_OFFSET_TEMP1_R(crate::FieldReader<u8, u8>);
impl SOC_ADC_REL_OFFSET_TEMP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SOC_ADC_REL_OFFSET_TEMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOC_ADC_REL_OFFSET_TEMP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOC_ADC_REL_OFFSET_TEMP1` writer - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub struct SOC_ADC_REL_OFFSET_TEMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_ADC_REL_OFFSET_TEMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED8` reader - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_R(crate::FieldReader<u8, u8>);
impl RESERVED8_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED8` writer - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SOC_ADC_ABS_OFFSET_TEMP1` reader - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub struct SOC_ADC_ABS_OFFSET_TEMP1_R(crate::FieldReader<u8, u8>);
impl SOC_ADC_ABS_OFFSET_TEMP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SOC_ADC_ABS_OFFSET_TEMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOC_ADC_ABS_OFFSET_TEMP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOC_ADC_ABS_OFFSET_TEMP1` writer - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub struct SOC_ADC_ABS_OFFSET_TEMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_ADC_ABS_OFFSET_TEMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_rel_offset_temp1(&self) -> SOC_ADC_REL_OFFSET_TEMP1_R {
        SOC_ADC_REL_OFFSET_TEMP1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_abs_offset_temp1(&self) -> SOC_ADC_ABS_OFFSET_TEMP1_R {
        SOC_ADC_ABS_OFFSET_TEMP1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_rel_offset_temp1(&mut self) -> SOC_ADC_REL_OFFSET_TEMP1_W {
        SOC_ADC_REL_OFFSET_TEMP1_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_abs_offset_temp1(&mut self) -> SOC_ADC_ABS_OFFSET_TEMP1_W {
        SOC_ADC_ABS_OFFSET_TEMP1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_offset_int](index.html) module"]
pub struct SOC_ADC_OFFSET_INT_SPEC;
impl crate::RegisterSpec for SOC_ADC_OFFSET_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_adc_offset_int::R](R) reader structure"]
impl crate::Readable for SOC_ADC_OFFSET_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soc_adc_offset_int::W](W) writer structure"]
impl crate::Writable for SOC_ADC_OFFSET_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOC_ADC_OFFSET_INT to value 0"]
impl crate::Resettable for SOC_ADC_OFFSET_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
