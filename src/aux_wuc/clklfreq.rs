#[doc = "Register `CLKLFREQ` reader"]
pub struct R(crate::R<CLKLFREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKLFREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKLFREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKLFREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKLFREQ` writer"]
pub struct W(crate::W<CLKLFREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKLFREQ_SPEC>;
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
impl From<crate::W<CLKLFREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKLFREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ` reader - 0:0\\]
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
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
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
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
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Low frequency request 0: Request clock frequency to be controlled by AON_WUC:AUXCLK and the system state 1: Request low frequency clock SCLK_LF as the clock source for AUX This bit must not be modified unless CLKLFACK.ACK matches the current value"]
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
#[doc = "Low Frequency Clock Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clklfreq](index.html) module"]
pub struct CLKLFREQ_SPEC;
impl crate::RegisterSpec for CLKLFREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clklfreq::R](R) reader structure"]
impl crate::Readable for CLKLFREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clklfreq::W](W) writer structure"]
impl crate::Writable for CLKLFREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKLFREQ to value 0"]
impl crate::Resettable for CLKLFREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
