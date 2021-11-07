#[doc = "Register `FSWSTAT` reader"]
pub struct R(crate::R<FSWSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSWSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSWSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSWSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSWSTAT` writer"]
pub struct W(crate::W<FSWSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSWSTAT_SPEC>;
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
impl From<crate::W<FSWSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSWSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
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
Internal. Only to be used through TI provided API."]
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
#[doc = "Field `SAFELV` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct SAFELV_R(crate::FieldReader<bool, bool>);
impl SAFELV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAFELV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAFELV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAFELV` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub struct SAFELV_W<'a> {
    w: &'a mut W,
}
impl<'a> SAFELV_W<'a> {
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
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn safelv(&self) -> SAFELV_R {
        SAFELV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn safelv(&mut self) -> SAFELV_W {
        SAFELV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fswstat](index.html) module"]
pub struct FSWSTAT_SPEC;
impl crate::RegisterSpec for FSWSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fswstat::R](R) reader structure"]
impl crate::Readable for FSWSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fswstat::W](W) writer structure"]
impl crate::Writable for FSWSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSWSTAT to value 0x01"]
impl crate::Resettable for FSWSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
