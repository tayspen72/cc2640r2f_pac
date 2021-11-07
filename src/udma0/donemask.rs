#[doc = "Register `DONEMASK` reader"]
pub struct R(crate::R<DONEMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DONEMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DONEMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DONEMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DONEMASK` writer"]
pub struct W(crate::W<DONEMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DONEMASK_SPEC>;
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
impl From<crate::W<DONEMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DONEMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNLS` reader - 31:0\\]
Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\]
= 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\]
= 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\]
from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\]
to contribute to generation of combined uDMA done signal."]
pub struct CHNLS_R(crate::FieldReader<u32, u32>);
impl CHNLS_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHNLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHNLS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHNLS` writer - 31:0\\]
Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\]
= 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\]
= 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\]
from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\]
to contribute to generation of combined uDMA done signal."]
pub struct CHNLS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\]
= 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\]
= 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\]
from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\]
to contribute to generation of combined uDMA done signal."]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Controls the propagation of the uDMA done and active state to the assigned peripheral. Specifically used for software channels. Read as: Bit \\[Ch\\]
= 0: uDMA done and active state for channel Ch is not blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is blocked from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: uDMA done and active state for channel Ch is blocked from reaching to the peripherals. Note that the uDMA done state for channel \\[Ch\\]
is not blocked from contributing to generation of combined uDMA done signal Write as: Bit \\[Ch\\]
= 0: Allows uDMA done and active stat to propagate to the peripherals. Note that this disables uDMA done state for channel \\[Ch\\]
from contributing to generation of combined uDMA done signal Bit \\[Ch\\]
= 1: Blocks uDMA done and active state to propagate to the peripherals. Note that this enables uDMA done for channel \\[Ch\\]
to contribute to generation of combined uDMA done signal."]
    #[inline(always)]
    pub fn chnls(&mut self) -> CHNLS_W {
        CHNLS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Request Done Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [donemask](index.html) module"]
pub struct DONEMASK_SPEC;
impl crate::RegisterSpec for DONEMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [donemask::R](R) reader structure"]
impl crate::Readable for DONEMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [donemask::W](W) writer structure"]
impl crate::Writable for DONEMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DONEMASK to value 0"]
impl crate::Resettable for DONEMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
