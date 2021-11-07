#[doc = "Register `SETBURST` reader"]
pub struct R(crate::R<SETBURST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETBURST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETBURST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETBURST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETBURST` writer"]
pub struct W(crate::W<SETBURST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETBURST_SPEC>;
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
impl From<crate::W<SETBURST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETBURST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNLS` reader - 31:0\\]
Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\]
= 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\]
to 0. Bit \\[Ch\\]
= 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
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
Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\]
= 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\]
to 0. Bit \\[Ch\\]
= 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
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
Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\]
= 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\]
to 0. Bit \\[Ch\\]
= 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\]
= 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\]
to 0. Bit \\[Ch\\]
= 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
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
#[doc = "Channel Set UseBurst\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setburst](index.html) module"]
pub struct SETBURST_SPEC;
impl crate::RegisterSpec for SETBURST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setburst::R](R) reader structure"]
impl crate::Readable for SETBURST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setburst::W](W) writer structure"]
impl crate::Writable for SETBURST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SETBURST to value 0"]
impl crate::Resettable for SETBURST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
