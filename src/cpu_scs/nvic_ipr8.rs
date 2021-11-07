#[doc = "Register `NVIC_IPR8` reader"]
pub struct R(crate::R<NVIC_IPR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR8` writer"]
pub struct W(crate::W<NVIC_IPR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR8_SPEC>;
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
impl From<crate::W<NVIC_IPR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_R(crate::FieldReader<u16, u16>);
impl RESERVED16_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED16_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `PRI_33` reader - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
pub struct PRI_33_R(crate::FieldReader<u8, u8>);
impl PRI_33_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_33_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_33` writer - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
pub struct PRI_33_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_33_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `PRI_32` reader - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
pub struct PRI_32_R(crate::FieldReader<u8, u8>);
impl PRI_32_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_32_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_32` writer - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
pub struct PRI_32_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    pub fn pri_33(&self) -> PRI_33_R {
        PRI_33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    pub fn pri_32(&self) -> PRI_32_R {
        PRI_32_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    pub fn pri_33(&mut self) -> PRI_33_W {
        PRI_33_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    pub fn pri_32(&mut self) -> PRI_32_W {
        PRI_32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr8](index.html) module"]
pub struct NVIC_IPR8_SPEC;
impl crate::RegisterSpec for NVIC_IPR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr8::R](R) reader structure"]
impl crate::Readable for NVIC_IPR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr8::W](W) writer structure"]
impl crate::Writable for NVIC_IPR8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IPR8 to value 0"]
impl crate::Resettable for NVIC_IPR8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
