#[doc = "Register `FRDCTL` reader"]
pub struct R(crate::R<FRDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRDCTL` writer"]
pub struct W(crate::W<FRDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRDCTL_SPEC>;
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
impl From<crate::W<FRDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
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
Internal. Only to be used through TI provided API."]
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
#[doc = "Field `RWAIT` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub struct RWAIT_R(crate::FieldReader<u8, u8>);
impl RWAIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RWAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWAIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWAIT` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub struct RWAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RWAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RM` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct RM_R(crate::FieldReader<u8, u8>);
impl RM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RM` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct RM_W<'a> {
    w: &'a mut W,
}
impl<'a> RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait(&self) -> RWAIT_R {
        RWAIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait(&mut self) -> RWAIT_W {
        RWAIT_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm(&mut self) -> RM_W {
        RM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frdctl](index.html) module"]
pub struct FRDCTL_SPEC;
impl crate::RegisterSpec for FRDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frdctl::R](R) reader structure"]
impl crate::Readable for FRDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frdctl::W](W) writer structure"]
impl crate::Writable for FRDCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRDCTL to value 0x0200"]
impl crate::Resettable for FRDCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
