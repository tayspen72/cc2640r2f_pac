#[doc = "Register `FPAC1` reader"]
pub struct R(crate::R<FPAC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPAC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPAC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPAC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPAC1` writer"]
pub struct W(crate::W<FPAC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPAC1_SPEC>;
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
impl From<crate::W<FPAC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPAC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED28_R(crate::FieldReader<u8, u8>);
impl RESERVED28_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED28_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED28` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED28_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `PSLEEPTDIS` reader - 27:16\\]
Internal. Only to be used through TI provided API."]
pub struct PSLEEPTDIS_R(crate::FieldReader<u16, u16>);
impl PSLEEPTDIS_R {
    pub(crate) fn new(bits: u16) -> Self {
        PSLEEPTDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSLEEPTDIS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSLEEPTDIS` writer - 27:16\\]
Internal. Only to be used through TI provided API."]
pub struct PSLEEPTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PSLEEPTDIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `PUMPRESET_PW` reader - 15:4\\]
Internal. Only to be used through TI provided API."]
pub struct PUMPRESET_PW_R(crate::FieldReader<u16, u16>);
impl PUMPRESET_PW_R {
    pub(crate) fn new(bits: u16) -> Self {
        PUMPRESET_PW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUMPRESET_PW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUMPRESET_PW` writer - 15:4\\]
Internal. Only to be used through TI provided API."]
pub struct PUMPRESET_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMPRESET_PW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | ((value as u32 & 0x0fff) << 4);
        self.w
    }
}
#[doc = "Field `RESERVED1` reader - 3:2\\]
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
#[doc = "Field `RESERVED1` writer - 3:2\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `PUMPPWR` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct PUMPPWR_R(crate::FieldReader<u8, u8>);
impl PUMPPWR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PUMPPWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUMPPWR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUMPPWR` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub struct PUMPPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMPPWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psleeptdis(&self) -> PSLEEPTDIS_R {
        PSLEEPTDIS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumpreset_pw(&self) -> PUMPRESET_PW_R {
        PUMPRESET_PW_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumppwr(&self) -> PUMPPWR_R {
        PUMPPWR_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&mut self) -> RESERVED28_W {
        RESERVED28_W { w: self }
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psleeptdis(&mut self) -> PSLEEPTDIS_W {
        PSLEEPTDIS_W { w: self }
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumpreset_pw(&mut self) -> PUMPRESET_PW_W {
        PUMPRESET_PW_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumppwr(&mut self) -> PUMPPWR_W {
        PUMPPWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpac1](index.html) module"]
pub struct FPAC1_SPEC;
impl crate::RegisterSpec for FPAC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpac1::R](R) reader structure"]
impl crate::Readable for FPAC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpac1::W](W) writer structure"]
impl crate::Writable for FPAC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPAC1 to value 0x0208_2081"]
impl crate::Resettable for FPAC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0208_2081
    }
}
