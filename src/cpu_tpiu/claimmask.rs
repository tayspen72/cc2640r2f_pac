#[doc = "Register `CLAIMMASK` reader"]
pub struct R(crate::R<CLAIMMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLAIMMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLAIMMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLAIMMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLAIMMASK` writer"]
pub struct W(crate::W<CLAIMMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLAIMMASK_SPEC>;
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
impl From<crate::W<CLAIMMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLAIMMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLAIMMASK` reader - 31:0\\]
This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
pub struct CLAIMMASK_R(crate::FieldReader<u32, u32>);
impl CLAIMMASK_R {
    pub(crate) fn new(bits: u32) -> Self {
        CLAIMMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLAIMMASK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLAIMMASK` writer - 31:0\\]
This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
pub struct CLAIMMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIMMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
    #[inline(always)]
    pub fn claimmask(&self) -> CLAIMMASK_R {
        CLAIMMASK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register forms one half of the Claim Tag value. When reading this register returns the number of bits that can be set (each bit is considered separately): 0: This claim tag bit is not implemented 1: This claim tag bit is not implemented The behavior when writing to this register is described in CLAIMSET."]
    #[inline(always)]
    pub fn claimmask(&mut self) -> CLAIMMASK_W {
        CLAIMMASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Claim Tag Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimmask](index.html) module"]
pub struct CLAIMMASK_SPEC;
impl crate::RegisterSpec for CLAIMMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [claimmask::R](R) reader structure"]
impl crate::Readable for CLAIMMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [claimmask::W](W) writer structure"]
impl crate::Writable for CLAIMMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLAIMMASK to value 0x0f"]
impl crate::Resettable for CLAIMMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
