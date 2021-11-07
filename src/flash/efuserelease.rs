#[doc = "Register `EFUSERELEASE` reader"]
pub struct R(crate::R<EFUSERELEASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EFUSERELEASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EFUSERELEASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EFUSERELEASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EFUSERELEASE` writer"]
pub struct W(crate::W<EFUSERELEASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EFUSERELEASE_SPEC>;
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
impl From<crate::W<EFUSERELEASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EFUSERELEASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ODPYEAR` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub struct ODPYEAR_R(crate::FieldReader<u8, u8>);
impl ODPYEAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ODPYEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODPYEAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODPYEAR` writer - 31:25\\]
Internal. Only to be used through TI provided API."]
pub struct ODPYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> ODPYEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | ((value as u32 & 0x7f) << 25);
        self.w
    }
}
#[doc = "Field `ODPMONTH` reader - 24:21\\]
Internal. Only to be used through TI provided API."]
pub struct ODPMONTH_R(crate::FieldReader<u8, u8>);
impl ODPMONTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        ODPMONTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODPMONTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODPMONTH` writer - 24:21\\]
Internal. Only to be used through TI provided API."]
pub struct ODPMONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ODPMONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 21)) | ((value as u32 & 0x0f) << 21);
        self.w
    }
}
#[doc = "Field `ODPDAY` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub struct ODPDAY_R(crate::FieldReader<u8, u8>);
impl ODPDAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        ODPDAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODPDAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODPDAY` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub struct ODPDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ODPDAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `EFUSEYEAR` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub struct EFUSEYEAR_R(crate::FieldReader<u8, u8>);
impl EFUSEYEAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFUSEYEAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSEYEAR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSEYEAR` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub struct EFUSEYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSEYEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `EFUSEMONTH` reader - 8:5\\]
Internal. Only to be used through TI provided API."]
pub struct EFUSEMONTH_R(crate::FieldReader<u8, u8>);
impl EFUSEMONTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFUSEMONTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSEMONTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSEMONTH` writer - 8:5\\]
Internal. Only to be used through TI provided API."]
pub struct EFUSEMONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSEMONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `EFUSEDAY` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub struct EFUSEDAY_R(crate::FieldReader<u8, u8>);
impl EFUSEDAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFUSEDAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSEDAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSEDAY` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub struct EFUSEDAY_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSEDAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpyear(&self) -> ODPYEAR_R {
        ODPYEAR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpmonth(&self) -> ODPMONTH_R {
        ODPMONTH_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpday(&self) -> ODPDAY_R {
        ODPDAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseyear(&self) -> EFUSEYEAR_R {
        EFUSEYEAR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efusemonth(&self) -> EFUSEMONTH_R {
        EFUSEMONTH_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseday(&self) -> EFUSEDAY_R {
        EFUSEDAY_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpyear(&mut self) -> ODPYEAR_W {
        ODPYEAR_W { w: self }
    }
    #[doc = "Bits 21:24 - 24:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpmonth(&mut self) -> ODPMONTH_W {
        ODPMONTH_W { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpday(&mut self) -> ODPDAY_W {
        ODPDAY_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseyear(&mut self) -> EFUSEYEAR_W {
        EFUSEYEAR_W { w: self }
    }
    #[doc = "Bits 5:8 - 8:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efusemonth(&mut self) -> EFUSEMONTH_W {
        EFUSEMONTH_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseday(&mut self) -> EFUSEDAY_W {
        EFUSEDAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [efuserelease](index.html) module"]
pub struct EFUSERELEASE_SPEC;
impl crate::RegisterSpec for EFUSERELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [efuserelease::R](R) reader structure"]
impl crate::Readable for EFUSERELEASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [efuserelease::W](W) writer structure"]
impl crate::Writable for EFUSERELEASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EFUSERELEASE to value 0"]
impl crate::Resettable for EFUSERELEASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
