#[doc = "Register `CLAIMTAG` reader"]
pub struct R(crate::R<CLAIMTAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLAIMTAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLAIMTAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLAIMTAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLAIMTAG` writer"]
pub struct W(crate::W<CLAIMTAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLAIMTAG_SPEC>;
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
impl From<crate::W<CLAIMTAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLAIMTAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLAIMTAG` reader - 31:0\\]
This register forms one half of the Claim Tag value. Reading this register returns the current Claim Tag value. Reading CLAIMMASK determines how many bits from this register must be used. The behavior when writing to this register is described in CLAIMCLR."]
pub struct CLAIMTAG_R(crate::FieldReader<u32, u32>);
impl CLAIMTAG_R {
    pub(crate) fn new(bits: u32) -> Self {
        CLAIMTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLAIMTAG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLAIMTAG` writer - 31:0\\]
This register forms one half of the Claim Tag value. Reading this register returns the current Claim Tag value. Reading CLAIMMASK determines how many bits from this register must be used. The behavior when writing to this register is described in CLAIMCLR."]
pub struct CLAIMTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIMTAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Reading this register returns the current Claim Tag value. Reading CLAIMMASK determines how many bits from this register must be used. The behavior when writing to this register is described in CLAIMCLR."]
    #[inline(always)]
    pub fn claimtag(&self) -> CLAIMTAG_R {
        CLAIMTAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. Reading this register returns the current Claim Tag value. Reading CLAIMMASK determines how many bits from this register must be used. The behavior when writing to this register is described in CLAIMCLR."]
    #[inline(always)]
    pub fn claimtag(&mut self) -> CLAIMTAG_W {
        CLAIMTAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Claim Tag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimtag](index.html) module"]
pub struct CLAIMTAG_SPEC;
impl crate::RegisterSpec for CLAIMTAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [claimtag::R](R) reader structure"]
impl crate::Readable for CLAIMTAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [claimtag::W](W) writer structure"]
impl crate::Writable for CLAIMTAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLAIMTAG to value 0"]
impl crate::Resettable for CLAIMTAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
