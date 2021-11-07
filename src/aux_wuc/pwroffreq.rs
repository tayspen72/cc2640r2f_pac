#[doc = "Register `PWROFFREQ` reader"]
pub struct R(crate::R<PWROFFREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWROFFREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWROFFREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWROFFREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWROFFREQ` writer"]
pub struct W(crate::W<PWROFFREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWROFFREQ_SPEC>;
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
impl From<crate::W<PWROFFREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWROFFREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ` reader - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
pub struct REQ_R(crate::FieldReader<bool, bool>);
impl REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REQ` writer - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
pub struct REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_W<'a> {
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
    #[doc = "Bit 0 - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power off request 0: No action 1: Request to power down AUX. Once set, this bit shall not be cleared. The bit will be reset again when AUX is powered up again. The request will only happen if AONCTLSTAT.AUX_FORCE_ON = 0 and MCUBUSSTAT.DISCONNECTED=1."]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W {
        REQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Off Request Requests power off request for the AUX domain. When powered off, the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwroffreq](index.html) module"]
pub struct PWROFFREQ_SPEC;
impl crate::RegisterSpec for PWROFFREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwroffreq::R](R) reader structure"]
impl crate::Readable for PWROFFREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwroffreq::W](W) writer structure"]
impl crate::Writable for PWROFFREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWROFFREQ to value 0"]
impl crate::Resettable for PWROFFREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
