#[doc = "Register `ID_AFR0` reader"]
pub struct R(crate::R<ID_AFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_AFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_AFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_AFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID_AFR0` writer"]
pub struct W(crate::W<ID_AFR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_AFR0_SPEC>;
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
impl From<crate::W<ID_AFR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_AFR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 31:0\\]
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
#[doc = "Field `RESERVED0` writer - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
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
#[doc = "Auxiliary Feature 0 This register provides some freedom for implementation defined features to be registered. Not used in Cortex-M.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_afr0](index.html) module"]
pub struct ID_AFR0_SPEC;
impl crate::RegisterSpec for ID_AFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_afr0::R](R) reader structure"]
impl crate::Readable for ID_AFR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id_afr0::W](W) writer structure"]
impl crate::Writable for ID_AFR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ID_AFR0 to value 0"]
impl crate::Resettable for ID_AFR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
