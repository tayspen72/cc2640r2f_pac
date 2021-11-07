#[doc = "Register `SIZE_AND_DIS_FLAGS` reader"]
pub struct R(crate::R<SIZE_AND_DIS_FLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIZE_AND_DIS_FLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIZE_AND_DIS_FLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIZE_AND_DIS_FLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIZE_AND_DIS_FLAGS` writer"]
pub struct W(crate::W<SIZE_AND_DIS_FLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIZE_AND_DIS_FLAGS_SPEC>;
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
impl From<crate::W<SIZE_AND_DIS_FLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIZE_AND_DIS_FLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE_OF_CCFG` reader - 31:16\\]
Total size of CCFG in bytes."]
pub struct SIZE_OF_CCFG_R(crate::FieldReader<u16, u16>);
impl SIZE_OF_CCFG_R {
    pub(crate) fn new(bits: u16) -> Self {
        SIZE_OF_CCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_OF_CCFG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE_OF_CCFG` writer - 31:16\\]
Total size of CCFG in bytes."]
pub struct SIZE_OF_CCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_OF_CCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `DISABLE_FLAGS` reader - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct DISABLE_FLAGS_R(crate::FieldReader<u16, u16>);
impl DISABLE_FLAGS_R {
    pub(crate) fn new(bits: u16) -> Self {
        DISABLE_FLAGS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_FLAGS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISABLE_FLAGS` writer - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct DISABLE_FLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_FLAGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | ((value as u32 & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Field `DIS_TCXO` reader - 3:3\\]
Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
pub struct DIS_TCXO_R(crate::FieldReader<bool, bool>);
impl DIS_TCXO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_TCXO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_TCXO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_TCXO` writer - 3:3\\]
Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
pub struct DIS_TCXO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_TCXO_W<'a> {
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
#[doc = "Field `DIS_GPRAM` reader - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
pub struct DIS_GPRAM_R(crate::FieldReader<bool, bool>);
impl DIS_GPRAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_GPRAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_GPRAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_GPRAM` writer - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
pub struct DIS_GPRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_GPRAM_W<'a> {
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
#[doc = "Field `DIS_ALT_DCDC_SETTING` reader - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub struct DIS_ALT_DCDC_SETTING_R(crate::FieldReader<bool, bool>);
impl DIS_ALT_DCDC_SETTING_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_ALT_DCDC_SETTING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_ALT_DCDC_SETTING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_ALT_DCDC_SETTING` writer - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
pub struct DIS_ALT_DCDC_SETTING_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_ALT_DCDC_SETTING_W<'a> {
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
#[doc = "Field `DIS_XOSC_OVR` reader - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
pub struct DIS_XOSC_OVR_R(crate::FieldReader<bool, bool>);
impl DIS_XOSC_OVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_XOSC_OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_XOSC_OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_XOSC_OVR` writer - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
pub struct DIS_XOSC_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_XOSC_OVR_W<'a> {
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
    #[doc = "Bits 16:31 - 31:16\\]
Total size of CCFG in bytes."]
    #[inline(always)]
    pub fn size_of_ccfg(&self) -> SIZE_OF_CCFG_R {
        SIZE_OF_CCFG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn disable_flags(&self) -> DISABLE_FLAGS_R {
        DISABLE_FLAGS_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 3 - 3:3\\]
Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
    #[inline(always)]
    pub fn dis_tcxo(&self) -> DIS_TCXO_R {
        DIS_TCXO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
    #[inline(always)]
    pub fn dis_gpram(&self) -> DIS_GPRAM_R {
        DIS_GPRAM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dis_alt_dcdc_setting(&self) -> DIS_ALT_DCDC_SETTING_R {
        DIS_ALT_DCDC_SETTING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
    #[inline(always)]
    pub fn dis_xosc_ovr(&self) -> DIS_XOSC_OVR_R {
        DIS_XOSC_OVR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Total size of CCFG in bytes."]
    #[inline(always)]
    pub fn size_of_ccfg(&mut self) -> SIZE_OF_CCFG_W {
        SIZE_OF_CCFG_W { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn disable_flags(&mut self) -> DISABLE_FLAGS_W {
        DISABLE_FLAGS_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Disable TCXO. 0: TCXO functionality enabled. 1: TCXO functionality disabled. Note: An external TCXO is required if DIS_TCXO = 0."]
    #[inline(always)]
    pub fn dis_tcxo(&mut self) -> DIS_TCXO_W {
        DIS_TCXO_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Disable GPRAM (or use the 8K VIMS RAM as CACHE RAM). 0: GPRAM is enabled and hence CACHE disabled. 1: GPRAM is disabled and instead CACHE is enabled (default). Notes: - Disabling CACHE will reduce CPU execution speed (up to 60%). - GPRAM is 8 K-bytes in size and located at 0x11000000-0x11001FFF if enabled. See: VIMS:CTL.MODE"]
    #[inline(always)]
    pub fn dis_gpram(&mut self) -> DIS_GPRAM_W {
        DIS_GPRAM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Disable alternate DC/DC settings. 0: Enable alternate DC/DC settings. 1: Disable alternate DC/DC settings. See: MODE_CONF_1.ALT_DCDC_VMIN MODE_CONF_1.ALT_DCDC_DITHER_EN MODE_CONF_1.ALT_DCDC_IPEAK NOTE! The DriverLib function SysCtrl_DCDC_VoltageConditionalControl() must be called regularly to apply this field (handled automatically if using TI RTOS!)."]
    #[inline(always)]
    pub fn dis_alt_dcdc_setting(&mut self) -> DIS_ALT_DCDC_SETTING_W {
        DIS_ALT_DCDC_SETTING_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Disable XOSC override functionality. 0: Enable XOSC override functionality. 1: Disable XOSC override functionality. See: MODE_CONF_1.DELTA_IBIAS_INIT MODE_CONF_1.DELTA_IBIAS_OFFSET MODE_CONF_1.XOSC_MAX_START"]
    #[inline(always)]
    pub fn dis_xosc_ovr(&mut self) -> DIS_XOSC_OVR_W {
        DIS_XOSC_OVR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCFG Size and Disable Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [size_and_dis_flags](index.html) module"]
pub struct SIZE_AND_DIS_FLAGS_SPEC;
impl crate::RegisterSpec for SIZE_AND_DIS_FLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [size_and_dis_flags::R](R) reader structure"]
impl crate::Readable for SIZE_AND_DIS_FLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [size_and_dis_flags::W](W) writer structure"]
impl crate::Writable for SIZE_AND_DIS_FLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIZE_AND_DIS_FLAGS to value 0xffff_ffff"]
impl crate::Resettable for SIZE_AND_DIS_FLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
