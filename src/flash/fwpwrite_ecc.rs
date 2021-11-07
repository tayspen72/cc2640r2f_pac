#[doc = "Register `FWPWRITE_ECC` reader"]
pub struct R(crate::R<FWPWRITE_ECC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWPWRITE_ECC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWPWRITE_ECC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWPWRITE_ECC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWPWRITE_ECC` writer"]
pub struct W(crate::W<FWPWRITE_ECC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWPWRITE_ECC_SPEC>;
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
impl From<crate::W<FWPWRITE_ECC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWPWRITE_ECC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECCBYTES07_00` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct ECCBYTES07_00_R(crate::FieldReader<u8, u8>);
impl ECCBYTES07_00_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECCBYTES07_00_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCBYTES07_00_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCBYTES07_00` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub struct ECCBYTES07_00_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCBYTES07_00_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `ECCBYTES15_08` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct ECCBYTES15_08_R(crate::FieldReader<u8, u8>);
impl ECCBYTES15_08_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECCBYTES15_08_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCBYTES15_08_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCBYTES15_08` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub struct ECCBYTES15_08_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCBYTES15_08_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `ECCBYTES23_16` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct ECCBYTES23_16_R(crate::FieldReader<u8, u8>);
impl ECCBYTES23_16_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECCBYTES23_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCBYTES23_16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCBYTES23_16` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct ECCBYTES23_16_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCBYTES23_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ECCBYTES31_24` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct ECCBYTES31_24_R(crate::FieldReader<u8, u8>);
impl ECCBYTES31_24_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECCBYTES31_24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECCBYTES31_24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECCBYTES31_24` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct ECCBYTES31_24_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCBYTES31_24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes07_00(&self) -> ECCBYTES07_00_R {
        ECCBYTES07_00_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes15_08(&self) -> ECCBYTES15_08_R {
        ECCBYTES15_08_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes23_16(&self) -> ECCBYTES23_16_R {
        ECCBYTES23_16_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes31_24(&self) -> ECCBYTES31_24_R {
        ECCBYTES31_24_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes07_00(&mut self) -> ECCBYTES07_00_W {
        ECCBYTES07_00_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes15_08(&mut self) -> ECCBYTES15_08_W {
        ECCBYTES15_08_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes23_16(&mut self) -> ECCBYTES23_16_W {
        ECCBYTES23_16_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes31_24(&mut self) -> ECCBYTES31_24_W {
        ECCBYTES31_24_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite_ecc](index.html) module"]
pub struct FWPWRITE_ECC_SPEC;
impl crate::RegisterSpec for FWPWRITE_ECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwpwrite_ecc::R](R) reader structure"]
impl crate::Readable for FWPWRITE_ECC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwpwrite_ecc::W](W) writer structure"]
impl crate::Writable for FWPWRITE_ECC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FWPWRITE_ECC to value 0xffff_ffff"]
impl crate::Resettable for FWPWRITE_ECC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
