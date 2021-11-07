#[doc = "Register `FDIAGCTL` reader"]
pub struct R(crate::R<FDIAGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDIAGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDIAGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDIAGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDIAGCTL` writer"]
pub struct W(crate::W<FDIAGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDIAGCTL_SPEC>;
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
impl From<crate::W<FDIAGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDIAGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIAGMODE` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct DIAGMODE_R(crate::FieldReader<u32, u32>);
impl DIAGMODE_R {
    pub(crate) fn new(bits: u32) -> Self {
        DIAGMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIAGMODE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIAGMODE` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct DIAGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIAGMODE_W<'a> {
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
    pub fn diagmode(&self) -> DIAGMODE_R {
        DIAGMODE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn diagmode(&mut self) -> DIAGMODE_W {
        DIAGMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdiagctl](index.html) module"]
pub struct FDIAGCTL_SPEC;
impl crate::RegisterSpec for FDIAGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdiagctl::R](R) reader structure"]
impl crate::Readable for FDIAGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdiagctl::W](W) writer structure"]
impl crate::Writable for FDIAGCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDIAGCTL to value 0"]
impl crate::Resettable for FDIAGCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
