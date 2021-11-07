#[doc = "Register `TRIGCNT` reader"]
pub struct R(crate::R<TRIGCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIGCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIGCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIGCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIGCNT` writer"]
pub struct W(crate::W<TRIGCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIGCNT_SPEC>;
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
impl From<crate::W<TRIGCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIGCNT_SPEC>) -> Self {
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
#[doc = "Field `CNT` reader - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
pub struct CNT_R(crate::FieldReader<u16, u16>);
impl CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT` writer - 15:0\\]
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
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
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
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
Number of stop events to ignore when AUX_TDC:TRIGCNTCFG.EN is 1. Read CNT to get the remaining number of stop events to ignore during a TDC measurement. Write CNT to update the remaining number of stop events to ignore during a TDC measurement. The TDC measurement ignores updates of CNT if there are no more stop events left to ignore. When AUX_TDC:TRIGCNTCFG.EN is 1, TRIGCNTLOAD.CNT is loaded into CNT at the start of the measurement."]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Counter Stop-counter control and status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigcnt](index.html) module"]
pub struct TRIGCNT_SPEC;
impl crate::RegisterSpec for TRIGCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trigcnt::R](R) reader structure"]
impl crate::Readable for TRIGCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trigcnt::W](W) writer structure"]
impl crate::Writable for TRIGCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIGCNT to value 0"]
impl crate::Resettable for TRIGCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
