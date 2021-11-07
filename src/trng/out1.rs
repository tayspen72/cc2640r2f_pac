#[doc = "Register `OUT1` reader"]
pub struct R(crate::R<OUT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT1` writer"]
pub struct W(crate::W<OUT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT1_SPEC>;
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
impl From<crate::W<OUT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE_63_32` reader - 31:0\\]
MSW of 64-bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
pub struct VALUE_63_32_R(crate::FieldReader<u32, u32>);
impl VALUE_63_32_R {
    pub(crate) fn new(bits: u32) -> Self {
        VALUE_63_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_63_32_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALUE_63_32` writer - 31:0\\]
MSW of 64-bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
pub struct VALUE_63_32_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_63_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MSW of 64-bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[inline(always)]
    pub fn value_63_32(&self) -> VALUE_63_32_R {
        VALUE_63_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MSW of 64-bit random value. New value ready when IRQFLAGSTAT.RDY = 1."]
    #[inline(always)]
    pub fn value_63_32(&mut self) -> VALUE_63_32_W {
        VALUE_63_32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Random Number Upper Word Readout Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out1](index.html) module"]
pub struct OUT1_SPEC;
impl crate::RegisterSpec for OUT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out1::R](R) reader structure"]
impl crate::Readable for OUT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out1::W](W) writer structure"]
impl crate::Writable for OUT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT1 to value 0"]
impl crate::Resettable for OUT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
