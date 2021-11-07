#[doc = "Register `CCFG_TAP_DAP_1` reader"]
pub struct R(crate::R<CCFG_TAP_DAP_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_TAP_DAP_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_TAP_DAP_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_TAP_DAP_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_TAP_DAP_1` writer"]
pub struct W(crate::W<CCFG_TAP_DAP_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_TAP_DAP_1_SPEC>;
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
impl From<crate::W<CCFG_TAP_DAP_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_TAP_DAP_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBIST2_TAP_ENABLE` reader - 23:16\\]
Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
pub struct PBIST2_TAP_ENABLE_R(crate::FieldReader<u8, u8>);
impl PBIST2_TAP_ENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBIST2_TAP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIST2_TAP_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIST2_TAP_ENABLE` writer - 23:16\\]
Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
pub struct PBIST2_TAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIST2_TAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PBIST1_TAP_ENABLE` reader - 15:8\\]
Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
pub struct PBIST1_TAP_ENABLE_R(crate::FieldReader<u8, u8>);
impl PBIST1_TAP_ENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBIST1_TAP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBIST1_TAP_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBIST1_TAP_ENABLE` writer - 15:8\\]
Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
pub struct PBIST1_TAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PBIST1_TAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `WUC_TAP_ENABLE` reader - 7:0\\]
Enable WUC TAP 0xC5: WUC TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: WUC TAP access will remain disabled out of power-up/system-reset."]
pub struct WUC_TAP_ENABLE_R(crate::FieldReader<u8, u8>);
impl WUC_TAP_ENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        WUC_TAP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUC_TAP_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUC_TAP_ENABLE` writer - 7:0\\]
Enable WUC TAP 0xC5: WUC TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: WUC TAP access will remain disabled out of power-up/system-reset."]
pub struct WUC_TAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUC_TAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - 23:16\\]
Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pbist2_tap_enable(&self) -> PBIST2_TAP_ENABLE_R {
        PBIST2_TAP_ENABLE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pbist1_tap_enable(&self) -> PBIST1_TAP_ENABLE_R {
        PBIST1_TAP_ENABLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enable WUC TAP 0xC5: WUC TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: WUC TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn wuc_tap_enable(&self) -> WUC_TAP_ENABLE_R {
        WUC_TAP_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - 23:16\\]
Enable PBIST2 TAP. 0xC5: PBIST2 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST2 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pbist2_tap_enable(&mut self) -> PBIST2_TAP_ENABLE_W {
        PBIST2_TAP_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PBIST1 TAP. 0xC5: PBIST1 TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PBIST1 TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn pbist1_tap_enable(&mut self) -> PBIST1_TAP_ENABLE_W {
        PBIST1_TAP_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enable WUC TAP 0xC5: WUC TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: WUC TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn wuc_tap_enable(&mut self) -> WUC_TAP_ENABLE_W {
        WUC_TAP_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Access Points Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_tap_dap_1](index.html) module"]
pub struct CCFG_TAP_DAP_1_SPEC;
impl crate::RegisterSpec for CCFG_TAP_DAP_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_tap_dap_1::R](R) reader structure"]
impl crate::Readable for CCFG_TAP_DAP_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_tap_dap_1::W](W) writer structure"]
impl crate::Writable for CCFG_TAP_DAP_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_TAP_DAP_1 to value 0xffc5_c5c5"]
impl crate::Resettable for CCFG_TAP_DAP_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffc5_c5c5
    }
}
