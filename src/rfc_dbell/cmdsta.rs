#[doc = "Register `CMDSTA` reader"]
pub struct R(crate::R<CMDSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDSTA` writer"]
pub struct W(crate::W<CMDSTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDSTA_SPEC>;
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
impl From<crate::W<CMDSTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDSTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STAT` reader - 31:0\\]
Status of the last command used"]
pub struct STAT_R(crate::FieldReader<u32, u32>);
impl STAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STAT` writer - 31:0\\]
Status of the last command used"]
pub struct STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Status of the last command used"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Status of the last command used"]
    #[inline(always)]
    pub fn stat(&mut self) -> STAT_W {
        STAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Doorbell Command Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdsta](index.html) module"]
pub struct CMDSTA_SPEC;
impl crate::RegisterSpec for CMDSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdsta::R](R) reader structure"]
impl crate::Readable for CMDSTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdsta::W](W) writer structure"]
impl crate::Writable for CMDSTA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDSTA to value 0"]
impl crate::Resettable for CMDSTA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}