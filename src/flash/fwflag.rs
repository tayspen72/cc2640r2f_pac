#[doc = "Register `FWFLAG` reader"]
pub struct R(crate::R<FWFLAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWFLAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWFLAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWFLAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWFLAG` writer"]
pub struct W(crate::W<FWFLAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWFLAG_SPEC>;
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
impl From<crate::W<FWFLAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWFLAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED3_R(crate::FieldReader<u32, u32>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 31:3\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | ((value as u32 & 0x1fff_ffff) << 3);
        self.w
    }
}
#[doc = "Field `FWFLAG` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct FWFLAG_R(crate::FieldReader<u8, u8>);
impl FWFLAG_R {
    pub(crate) fn new(bits: u8) -> Self {
        FWFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWFLAG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWFLAG` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct FWFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> FWFLAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwflag(&self) -> FWFLAG_R {
        FWFLAG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fwflag(&mut self) -> FWFLAG_W {
        FWFLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwflag](index.html) module"]
pub struct FWFLAG_SPEC;
impl crate::RegisterSpec for FWFLAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwflag::R](R) reader structure"]
impl crate::Readable for FWFLAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwflag::W](W) writer structure"]
impl crate::Writable for FWFLAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FWFLAG to value 0"]
impl crate::Resettable for FWFLAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
