#[doc = "Register `LSUCNT` reader"]
pub struct R(crate::R<LSUCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSUCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSUCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSUCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSUCNT` writer"]
pub struct W(crate::W<LSUCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSUCNT_SPEC>;
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
impl From<crate::W<LSUCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSUCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_R(crate::FieldReader<u32, u32>);
impl RESERVED8_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED8_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Field `LSUCNT` reader - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
pub struct LSUCNT_R(crate::FieldReader<u8, u8>);
impl LSUCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSUCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSUCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSUCNT` writer - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
pub struct LSUCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSUCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:7 - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
    #[inline(always)]
    pub fn lsucnt(&self) -> LSUCNT_R {
        LSUCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
    #[inline(always)]
    pub fn lsucnt(&mut self) -> LSUCNT_W {
        LSUCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsucnt](index.html) module"]
pub struct LSUCNT_SPEC;
impl crate::RegisterSpec for LSUCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsucnt::R](R) reader structure"]
impl crate::Readable for LSUCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsucnt::W](W) writer structure"]
impl crate::Writable for LSUCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSUCNT to value 0"]
impl crate::Resettable for LSUCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
