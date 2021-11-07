#[doc = "Register `RADCEXTCFG` reader"]
pub struct R(crate::R<RADCEXTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RADCEXTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RADCEXTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RADCEXTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RADCEXTCFG` writer"]
pub struct W(crate::W<RADCEXTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RADCEXTCFG_SPEC>;
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
impl From<crate::W<RADCEXTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RADCEXTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPM_IBIAS_WAIT_CNT` reader - 31:22\\]
Internal. Only to be used through TI provided API."]
pub struct HPM_IBIAS_WAIT_CNT_R(crate::FieldReader<u16, u16>);
impl HPM_IBIAS_WAIT_CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        HPM_IBIAS_WAIT_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPM_IBIAS_WAIT_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPM_IBIAS_WAIT_CNT` writer - 31:22\\]
Internal. Only to be used through TI provided API."]
pub struct HPM_IBIAS_WAIT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_IBIAS_WAIT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | ((value as u32 & 0x03ff) << 22);
        self.w
    }
}
#[doc = "Field `LPM_IBIAS_WAIT_CNT` reader - 21:16\\]
Internal. Only to be used through TI provided API."]
pub struct LPM_IBIAS_WAIT_CNT_R(crate::FieldReader<u8, u8>);
impl LPM_IBIAS_WAIT_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPM_IBIAS_WAIT_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_IBIAS_WAIT_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_IBIAS_WAIT_CNT` writer - 21:16\\]
Internal. Only to be used through TI provided API."]
pub struct LPM_IBIAS_WAIT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_IBIAS_WAIT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `IDAC_STEP` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct IDAC_STEP_R(crate::FieldReader<u8, u8>);
impl IDAC_STEP_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDAC_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDAC_STEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDAC_STEP` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct IDAC_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAC_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `RADC_DAC_TH` reader - 11:6\\]
Internal. Only to be used through TI provided API."]
pub struct RADC_DAC_TH_R(crate::FieldReader<u8, u8>);
impl RADC_DAC_TH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RADC_DAC_TH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADC_DAC_TH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADC_DAC_TH` writer - 11:6\\]
Internal. Only to be used through TI provided API."]
pub struct RADC_DAC_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> RADC_DAC_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `RADC_MODE_IS_SAR` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct RADC_MODE_IS_SAR_R(crate::FieldReader<bool, bool>);
impl RADC_MODE_IS_SAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RADC_MODE_IS_SAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RADC_MODE_IS_SAR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RADC_MODE_IS_SAR` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct RADC_MODE_IS_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RADC_MODE_IS_SAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_R(crate::FieldReader<u8, u8>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:31 - 31:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpm_ibias_wait_cnt(&self) -> HPM_IBIAS_WAIT_CNT_R {
        HPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt(&self) -> LPM_IBIAS_WAIT_CNT_R {
        LPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn idac_step(&self) -> IDAC_STEP_R {
        IDAC_STEP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn radc_dac_th(&self) -> RADC_DAC_TH_R {
        RADC_DAC_TH_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn radc_mode_is_sar(&self) -> RADC_MODE_IS_SAR_R {
        RADC_MODE_IS_SAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 22:31 - 31:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpm_ibias_wait_cnt(&mut self) -> HPM_IBIAS_WAIT_CNT_W {
        HPM_IBIAS_WAIT_CNT_W { w: self }
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt(&mut self) -> LPM_IBIAS_WAIT_CNT_W {
        LPM_IBIAS_WAIT_CNT_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn idac_step(&mut self) -> IDAC_STEP_W {
        IDAC_STEP_W { w: self }
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn radc_dac_th(&mut self) -> RADC_DAC_TH_W {
        RADC_DAC_TH_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn radc_mode_is_sar(&mut self) -> RADC_MODE_IS_SAR_W {
        RADC_MODE_IS_SAR_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RADC External Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [radcextcfg](index.html) module"]
pub struct RADCEXTCFG_SPEC;
impl crate::RegisterSpec for RADCEXTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [radcextcfg::R](R) reader structure"]
impl crate::Readable for RADCEXTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [radcextcfg::W](W) writer structure"]
impl crate::Writable for RADCEXTCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RADCEXTCFG to value 0"]
impl crate::Resettable for RADCEXTCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
