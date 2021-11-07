#[doc = "Register `FVWLCT` reader"]
pub struct R(crate::R<FVWLCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FVWLCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FVWLCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FVWLCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FVWLCT` writer"]
pub struct W(crate::W<FVWLCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FVWLCT_SPEC>;
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
impl From<crate::W<FVWLCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FVWLCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED5` reader - 31:5\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED5_R(crate::FieldReader<u32, u32>);
impl RESERVED5_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED5_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED5` writer - 31:5\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | ((value as u32 & 0x07ff_ffff) << 5);
        self.w
    }
}
#[doc = "Field `VWLCT_P` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub struct VWLCT_P_R(crate::FieldReader<u8, u8>);
impl VWLCT_P_R {
    pub(crate) fn new(bits: u8) -> Self {
        VWLCT_P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VWLCT_P_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VWLCT_P` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub struct VWLCT_P_W<'a> {
    w: &'a mut W,
}
impl<'a> VWLCT_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:31 - 31:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwlct_p(&self) -> VWLCT_P_R {
        VWLCT_P_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:31 - 31:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vwlct_p(&mut self) -> VWLCT_P_W {
        VWLCT_P_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvwlct](index.html) module"]
pub struct FVWLCT_SPEC;
impl crate::RegisterSpec for FVWLCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fvwlct::R](R) reader structure"]
impl crate::Readable for FVWLCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fvwlct::W](W) writer structure"]
impl crate::Writable for FVWLCT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FVWLCT to value 0x08"]
impl crate::Resettable for FVWLCT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
