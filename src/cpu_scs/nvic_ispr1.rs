#[doc = "Register `NVIC_ISPR1` reader"]
pub struct R(crate::R<NVIC_ISPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ISPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ISPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ISPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ISPR1` writer"]
pub struct W(crate::W<NVIC_ISPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ISPR1_SPEC>;
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
impl From<crate::W<NVIC_ISPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ISPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_R(crate::FieldReader<u32, u32>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | ((value as u32 & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "Field `SETPEND33` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
pub struct SETPEND33_R(crate::FieldReader<bool, bool>);
impl SETPEND33_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETPEND33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETPEND33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETPEND33` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
pub struct SETPEND33_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND33_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SETPEND32` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
pub struct SETPEND32_R(crate::FieldReader<bool, bool>);
impl SETPEND32_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETPEND32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETPEND32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETPEND32` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
pub struct SETPEND32_W<'a> {
    w: &'a mut W,
}
impl<'a> SETPEND32_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend33(&self) -> SETPEND33_R {
        SETPEND33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend32(&self) -> SETPEND32_R {
        SETPEND32_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend33(&mut self) -> SETPEND33_W {
        SETPEND33_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend32(&mut self) -> SETPEND32_W {
        SETPEND32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ispr1](index.html) module"]
pub struct NVIC_ISPR1_SPEC;
impl crate::RegisterSpec for NVIC_ISPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ispr1::R](R) reader structure"]
impl crate::Readable for NVIC_ISPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ispr1::W](W) writer structure"]
impl crate::Writable for NVIC_ISPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_ISPR1 to value 0"]
impl crate::Resettable for NVIC_ISPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
