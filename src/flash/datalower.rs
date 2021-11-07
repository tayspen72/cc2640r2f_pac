#[doc = "Register `DATALOWER` reader"]
pub struct R(crate::R<DATALOWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATALOWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATALOWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATALOWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATALOWER` writer"]
pub struct W(crate::W<DATALOWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATALOWER_SPEC>;
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
impl From<crate::W<DATALOWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATALOWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
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
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datalower](index.html) module"]
pub struct DATALOWER_SPEC;
impl crate::RegisterSpec for DATALOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datalower::R](R) reader structure"]
impl crate::Readable for DATALOWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datalower::W](W) writer structure"]
impl crate::Writable for DATALOWER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATALOWER to value 0"]
impl crate::Resettable for DATALOWER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
