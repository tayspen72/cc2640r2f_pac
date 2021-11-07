#[doc = "Register `FCFG_BNK_TYPE` reader"]
pub struct R(crate::R<FCFG_BNK_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG_BNK_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG_BNK_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG_BNK_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG_BNK_TYPE` writer"]
pub struct W(crate::W<FCFG_BNK_TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG_BNK_TYPE_SPEC>;
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
impl From<crate::W<FCFG_BNK_TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG_BNK_TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B7_TYPE` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct B7_TYPE_R(crate::FieldReader<u8, u8>);
impl B7_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        B7_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B7_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B7_TYPE` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct B7_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B7_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `B6_TYPE` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub struct B6_TYPE_R(crate::FieldReader<u8, u8>);
impl B6_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        B6_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B6_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B6_TYPE` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub struct B6_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B6_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `B5_TYPE` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub struct B5_TYPE_R(crate::FieldReader<u8, u8>);
impl B5_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        B5_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B5_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B5_TYPE` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub struct B5_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B5_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `B4_TYPE` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct B4_TYPE_R(crate::FieldReader<u8, u8>);
impl B4_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        B4_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B4_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B4_TYPE` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct B4_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B4_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `B3_TYPE` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct B3_TYPE_R(crate::FieldReader<u8, u8>);
impl B3_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        B3_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B3_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B3_TYPE` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub struct B3_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B3_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `B2_TYPE` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub struct B2_TYPE_R(crate::FieldReader<u8, u8>);
impl B2_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        B2_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2_TYPE` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub struct B2_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B2_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `B1_TYPE` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct B1_TYPE_R(crate::FieldReader<u8, u8>);
impl B1_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        B1_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1_TYPE` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub struct B1_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B1_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `B0_TYPE` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct B0_TYPE_R(crate::FieldReader<u8, u8>);
impl B0_TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        B0_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B0_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B0_TYPE` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct B0_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_type(&self) -> B7_TYPE_R {
        B7_TYPE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b6_type(&self) -> B6_TYPE_R {
        B6_TYPE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b5_type(&self) -> B5_TYPE_R {
        B5_TYPE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b4_type(&self) -> B4_TYPE_R {
        B4_TYPE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b3_type(&self) -> B3_TYPE_R {
        B3_TYPE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_type(&self) -> B2_TYPE_R {
        B2_TYPE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_type(&self) -> B1_TYPE_R {
        B1_TYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_type(&self) -> B0_TYPE_R {
        B0_TYPE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_type(&mut self) -> B7_TYPE_W {
        B7_TYPE_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b6_type(&mut self) -> B6_TYPE_W {
        B6_TYPE_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b5_type(&mut self) -> B5_TYPE_W {
        B5_TYPE_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b4_type(&mut self) -> B4_TYPE_W {
        B4_TYPE_W { w: self }
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b3_type(&mut self) -> B3_TYPE_W {
        B3_TYPE_W { w: self }
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_type(&mut self) -> B2_TYPE_W {
        B2_TYPE_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_type(&mut self) -> B1_TYPE_W {
        B1_TYPE_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_type(&mut self) -> B0_TYPE_W {
        B0_TYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_bnk_type](index.html) module"]
pub struct FCFG_BNK_TYPE_SPEC;
impl crate::RegisterSpec for FCFG_BNK_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg_bnk_type::R](R) reader structure"]
impl crate::Readable for FCFG_BNK_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg_bnk_type::W](W) writer structure"]
impl crate::Writable for FCFG_BNK_TYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCFG_BNK_TYPE to value 0x03"]
impl crate::Resettable for FCFG_BNK_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
