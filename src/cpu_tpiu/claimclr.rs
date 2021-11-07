#[doc = "Register `CLAIMCLR` reader"]
pub struct R(crate::R<CLAIMCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLAIMCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLAIMCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLAIMCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLAIMCLR` writer"]
pub struct W(crate::W<CLAIMCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLAIMCLR_SPEC>;
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
impl From<crate::W<CLAIMCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLAIMCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLAIMCLR` reader - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location enables individual bits to be cleared (each bit is considered separately): 0: No effect 1: Clear this bit in the claim tag. The behavior when reading from this location is described in CLAIMTAG."]
pub struct CLAIMCLR_R(crate::FieldReader<u32, u32>);
impl CLAIMCLR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CLAIMCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLAIMCLR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLAIMCLR` writer - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location enables individual bits to be cleared (each bit is considered separately): 0: No effect 1: Clear this bit in the claim tag. The behavior when reading from this location is described in CLAIMTAG."]
pub struct CLAIMCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIMCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location enables individual bits to be cleared (each bit is considered separately): 0: No effect 1: Clear this bit in the claim tag. The behavior when reading from this location is described in CLAIMTAG."]
    #[inline(always)]
    pub fn claimclr(&self) -> CLAIMCLR_R {
        CLAIMCLR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Writing to this location enables individual bits to be cleared (each bit is considered separately): 0: No effect 1: Clear this bit in the claim tag. The behavior when reading from this location is described in CLAIMTAG."]
    #[inline(always)]
    pub fn claimclr(&mut self) -> CLAIMCLR_W {
        CLAIMCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Claim Tag Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimclr](index.html) module"]
pub struct CLAIMCLR_SPEC;
impl crate::RegisterSpec for CLAIMCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [claimclr::R](R) reader structure"]
impl crate::Readable for CLAIMCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [claimclr::W](W) writer structure"]
impl crate::Writable for CLAIMCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLAIMCLR to value 0"]
impl crate::Resettable for CLAIMCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
