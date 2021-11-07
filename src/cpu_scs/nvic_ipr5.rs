#[doc = "Register `NVIC_IPR5` reader"]
pub struct R(crate::R<NVIC_IPR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR5` writer"]
pub struct W(crate::W<NVIC_IPR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR5_SPEC>;
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
impl From<crate::W<NVIC_IPR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_23` reader - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
pub struct PRI_23_R(crate::FieldReader<u8, u8>);
impl PRI_23_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_23_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_23` writer - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
pub struct PRI_23_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `PRI_22` reader - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
pub struct PRI_22_R(crate::FieldReader<u8, u8>);
impl PRI_22_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_22_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_22` writer - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
pub struct PRI_22_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PRI_21` reader - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
pub struct PRI_21_R(crate::FieldReader<u8, u8>);
impl PRI_21_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_21_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_21` writer - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
pub struct PRI_21_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `PRI_20` reader - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
pub struct PRI_20_R(crate::FieldReader<u8, u8>);
impl PRI_20_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_20_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_20` writer - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
pub struct PRI_20_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    pub fn pri_23(&self) -> PRI_23_R {
        PRI_23_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    pub fn pri_22(&self) -> PRI_22_R {
        PRI_22_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    pub fn pri_21(&self) -> PRI_21_R {
        PRI_21_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    pub fn pri_20(&self) -> PRI_20_R {
        PRI_20_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    pub fn pri_23(&mut self) -> PRI_23_W {
        PRI_23_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    pub fn pri_22(&mut self) -> PRI_22_W {
        PRI_22_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    pub fn pri_21(&mut self) -> PRI_21_W {
        PRI_21_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    pub fn pri_20(&mut self) -> PRI_20_W {
        PRI_20_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ipr5](index.html) module"]
pub struct NVIC_IPR5_SPEC;
impl crate::RegisterSpec for NVIC_IPR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr5::R](R) reader structure"]
impl crate::Readable for NVIC_IPR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr5::W](W) writer structure"]
impl crate::Writable for NVIC_IPR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IPR5 to value 0"]
impl crate::Resettable for NVIC_IPR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
