#[doc = "Register `VOLT_TRIM` reader"]
pub struct R(crate::R<VOLT_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VOLT_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VOLT_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VOLT_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VOLT_TRIM` writer"]
pub struct W(crate::W<VOLT_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VOLT_TRIM_SPEC>;
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
impl From<crate::W<VOLT_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VOLT_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED3` reader - 31:29\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED3_R(crate::FieldReader<u8, u8>);
impl RESERVED3_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED3` writer - 31:29\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
#[doc = "Field `VDDR_TRIM_HH` reader - 28:24\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_TRIM_HH_R(crate::FieldReader<u8, u8>);
impl VDDR_TRIM_HH_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_TRIM_HH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_TRIM_HH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_TRIM_HH` writer - 28:24\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_TRIM_HH_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_HH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `RESERVED2` reader - 23:21\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED2_R(crate::FieldReader<u8, u8>);
impl RESERVED2_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED2` writer - 23:21\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `VDDR_TRIM_H` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_TRIM_H_R(crate::FieldReader<u8, u8>);
impl VDDR_TRIM_H_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_TRIM_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_TRIM_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_TRIM_H` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_TRIM_H_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED1` reader - 15:13\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED1_R(crate::FieldReader<u8, u8>);
impl RESERVED1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED1` writer - 15:13\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `VDDR_TRIM_SLEEP_H` reader - 12:8\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_TRIM_SLEEP_H_R(crate::FieldReader<u8, u8>);
impl VDDR_TRIM_SLEEP_H_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_TRIM_SLEEP_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_TRIM_SLEEP_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_TRIM_SLEEP_H` writer - 12:8\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_TRIM_SLEEP_H_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_SLEEP_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_R(crate::FieldReader<u8, u8>);
impl RESERVED0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED0` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `TRIMBOD_H` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMBOD_H_R(crate::FieldReader<u8, u8>);
impl TRIMBOD_H_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIMBOD_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIMBOD_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIMBOD_H` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub struct TRIMBOD_H_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIMBOD_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_hh(&self) -> VDDR_TRIM_HH_R {
        VDDR_TRIM_HH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_h(&self) -> VDDR_TRIM_H_R {
        VDDR_TRIM_H_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep_h(&self) -> VDDR_TRIM_SLEEP_H_R {
        VDDR_TRIM_SLEEP_H_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_h(&self) -> TRIMBOD_H_R {
        TRIMBOD_H_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_hh(&mut self) -> VDDR_TRIM_HH_W {
        VDDR_TRIM_HH_W { w: self }
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_h(&mut self) -> VDDR_TRIM_H_W {
        VDDR_TRIM_H_W { w: self }
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep_h(&mut self) -> VDDR_TRIM_SLEEP_H_W {
        VDDR_TRIM_SLEEP_H_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimbod_h(&mut self) -> TRIMBOD_H_W {
        TRIMBOD_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [volt_trim](index.html) module"]
pub struct VOLT_TRIM_SPEC;
impl crate::RegisterSpec for VOLT_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [volt_trim::R](R) reader structure"]
impl crate::Readable for VOLT_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [volt_trim::W](W) writer structure"]
impl crate::Writable for VOLT_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VOLT_TRIM to value 0xffff_ffe0"]
impl crate::Resettable for VOLT_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffe0
    }
}
