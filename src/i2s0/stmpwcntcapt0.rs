#[doc = "Register `STMPWCNTCAPT0` reader"]
pub struct R(crate::R<STMPWCNTCAPT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STMPWCNTCAPT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STMPWCNTCAPT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STMPWCNTCAPT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STMPWCNTCAPT0` writer"]
pub struct W(crate::W<STMPWCNTCAPT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STMPWCNTCAPT0_SPEC>;
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
impl From<crate::W<STMPWCNTCAPT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STMPWCNTCAPT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_R(crate::FieldReader<u16, u16>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CAPT_VALUE` reader - 15:0\\]
The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
pub struct CAPT_VALUE_R(crate::FieldReader<u16, u16>);
impl CAPT_VALUE_R {
    pub(crate) fn new(bits: u16) -> Self {
        CAPT_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPT_VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPT_VALUE` writer - 15:0\\]
The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
pub struct CAPT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[inline(always)]
    pub fn capt_value(&self) -> CAPT_VALUE_R {
        CAPT_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
The value of the samplestamp WCLK counter (STMPWCNT.CURR_VALUE) last time an event was pulsed (event source selected in EVENT:I2SSTMPSEL0.EV for channel 0). This number corresponds to the number of positive WCLK edges since the samplestamp generator was enabled (not taking modification through STMPWADD/STMPWSET into account). The value is cleared when STMPCTL.STMP_EN = 0."]
    #[inline(always)]
    pub fn capt_value(&mut self) -> CAPT_VALUE_W {
        CAPT_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Captured WCLK Counter Value, Capture Channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmpwcntcapt0](index.html) module"]
pub struct STMPWCNTCAPT0_SPEC;
impl crate::RegisterSpec for STMPWCNTCAPT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stmpwcntcapt0::R](R) reader structure"]
impl crate::Readable for STMPWCNTCAPT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stmpwcntcapt0::W](W) writer structure"]
impl crate::Writable for STMPWCNTCAPT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STMPWCNTCAPT0 to value 0"]
impl crate::Resettable for STMPWCNTCAPT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}