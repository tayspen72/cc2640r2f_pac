#[doc = "Register `LOOPADDR` reader"]
pub struct R(crate::R<LOOPADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOOPADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOOPADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOOPADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOOPADDR` writer"]
pub struct W(crate::W<LOOPADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOOPADDR_SPEC>;
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
impl From<crate::W<LOOPADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOOPADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct STOP_R(crate::FieldReader<u16, u16>);
impl STOP_R {
    pub(crate) fn new(bits: u16) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `START` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct START_R(crate::FieldReader<u16, u16>);
impl START_R {
    pub(crate) fn new(bits: u16) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loopaddr](index.html) module"]
pub struct LOOPADDR_SPEC;
impl crate::RegisterSpec for LOOPADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [loopaddr::R](R) reader structure"]
impl crate::Readable for LOOPADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [loopaddr::W](W) writer structure"]
impl crate::Writable for LOOPADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOOPADDR to value 0"]
impl crate::Resettable for LOOPADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
