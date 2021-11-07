#[doc = "Register `CONFIG_OSC_TOP` reader"]
pub struct R(crate::R<CONFIG_OSC_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_OSC_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_OSC_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_OSC_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_OSC_TOP` writer"]
pub struct W(crate::W<CONFIG_OSC_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_OSC_TOP_SPEC>;
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
impl From<crate::W<CONFIG_OSC_TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_OSC_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSC_HF_ROW_Q12` reader - 29:26\\]
Internal. Only to be used through TI provided API."]
pub struct XOSC_HF_ROW_Q12_R(crate::FieldReader<u8, u8>);
impl XOSC_HF_ROW_Q12_R {
    pub(crate) fn new(bits: u8) -> Self {
        XOSC_HF_ROW_Q12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_HF_ROW_Q12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_HF_ROW_Q12` writer - 29:26\\]
Internal. Only to be used through TI provided API."]
pub struct XOSC_HF_ROW_Q12_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_ROW_Q12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | ((value as u32 & 0x0f) << 26);
        self.w
    }
}
#[doc = "Field `XOSC_HF_COLUMN_Q12` reader - 25:10\\]
Internal. Only to be used through TI provided API."]
pub struct XOSC_HF_COLUMN_Q12_R(crate::FieldReader<u16, u16>);
impl XOSC_HF_COLUMN_Q12_R {
    pub(crate) fn new(bits: u16) -> Self {
        XOSC_HF_COLUMN_Q12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_HF_COLUMN_Q12_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XOSC_HF_COLUMN_Q12` writer - 25:10\\]
Internal. Only to be used through TI provided API."]
pub struct XOSC_HF_COLUMN_Q12_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_HF_COLUMN_Q12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 10)) | ((value as u32 & 0xffff) << 10);
        self.w
    }
}
#[doc = "Field `RCOSCLF_CTUNE_TRIM` reader - 9:2\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCLF_CTUNE_TRIM_R(crate::FieldReader<u8, u8>);
impl RCOSCLF_CTUNE_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCOSCLF_CTUNE_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOSCLF_CTUNE_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOSCLF_CTUNE_TRIM` writer - 9:2\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCLF_CTUNE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLF_CTUNE_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | ((value as u32 & 0xff) << 2);
        self.w
    }
}
#[doc = "Field `RCOSCLF_RTUNE_TRIM` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCLF_RTUNE_TRIM_R(crate::FieldReader<u8, u8>);
impl RCOSCLF_RTUNE_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCOSCLF_RTUNE_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCOSCLF_RTUNE_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCOSCLF_RTUNE_TRIM` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct RCOSCLF_RTUNE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLF_RTUNE_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:29 - 29:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&self) -> XOSC_HF_ROW_Q12_R {
        XOSC_HF_ROW_Q12_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bits 10:25 - 25:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&self) -> XOSC_HF_COLUMN_Q12_R {
        XOSC_HF_COLUMN_Q12_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bits 2:9 - 9:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&self) -> RCOSCLF_CTUNE_TRIM_R {
        RCOSCLF_CTUNE_TRIM_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&self) -> RCOSCLF_RTUNE_TRIM_R {
        RCOSCLF_RTUNE_TRIM_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 26:29 - 29:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&mut self) -> XOSC_HF_ROW_Q12_W {
        XOSC_HF_ROW_Q12_W { w: self }
    }
    #[doc = "Bits 10:25 - 25:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&mut self) -> XOSC_HF_COLUMN_Q12_W {
        XOSC_HF_COLUMN_Q12_W { w: self }
    }
    #[doc = "Bits 2:9 - 9:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&mut self) -> RCOSCLF_CTUNE_TRIM_W {
        RCOSCLF_CTUNE_TRIM_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&mut self) -> RCOSCLF_RTUNE_TRIM_W {
        RCOSCLF_RTUNE_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_osc_top](index.html) module"]
pub struct CONFIG_OSC_TOP_SPEC;
impl crate::RegisterSpec for CONFIG_OSC_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_osc_top::R](R) reader structure"]
impl crate::Readable for CONFIG_OSC_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_osc_top::W](W) writer structure"]
impl crate::Writable for CONFIG_OSC_TOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG_OSC_TOP to value 0xfc00_fc00"]
impl crate::Resettable for CONFIG_OSC_TOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfc00_fc00
    }
}
