#[doc = "Register `TBILR` reader"]
pub struct R(crate::R<TBILR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBILR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBILR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBILR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBILR` writer"]
pub struct W(crate::W<TBILR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBILR_SPEC>;
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
impl From<crate::W<TBILR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBILR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBILR` reader - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
pub struct TBILR_R(crate::FieldReader<u32, u32>);
impl TBILR_R {
    pub(crate) fn new(bits: u32) -> Self {
        TBILR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBILR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBILR` writer - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
pub struct TBILR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBILR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
    #[inline(always)]
    pub fn tbilr(&self) -> TBILR_R {
        TBILR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
    #[inline(always)]
    pub fn tbilr(&mut self) -> TBILR_W {
        TBILR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer B Interval Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbilr](index.html) module"]
pub struct TBILR_SPEC;
impl crate::RegisterSpec for TBILR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbilr::R](R) reader structure"]
impl crate::Readable for TBILR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbilr::W](W) writer structure"]
impl crate::Writable for TBILR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBILR to value 0xffff"]
impl crate::Resettable for TBILR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
