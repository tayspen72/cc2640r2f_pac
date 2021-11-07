#[doc = "Register `PERBUSDMACLKDIV` reader"]
pub struct R(crate::R<PERBUSDMACLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERBUSDMACLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERBUSDMACLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERBUSDMACLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERBUSDMACLKDIV` writer"]
pub struct W(crate::W<PERBUSDMACLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERBUSDMACLKDIV_SPEC>;
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
impl From<crate::W<PERBUSDMACLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERBUSDMACLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPARE` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct SPARE_R(crate::FieldReader<u32, u32>);
impl SPARE_R {
    pub(crate) fn new(bits: u32) -> Self {
        SPARE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPARE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPARE` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct SPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE_W<'a> {
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
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&mut self) -> SPARE_W {
        SPARE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perbusdmaclkdiv](index.html) module"]
pub struct PERBUSDMACLKDIV_SPEC;
impl crate::RegisterSpec for PERBUSDMACLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perbusdmaclkdiv::R](R) reader structure"]
impl crate::Readable for PERBUSDMACLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perbusdmaclkdiv::W](W) writer structure"]
impl crate::Writable for PERBUSDMACLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERBUSDMACLKDIV to value 0"]
impl crate::Resettable for PERBUSDMACLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
