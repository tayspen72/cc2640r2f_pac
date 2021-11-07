#[doc = "Register `DYN_CG` reader"]
pub struct R(crate::R<DYN_CG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYN_CG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYN_CG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYN_CG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYN_CG` writer"]
pub struct W(crate::W<DYN_CG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYN_CG_SPEC>;
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
impl From<crate::W<DYN_CG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYN_CG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DYN_CG` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct DYN_CG_R(crate::FieldReader<u8, u8>);
impl DYN_CG_R {
    pub(crate) fn new(bits: u8) -> Self {
        DYN_CG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DYN_CG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DYN_CG` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct DYN_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> DYN_CG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dyn_cg(&self) -> DYN_CG_R {
        DYN_CG_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dyn_cg(&mut self) -> DYN_CG_W {
        DYN_CG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dyn_cg](index.html) module"]
pub struct DYN_CG_SPEC;
impl crate::RegisterSpec for DYN_CG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dyn_cg::R](R) reader structure"]
impl crate::Readable for DYN_CG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dyn_cg::W](W) writer structure"]
impl crate::Writable for DYN_CG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYN_CG to value 0"]
impl crate::Resettable for DYN_CG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
