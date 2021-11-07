#[doc = "Register `AONCTLSTAT` reader"]
pub struct R(crate::R<AONCTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AONCTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AONCTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AONCTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AONCTLSTAT` writer"]
pub struct W(crate::W<AONCTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AONCTLSTAT_SPEC>;
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
impl From<crate::W<AONCTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AONCTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUX_FORCE_ON` reader - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
pub struct AUX_FORCE_ON_R(crate::FieldReader<bool, bool>);
impl AUX_FORCE_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUX_FORCE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUX_FORCE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUX_FORCE_ON` writer - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
pub struct AUX_FORCE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> AUX_FORCE_ON_W<'a> {
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
#[doc = "Field `SCE_RUN_EN` reader - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
pub struct SCE_RUN_EN_R(crate::FieldReader<bool, bool>);
impl SCE_RUN_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCE_RUN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCE_RUN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCE_RUN_EN` writer - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
pub struct SCE_RUN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCE_RUN_EN_W<'a> {
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
    #[doc = "Bit 1 - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
    #[inline(always)]
    pub fn aux_force_on(&self) -> AUX_FORCE_ON_R {
        AUX_FORCE_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
    #[inline(always)]
    pub fn sce_run_en(&self) -> SCE_RUN_EN_R {
        SCE_RUN_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
    #[inline(always)]
    pub fn aux_force_on(&mut self) -> AUX_FORCE_ON_W {
        AUX_FORCE_ON_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
    #[inline(always)]
    pub fn sce_run_en(&mut self) -> SCE_RUN_EN_W {
        SCE_RUN_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AON Domain Control Status Status of AUX domain control from AON_WUC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aonctlstat](index.html) module"]
pub struct AONCTLSTAT_SPEC;
impl crate::RegisterSpec for AONCTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aonctlstat::R](R) reader structure"]
impl crate::Readable for AONCTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aonctlstat::W](W) writer structure"]
impl crate::Writable for AONCTLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AONCTLSTAT to value 0"]
impl crate::Resettable for AONCTLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
