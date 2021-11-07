#[doc = "Register `PWRDWNREQ` reader"]
pub struct R(crate::R<PWRDWNREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRDWNREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRDWNREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRDWNREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRDWNREQ` writer"]
pub struct W(crate::W<PWRDWNREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRDWNREQ_SPEC>;
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
impl From<crate::W<PWRDWNREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRDWNREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ` reader - 0:0\\]
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
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
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
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
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Power down request 0: Request for system to be in active mode 1: Request for system to be in power down mode When REQ is 1 one shall assume that the system is in power down, and that current supply is limited. When setting REQ = 0, one shall assume that the system is in power down until PWRDWNACK.ACK = 0"]
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
#[doc = "Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrdwnreq](index.html) module"]
pub struct PWRDWNREQ_SPEC;
impl crate::RegisterSpec for PWRDWNREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrdwnreq::R](R) reader structure"]
impl crate::Readable for PWRDWNREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrdwnreq::W](W) writer structure"]
impl crate::Writable for PWRDWNREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRDWNREQ to value 0"]
impl crate::Resettable for PWRDWNREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
