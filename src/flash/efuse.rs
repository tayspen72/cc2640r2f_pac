#[doc = "Register `EFUSE` reader"]
pub struct R(crate::R<EFUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSE` writer"]
pub struct W(crate::W<EFUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSE_SPEC>;
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
impl From<crate::W<EFUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED29` reader - 31:29\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED29_R(crate::FieldReader<u8, u8>);
impl RESERVED29_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED29_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED29` writer - 31:29\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED29_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
#[doc = "Field `INSTRUCTION` reader - 28:24\\]
Internal. Only to be used through TI provided API."]
pub struct INSTRUCTION_R(crate::FieldReader<u8, u8>);
impl INSTRUCTION_R {
    pub(crate) fn new(bits: u8) -> Self {
        INSTRUCTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INSTRUCTION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INSTRUCTION` writer - 28:24\\]
Internal. Only to be used through TI provided API."]
pub struct INSTRUCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTRUCTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `RESERVED16` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED16_R(crate::FieldReader<u8, u8>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DUMPWORD` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct DUMPWORD_R(crate::FieldReader<u16, u16>);
impl DUMPWORD_R {
    pub(crate) fn new(bits: u16) -> Self {
        DUMPWORD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUMPWORD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUMPWORD` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct DUMPWORD_W<'a> {
    w: &'a mut W,
}
impl<'a> DUMPWORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn instruction(&self) -> INSTRUCTION_R {
        INSTRUCTION_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dumpword(&self) -> DUMPWORD_R {
        DUMPWORD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved29(&mut self) -> RESERVED29_W {
        RESERVED29_W { w: self }
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W {
        INSTRUCTION_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dumpword(&mut self) -> DUMPWORD_W {
        DUMPWORD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuse](index.html) module"]
pub struct EFUSE_SPEC;
impl crate::RegisterSpec for EFUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuse::R](R) reader structure"]
impl crate::Readable for EFUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuse::W](W) writer structure"]
impl crate::Writable for EFUSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EFUSE to value 0"]
impl crate::Resettable for EFUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
