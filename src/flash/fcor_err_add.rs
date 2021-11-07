#[doc = "Register `FCOR_ERR_ADD` reader"]
pub struct R(crate::R<FCOR_ERR_ADD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCOR_ERR_ADD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCOR_ERR_ADD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCOR_ERR_ADD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCOR_ERR_ADD` writer"]
pub struct W(crate::W<FCOR_ERR_ADD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCOR_ERR_ADD_SPEC>;
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
impl From<crate::W<FCOR_ERR_ADD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCOR_ERR_ADD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCOR_ERR_ADD` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct FCOR_ERR_ADD_R(crate::FieldReader<u32, u32>);
impl FCOR_ERR_ADD_R {
    pub(crate) fn new(bits: u32) -> Self {
        FCOR_ERR_ADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCOR_ERR_ADD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCOR_ERR_ADD` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct FCOR_ERR_ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCOR_ERR_ADD_W<'a> {
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
    pub fn fcor_err_add(&self) -> FCOR_ERR_ADD_R {
        FCOR_ERR_ADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fcor_err_add(&mut self) -> FCOR_ERR_ADD_W {
        FCOR_ERR_ADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcor_err_add](index.html) module"]
pub struct FCOR_ERR_ADD_SPEC;
impl crate::RegisterSpec for FCOR_ERR_ADD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcor_err_add::R](R) reader structure"]
impl crate::Readable for FCOR_ERR_ADD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcor_err_add::W](W) writer structure"]
impl crate::Writable for FCOR_ERR_ADD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCOR_ERR_ADD to value 0"]
impl crate::Resettable for FCOR_ERR_ADD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
