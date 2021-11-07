#[doc = "Register `HWOPT` reader"]
pub struct R(crate::R<HWOPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWOPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWOPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWOPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWOPT` writer"]
pub struct W(crate::W<HWOPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWOPT_SPEC>;
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
impl From<crate::W<HWOPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWOPT_SPEC>) -> Self {
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
#[doc = "Field `NR_OF_FROS` reader - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
pub struct NR_OF_FROS_R(crate::FieldReader<u8, u8>);
impl NR_OF_FROS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NR_OF_FROS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NR_OF_FROS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NR_OF_FROS` writer - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
pub struct NR_OF_FROS_W<'a> {
    w: &'a mut W,
}
impl<'a> NR_OF_FROS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 5:0\\]
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
#[doc = "Field `RESERVED0` writer - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
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
    #[doc = "Bits 6:11 - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
    #[inline(always)]
    pub fn nr_of_fros(&self) -> NR_OF_FROS_R {
        NR_OF_FROS_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 6:11 - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
    #[inline(always)]
    pub fn nr_of_fros(&mut self) -> NR_OF_FROS_W {
        NR_OF_FROS_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
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
#[doc = "TRNG Engine Options Information\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwopt](index.html) module"]
pub struct HWOPT_SPEC;
impl crate::RegisterSpec for HWOPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwopt::R](R) reader structure"]
impl crate::Readable for HWOPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwopt::W](W) writer structure"]
impl crate::Writable for HWOPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HWOPT to value 0x0600"]
impl crate::Resettable for HWOPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0600
    }
}
