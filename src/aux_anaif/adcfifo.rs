#[doc = "Register `ADCFIFO` reader"]
pub struct R(crate::R<ADCFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCFIFO` writer"]
pub struct W(crate::W<ADCFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCFIFO_SPEC>;
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
impl From<crate::W<ADCFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED12_R(crate::FieldReader<u32, u32>);
impl RESERVED12_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED12_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | ((value as u32 & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Field `DATA` reader - 11:0\\]
FIFO data. Read: Get oldest ADC sample from FIFO. Write: Write dummy sample to FIFO. This is useful for code development when you do not have real ADC samples."]
pub struct DATA_R(crate::FieldReader<u16, u16>);
impl DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - 11:0\\]
FIFO data. Read: Get oldest ADC sample from FIFO. Write: Write dummy sample to FIFO. This is useful for code development when you do not have real ADC samples."]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 0:11 - 11:0\\]
FIFO data. Read: Get oldest ADC sample from FIFO. Write: Write dummy sample to FIFO. This is useful for code development when you do not have real ADC samples."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 0:11 - 11:0\\]
FIFO data. Read: Get oldest ADC sample from FIFO. Write: Write dummy sample to FIFO. This is useful for code development when you do not have real ADC samples."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcfifo](index.html) module"]
pub struct ADCFIFO_SPEC;
impl crate::RegisterSpec for ADCFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcfifo::R](R) reader structure"]
impl crate::Readable for ADCFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcfifo::W](W) writer structure"]
impl crate::Writable for ADCFIFO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCFIFO to value 0"]
impl crate::Resettable for ADCFIFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
