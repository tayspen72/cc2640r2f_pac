#[doc = "Register `FVHVCT3` reader"]
pub struct R(crate::R<FVHVCT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FVHVCT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FVHVCT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FVHVCT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FVHVCT3` writer"]
pub struct W(crate::W<FVHVCT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FVHVCT3_SPEC>;
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
impl From<crate::W<FVHVCT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FVHVCT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED20` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED20_R(crate::FieldReader<u16, u16>);
impl RESERVED20_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED20_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED20` writer - 31:20\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED20_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | ((value as u32 & 0x0fff) << 20);
        self.w
    }
}
#[doc = "Field `WCT` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct WCT_R(crate::FieldReader<u8, u8>);
impl WCT_R {
    pub(crate) fn new(bits: u8) -> Self {
        WCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCT` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct WCT_W<'a> {
    w: &'a mut W,
}
impl<'a> WCT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `RESERVED4` reader - 15:4\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED4_R(crate::FieldReader<u16, u16>);
impl RESERVED4_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESERVED4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED4` writer - 15:4\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | ((value as u32 & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Field `VHVCT_READ` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct VHVCT_READ_R(crate::FieldReader<u8, u8>);
impl VHVCT_READ_R {
    pub(crate) fn new(bits: u8) -> Self {
        VHVCT_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VHVCT_READ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VHVCT_READ` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct VHVCT_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> VHVCT_READ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wct(&self) -> WCT_R {
        WCT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_read(&self) -> VHVCT_READ_R {
        VHVCT_READ_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&mut self) -> RESERVED20_W {
        RESERVED20_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wct(&mut self) -> WCT_W {
        WCT_W { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&mut self) -> RESERVED4_W {
        RESERVED4_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhvct_read(&mut self) -> VHVCT_READ_W {
        VHVCT_READ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvhvct3](index.html) module"]
pub struct FVHVCT3_SPEC;
impl crate::RegisterSpec for FVHVCT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fvhvct3::R](R) reader structure"]
impl crate::Readable for FVHVCT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fvhvct3::W](W) writer structure"]
impl crate::Writable for FVHVCT3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FVHVCT3 to value 0x000f_0000"]
impl crate::Resettable for FVHVCT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000f_0000
    }
}
