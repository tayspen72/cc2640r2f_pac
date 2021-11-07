#[doc = "Register `CFG0` reader"]
pub struct R(crate::R<CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG0` writer"]
pub struct W(crate::W<CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG0_SPEC>;
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
impl From<crate::W<CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAX_REFILL_CYCLES` reader - 31:16\\]
This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
pub struct MAX_REFILL_CYCLES_R(crate::FieldReader<u16, u16>);
impl MAX_REFILL_CYCLES_R {
    pub(crate) fn new(bits: u16) -> Self {
        MAX_REFILL_CYCLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAX_REFILL_CYCLES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAX_REFILL_CYCLES` writer - 31:16\\]
This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
pub struct MAX_REFILL_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_REFILL_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED12` reader - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED12_R(crate::FieldReader<u8, u8>);
impl RESERVED12_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED12` writer - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `SMPL_DIV` reader - 11:8\\]
This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
pub struct SMPL_DIV_R(crate::FieldReader<u8, u8>);
impl SMPL_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMPL_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPL_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPL_DIV` writer - 11:8\\]
This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
pub struct SMPL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `MIN_REFILL_CYCLES` reader - 7:0\\]
This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples"]
pub struct MIN_REFILL_CYCLES_R(crate::FieldReader<u8, u8>);
impl MIN_REFILL_CYCLES_R {
    pub(crate) fn new(bits: u8) -> Self {
        MIN_REFILL_CYCLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN_REFILL_CYCLES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN_REFILL_CYCLES` writer - 7:0\\]
This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples"]
pub struct MIN_REFILL_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_REFILL_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
    #[inline(always)]
    pub fn max_refill_cycles(&self) -> MAX_REFILL_CYCLES_R {
        MAX_REFILL_CYCLES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
    #[inline(always)]
    pub fn smpl_div(&self) -> SMPL_DIV_R {
        SMPL_DIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples"]
    #[inline(always)]
    pub fn min_refill_cycles(&self) -> MIN_REFILL_CYCLES_R {
        MIN_REFILL_CYCLES_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
    #[inline(always)]
    pub fn max_refill_cycles(&mut self) -> MAX_REFILL_CYCLES_W {
        MAX_REFILL_CYCLES_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
    #[inline(always)]
    pub fn smpl_div(&mut self) -> SMPL_DIV_W {
        SMPL_DIV_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples"]
    #[inline(always)]
    pub fn min_refill_cycles(&mut self) -> MIN_REFILL_CYCLES_W {
        MIN_REFILL_CYCLES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](index.html) module"]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg0::R](R) reader structure"]
impl crate::Readable for CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg0::W](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
