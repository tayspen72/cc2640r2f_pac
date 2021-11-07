#[doc = "Register `FVHVCT1` reader"]
pub struct R(crate::R<FVHVCT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FVHVCT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FVHVCT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FVHVCT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FVHVCT1` writer"]
pub struct W(crate::W<FVHVCT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FVHVCT1_SPEC>;
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
impl From<crate::W<FVHVCT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FVHVCT1_SPEC>) -> Self {
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
#[doc = "Field `TRIM13_E` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM13_E_R(crate::FieldReader<u8, u8>);
impl TRIM13_E_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM13_E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM13_E_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM13_E` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM13_E_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM13_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `VHVCT_E` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct VHVCT_E_R(crate::FieldReader<u8, u8>);
impl VHVCT_E_R {
    pub(crate) fn new(bits: u8) -> Self {
        VHVCT_E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VHVCT_E_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VHVCT_E` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct VHVCT_E_W<'a> {
    w: &'a mut W,
}
impl<'a> VHVCT_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED8` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED8_R(crate::FieldReader<u8, u8>);
impl RESERVED8_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED8` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TRIM13_PV` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM13_PV_R(crate::FieldReader<u8, u8>);
impl TRIM13_PV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM13_PV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM13_PV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM13_PV` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM13_PV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM13_PV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `VHVCT_PV` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct VHVCT_PV_R(crate::FieldReader<u8, u8>);
impl VHVCT_PV_R {
    pub(crate) fn new(bits: u8) -> Self {
        VHVCT_PV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VHVCT_PV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VHVCT_PV` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct VHVCT_PV_W<'a> {
    w: &'a mut W,
}
impl<'a> VHVCT_PV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
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
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_e(&self) -> TRIM13_E_R {
        TRIM13_E_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_e(&self) -> VHVCT_E_R {
        VHVCT_E_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_pv(&self) -> TRIM13_PV_R {
        TRIM13_PV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_pv(&self) -> VHVCT_PV_R {
        VHVCT_PV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_e(&mut self) -> TRIM13_E_W {
        TRIM13_E_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_e(&mut self) -> VHVCT_E_W {
        VHVCT_E_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_pv(&mut self) -> TRIM13_PV_W {
        TRIM13_PV_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_pv(&mut self) -> VHVCT_PV_W {
        VHVCT_PV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvhvct1](index.html) module"]
pub struct FVHVCT1_SPEC;
impl crate::RegisterSpec for FVHVCT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fvhvct1::R](R) reader structure"]
impl crate::Readable for FVHVCT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fvhvct1::W](W) writer structure"]
impl crate::Writable for FVHVCT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FVHVCT1 to value 0x0084_0088"]
impl crate::Resettable for FVHVCT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0084_0088
    }
}
