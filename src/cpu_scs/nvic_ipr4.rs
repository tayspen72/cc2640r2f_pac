#[doc = "Register `NVIC_IPR4` reader"]
pub struct R(crate::R<NVIC_IPR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR4` writer"]
pub struct W(crate::W<NVIC_IPR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR4_SPEC>;
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
impl From<crate::W<NVIC_IPR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_19` reader - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
pub struct PRI_19_R(crate::FieldReader<u8, u8>);
impl PRI_19_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_19_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_19` writer - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
pub struct PRI_19_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `PRI_18` reader - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
pub struct PRI_18_R(crate::FieldReader<u8, u8>);
impl PRI_18_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_18_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_18` writer - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
pub struct PRI_18_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PRI_17` reader - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
pub struct PRI_17_R(crate::FieldReader<u8, u8>);
impl PRI_17_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_17_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_17` writer - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
pub struct PRI_17_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `PRI_16` reader - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
pub struct PRI_16_R(crate::FieldReader<u8, u8>);
impl PRI_16_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_16` writer - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
pub struct PRI_16_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub fn pri_19(&self) -> PRI_19_R {
        PRI_19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub fn pri_18(&self) -> PRI_18_R {
        PRI_18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub fn pri_17(&self) -> PRI_17_R {
        PRI_17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub fn pri_16(&self) -> PRI_16_R {
        PRI_16_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub fn pri_19(&mut self) -> PRI_19_W {
        PRI_19_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub fn pri_18(&mut self) -> PRI_18_W {
        PRI_18_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub fn pri_17(&mut self) -> PRI_17_W {
        PRI_17_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub fn pri_16(&mut self) -> PRI_16_W {
        PRI_16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr4](index.html) module"]
pub struct NVIC_IPR4_SPEC;
impl crate::RegisterSpec for NVIC_IPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr4::R](R) reader structure"]
impl crate::Readable for NVIC_IPR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr4::W](W) writer structure"]
impl crate::Writable for NVIC_IPR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IPR4 to value 0"]
impl crate::Resettable for NVIC_IPR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
