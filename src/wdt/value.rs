#[doc = "Register `VALUE` reader"]
pub struct R(crate::R<VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VALUE` writer"]
pub struct W(crate::W<VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VALUE_SPEC>;
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
impl From<crate::W<VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTVALUE` reader - 31:0\\]
This register contains the current count value of the timer."]
pub struct WDTVALUE_R(crate::FieldReader<u32, u32>);
impl WDTVALUE_R {
    pub(crate) fn new(bits: u32) -> Self {
        WDTVALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTVALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTVALUE` writer - 31:0\\]
This register contains the current count value of the timer."]
pub struct WDTVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the current count value of the timer."]
    #[inline(always)]
    pub fn wdtvalue(&self) -> WDTVALUE_R {
        WDTVALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the current count value of the timer."]
    #[inline(always)]
    pub fn wdtvalue(&mut self) -> WDTVALUE_W {
        WDTVALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Count Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [value](index.html) module"]
pub struct VALUE_SPEC;
impl crate::RegisterSpec for VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [value::R](R) reader structure"]
impl crate::Readable for VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [value::W](W) writer structure"]
impl crate::Writable for VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VALUE to value 0xffff_ffff"]
impl crate::Resettable for VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
