#[doc = "Register `RTCSUBSECINC1` reader"]
pub struct R(crate::R<RTCSUBSECINC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCSUBSECINC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCSUBSECINC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCSUBSECINC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCSUBSECINC1` writer"]
pub struct W(crate::W<RTCSUBSECINC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCSUBSECINC1_SPEC>;
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
impl From<crate::W<RTCSUBSECINC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCSUBSECINC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INC23_16` reader - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
pub struct INC23_16_R(crate::FieldReader<u8, u8>);
impl INC23_16_R {
    pub(crate) fn new(bits: u8) -> Self {
        INC23_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC23_16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INC23_16` writer - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
pub struct INC23_16_W<'a> {
    w: &'a mut W,
}
impl<'a> INC23_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
    #[inline(always)]
    pub fn inc23_16(&self) -> INC23_16_R {
        INC23_16_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Bits 23:16 of the RTC sub-second increment value."]
    #[inline(always)]
    pub fn inc23_16(&mut self) -> INC23_16_W {
        INC23_16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsubsecinc1](index.html) module"]
pub struct RTCSUBSECINC1_SPEC;
impl crate::RegisterSpec for RTCSUBSECINC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcsubsecinc1::R](R) reader structure"]
impl crate::Readable for RTCSUBSECINC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcsubsecinc1::W](W) writer structure"]
impl crate::Writable for RTCSUBSECINC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCSUBSECINC1 to value 0"]
impl crate::Resettable for RTCSUBSECINC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
