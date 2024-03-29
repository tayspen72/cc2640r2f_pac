#[doc = "Register `AESAUTHLEN` reader"]
pub struct R(crate::R<AESAUTHLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESAUTHLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESAUTHLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESAUTHLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESAUTHLEN` writer"]
pub struct W(crate::W<AESAUTHLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESAUTHLEN_SPEC>;
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
impl From<crate::W<AESAUTHLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESAUTHLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` reader - 31:0\\]
Authentication data length in bytes for combined mode, CCM only. Supported AAD-lengths for CCM are from 0 to (216 - 28) bytes. Once processing with this context is started, this length decrements to zero. Writing this register triggers the engine to start using this context for CCM."]
pub struct LEN_R(crate::FieldReader<u32, u32>);
impl LEN_R {
    pub(crate) fn new(bits: u32) -> Self {
        LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEN` writer - 31:0\\]
Authentication data length in bytes for combined mode, CCM only. Supported AAD-lengths for CCM are from 0 to (216 - 28) bytes. Once processing with this context is started, this length decrements to zero. Writing this register triggers the engine to start using this context for CCM."]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Authentication data length in bytes for combined mode, CCM only. Supported AAD-lengths for CCM are from 0 to (216 - 28) bytes. Once processing with this context is started, this length decrements to zero. Writing this register triggers the engine to start using this context for CCM."]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Authentication data length in bytes for combined mode, CCM only. Supported AAD-lengths for CCM are from 0 to (216 - 28) bytes. Once processing with this context is started, this length decrements to zero. Writing this register triggers the engine to start using this context for CCM."]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Authentication Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesauthlen](index.html) module"]
pub struct AESAUTHLEN_SPEC;
impl crate::RegisterSpec for AESAUTHLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aesauthlen::R](R) reader structure"]
impl crate::Readable for AESAUTHLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesauthlen::W](W) writer structure"]
impl crate::Writable for AESAUTHLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESAUTHLEN to value 0"]
impl crate::Resettable for AESAUTHLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
