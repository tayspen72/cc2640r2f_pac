#[doc = "Register `CPUID` reader"]
pub struct R(crate::R<CPUID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUID` writer"]
pub struct W(crate::W<CPUID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUID_SPEC>;
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
impl From<crate::W<CPUID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IMPLEMENTER` reader - 31:24\\]
Implementor code."]
pub struct IMPLEMENTER_R(crate::FieldReader<u8, u8>);
impl IMPLEMENTER_R {
    pub(crate) fn new(bits: u8) -> Self {
        IMPLEMENTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IMPLEMENTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMPLEMENTER` writer - 31:24\\]
Implementor code."]
pub struct IMPLEMENTER_W<'a> {
    w: &'a mut W,
}
impl<'a> IMPLEMENTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `VARIANT` reader - 23:20\\]
Implementation defined variant number."]
pub struct VARIANT_R(crate::FieldReader<u8, u8>);
impl VARIANT_R {
    pub(crate) fn new(bits: u8) -> Self {
        VARIANT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VARIANT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VARIANT` writer - 23:20\\]
Implementation defined variant number."]
pub struct VARIANT_W<'a> {
    w: &'a mut W,
}
impl<'a> VARIANT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `CONSTANT` reader - 19:16\\]
Reads as 0xF"]
pub struct CONSTANT_R(crate::FieldReader<u8, u8>);
impl CONSTANT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONSTANT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONSTANT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONSTANT` writer - 19:16\\]
Reads as 0xF"]
pub struct CONSTANT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSTANT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `PARTNO` reader - 15:4\\]
Number of processor within family."]
pub struct PARTNO_R(crate::FieldReader<u16, u16>);
impl PARTNO_R {
    pub(crate) fn new(bits: u16) -> Self {
        PARTNO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARTNO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARTNO` writer - 15:4\\]
Number of processor within family."]
pub struct PARTNO_W<'a> {
    w: &'a mut W,
}
impl<'a> PARTNO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | ((value as u32 & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Field `REVISION` reader - 3:0\\]
Implementation defined revision number."]
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVISION` writer - 3:0\\]
Implementation defined revision number."]
pub struct REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Implementor code."]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Implementation defined variant number."]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reads as 0xF"]
    #[inline(always)]
    pub fn constant(&self) -> CONSTANT_R {
        CONSTANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Number of processor within family."]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Implementation defined revision number."]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Implementor code."]
    #[inline(always)]
    pub fn implementer(&mut self) -> IMPLEMENTER_W {
        IMPLEMENTER_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Implementation defined variant number."]
    #[inline(always)]
    pub fn variant(&mut self) -> VARIANT_W {
        VARIANT_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reads as 0xF"]
    #[inline(always)]
    pub fn constant(&mut self) -> CONSTANT_W {
        CONSTANT_W { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\]
Number of processor within family."]
    #[inline(always)]
    pub fn partno(&mut self) -> PARTNO_W {
        PARTNO_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Implementation defined revision number."]
    #[inline(always)]
    pub fn revision(&mut self) -> REVISION_W {
        REVISION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuid](index.html) module"]
pub struct CPUID_SPEC;
impl crate::RegisterSpec for CPUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuid::R](R) reader structure"]
impl crate::Readable for CPUID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuid::W](W) writer structure"]
impl crate::Writable for CPUID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPUID to value 0x412f_c231"]
impl crate::Resettable for CPUID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x412f_c231
    }
}
