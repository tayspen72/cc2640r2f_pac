#[doc = "Register `TBR` reader"]
pub struct R(crate::R<TBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBR` writer"]
pub struct W(crate::W<TBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBR_SPEC>;
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
impl From<crate::W<TBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBR` reader - 31:0\\]
GPT Timer B Register Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer B Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
pub struct TBR_R(crate::FieldReader<u32, u32>);
impl TBR_R {
    pub(crate) fn new(bits: u32) -> Self {
        TBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBR` writer - 31:0\\]
GPT Timer B Register Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer B Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
pub struct TBR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Register Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer B Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    pub fn tbr(&self) -> TBR_R {
        TBR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Register Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBILR register either on the next cycle or on the next timeout. A read returns the current value of the Timer B Count Register, in all cases except for Input Edge count and Timer modes. In the Input Edge Count Mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place."]
    #[inline(always)]
    pub fn tbr(&mut self) -> TBR_W {
        TBR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer B Register This register shows the current value of the Timer B counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler in Input Edge Count, Input Edge Time, and PWM modes, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TBV register. To read the value of the prescalar in periodic snapshot mode, read the Timer B Prescale Snapshot (TBPS) register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbr](index.html) module"]
pub struct TBR_SPEC;
impl crate::RegisterSpec for TBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbr::R](R) reader structure"]
impl crate::Readable for TBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbr::W](W) writer structure"]
impl crate::Writable for TBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBR to value 0xffff"]
impl crate::Resettable for TBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
