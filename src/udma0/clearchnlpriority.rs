#[doc = "Register `CLEARCHNLPRIORITY` reader"]
pub struct R(crate::R<CLEARCHNLPRIORITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLEARCHNLPRIORITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLEARCHNLPRIORITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLEARCHNLPRIORITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLEARCHNLPRIORITY` writer"]
pub struct W(crate::W<CLEARCHNLPRIORITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEARCHNLPRIORITY_SPEC>;
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
impl From<crate::W<CLEARCHNLPRIORITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEARCHNLPRIORITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNLS` reader - 31:0\\]
Clear the appropriate bit to select the default priority level for the specified uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHNLPRIORITY.CHNLS to set channel Ch to the high priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the default priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
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
Clear the appropriate bit to select the default priority level for the specified uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHNLPRIORITY.CHNLS to set channel Ch to the high priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the default priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
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
Clear the appropriate bit to select the default priority level for the specified uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHNLPRIORITY.CHNLS to set channel Ch to the high priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the default priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Clear the appropriate bit to select the default priority level for the specified uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHNLPRIORITY.CHNLS to set channel Ch to the high priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the default priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
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
#[doc = "Clear Channel Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearchnlpriority](index.html) module"]
pub struct CLEARCHNLPRIORITY_SPEC;
impl crate::RegisterSpec for CLEARCHNLPRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clearchnlpriority::R](R) reader structure"]
impl crate::Readable for CLEARCHNLPRIORITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clearchnlpriority::W](W) writer structure"]
impl crate::Writable for CLEARCHNLPRIORITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLEARCHNLPRIORITY to value 0"]
impl crate::Resettable for CLEARCHNLPRIORITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
