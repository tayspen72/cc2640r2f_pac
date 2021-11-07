#[doc = "Register `SHDW_OSC_BIAS_LDO_TRIM` reader"]
pub struct R(crate::R<SHDW_OSC_BIAS_LDO_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHDW_OSC_BIAS_LDO_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHDW_OSC_BIAS_LDO_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHDW_OSC_BIAS_LDO_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHDW_OSC_BIAS_LDO_TRIM` writer"]
pub struct W(crate::W<SHDW_OSC_BIAS_LDO_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHDW_OSC_BIAS_LDO_TRIM_SPEC>;
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
impl From<crate::W<SHDW_OSC_BIAS_LDO_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHDW_OSC_BIAS_LDO_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET_RCOSC_HF_COARSE_RESISTOR` reader - 28:27\\]
Internal. Only to be used through TI provided API."]
pub struct SET_RCOSC_HF_COARSE_RESISTOR_R(crate::FieldReader<u8, u8>);
impl SET_RCOSC_HF_COARSE_RESISTOR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SET_RCOSC_HF_COARSE_RESISTOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET_RCOSC_HF_COARSE_RESISTOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SET_RCOSC_HF_COARSE_RESISTOR` writer - 28:27\\]
Internal. Only to be used through TI provided API."]
pub struct SET_RCOSC_HF_COARSE_RESISTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_RCOSC_HF_COARSE_RESISTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `TRIMMAG` reader - 26:23\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMMAG_R(crate::FieldReader<u8, u8>);
impl TRIMMAG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMMAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMMAG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMMAG` writer - 26:23\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMMAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMMAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | ((value as u32 & 0x0f) << 23);
        self.w
    }
}
#[doc = "Field `TRIMIREF` reader - 22:18\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMIREF_R(crate::FieldReader<u8, u8>);
impl TRIMIREF_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMIREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMIREF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMIREF` writer - 22:18\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMIREF_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMIREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | ((value as u32 & 0x1f) << 18);
        self.w
    }
}
#[doc = "Field `ITRIM_DIG_LDO` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub struct ITRIM_DIG_LDO_R(crate::FieldReader<u8, u8>);
impl ITRIM_DIG_LDO_R {
    pub(crate) fn new(bits: u8) -> Self {
        ITRIM_DIG_LDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITRIM_DIG_LDO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITRIM_DIG_LDO` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub struct ITRIM_DIG_LDO_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIM_DIG_LDO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `VTRIM_DIG` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct VTRIM_DIG_R(crate::FieldReader<u8, u8>);
impl VTRIM_DIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        VTRIM_DIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIM_DIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIM_DIG` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct VTRIM_DIG_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_DIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `VTRIM_COARSE` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub struct VTRIM_COARSE_R(crate::FieldReader<u8, u8>);
impl VTRIM_COARSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        VTRIM_COARSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIM_COARSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIM_COARSE` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub struct VTRIM_COARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_COARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RCOSCHF_CTRIM` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCHF_CTRIM_R(crate::FieldReader<u8, u8>);
impl RCOSCHF_CTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCOSCHF_CTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOSCHF_CTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOSCHF_CTRIM` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCHF_CTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHF_CTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:28 - 28:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_rcosc_hf_coarse_resistor(&self) -> SET_RCOSC_HF_COARSE_RESISTOR_R {
        SET_RCOSC_HF_COARSE_RESISTOR_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 23:26 - 26:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimmag(&self) -> TRIMMAG_R {
        TRIMMAG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimiref(&self) -> TRIMIREF_R {
        TRIMIREF_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_dig_ldo(&self) -> ITRIM_DIG_LDO_R {
        ITRIM_DIG_LDO_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_dig(&self) -> VTRIM_DIG_R {
        VTRIM_DIG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_coarse(&self) -> VTRIM_COARSE_R {
        VTRIM_COARSE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_ctrim(&self) -> RCOSCHF_CTRIM_R {
        RCOSCHF_CTRIM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 27:28 - 28:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn set_rcosc_hf_coarse_resistor(&mut self) -> SET_RCOSC_HF_COARSE_RESISTOR_W {
        SET_RCOSC_HF_COARSE_RESISTOR_W { w: self }
    }
    #[doc = "Bits 23:26 - 26:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimmag(&mut self) -> TRIMMAG_W {
        TRIMMAG_W { w: self }
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimiref(&mut self) -> TRIMIREF_W {
        TRIMIREF_W { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_dig_ldo(&mut self) -> ITRIM_DIG_LDO_W {
        ITRIM_DIG_LDO_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_dig(&mut self) -> VTRIM_DIG_W {
        VTRIM_DIG_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_coarse(&mut self) -> VTRIM_COARSE_W {
        VTRIM_COARSE_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_ctrim(&mut self) -> RCOSCHF_CTRIM_W {
        RCOSCHF_CTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_osc_bias_ldo_trim](index.html) module"]
pub struct SHDW_OSC_BIAS_LDO_TRIM_SPEC;
impl crate::RegisterSpec for SHDW_OSC_BIAS_LDO_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shdw_osc_bias_ldo_trim::R](R) reader structure"]
impl crate::Readable for SHDW_OSC_BIAS_LDO_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shdw_osc_bias_ldo_trim::W](W) writer structure"]
impl crate::Writable for SHDW_OSC_BIAS_LDO_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHDW_OSC_BIAS_LDO_TRIM to value 0"]
impl crate::Resettable for SHDW_OSC_BIAS_LDO_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
