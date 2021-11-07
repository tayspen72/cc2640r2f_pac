#[doc = "Register `FRAW_ECC` reader"]
pub struct R(crate::R<FRAW_ECC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAW_ECC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAW_ECC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAW_ECC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAW_ECC` writer"]
pub struct W(crate::W<FRAW_ECC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAW_ECC_SPEC>;
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
impl From<crate::W<FRAW_ECC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAW_ECC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAW_ECC` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct RAW_ECC_R(crate::FieldReader<u32, u32>);
impl RAW_ECC_R {
    pub(crate) fn new(bits: u32) -> Self {
        RAW_ECC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAW_ECC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAW_ECC` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct RAW_ECC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAW_ECC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn raw_ecc(&self) -> RAW_ECC_R {
        RAW_ECC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn raw_ecc(&mut self) -> RAW_ECC_W {
        RAW_ECC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fraw_ecc](index.html) module"]
pub struct FRAW_ECC_SPEC;
impl crate::RegisterSpec for FRAW_ECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fraw_ecc::R](R) reader structure"]
impl crate::Readable for FRAW_ECC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fraw_ecc::W](W) writer structure"]
impl crate::Writable for FRAW_ECC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAW_ECC to value 0"]
impl crate::Resettable for FRAW_ECC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
