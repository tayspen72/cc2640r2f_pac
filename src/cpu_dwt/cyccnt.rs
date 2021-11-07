#[doc = "Register `CYCCNT` reader"]
pub struct R(crate::R<CYCCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CYCCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CYCCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CYCCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CYCCNT` writer"]
pub struct W(crate::W<CYCCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CYCCNT_SPEC>;
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
impl From<crate::W<CYCCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CYCCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CYCCNT` reader - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
pub struct CYCCNT_R(crate::FieldReader<u32, u32>);
impl CYCCNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CYCCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CYCCNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CYCCNT` writer - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
pub struct CYCCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
    #[inline(always)]
    pub fn cyccnt(&self) -> CYCCNT_R {
        CYCCNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Current PC Sampler Cycle Counter count value. When enabled, this counter counts the number of core cycles, except when the core is halted. The cycle counter is a free running counter, counting upwards (this counter will not advance in power modes where free-running clock to CPU stops). It wraps around to 0 on overflow. The debugger must initialize this to 0 when first enabling."]
    #[inline(always)]
    pub fn cyccnt(&mut self) -> CYCCNT_W {
        CYCCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current PC Sampler Cycle Count This register is used to count the number of core cycles. This counter can measure elapsed execution time. This is a free-running counter (this counter will not advance in power modes where free-running clock to CPU stops). The counter has three functions: 1: When CTRL.PCSAMPLEENA = 1, the PC is sampled and emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 2: When CTRL.CYCEVTENA = 1 , (and CTRL.PCSAMPLEENA = 0), an event is emitted when the selected tapped bit changes value (0 to 1 or 1 to 0) and any post-scalar value counts to 0. 3: Applications and debuggers can use the counter to measure elapsed execution time. By subtracting a start and an end time, an application can measure time between in-core clocks (other than when Halted in debug). This is valid to 2^32 core clock cycles (for example, almost 89.5 seconds at 48MHz).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cyccnt](index.html) module"]
pub struct CYCCNT_SPEC;
impl crate::RegisterSpec for CYCCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cyccnt::R](R) reader structure"]
impl crate::Readable for CYCCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cyccnt::W](W) writer structure"]
impl crate::Writable for CYCCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CYCCNT to value 0"]
impl crate::Resettable for CYCCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
