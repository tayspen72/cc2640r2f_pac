#[doc = "Register `ID_PFR1` reader"]
pub struct R(crate::R<ID_PFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_PFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_PFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_PFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID_PFR1` writer"]
pub struct W(crate::W<ID_PFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_PFR1_SPEC>;
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
impl From<crate::W<ID_PFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_PFR1_SPEC>) -> Self {
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
#[doc = "Field `MICROCONTROLLER_PROGRAMMERS_MODEL` reader - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
pub struct MICROCONTROLLER_PROGRAMMERS_MODEL_R(crate::FieldReader<u8, u8>);
impl MICROCONTROLLER_PROGRAMMERS_MODEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MICROCONTROLLER_PROGRAMMERS_MODEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MICROCONTROLLER_PROGRAMMERS_MODEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MICROCONTROLLER_PROGRAMMERS_MODEL` writer - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
pub struct MICROCONTROLLER_PROGRAMMERS_MODEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MICROCONTROLLER_PROGRAMMERS_MODEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_R(crate::FieldReader<u8, u8>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
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
    #[doc = "Bits 8:11 - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
    #[inline(always)]
    pub fn microcontroller_programmers_model(&self) -> MICROCONTROLLER_PROGRAMMERS_MODEL_R {
        MICROCONTROLLER_PROGRAMMERS_MODEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
    #[inline(always)]
    pub fn microcontroller_programmers_model(&mut self) -> MICROCONTROLLER_PROGRAMMERS_MODEL_W {
        MICROCONTROLLER_PROGRAMMERS_MODEL_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Processor Feature 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_pfr1](index.html) module"]
pub struct ID_PFR1_SPEC;
impl crate::RegisterSpec for ID_PFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_pfr1::R](R) reader structure"]
impl crate::Readable for ID_PFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id_pfr1::W](W) writer structure"]
impl crate::Writable for ID_PFR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ID_PFR1 to value 0x0200"]
impl crate::Resettable for ID_PFR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
