#[doc = "Register `VDCTL` reader"]
pub struct R(crate::R<VDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDCTL` writer"]
pub struct W(crate::W<VDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDCTL_SPEC>;
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
impl From<crate::W<VDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u32, u32>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Field `MCU_VD` reader - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub struct MCU_VD_R(crate::FieldReader<bool, bool>);
impl MCU_VD_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCU_VD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_VD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_VD` writer - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub struct MCU_VD_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_VD_W<'a> {
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
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_R(crate::FieldReader<bool, bool>);
impl RESERVED1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
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
#[doc = "Field `ULDO` reader - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub struct ULDO_R(crate::FieldReader<bool, bool>);
impl ULDO_R {
    pub(crate) fn new(bits: bool) -> Self {
        ULDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULDO` writer - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
pub struct ULDO_W<'a> {
    w: &'a mut W,
}
impl<'a> ULDO_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    pub fn mcu_vd(&self) -> MCU_VD_R {
        MCU_VD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    pub fn uldo(&self) -> ULDO_R {
        ULDO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Request WUC to power down the MCU voltage domain 0: No request 1: Assert request when possible. An asserted power down request will result in a boot of the MCU system when powered up again. The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    pub fn mcu_vd(&mut self) -> MCU_VD_W {
        MCU_VD_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Request WUC to switch to uLDO. 0: No request 1: Assert request when possible The bit will have no effect before the following requirements are met: 1. PDCTL1.CPU_ON = 0 2. PDCTL1.VIMS_MODE = 0 3. SECDMACLKGDS.DMA_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 4. SECDMACLKGDS.CRYPTO_CLK_EN = 0 (Note: Setting must be loaded with CLKLOADCTL.LOAD) 5. RFC do no request access to BUS 6. System CPU in deepsleep"]
    #[inline(always)]
    pub fn uldo(&mut self) -> ULDO_W {
        ULDO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCU Voltage Domain Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdctl](index.html) module"]
pub struct VDCTL_SPEC;
impl crate::RegisterSpec for VDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdctl::R](R) reader structure"]
impl crate::Readable for VDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdctl::W](W) writer structure"]
impl crate::Writable for VDCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VDCTL to value 0"]
impl crate::Resettable for VDCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
