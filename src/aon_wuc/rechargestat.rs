#[doc = "Register `RECHARGESTAT` reader"]
pub struct R(crate::R<RECHARGESTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECHARGESTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECHARGESTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECHARGESTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECHARGESTAT` writer"]
pub struct W(crate::W<RECHARGESTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECHARGESTAT_SPEC>;
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
impl From<crate::W<RECHARGESTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECHARGESTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED20_R(crate::FieldReader<u16, u16>);
impl RESERVED20_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED20_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED20` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | ((value as u32 & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Field `VDDR_SMPLS` reader - 19:16\\]
The last 4 VDDR samples, bit 0 being the newest. The register is being updated in every recharge period with a shift left, and bit 0 is updated with the last VDDR sample, ie a 1 is shiftet in in case VDDR > VDDR_threshold just before recharge starts. Otherwise a 0 will be shifted in."]
pub struct VDDR_SMPLS_R(crate::FieldReader<u8, u8>);
impl VDDR_SMPLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_SMPLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_SMPLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_SMPLS` writer - 19:16\\]
The last 4 VDDR samples, bit 0 being the newest. The register is being updated in every recharge period with a shift left, and bit 0 is updated with the last VDDR sample, ie a 1 is shiftet in in case VDDR > VDDR_threshold just before recharge starts. Otherwise a 0 will be shifted in."]
pub struct VDDR_SMPLS_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_SMPLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `MAX_USED_PER` reader - 15:0\\]
The maximum value of recharge period seen with VDDR>threshold. The VDDR voltage is compared against the threshold voltage at just before each recharge. If VDDR is above threshold, MAX_USED_PER is updated with max ( current recharge peride; MAX_USED_PER ) This way MAX_USED_PER can track the recharge period where VDDR is decharged to the threshold value. We can therefore use the value as an indication of the leakage current during recharge. This bitfield is cleared to 0 when writing this register."]
pub struct MAX_USED_PER_R(crate::FieldReader<u16, u16>);
impl MAX_USED_PER_R {
    pub(crate) fn new(bits: u16) -> Self {
        MAX_USED_PER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_USED_PER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_USED_PER` writer - 15:0\\]
The maximum value of recharge period seen with VDDR>threshold. The VDDR voltage is compared against the threshold voltage at just before each recharge. If VDDR is above threshold, MAX_USED_PER is updated with max ( current recharge peride; MAX_USED_PER ) This way MAX_USED_PER can track the recharge period where VDDR is decharged to the threshold value. We can therefore use the value as an indication of the leakage current during recharge. This bitfield is cleared to 0 when writing this register."]
pub struct MAX_USED_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_USED_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The last 4 VDDR samples, bit 0 being the newest. The register is being updated in every recharge period with a shift left, and bit 0 is updated with the last VDDR sample, ie a 1 is shiftet in in case VDDR > VDDR_threshold just before recharge starts. Otherwise a 0 will be shifted in."]
    #[inline(always)]
    pub fn vddr_smpls(&self) -> VDDR_SMPLS_R {
        VDDR_SMPLS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - 15:0\\]
The maximum value of recharge period seen with VDDR>threshold. The VDDR voltage is compared against the threshold voltage at just before each recharge. If VDDR is above threshold, MAX_USED_PER is updated with max ( current recharge peride; MAX_USED_PER ) This way MAX_USED_PER can track the recharge period where VDDR is decharged to the threshold value. We can therefore use the value as an indication of the leakage current during recharge. This bitfield is cleared to 0 when writing this register."]
    #[inline(always)]
    pub fn max_used_per(&self) -> MAX_USED_PER_R {
        MAX_USED_PER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
The last 4 VDDR samples, bit 0 being the newest. The register is being updated in every recharge period with a shift left, and bit 0 is updated with the last VDDR sample, ie a 1 is shiftet in in case VDDR > VDDR_threshold just before recharge starts. Otherwise a 0 will be shifted in."]
    #[inline(always)]
    pub fn vddr_smpls(&mut self) -> VDDR_SMPLS_W {
        VDDR_SMPLS_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
The maximum value of recharge period seen with VDDR>threshold. The VDDR voltage is compared against the threshold voltage at just before each recharge. If VDDR is above threshold, MAX_USED_PER is updated with max ( current recharge peride; MAX_USED_PER ) This way MAX_USED_PER can track the recharge period where VDDR is decharged to the threshold value. We can therefore use the value as an indication of the leakage current during recharge. This bitfield is cleared to 0 when writing this register."]
    #[inline(always)]
    pub fn max_used_per(&mut self) -> MAX_USED_PER_W {
        MAX_USED_PER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargestat](index.html) module"]
pub struct RECHARGESTAT_SPEC;
impl crate::RegisterSpec for RECHARGESTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rechargestat::R](R) reader structure"]
impl crate::Readable for RECHARGESTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rechargestat::W](W) writer structure"]
impl crate::Writable for RECHARGESTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RECHARGESTAT to value 0"]
impl crate::Resettable for RECHARGESTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
