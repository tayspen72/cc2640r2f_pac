#[doc = "Register `ID_DFR0` reader"]
pub struct R(crate::R<ID_DFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_DFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_DFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_DFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID_DFR0` writer"]
pub struct W(crate::W<ID_DFR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_DFR0_SPEC>;
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
impl From<crate::W<ID_DFR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_DFR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED24_R(crate::FieldReader<u8, u8>);
impl RESERVED24_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `MICROCONTROLLER_DEBUG_MODEL` reader - 23:20\\]
Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)"]
pub struct MICROCONTROLLER_DEBUG_MODEL_R(crate::FieldReader<u8, u8>);
impl MICROCONTROLLER_DEBUG_MODEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MICROCONTROLLER_DEBUG_MODEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MICROCONTROLLER_DEBUG_MODEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MICROCONTROLLER_DEBUG_MODEL` writer - 23:20\\]
Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)"]
pub struct MICROCONTROLLER_DEBUG_MODEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MICROCONTROLLER_DEBUG_MODEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 19:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_R(crate::FieldReader<u32, u32>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 19:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)"]
    #[inline(always)]
    pub fn microcontroller_debug_model(&self) -> MICROCONTROLLER_DEBUG_MODEL_R {
        MICROCONTROLLER_DEBUG_MODEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 0:19 - 19:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Microcontroller Debug Model - memory mapped 0x0: Not supported 0x1: Microcontroller debug v1 (ITMv1 and DWTv1)"]
    #[inline(always)]
    pub fn microcontroller_debug_model(&mut self) -> MICROCONTROLLER_DEBUG_MODEL_W {
        MICROCONTROLLER_DEBUG_MODEL_W { w: self }
    }
    #[doc = "Bits 0:19 - 19:0\\]
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
#[doc = "Debug Feature 0 This register provides a high level view of the debug system. Further details are provided in the debug infrastructure itself.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_dfr0](index.html) module"]
pub struct ID_DFR0_SPEC;
impl crate::RegisterSpec for ID_DFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_dfr0::R](R) reader structure"]
impl crate::Readable for ID_DFR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id_dfr0::W](W) writer structure"]
impl crate::Writable for ID_DFR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ID_DFR0 to value 0x0010_0000"]
impl crate::Resettable for ID_DFR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0000
    }
}
