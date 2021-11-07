#[doc = "Register `AESTAGOUT` reader"]
pub struct R(crate::R<AESTAGOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESTAGOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESTAGOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESTAGOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESTAGOUT` writer"]
pub struct W(crate::W<AESTAGOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESTAGOUT_SPEC>;
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
impl From<crate::W<AESTAGOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESTAGOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAG` reader - 31:0\\]
This register contains the authentication TAG for the combined and authentication-only modes."]
pub struct TAG_R(crate::FieldReader<u32, u32>);
impl TAG_R {
    pub(crate) fn new(bits: u32) -> Self {
        TAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAG` writer - 31:0\\]
This register contains the authentication TAG for the combined and authentication-only modes."]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the authentication TAG for the combined and authentication-only modes."]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the authentication TAG for the combined and authentication-only modes."]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Tag Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aestagout](index.html) module"]
pub struct AESTAGOUT_SPEC;
impl crate::RegisterSpec for AESTAGOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aestagout::R](R) reader structure"]
impl crate::Readable for AESTAGOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aestagout::W](W) writer structure"]
impl crate::Writable for AESTAGOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESTAGOUT to value 0"]
impl crate::Resettable for AESTAGOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
