#[doc = "Register `ID_MMFR2` reader"]
pub struct R(crate::R<ID_MMFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_MMFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_MMFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_MMFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID_MMFR2` writer"]
pub struct W(crate::W<ID_MMFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_MMFR2_SPEC>;
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
impl From<crate::W<ID_MMFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_MMFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED28` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED28_R(crate::FieldReader<u8, u8>);
impl RESERVED28_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED28_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED28` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED28_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
#[doc = "Field `WAIT_FOR_INTERRUPT_STALLING` reader - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
pub struct WAIT_FOR_INTERRUPT_STALLING_R(crate::FieldReader<bool, bool>);
impl WAIT_FOR_INTERRUPT_STALLING_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAIT_FOR_INTERRUPT_STALLING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAIT_FOR_INTERRUPT_STALLING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT_FOR_INTERRUPT_STALLING` writer - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
pub struct WAIT_FOR_INTERRUPT_STALLING_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_FOR_INTERRUPT_STALLING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 23:0\\]
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
#[doc = "Field `RESERVED0` writer - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
    #[inline(always)]
    pub fn wait_for_interrupt_stalling(&self) -> WAIT_FOR_INTERRUPT_STALLING_R {
        WAIT_FOR_INTERRUPT_STALLING_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&mut self) -> RESERVED28_W {
        RESERVED28_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
    #[inline(always)]
    pub fn wait_for_interrupt_stalling(&mut self) -> WAIT_FOR_INTERRUPT_STALLING_W {
        WAIT_FOR_INTERRUPT_STALLING_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
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
#[doc = "Memory Model Feature 2 General information on the memory model and memory management support.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_mmfr2](index.html) module"]
pub struct ID_MMFR2_SPEC;
impl crate::RegisterSpec for ID_MMFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_mmfr2::R](R) reader structure"]
impl crate::Readable for ID_MMFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id_mmfr2::W](W) writer structure"]
impl crate::Writable for ID_MMFR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ID_MMFR2 to value 0x0100_0000"]
impl crate::Resettable for ID_MMFR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0000
    }
}
