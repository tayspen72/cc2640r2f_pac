#[doc = "Register `PCSR` reader"]
pub struct R(crate::R<PCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCSR` writer"]
pub struct W(crate::W<PCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSR_SPEC>;
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
impl From<crate::W<PCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIASAMPLE` reader - 31:0\\]
Execution instruction address sample, or 0xFFFFFFFF if the core is halted."]
pub struct EIASAMPLE_R(crate::FieldReader<u32, u32>);
impl EIASAMPLE_R {
    pub(crate) fn new(bits: u32) -> Self {
        EIASAMPLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIASAMPLE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIASAMPLE` writer - 31:0\\]
Execution instruction address sample, or 0xFFFFFFFF if the core is halted."]
pub struct EIASAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIASAMPLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Execution instruction address sample, or 0xFFFFFFFF if the core is halted."]
    #[inline(always)]
    pub fn eiasample(&self) -> EIASAMPLE_R {
        EIASAMPLE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Execution instruction address sample, or 0xFFFFFFFF if the core is halted."]
    #[inline(always)]
    pub fn eiasample(&mut self) -> EIASAMPLE_W {
        EIASAMPLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program Counter Sample This register is used to enable coarse-grained software profiling using a debug agent, without changing the currently executing code. If the core is not in debug state, the value returned is the instruction address of a recently executed instruction. If the core is in debug state, the value returned is 0xFFFFFFFF.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsr](index.html) module"]
pub struct PCSR_SPEC;
impl crate::RegisterSpec for PCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcsr::R](R) reader structure"]
impl crate::Readable for PCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcsr::W](W) writer structure"]
impl crate::Writable for PCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCSR to value 0"]
impl crate::Resettable for PCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
