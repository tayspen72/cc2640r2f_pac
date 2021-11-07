#[doc = "Register `CONFIG_MISC_ADC_DIV10` reader"]
pub struct R(crate::R<CONFIG_MISC_ADC_DIV10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_MISC_ADC_DIV10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_MISC_ADC_DIV10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_MISC_ADC_DIV10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_MISC_ADC_DIV10` writer"]
pub struct W(crate::W<CONFIG_MISC_ADC_DIV10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_MISC_ADC_DIV10_SPEC>;
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
impl From<crate::W<CONFIG_MISC_ADC_DIV10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_MISC_ADC_DIV10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSSI_OFFSET` reader - 16:9\\]
Internal. Only to be used through TI provided API."]
pub struct RSSI_OFFSET_R(crate::FieldReader<u8, u8>);
impl RSSI_OFFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSSI_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSSI_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSSI_OFFSET` writer - 16:9\\]
Internal. Only to be used through TI provided API."]
pub struct RSSI_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RSSI_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 9)) | ((value as u32 & 0xff) << 9);
        self.w
    }
}
#[doc = "Field `QUANTCTLTHRES` reader - 8:6\\]
Internal. Only to be used through TI provided API."]
pub struct QUANTCTLTHRES_R(crate::FieldReader<u8, u8>);
impl QUANTCTLTHRES_R {
    pub(crate) fn new(bits: u8) -> Self {
        QUANTCTLTHRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUANTCTLTHRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUANTCTLTHRES` writer - 8:6\\]
Internal. Only to be used through TI provided API."]
pub struct QUANTCTLTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> QUANTCTLTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `DACTRIM` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub struct DACTRIM_R(crate::FieldReader<u8, u8>);
impl DACTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        DACTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACTRIM` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub struct DACTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DACTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:16 - 16:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssi_offset(&self) -> RSSI_OFFSET_R {
        RSSI_OFFSET_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn quantctlthres(&self) -> QUANTCTLTHRES_R {
        QUANTCTLTHRES_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dactrim(&self) -> DACTRIM_R {
        DACTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 9:16 - 16:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssi_offset(&mut self) -> RSSI_OFFSET_W {
        RSSI_OFFSET_W { w: self }
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn quantctlthres(&mut self) -> QUANTCTLTHRES_W {
        QUANTCTLTHRES_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dactrim(&mut self) -> DACTRIM_W {
        DACTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_misc_adc_div10](index.html) module"]
pub struct CONFIG_MISC_ADC_DIV10_SPEC;
impl crate::RegisterSpec for CONFIG_MISC_ADC_DIV10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_misc_adc_div10::R](R) reader structure"]
impl crate::Readable for CONFIG_MISC_ADC_DIV10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_misc_adc_div10::W](W) writer structure"]
impl crate::Writable for CONFIG_MISC_ADC_DIV10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG_MISC_ADC_DIV10 to value 0xffff_ffff"]
impl crate::Resettable for CONFIG_MISC_ADC_DIV10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
