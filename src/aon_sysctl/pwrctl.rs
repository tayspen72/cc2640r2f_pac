#[doc = "Register `PWRCTL` reader"]
pub struct R(crate::R<PWRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCTL` writer"]
pub struct W(crate::W<PWRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCTL_SPEC>;
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
impl From<crate::W<PWRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCTL_SPEC>) -> Self {
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
#[doc = "Field `DCDC_ACTIVE` reader - 2:2\\]
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDRin active mode. 1: Use DCDC for regulation of VDDRin active mode."]
pub struct DCDC_ACTIVE_R(crate::FieldReader<bool, bool>);
impl DCDC_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_ACTIVE` writer - 2:2\\]
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDRin active mode. 1: Use DCDC for regulation of VDDRin active mode."]
pub struct DCDC_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_ACTIVE_W<'a> {
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
#[doc = "Field `EXT_REG_MODE` reader - 1:1\\]
Status of source for VDDRsupply: 0: DCDC/GLDO are generating VDDR 1: DCDC/GLDO are bypassed, external regulator supplies VDDR"]
pub struct EXT_REG_MODE_R(crate::FieldReader<bool, bool>);
impl EXT_REG_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXT_REG_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_REG_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXT_REG_MODE` writer - 1:1\\]
Status of source for VDDRsupply: 0: DCDC/GLDO are generating VDDR 1: DCDC/GLDO are bypassed, external regulator supplies VDDR"]
pub struct EXT_REG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_REG_MODE_W<'a> {
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
#[doc = "Field `DCDC_EN` reader - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
pub struct DCDC_EN_R(crate::FieldReader<bool, bool>);
impl DCDC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_EN` writer - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
pub struct DCDC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_EN_W<'a> {
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
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDRin active mode. 1: Use DCDC for regulation of VDDRin active mode."]
    #[inline(always)]
    pub fn dcdc_active(&self) -> DCDC_ACTIVE_R {
        DCDC_ACTIVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of source for VDDRsupply: 0: DCDC/GLDO are generating VDDR 1: DCDC/GLDO are bypassed, external regulator supplies VDDR"]
    #[inline(always)]
    pub fn ext_reg_mode(&self) -> EXT_REG_MODE_R {
        EXT_REG_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
    #[inline(always)]
    pub fn dcdc_en(&self) -> DCDC_EN_R {
        DCDC_EN_R::new((self.bits & 0x01) != 0)
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
Select to use DCDC regulator for VDDR in active mode 0: Use GLDO for regulation of VDDRin active mode. 1: Use DCDC for regulation of VDDRin active mode."]
    #[inline(always)]
    pub fn dcdc_active(&mut self) -> DCDC_ACTIVE_W {
        DCDC_ACTIVE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Status of source for VDDRsupply: 0: DCDC/GLDO are generating VDDR 1: DCDC/GLDO are bypassed, external regulator supplies VDDR"]
    #[inline(always)]
    pub fn ext_reg_mode(&mut self) -> EXT_REG_MODE_W {
        EXT_REG_MODE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Select to use DCDC regulator during recharge of VDDR 0: Use GLDO for recharge of VDDR 1: Use DCDC for recharge of VDDR Note: This bitfield should be set to the same as DCDC_ACTIVE"]
    #[inline(always)]
    pub fn dcdc_en(&mut self) -> DCDC_EN_W {
        DCDC_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrctl](index.html) module"]
pub struct PWRCTL_SPEC;
impl crate::RegisterSpec for PWRCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrctl::R](R) reader structure"]
impl crate::Readable for PWRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrctl::W](W) writer structure"]
impl crate::Writable for PWRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRCTL to value 0"]
impl crate::Resettable for PWRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
