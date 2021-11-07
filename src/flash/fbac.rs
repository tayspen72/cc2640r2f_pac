#[doc = "Register `FBAC` reader"]
pub struct R(crate::R<FBAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBAC` writer"]
pub struct W(crate::W<FBAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBAC_SPEC>;
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
impl From<crate::W<FBAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED17` reader - 31:17\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED17_R(crate::FieldReader<u16, u16>);
impl RESERVED17_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED17_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED17` writer - 31:17\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED17_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 17)) | ((value as u32 & 0x7fff) << 17);
        self.w
    }
}
#[doc = "Field `OTPPROTDIS` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct OTPPROTDIS_R(crate::FieldReader<bool, bool>);
impl OTPPROTDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTPPROTDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTPPROTDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTPPROTDIS` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct OTPPROTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTPPROTDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `BAGP` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct BAGP_R(crate::FieldReader<u8, u8>);
impl BAGP_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAGP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAGP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAGP` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct BAGP_W<'a> {
    w: &'a mut W,
}
impl<'a> BAGP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `VREADS` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct VREADS_R(crate::FieldReader<u8, u8>);
impl VREADS_R {
    pub(crate) fn new(bits: u8) -> Self {
        VREADS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREADS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREADS` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct VREADS_W<'a> {
    w: &'a mut W,
}
impl<'a> VREADS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&self) -> RESERVED17_R {
        RESERVED17_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otpprotdis(&self) -> OTPPROTDIS_R {
        OTPPROTDIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bagp(&self) -> BAGP_R {
        BAGP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vreads(&self) -> VREADS_R {
        VREADS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&mut self) -> RESERVED17_W {
        RESERVED17_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otpprotdis(&mut self) -> OTPPROTDIS_W {
        OTPPROTDIS_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bagp(&mut self) -> BAGP_W {
        BAGP_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vreads(&mut self) -> VREADS_W {
        VREADS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbac](index.html) module"]
pub struct FBAC_SPEC;
impl crate::RegisterSpec for FBAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbac::R](R) reader structure"]
impl crate::Readable for FBAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbac::W](W) writer structure"]
impl crate::Writable for FBAC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FBAC to value 0x0f"]
impl crate::Resettable for FBAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
