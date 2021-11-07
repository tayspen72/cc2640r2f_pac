#[doc = "Register `RATCH5VAL` reader"]
pub struct R(crate::R<RATCH5VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RATCH5VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RATCH5VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RATCH5VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RATCH5VAL` writer"]
pub struct W(crate::W<RATCH5VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RATCH5VAL_SPEC>;
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
impl From<crate::W<RATCH5VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RATCH5VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
pub struct VAL_R(crate::FieldReader<u32, u32>);
impl VAL_R {
    pub(crate) fn new(bits: u32) -> Self {
        VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL` writer - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel 5 Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratch5val](index.html) module"]
pub struct RATCH5VAL_SPEC;
impl crate::RegisterSpec for RATCH5VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ratch5val::R](R) reader structure"]
impl crate::Readable for RATCH5VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ratch5val::W](W) writer structure"]
impl crate::Writable for RATCH5VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RATCH5VAL to value 0"]
impl crate::Resettable for RATCH5VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
