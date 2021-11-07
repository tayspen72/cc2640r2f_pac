#[doc = "Register `CH2CMPINC` reader"]
pub struct R(crate::R<CH2CMPINC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2CMPINC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2CMPINC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2CMPINC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2CMPINC` writer"]
pub struct W(crate::W<CH2CMPINC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2CMPINC_SPEC>;
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
impl From<crate::W<CH2CMPINC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2CMPINC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
pub struct VALUE_R(crate::FieldReader<u32, u32>);
impl VALUE_R {
    pub(crate) fn new(bits: u32) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALUE` writer - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
If CHCTL.CH2_CONT_EN is set, this value is added to CH2CMP.VALUE on every channel 2 compare event."]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\]
event.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2cmpinc](index.html) module"]
pub struct CH2CMPINC_SPEC;
impl crate::RegisterSpec for CH2CMPINC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2cmpinc::R](R) reader structure"]
impl crate::Readable for CH2CMPINC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2cmpinc::W](W) writer structure"]
impl crate::Writable for CH2CMPINC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH2CMPINC to value 0"]
impl crate::Resettable for CH2CMPINC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
