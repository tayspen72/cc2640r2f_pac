#[doc = "Register `FTCR` reader"]
pub struct R(crate::R<FTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTCR` writer"]
pub struct W(crate::W<FTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTCR_SPEC>;
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
impl From<crate::W<FTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED7` reader - 31:7\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED7_R(crate::FieldReader<u32, u32>);
impl RESERVED7_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED7_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED7` writer - 31:7\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | ((value as u32 & 0x01ff_ffff) << 7);
        self.w
    }
}
#[doc = "Field `TCR` reader - 6:0\\]
Internal. Only to be used through TI provided API."]
pub struct TCR_R(crate::FieldReader<u8, u8>);
impl TCR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCR` writer - 6:0\\]
Internal. Only to be used through TI provided API."]
pub struct TCR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - 31:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:31 - 31:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&mut self) -> RESERVED7_W {
        RESERVED7_W { w: self }
    }
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tcr(&mut self) -> TCR_W {
        TCR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftcr](index.html) module"]
pub struct FTCR_SPEC;
impl crate::RegisterSpec for FTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftcr::R](R) reader structure"]
impl crate::Readable for FTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftcr::W](W) writer structure"]
impl crate::Writable for FTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FTCR to value 0"]
impl crate::Resettable for FTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
