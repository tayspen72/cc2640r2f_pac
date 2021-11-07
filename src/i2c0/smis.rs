#[doc = "Register `SMIS` reader"]
pub struct R(crate::R<SMIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMIS` writer"]
pub struct W(crate::W<SMIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMIS_SPEC>;
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
impl From<crate::W<SMIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_R(crate::FieldReader<u32, u32>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Field `STOPMIS` reader - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
pub struct STOPMIS_R(crate::FieldReader<bool, bool>);
impl STOPMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOPMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPMIS` writer - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
pub struct STOPMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `STARTMIS` reader - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
pub struct STARTMIS_R(crate::FieldReader<bool, bool>);
impl STARTMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STARTMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STARTMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTMIS` writer - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
pub struct STARTMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DATAMIS` reader - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
pub struct DATAMIS_R(crate::FieldReader<bool, bool>);
impl DATAMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATAMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAMIS` writer - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
pub struct DATAMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAMIS_W<'a> {
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
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
    #[inline(always)]
    pub fn stopmis(&self) -> STOPMIS_R {
        STOPMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
    #[inline(always)]
    pub fn startmis(&self) -> STARTMIS_R {
        STARTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[inline(always)]
    pub fn datamis(&self) -> DATAMIS_R {
        DATAMIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
    #[inline(always)]
    pub fn stopmis(&mut self) -> STOPMIS_W {
        STOPMIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
    #[inline(always)]
    pub fn startmis(&mut self) -> STARTMIS_W {
        STARTMIS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[inline(always)]
    pub fn datamis(&mut self) -> DATAMIS_W {
        DATAMIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smis](index.html) module"]
pub struct SMIS_SPEC;
impl crate::RegisterSpec for SMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smis::R](R) reader structure"]
impl crate::Readable for SMIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smis::W](W) writer structure"]
impl crate::Writable for SMIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMIS to value 0"]
impl crate::Resettable for SMIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
