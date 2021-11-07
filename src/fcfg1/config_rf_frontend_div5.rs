#[doc = "Register `CONFIG_RF_FRONTEND_DIV5` reader"]
pub struct R(crate::R<CONFIG_RF_FRONTEND_DIV5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_RF_FRONTEND_DIV5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_RF_FRONTEND_DIV5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_RF_FRONTEND_DIV5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_RF_FRONTEND_DIV5` writer"]
pub struct W(crate::W<CONFIG_RF_FRONTEND_DIV5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_RF_FRONTEND_DIV5_SPEC>;
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
impl From<crate::W<CONFIG_RF_FRONTEND_DIV5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_RF_FRONTEND_DIV5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFAMP_IB` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct IFAMP_IB_R(crate::FieldReader<u8, u8>);
impl IFAMP_IB_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFAMP_IB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFAMP_IB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFAMP_IB` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct IFAMP_IB_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAMP_IB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `LNA_IB` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub struct LNA_IB_R(crate::FieldReader<u8, u8>);
impl LNA_IB_R {
    pub(crate) fn new(bits: u8) -> Self {
        LNA_IB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_IB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNA_IB` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub struct LNA_IB_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_IB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `IFAMP_TRIM` reader - 23:19\\]
Internal. Only to be used through TI provided API."]
pub struct IFAMP_TRIM_R(crate::FieldReader<u8, u8>);
impl IFAMP_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        IFAMP_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFAMP_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFAMP_TRIM` writer - 23:19\\]
Internal. Only to be used through TI provided API."]
pub struct IFAMP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAMP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `CTL_PA0_TRIM` reader - 18:14\\]
Internal. Only to be used through TI provided API."]
pub struct CTL_PA0_TRIM_R(crate::FieldReader<u8, u8>);
impl CTL_PA0_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTL_PA0_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTL_PA0_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTL_PA0_TRIM` writer - 18:14\\]
Internal. Only to be used through TI provided API."]
pub struct CTL_PA0_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL_PA0_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 14)) | ((value as u32 & 0x1f) << 14);
        self.w
    }
}
#[doc = "Field `RFLDO_TRIM_OUTPUT` reader - 6:0\\]
Internal. Only to be used through TI provided API."]
pub struct RFLDO_TRIM_OUTPUT_R(crate::FieldReader<u8, u8>);
impl RFLDO_TRIM_OUTPUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFLDO_TRIM_OUTPUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFLDO_TRIM_OUTPUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFLDO_TRIM_OUTPUT` writer - 6:0\\]
Internal. Only to be used through TI provided API."]
pub struct RFLDO_TRIM_OUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RFLDO_TRIM_OUTPUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_ib(&self) -> IFAMP_IB_R {
        IFAMP_IB_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lna_ib(&self) -> LNA_IB_R {
        LNA_IB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_trim(&self) -> IFAMP_TRIM_R {
        IFAMP_TRIM_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - 18:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctl_pa0_trim(&self) -> CTL_PA0_TRIM_R {
        CTL_PA0_TRIM_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfldo_trim_output(&self) -> RFLDO_TRIM_OUTPUT_R {
        RFLDO_TRIM_OUTPUT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_ib(&mut self) -> IFAMP_IB_W {
        IFAMP_IB_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lna_ib(&mut self) -> LNA_IB_W {
        LNA_IB_W { w: self }
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_trim(&mut self) -> IFAMP_TRIM_W {
        IFAMP_TRIM_W { w: self }
    }
    #[doc = "Bits 14:18 - 18:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctl_pa0_trim(&mut self) -> CTL_PA0_TRIM_W {
        CTL_PA0_TRIM_W { w: self }
    }
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfldo_trim_output(&mut self) -> RFLDO_TRIM_OUTPUT_W {
        RFLDO_TRIM_OUTPUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_rf_frontend_div5](index.html) module"]
pub struct CONFIG_RF_FRONTEND_DIV5_SPEC;
impl crate::RegisterSpec for CONFIG_RF_FRONTEND_DIV5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_rf_frontend_div5::R](R) reader structure"]
impl crate::Readable for CONFIG_RF_FRONTEND_DIV5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_rf_frontend_div5::W](W) writer structure"]
impl crate::Writable for CONFIG_RF_FRONTEND_DIV5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONFIG_RF_FRONTEND_DIV5 to value 0xffff_ffff"]
impl crate::Resettable for CONFIG_RF_FRONTEND_DIV5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
