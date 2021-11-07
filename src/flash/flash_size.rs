#[doc = "Register `FLASH_SIZE` reader"]
pub struct R(crate::R<FLASH_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_SIZE` writer"]
pub struct W(crate::W<FLASH_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SIZE_SPEC>;
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
impl From<crate::W<FLASH_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED8_R(crate::FieldReader<u32, u32>);
impl RESERVED8_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED8_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED8` writer - 31:8\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Field `SECTORS` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct SECTORS_R(crate::FieldReader<u8, u8>);
impl SECTORS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SECTORS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECTORS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECTORS` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct SECTORS_W<'a> {
    w: &'a mut W,
}
impl<'a> SECTORS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sectors(&self) -> SECTORS_R {
        SECTORS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sectors(&mut self) -> SECTORS_W {
        SECTORS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_size](index.html) module"]
pub struct FLASH_SIZE_SPEC;
impl crate::RegisterSpec for FLASH_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_size::R](R) reader structure"]
impl crate::Readable for FLASH_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_size::W](W) writer structure"]
impl crate::Writable for FLASH_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_SIZE to value 0"]
impl crate::Resettable for FLASH_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
