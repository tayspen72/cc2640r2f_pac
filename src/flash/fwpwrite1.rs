#[doc = "Register `FWPWRITE1` reader"]
pub struct R(crate::R<FWPWRITE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWPWRITE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWPWRITE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWPWRITE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWPWRITE1` writer"]
pub struct W(crate::W<FWPWRITE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWPWRITE1_SPEC>;
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
impl From<crate::W<FWPWRITE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWPWRITE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FWPWRITE1` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct FWPWRITE1_R(crate::FieldReader<u32, u32>);
impl FWPWRITE1_R {
    pub(crate) fn new(bits: u32) -> Self {
        FWPWRITE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWPWRITE1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWPWRITE1` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct FWPWRITE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FWPWRITE1_W<'a> {
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
    pub fn fwpwrite1(&self) -> FWPWRITE1_R {
        FWPWRITE1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwpwrite1(&mut self) -> FWPWRITE1_W {
        FWPWRITE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite1](index.html) module"]
pub struct FWPWRITE1_SPEC;
impl crate::RegisterSpec for FWPWRITE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwpwrite1::R](R) reader structure"]
impl crate::Readable for FWPWRITE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwpwrite1::W](W) writer structure"]
impl crate::Writable for FWPWRITE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FWPWRITE1 to value 0xffff_ffff"]
impl crate::Resettable for FWPWRITE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
