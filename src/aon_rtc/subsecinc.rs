#[doc = "Register `SUBSECINC` reader"]
pub struct R(crate::R<SUBSECINC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBSECINC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBSECINC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBSECINC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBSECINC` writer"]
pub struct W(crate::W<SUBSECINC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBSECINC_SPEC>;
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
impl From<crate::W<SUBSECINC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBSECINC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED24_R(crate::FieldReader<u8, u8>);
impl RESERVED24_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `VALUEINC` reader - 23:0\\]
This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\]
are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\]
are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_WUC:RTCSUBSECINC1 , AUX_WUC:RTCSUBSECINC0 and AUX_WUC:RTCSUBSECINCCTL"]
pub struct VALUEINC_R(crate::FieldReader<u32, u32>);
impl VALUEINC_R {
    pub(crate) fn new(bits: u32) -> Self {
        VALUEINC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUEINC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALUEINC` writer - 23:0\\]
This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\]
are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\]
are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_WUC:RTCSUBSECINC1 , AUX_WUC:RTCSUBSECINC0 and AUX_WUC:RTCSUBSECINCCTL"]
pub struct VALUEINC_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUEINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:23 - 23:0\\]
This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\]
are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\]
are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_WUC:RTCSUBSECINC1 , AUX_WUC:RTCSUBSECINC0 and AUX_WUC:RTCSUBSECINCCTL"]
    #[inline(always)]
    pub fn valueinc(&self) -> VALUEINC_R {
        VALUEINC_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
This value compensates for a SCLK_LF clock which has an offset from 32768 Hz. The compensation value can be found as 2^38 / freq, where freq is SCLK_LF clock frequency in Hertz This value is added to SUBSEC.VALUE on every cycle, and carry of this is added to SEC.VALUE. To perform the addition, bits \\[23:6\\]
are aligned with SUBSEC.VALUE bits \\[17:0\\]. The remaining bits \\[5:0\\]
are accumulated in a hidden 6-bit register that generates a carry into the above mentioned addition on overflow. The default value corresponds to incrementing by precisely 1/32768 of a second. NOTE: This register is read only. Modification of the register value must be done using registers AUX_WUC:RTCSUBSECINC1 , AUX_WUC:RTCSUBSECINC0 and AUX_WUC:RTCSUBSECINCCTL"]
    #[inline(always)]
    pub fn valueinc(&mut self) -> VALUEINC_W {
        VALUEINC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [subsecinc](index.html) module"]
pub struct SUBSECINC_SPEC;
impl crate::RegisterSpec for SUBSECINC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [subsecinc::R](R) reader structure"]
impl crate::Readable for SUBSECINC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [subsecinc::W](W) writer structure"]
impl crate::Writable for SUBSECINC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUBSECINC to value 0x0080_0000"]
impl crate::Resettable for SUBSECINC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0080_0000
    }
}
