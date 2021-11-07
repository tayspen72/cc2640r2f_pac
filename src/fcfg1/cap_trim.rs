#[doc = "Register `CAP_TRIM` reader"]
pub struct R(crate::R<CAP_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP_TRIM` writer"]
pub struct W(crate::W<CAP_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_TRIM_SPEC>;
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
impl From<crate::W<CAP_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLUX_CAP_0P28_TRIM` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct FLUX_CAP_0P28_TRIM_R(crate::FieldReader<u16, u16>);
impl FLUX_CAP_0P28_TRIM_R {
    pub(crate) fn new(bits: u16) -> Self {
        FLUX_CAP_0P28_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLUX_CAP_0P28_TRIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLUX_CAP_0P28_TRIM` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct FLUX_CAP_0P28_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUX_CAP_0P28_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `FLUX_CAP_0P4_TRIM` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct FLUX_CAP_0P4_TRIM_R(crate::FieldReader<u16, u16>);
impl FLUX_CAP_0P4_TRIM_R {
    pub(crate) fn new(bits: u16) -> Self {
        FLUX_CAP_0P4_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLUX_CAP_0P4_TRIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLUX_CAP_0P4_TRIM` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct FLUX_CAP_0P4_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUX_CAP_0P4_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flux_cap_0p28_trim(&self) -> FLUX_CAP_0P28_TRIM_R {
        FLUX_CAP_0P28_TRIM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flux_cap_0p4_trim(&self) -> FLUX_CAP_0P4_TRIM_R {
        FLUX_CAP_0P4_TRIM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flux_cap_0p28_trim(&mut self) -> FLUX_CAP_0P28_TRIM_W {
        FLUX_CAP_0P28_TRIM_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flux_cap_0p4_trim(&mut self) -> FLUX_CAP_0P4_TRIM_W {
        FLUX_CAP_0P4_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_trim](index.html) module"]
pub struct CAP_TRIM_SPEC;
impl crate::RegisterSpec for CAP_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_trim::R](R) reader structure"]
impl crate::Readable for CAP_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_trim::W](W) writer structure"]
impl crate::Writable for CAP_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP_TRIM to value 0xffff_ffff"]
impl crate::Resettable for CAP_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
