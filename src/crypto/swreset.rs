#[doc = "Register `SWRESET` reader"]
pub struct R(crate::R<SWRESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWRESET` writer"]
pub struct W(crate::W<SWRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRESET_SPEC>;
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
impl From<crate::W<SWRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_R(crate::FieldReader<u32, u32>);
impl RESERVED1_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | ((value as u32 & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Field `RESET` reader - 0:0\\]
If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the Written Area flags; therefore the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - 0:0\\]
If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the Written Area flags; therefore the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the Written Area flags; therefore the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
If this bit is set to 1, the following modules are reset: - Master control internal state is reset. That includes interrupt, error status register and result available interrupt generation FSM. - Key store module state is reset. That includes clearing the Written Area flags; therefore the keys must be reloaded to the key store module. Writing 0 has no effect. The bit is self cleared after executing the reset."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreset](index.html) module"]
pub struct SWRESET_SPEC;
impl crate::RegisterSpec for SWRESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swreset::R](R) reader structure"]
impl crate::Readable for SWRESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swreset::W](W) writer structure"]
impl crate::Writable for SWRESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWRESET to value 0"]
impl crate::Resettable for SWRESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
