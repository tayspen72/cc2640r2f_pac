#[doc = "Register `LAR` reader"]
pub struct R(crate::R<LAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAR` writer"]
pub struct W(crate::W<LAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAR_SPEC>;
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
impl From<crate::W<LAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_ACCESS` reader - 31:0\\]
A privileged write of 0xC5ACCE55 enables more write access to Control Registers TER, TPR and TCR. An invalid write removes write access."]
pub struct LOCK_ACCESS_R(crate::FieldReader<u32, u32>);
impl LOCK_ACCESS_R {
    pub(crate) fn new(bits: u32) -> Self {
        LOCK_ACCESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_ACCESS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK_ACCESS` writer - 31:0\\]
A privileged write of 0xC5ACCE55 enables more write access to Control Registers TER, TPR and TCR. An invalid write removes write access."]
pub struct LOCK_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_ACCESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A privileged write of 0xC5ACCE55 enables more write access to Control Registers TER, TPR and TCR. An invalid write removes write access."]
    #[inline(always)]
    pub fn lock_access(&self) -> LOCK_ACCESS_R {
        LOCK_ACCESS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A privileged write of 0xC5ACCE55 enables more write access to Control Registers TER, TPR and TCR. An invalid write removes write access."]
    #[inline(always)]
    pub fn lock_access(&mut self) -> LOCK_ACCESS_W {
        LOCK_ACCESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lar](index.html) module"]
pub struct LAR_SPEC;
impl crate::RegisterSpec for LAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lar::R](R) reader structure"]
impl crate::Readable for LAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lar::W](W) writer structure"]
impl crate::Writable for LAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LAR to value 0"]
impl crate::Resettable for LAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}