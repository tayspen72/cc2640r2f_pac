#[doc = "Register `LDO_TRIM` reader"]
pub struct R(crate::R<LDO_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDO_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDO_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDO_TRIM` writer"]
pub struct W(crate::W<LDO_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO_TRIM_SPEC>;
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
impl From<crate::W<LDO_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDO_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED4` reader - 31:29\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED4_R(crate::FieldReader<u8, u8>);
impl RESERVED4_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 31:29\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
        self.w
    }
}
#[doc = "Field `VDDR_TRIM_SLEEP` reader - 28:24\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_TRIM_SLEEP_R(crate::FieldReader<u8, u8>);
impl VDDR_TRIM_SLEEP_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDR_TRIM_SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDR_TRIM_SLEEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDR_TRIM_SLEEP` writer - 28:24\\]
Internal. Only to be used through TI provided API."]
pub struct VDDR_TRIM_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDR_TRIM_SLEEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `RESERVED3` reader - 23:19\\]
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
#[doc = "Field `RESERVED3` writer - 23:19\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
#[doc = "Field `GLDO_CURSRC` reader - 18:16\\]
Internal. Only to be used through TI provided API."]
pub struct GLDO_CURSRC_R(crate::FieldReader<u8, u8>);
impl GLDO_CURSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        GLDO_CURSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLDO_CURSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLDO_CURSRC` writer - 18:16\\]
Internal. Only to be used through TI provided API."]
pub struct GLDO_CURSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> GLDO_CURSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED2` reader - 15:13\\]
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
#[doc = "Field `RESERVED2` writer - 15:13\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `ITRIM_DIGLDO_LOAD` reader - 12:11\\]
Internal. Only to be used through TI provided API."]
pub struct ITRIM_DIGLDO_LOAD_R(crate::FieldReader<u8, u8>);
impl ITRIM_DIGLDO_LOAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ITRIM_DIGLDO_LOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITRIM_DIGLDO_LOAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITRIM_DIGLDO_LOAD` writer - 12:11\\]
Internal. Only to be used through TI provided API."]
pub struct ITRIM_DIGLDO_LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIM_DIGLDO_LOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `ITRIM_UDIGLDO` reader - 10:8\\]
Internal. Only to be used through TI provided API."]
pub struct ITRIM_UDIGLDO_R(crate::FieldReader<u8, u8>);
impl ITRIM_UDIGLDO_R {
    pub(crate) fn new(bits: u8) -> Self {
        ITRIM_UDIGLDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITRIM_UDIGLDO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITRIM_UDIGLDO` writer - 10:8\\]
Internal. Only to be used through TI provided API."]
pub struct ITRIM_UDIGLDO_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIM_UDIGLDO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED1` reader - 7:3\\]
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
#[doc = "Field `RESERVED1` writer - 7:3\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `VTRIM_DELTA` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct VTRIM_DELTA_R(crate::FieldReader<u8, u8>);
impl VTRIM_DELTA_R {
    pub(crate) fn new(bits: u8) -> Self {
        VTRIM_DELTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VTRIM_DELTA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VTRIM_DELTA` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub struct VTRIM_DELTA_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_DELTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 29) & 0x07) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep(&self) -> VDDR_TRIM_SLEEP_R {
        VDDR_TRIM_SLEEP_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gldo_cursrc(&self) -> GLDO_CURSRC_R {
        GLDO_CURSRC_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_digldo_load(&self) -> ITRIM_DIGLDO_LOAD_R {
        ITRIM_DIGLDO_LOAD_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_udigldo(&self) -> ITRIM_UDIGLDO_R {
        ITRIM_UDIGLDO_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_delta(&self) -> VTRIM_DELTA_R {
        VTRIM_DELTA_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 29:31 - 31:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 24:28 - 28:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr_trim_sleep(&mut self) -> VDDR_TRIM_SLEEP_W {
        VDDR_TRIM_SLEEP_W { w: self }
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gldo_cursrc(&mut self) -> GLDO_CURSRC_W {
        GLDO_CURSRC_W { w: self }
    }
    #[doc = "Bits 13:15 - 15:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 11:12 - 12:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_digldo_load(&mut self) -> ITRIM_DIGLDO_LOAD_W {
        ITRIM_DIGLDO_LOAD_W { w: self }
    }
    #[doc = "Bits 8:10 - 10:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_udigldo(&mut self) -> ITRIM_UDIGLDO_W {
        ITRIM_UDIGLDO_W { w: self }
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_delta(&mut self) -> VTRIM_DELTA_W {
        VTRIM_DELTA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo_trim](index.html) module"]
pub struct LDO_TRIM_SPEC;
impl crate::RegisterSpec for LDO_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo_trim::R](R) reader structure"]
impl crate::Readable for LDO_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo_trim::W](W) writer structure"]
impl crate::Writable for LDO_TRIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LDO_TRIM to value 0xe0f8_e0fb"]
impl crate::Resettable for LDO_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe0f8_e0fb
    }
}
