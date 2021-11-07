#[doc = "Register `FLOCK` reader"]
pub struct R(crate::R<FLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLOCK` writer"]
pub struct W(crate::W<FLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOCK_SPEC>;
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
impl From<crate::W<FLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
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
Internal. Only to be used through TI provided API."]
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
#[doc = "Field `ENCOM` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct ENCOM_R(crate::FieldReader<u16, u16>);
impl ENCOM_R {
    pub(crate) fn new(bits: u16) -> Self {
        ENCOM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENCOM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENCOM` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct ENCOM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn encom(&self) -> ENCOM_R {
        ENCOM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn encom(&mut self) -> ENCOM_W {
        ENCOM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flock](index.html) module"]
pub struct FLOCK_SPEC;
impl crate::RegisterSpec for FLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flock::R](R) reader structure"]
impl crate::Readable for FLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flock::W](W) writer structure"]
impl crate::Writable for FLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLOCK to value 0x55aa"]
impl crate::Resettable for FLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x55aa
    }
}
