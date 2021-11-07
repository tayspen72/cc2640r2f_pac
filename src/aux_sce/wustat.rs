#[doc = "Register `WUSTAT` reader"]
pub struct R(crate::R<WUSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUSTAT` writer"]
pub struct W(crate::W<WUSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUSTAT_SPEC>;
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
impl From<crate::W<WUSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED18` reader - 31:18\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED18_R(crate::FieldReader<u16, u16>);
impl RESERVED18_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED18_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED18` writer - 31:18\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED18_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 18)) | ((value as u32 & 0x3fff) << 18);
        self.w
    }
}
#[doc = "Field `EXC_VECTOR` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub struct EXC_VECTOR_R(crate::FieldReader<u8, u8>);
impl EXC_VECTOR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXC_VECTOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXC_VECTOR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXC_VECTOR` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub struct EXC_VECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> EXC_VECTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED9` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED9_R(crate::FieldReader<u8, u8>);
impl RESERVED9_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED9` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `WU_SIGNAL` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct WU_SIGNAL_R(crate::FieldReader<bool, bool>);
impl WU_SIGNAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WU_SIGNAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WU_SIGNAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WU_SIGNAL` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct WU_SIGNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> WU_SIGNAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `EV_SIGNALS` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct EV_SIGNALS_R(crate::FieldReader<u8, u8>);
impl EV_SIGNALS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EV_SIGNALS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EV_SIGNALS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EV_SIGNALS` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct EV_SIGNALS_W<'a> {
    w: &'a mut W,
}
impl<'a> EV_SIGNALS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exc_vector(&self) -> EXC_VECTOR_R {
        EXC_VECTOR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wu_signal(&self) -> WU_SIGNAL_R {
        WU_SIGNAL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev_signals(&self) -> EV_SIGNALS_R {
        EV_SIGNALS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&mut self) -> RESERVED18_W {
        RESERVED18_W { w: self }
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exc_vector(&mut self) -> EXC_VECTOR_W {
        EXC_VECTOR_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wu_signal(&mut self) -> WU_SIGNAL_W {
        WU_SIGNAL_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev_signals(&mut self) -> EV_SIGNALS_W {
        EV_SIGNALS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wustat](index.html) module"]
pub struct WUSTAT_SPEC;
impl crate::RegisterSpec for WUSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wustat::R](R) reader structure"]
impl crate::Readable for WUSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wustat::W](W) writer structure"]
impl crate::Writable for WUSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUSTAT to value 0"]
impl crate::Resettable for WUSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
