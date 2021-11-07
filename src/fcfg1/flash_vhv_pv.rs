#[doc = "Register `FLASH_VHV_PV` reader"]
pub struct R(crate::R<FLASH_VHV_PV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_VHV_PV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_VHV_PV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_VHV_PV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_VHV_PV` writer"]
pub struct W(crate::W<FLASH_VHV_PV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_VHV_PV_SPEC>;
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
impl From<crate::W<FLASH_VHV_PV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_VHV_PV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED1` reader - 31:28\\]
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
#[doc = "Field `RESERVED1` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `TRIM13_PV` reader - 27:24\\]
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
#[doc = "Field `TRIM13_PV` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub struct TRIM13_PV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM13_PV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `RESERVED0` reader - 23:20\\]
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
#[doc = "Field `RESERVED0` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `VHV_PV` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct VHV_PV_R(crate::FieldReader<u8, u8>);
impl VHV_PV_R {
    pub(crate) fn new(bits: u8) -> Self {
        VHV_PV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VHV_PV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VHV_PV` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub struct VHV_PV_W<'a> {
    w: &'a mut W,
}
impl<'a> VHV_PV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `VCG2P5` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct VCG2P5_R(crate::FieldReader<u8, u8>);
impl VCG2P5_R {
    pub(crate) fn new(bits: u8) -> Self {
        VCG2P5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCG2P5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCG2P5` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub struct VCG2P5_W<'a> {
    w: &'a mut W,
}
impl<'a> VCG2P5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `VINH` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct VINH_R(crate::FieldReader<u8, u8>);
impl VINH_R {
    pub(crate) fn new(bits: u8) -> Self {
        VINH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VINH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VINH` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub struct VINH_W<'a> {
    w: &'a mut W,
}
impl<'a> VINH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_pv(&self) -> TRIM13_PV_R {
        TRIM13_PV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_pv(&self) -> VHV_PV_R {
        VHV_PV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vcg2p5(&self) -> VCG2P5_R {
        VCG2P5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinh(&self) -> VINH_R {
        VINH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim13_pv(&mut self) -> TRIM13_PV_W {
        TRIM13_PV_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vhv_pv(&mut self) -> VHV_PV_W {
        VHV_PV_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vcg2p5(&mut self) -> VCG2P5_W {
        VCG2P5_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vinh(&mut self) -> VINH_W {
        VINH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_vhv_pv](index.html) module"]
pub struct FLASH_VHV_PV_SPEC;
impl crate::RegisterSpec for FLASH_VHV_PV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_vhv_pv::R](R) reader structure"]
impl crate::Readable for FLASH_VHV_PV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_vhv_pv::W](W) writer structure"]
impl crate::Writable for FLASH_VHV_PV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_VHV_PV to value 0x0008_0001"]
impl crate::Resettable for FLASH_VHV_PV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0001
    }
}
