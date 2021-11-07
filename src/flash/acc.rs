#[doc = "Register `ACC` reader"]
pub struct R(crate::R<ACC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACC` writer"]
pub struct W(crate::W<ACC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACC_SPEC>;
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
impl From<crate::W<ACC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED24` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
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
Internal. Only to be used through TI provided API."]
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
#[doc = "Field `ACCUMULATOR` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub struct ACCUMULATOR_R(crate::FieldReader<u32, u32>);
impl ACCUMULATOR_R {
    pub(crate) fn new(bits: u32) -> Self {
        ACCUMULATOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCUMULATOR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACCUMULATOR` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub struct ACCUMULATOR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCUMULATOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn accumulator(&self) -> ACCUMULATOR_R {
        ACCUMULATOR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn accumulator(&mut self) -> ACCUMULATOR_W {
        ACCUMULATOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc](index.html) module"]
pub struct ACC_SPEC;
impl crate::RegisterSpec for ACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acc::R](R) reader structure"]
impl crate::Readable for ACC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acc::W](W) writer structure"]
impl crate::Writable for ACC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACC to value 0"]
impl crate::Resettable for ACC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
