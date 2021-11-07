#[doc = "Register `CLEARCHANNELEN` reader"]
pub struct R(crate::R<CLEARCHANNELEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLEARCHANNELEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLEARCHANNELEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLEARCHANNELEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLEARCHANNELEN` writer"]
pub struct W(crate::W<CLEARCHANNELEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEARCHANNELEN_SPEC>;
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
impl From<crate::W<CLEARCHANNELEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEARCHANNELEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNLS` reader - 31:0\\]
Set the appropriate bit to disable the corresponding uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHANNELEN.CHNLS to enable uDMA channels. Bit \\[Ch\\]
= 1: Disables channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
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
Set the appropriate bit to disable the corresponding uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHANNELEN.CHNLS to enable uDMA channels. Bit \\[Ch\\]
= 1: Disables channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
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
Set the appropriate bit to disable the corresponding uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHANNELEN.CHNLS to enable uDMA channels. Bit \\[Ch\\]
= 1: Disables channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to disable the corresponding uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHANNELEN.CHNLS to enable uDMA channels. Bit \\[Ch\\]
= 1: Disables channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
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
#[doc = "Clear Channel Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearchannelen](index.html) module"]
pub struct CLEARCHANNELEN_SPEC;
impl crate::RegisterSpec for CLEARCHANNELEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clearchannelen::R](R) reader structure"]
impl crate::Readable for CLEARCHANNELEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clearchannelen::W](W) writer structure"]
impl crate::Writable for CLEARCHANNELEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLEARCHANNELEN to value 0"]
impl crate::Resettable for CLEARCHANNELEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
