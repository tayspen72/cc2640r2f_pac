#[doc = "Register `ALARMSTOP` reader"]
pub struct R(crate::R<ALARMSTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALARMSTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALARMSTOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALARMSTOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALARMSTOP` writer"]
pub struct W(crate::W<ALARMSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALARMSTOP_SPEC>;
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
impl From<crate::W<ALARMSTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALARMSTOP_SPEC>) -> Self {
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
#[doc = "Field `FRO_FLAGS` reader - 23:0\\]
Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\]
indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
pub struct FRO_FLAGS_R(crate::FieldReader<u32, u32>);
impl FRO_FLAGS_R {
    pub(crate) fn new(bits: u32) -> Self {
        FRO_FLAGS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRO_FLAGS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRO_FLAGS` writer - 23:0\\]
Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\]
indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
pub struct FRO_FLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO_FLAGS_W<'a> {
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
Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\]
indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
    #[inline(always)]
    pub fn fro_flags(&self) -> FRO_FLAGS_R {
        FRO_FLAGS_R::new((self.bits & 0x00ff_ffff) as u32)
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
Logging bits for the 'alarm events' of individual FROs. A '1' in bit \\[n\\]
indicates FRO 'n' experienced more than one 'alarm event' in quick succession and has been turned off. A '1' in this field forces the corresponding bit in FROEN.FRO_MASK to '0'."]
    #[inline(always)]
    pub fn fro_flags(&mut self) -> FRO_FLAGS_W {
        FRO_FLAGS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alarm Shutdown\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alarmstop](index.html) module"]
pub struct ALARMSTOP_SPEC;
impl crate::RegisterSpec for ALARMSTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alarmstop::R](R) reader structure"]
impl crate::Readable for ALARMSTOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alarmstop::W](W) writer structure"]
impl crate::Writable for ALARMSTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALARMSTOP to value 0"]
impl crate::Resettable for ALARMSTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
