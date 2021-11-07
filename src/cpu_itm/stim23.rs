#[doc = "Register `STIM23` reader"]
pub struct R(crate::R<STIM23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STIM23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STIM23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STIM23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STIM23` writer"]
pub struct W(crate::W<STIM23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STIM23_SPEC>;
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
impl From<crate::W<STIM23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STIM23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STIM23` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA23 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub struct STIM23_R(crate::FieldReader<u32, u32>);
impl STIM23_R {
    pub(crate) fn new(bits: u32) -> Self {
        STIM23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STIM23_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STIM23` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA23 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub struct STIM23_W<'a> {
    w: &'a mut W,
}
impl<'a> STIM23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA23 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim23(&self) -> STIM23_R {
        STIM23_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA23 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim23(&mut self) -> STIM23_W {
        STIM23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stimulus Port 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim23](index.html) module"]
pub struct STIM23_SPEC;
impl crate::RegisterSpec for STIM23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stim23::R](R) reader structure"]
impl crate::Readable for STIM23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stim23::W](W) writer structure"]
impl crate::Writable for STIM23_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STIM23 to value 0"]
impl crate::Resettable for STIM23_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
