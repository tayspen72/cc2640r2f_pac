#[doc = "Register `ADCREF1` reader"]
pub struct R(crate::R<ADCREF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCREF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCREF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCREF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCREF1` writer"]
pub struct W(crate::W<ADCREF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCREF1_SPEC>;
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
impl From<crate::W<ADCREF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCREF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED6_R(crate::FieldReader<u8, u8>);
impl RESERVED6_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u8 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `VTRIM` reader - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
pub struct VTRIM_R(crate::FieldReader<u8, u8>);
impl VTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        VTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIM` writer - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
pub struct VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u8 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
    #[inline(always)]
    pub fn vtrim(&self) -> VTRIM_R {
        VTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Trim output voltage of ADC fixed reference (64 steps, 2's complement). Applies only for ADCREF0.SRC = 0. Examples: 0x00 - nominal voltage 1.43V 0x01 - nominal + 0.4% 1.435V 0x3F - nominal - 0.4% 1.425V 0x1F - maximum voltage 1.6V 0x20 - minimum voltage 1.3V"]
    #[inline(always)]
    pub fn vtrim(&mut self) -> VTRIM_W {
        VTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcref1](index.html) module"]
pub struct ADCREF1_SPEC;
impl crate::RegisterSpec for ADCREF1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [adcref1::R](R) reader structure"]
impl crate::Readable for ADCREF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcref1::W](W) writer structure"]
impl crate::Writable for ADCREF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCREF1 to value 0"]
impl crate::Resettable for ADCREF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}