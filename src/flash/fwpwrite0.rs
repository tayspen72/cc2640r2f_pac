#[doc = "Register `FWPWRITE0` reader"]
pub struct R(crate::R<FWPWRITE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWPWRITE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWPWRITE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWPWRITE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWPWRITE0` writer"]
pub struct W(crate::W<FWPWRITE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWPWRITE0_SPEC>;
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
impl From<crate::W<FWPWRITE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWPWRITE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FWPWRITE0` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct FWPWRITE0_R(crate::FieldReader<u32, u32>);
impl FWPWRITE0_R {
    pub(crate) fn new(bits: u32) -> Self {
        FWPWRITE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWPWRITE0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWPWRITE0` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct FWPWRITE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FWPWRITE0_W<'a> {
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
    pub fn fwpwrite0(&self) -> FWPWRITE0_R {
        FWPWRITE0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwpwrite0(&mut self) -> FWPWRITE0_W {
        FWPWRITE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpwrite0](index.html) module"]
pub struct FWPWRITE0_SPEC;
impl crate::RegisterSpec for FWPWRITE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwpwrite0::R](R) reader structure"]
impl crate::Readable for FWPWRITE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwpwrite0::W](W) writer structure"]
impl crate::Writable for FWPWRITE0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FWPWRITE0 to value 0xffff_ffff"]
impl crate::Resettable for FWPWRITE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
