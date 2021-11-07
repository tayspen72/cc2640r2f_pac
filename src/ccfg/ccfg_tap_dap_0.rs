#[doc = "Register `CCFG_TAP_DAP_0` reader"]
pub struct R(crate::R<CCFG_TAP_DAP_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_TAP_DAP_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_TAP_DAP_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_TAP_DAP_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_TAP_DAP_0` writer"]
pub struct W(crate::W<CCFG_TAP_DAP_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_TAP_DAP_0_SPEC>;
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
impl From<crate::W<CCFG_TAP_DAP_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_TAP_DAP_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_DAP_ENABLE` reader - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
pub struct CPU_DAP_ENABLE_R(crate::FieldReader<u8, u8>);
impl CPU_DAP_ENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_DAP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_DAP_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_DAP_ENABLE` writer - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
pub struct CPU_DAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_DAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PRCM_TAP_ENABLE` reader - 15:8\\]
Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
pub struct PRCM_TAP_ENABLE_R(crate::FieldReader<u8, u8>);
impl PRCM_TAP_ENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRCM_TAP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRCM_TAP_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRCM_TAP_ENABLE` writer - 15:8\\]
Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
pub struct PRCM_TAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRCM_TAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TEST_TAP_ENABLE` reader - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
pub struct TEST_TAP_ENABLE_R(crate::FieldReader<u8, u8>);
impl TEST_TAP_ENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEST_TAP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_TAP_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_TAP_ENABLE` writer - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
pub struct TEST_TAP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_TAP_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn cpu_dap_enable(&self) -> CPU_DAP_ENABLE_R {
        CPU_DAP_ENABLE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn prcm_tap_enable(&self) -> PRCM_TAP_ENABLE_R {
        PRCM_TAP_ENABLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn test_tap_enable(&self) -> TEST_TAP_ENABLE_R {
        TEST_TAP_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - 23:16\\]
Enable CPU DAP. 0xC5: Main CPU DAP access is enabled during power-up/system-reset by ROM boot FW. Any other value: Main CPU DAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn cpu_dap_enable(&mut self) -> CPU_DAP_ENABLE_W {
        CPU_DAP_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Enable PRCM TAP. 0xC5: PRCM TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: PRCM TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn prcm_tap_enable(&mut self) -> PRCM_TAP_ENABLE_W {
        PRCM_TAP_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Enable Test TAP. 0xC5: TEST TAP access is enabled during power-up/system-reset by ROM boot FW if enabled by corresponding configuration value in FCFG1 defined by TI. Any other value: TEST TAP access will remain disabled out of power-up/system-reset."]
    #[inline(always)]
    pub fn test_tap_enable(&mut self) -> TEST_TAP_ENABLE_W {
        TEST_TAP_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Access Points Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_tap_dap_0](index.html) module"]
pub struct CCFG_TAP_DAP_0_SPEC;
impl crate::RegisterSpec for CCFG_TAP_DAP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_tap_dap_0::R](R) reader structure"]
impl crate::Readable for CCFG_TAP_DAP_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_tap_dap_0::W](W) writer structure"]
impl crate::Writable for CCFG_TAP_DAP_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_TAP_DAP_0 to value 0xffc5_c5c5"]
impl crate::Resettable for CCFG_TAP_DAP_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffc5_c5c5
    }
}
