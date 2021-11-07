#[doc = "Register `FEDACSDIS` reader"]
pub struct R(crate::R<FEDACSDIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEDACSDIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEDACSDIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEDACSDIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEDACSDIS` writer"]
pub struct W(crate::W<FEDACSDIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEDACSDIS_SPEC>;
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
impl From<crate::W<FEDACSDIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEDACSDIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECTORID0` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct SECTORID0_R(crate::FieldReader<u32, u32>);
impl SECTORID0_R {
    pub(crate) fn new(bits: u32) -> Self {
        SECTORID0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECTORID0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECTORID0` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub struct SECTORID0_W<'a> {
    w: &'a mut W,
}
impl<'a> SECTORID0_W<'a> {
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
    pub fn sectorid0(&self) -> SECTORID0_R {
        SECTORID0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sectorid0(&mut self) -> SECTORID0_W {
        SECTORID0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fedacsdis](index.html) module"]
pub struct FEDACSDIS_SPEC;
impl crate::RegisterSpec for FEDACSDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fedacsdis::R](R) reader structure"]
impl crate::Readable for FEDACSDIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fedacsdis::W](W) writer structure"]
impl crate::Writable for FEDACSDIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FEDACSDIS to value 0"]
impl crate::Resettable for FEDACSDIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
