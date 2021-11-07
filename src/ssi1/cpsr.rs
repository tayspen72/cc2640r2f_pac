#[doc = "Register `CPSR` reader"]
pub struct R(crate::R<CPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPSR` writer"]
pub struct W(crate::W<CPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPSR_SPEC>;
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
impl From<crate::W<CPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPSDVSR` reader - 7:0\\]
Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
pub struct CPSDVSR_R(crate::FieldReader<u8, u8>);
impl CPSDVSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPSDVSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPSDVSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPSDVSR` writer - 7:0\\]
Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
pub struct CPSDVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSDVSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CPSDVSR_R {
        CPSDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
    #[inline(always)]
    pub fn cpsdvsr(&mut self) -> CPSDVSR_W {
        CPSDVSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Prescale\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpsr](index.html) module"]
pub struct CPSR_SPEC;
impl crate::RegisterSpec for CPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpsr::R](R) reader structure"]
impl crate::Readable for CPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpsr::W](W) writer structure"]
impl crate::Writable for CPSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPSR to value 0"]
impl crate::Resettable for CPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
