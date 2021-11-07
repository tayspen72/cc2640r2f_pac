#[doc = "Register `ALARMCNT` reader"]
pub struct R(crate::R<ALARMCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARMCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARMCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARMCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARMCNT` writer"]
pub struct W(crate::W<ALARMCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARMCNT_SPEC>;
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
impl From<crate::W<ALARMCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARMCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED30_R(crate::FieldReader<u8, u8>);
impl RESERVED30_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED30_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED30_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `SHUTDOWN_CNT` reader - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
pub struct SHUTDOWN_CNT_R(crate::FieldReader<u8, u8>);
impl SHUTDOWN_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SHUTDOWN_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHUTDOWN_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHUTDOWN_CNT` writer - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
pub struct SHUTDOWN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHUTDOWN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `RESERVED21` reader - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED21_R(crate::FieldReader<u8, u8>);
impl RESERVED21_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED21_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED21` writer - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED21_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `SHUTDOWN_THR` reader - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
pub struct SHUTDOWN_THR_R(crate::FieldReader<u8, u8>);
impl SHUTDOWN_THR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SHUTDOWN_THR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHUTDOWN_THR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHUTDOWN_THR` writer - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
pub struct SHUTDOWN_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> SHUTDOWN_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED8` reader - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_R(crate::FieldReader<u8, u8>);
impl RESERVED8_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED8` writer - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ALARM_THR` reader - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
pub struct ALARM_THR_R(crate::FieldReader<u8, u8>);
impl ALARM_THR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ALARM_THR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM_THR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM_THR` writer - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
pub struct ALARM_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
    #[inline(always)]
    pub fn shutdown_cnt(&self) -> SHUTDOWN_CNT_R {
        SHUTDOWN_CNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&self) -> RESERVED21_R {
        RESERVED21_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
    #[inline(always)]
    pub fn shutdown_thr(&self) -> SHUTDOWN_THR_R {
        SHUTDOWN_THR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
    #[inline(always)]
    pub fn alarm_thr(&self) -> ALARM_THR_R {
        ALARM_THR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&mut self) -> RESERVED30_W {
        RESERVED30_W { w: self }
    }
    #[doc = "Bits 24:29 - 29:24\\]
Read-only, indicates the number of '1' bits in ALARMSTOP register. The maximum value equals the number of FROs."]
    #[inline(always)]
    pub fn shutdown_cnt(&mut self) -> SHUTDOWN_CNT_W {
        SHUTDOWN_CNT_W { w: self }
    }
    #[doc = "Bits 21:23 - 23:21\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved21(&mut self) -> RESERVED21_W {
        RESERVED21_W { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\]
Threshold setting for generating IRQFLAGSTAT.SHUTDOWN_OVF interrupt. The interrupt is triggered when SHUTDOWN_CNT value exceeds this bit field."]
    #[inline(always)]
    pub fn shutdown_thr(&mut self) -> SHUTDOWN_THR_W {
        SHUTDOWN_THR_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Alarm detection threshold for the repeating pattern detectors on each FRO. An FRO 'alarm event' is declared when a repeating pattern (of up to four samples length) is detected continuously for the number of samples defined by this field's value. Reset value 0xFF should keep the number of 'alarm events' to a manageable level."]
    #[inline(always)]
    pub fn alarm_thr(&mut self) -> ALARM_THR_W {
        ALARM_THR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarmcnt](index.html) module"]
pub struct ALARMCNT_SPEC;
impl crate::RegisterSpec for ALARMCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarmcnt::R](R) reader structure"]
impl crate::Readable for ALARMCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarmcnt::W](W) writer structure"]
impl crate::Writable for ALARMCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALARMCNT to value 0xff"]
impl crate::Resettable for ALARMCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
