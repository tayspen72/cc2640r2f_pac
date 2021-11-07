#[doc = "Register `TAILR` reader"]
pub struct R(crate::R<TAILR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAILR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAILR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAILR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAILR` writer"]
pub struct W(crate::W<TAILR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAILR_SPEC>;
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
impl From<crate::W<TAILR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAILR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAILR` reader - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
pub struct TAILR_R(crate::FieldReader<u32, u32>);
impl TAILR_R {
    pub(crate) fn new(bits: u32) -> Self {
        TAILR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAILR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAILR` writer - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
pub struct TAILR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAILR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
    #[inline(always)]
    pub fn tailr(&self) -> TAILR_R {
        TAILR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
    #[inline(always)]
    pub fn tailr(&mut self) -> TAILR_W {
        TAILR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer A Interval Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tailr](index.html) module"]
pub struct TAILR_SPEC;
impl crate::RegisterSpec for TAILR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tailr::R](R) reader structure"]
impl crate::Readable for TAILR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tailr::W](W) writer structure"]
impl crate::Writable for TAILR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAILR to value 0xffff_ffff"]
impl crate::Resettable for TAILR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
