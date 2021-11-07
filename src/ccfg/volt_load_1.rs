#[doc = "Register `VOLT_LOAD_1` reader"]
pub struct R(crate::R<VOLT_LOAD_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOLT_LOAD_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOLT_LOAD_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOLT_LOAD_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOLT_LOAD_1` writer"]
pub struct W(crate::W<VOLT_LOAD_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOLT_LOAD_1_SPEC>;
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
impl From<crate::W<VOLT_LOAD_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOLT_LOAD_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDR_EXT_TP125` reader - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct VDDR_EXT_TP125_R(crate::FieldReader<u8, u8>);
impl VDDR_EXT_TP125_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_EXT_TP125_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_EXT_TP125_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_EXT_TP125` writer - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct VDDR_EXT_TP125_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TP125_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `VDDR_EXT_TP105` reader - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct VDDR_EXT_TP105_R(crate::FieldReader<u8, u8>);
impl VDDR_EXT_TP105_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_EXT_TP105_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_EXT_TP105_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_EXT_TP105` writer - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct VDDR_EXT_TP105_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TP105_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `VDDR_EXT_TP85` reader - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct VDDR_EXT_TP85_R(crate::FieldReader<u8, u8>);
impl VDDR_EXT_TP85_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_EXT_TP85_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_EXT_TP85_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_EXT_TP85` writer - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct VDDR_EXT_TP85_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TP85_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `VDDR_EXT_TP65` reader - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct VDDR_EXT_TP65_R(crate::FieldReader<u8, u8>);
impl VDDR_EXT_TP65_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_EXT_TP65_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_EXT_TP65_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_EXT_TP65` writer - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub struct VDDR_EXT_TP65_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_EXT_TP65_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp125(&self) -> VDDR_EXT_TP125_R {
        VDDR_EXT_TP125_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp105(&self) -> VDDR_EXT_TP105_R {
        VDDR_EXT_TP105_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp85(&self) -> VDDR_EXT_TP85_R {
        VDDR_EXT_TP85_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp65(&self) -> VDDR_EXT_TP65_R {
        VDDR_EXT_TP65_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp125(&mut self) -> VDDR_EXT_TP125_W {
        VDDR_EXT_TP125_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp105(&mut self) -> VDDR_EXT_TP105_W {
        VDDR_EXT_TP105_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp85(&mut self) -> VDDR_EXT_TP85_W {
        VDDR_EXT_TP85_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp65(&mut self) -> VDDR_EXT_TP65_W {
        VDDR_EXT_TP65_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [volt_load_1](index.html) module"]
pub struct VOLT_LOAD_1_SPEC;
impl crate::RegisterSpec for VOLT_LOAD_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [volt_load_1::R](R) reader structure"]
impl crate::Readable for VOLT_LOAD_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [volt_load_1::W](W) writer structure"]
impl crate::Writable for VOLT_LOAD_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VOLT_LOAD_1 to value 0xffff_ffff"]
impl crate::Resettable for VOLT_LOAD_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
