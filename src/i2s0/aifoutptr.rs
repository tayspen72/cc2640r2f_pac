#[doc = "Register `AIFOUTPTR` reader"]
pub struct R(crate::R<AIFOUTPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIFOUTPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIFOUTPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIFOUTPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIFOUTPTR` writer"]
pub struct W(crate::W<AIFOUTPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIFOUTPTR_SPEC>;
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
impl From<crate::W<AIFOUTPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIFOUTPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTR` reader - 31:0\\]
Value of the DMA output buffer pointer currently used by the DMA controller Incremented by 1 (byte) or 2 (word) for each AHB access."]
pub struct PTR_R(crate::FieldReader<u32, u32>);
impl PTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        PTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTR` writer - 31:0\\]
Value of the DMA output buffer pointer currently used by the DMA controller Incremented by 1 (byte) or 2 (word) for each AHB access."]
pub struct PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value of the DMA output buffer pointer currently used by the DMA controller Incremented by 1 (byte) or 2 (word) for each AHB access."]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value of the DMA output buffer pointer currently used by the DMA controller Incremented by 1 (byte) or 2 (word) for each AHB access."]
    #[inline(always)]
    pub fn ptr(&mut self) -> PTR_W {
        PTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Output Buffer Current Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifoutptr](index.html) module"]
pub struct AIFOUTPTR_SPEC;
impl crate::RegisterSpec for AIFOUTPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aifoutptr::R](R) reader structure"]
impl crate::Readable for AIFOUTPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aifoutptr::W](W) writer structure"]
impl crate::Writable for AIFOUTPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AIFOUTPTR to value 0"]
impl crate::Resettable for AIFOUTPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
