#[doc = "Register `FVNVCT` reader"]
pub struct R(crate::R<FVNVCT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FVNVCT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FVNVCT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FVNVCT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FVNVCT` writer"]
pub struct W(crate::W<FVNVCT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FVNVCT_SPEC>;
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
impl From<crate::W<FVNVCT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FVNVCT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED13` reader - 31:13\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED13_R(crate::FieldReader<u32, u32>);
impl RESERVED13_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED13_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED13` writer - 31:13\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED13_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0007_ffff << 13)) | ((value as u32 & 0x0007_ffff) << 13);
        self.w
    }
}
#[doc = "Field `VCG2P5CT` reader - 12:8\\]
Internal. Only to be used through TI provided API."]
pub struct VCG2P5CT_R(crate::FieldReader<u8, u8>);
impl VCG2P5CT_R {
    pub(crate) fn new(bits: u8) -> Self {
        VCG2P5CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCG2P5CT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCG2P5CT` writer - 12:8\\]
Internal. Only to be used through TI provided API."]
pub struct VCG2P5CT_W<'a> {
    w: &'a mut W,
}
impl<'a> VCG2P5CT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED5` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED5_R(crate::FieldReader<u8, u8>);
impl RESERVED5_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED5` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `VIN_CT` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_CT_R(crate::FieldReader<u8, u8>);
impl VIN_CT_R {
    pub(crate) fn new(bits: u8) -> Self {
        VIN_CT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VIN_CT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIN_CT` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub struct VIN_CT_W<'a> {
    w: &'a mut W,
}
impl<'a> VIN_CT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 13:31 - 31:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vcg2p5ct(&self) -> VCG2P5CT_R {
        VCG2P5CT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_ct(&self) -> VIN_CT_R {
        VIN_CT_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 13:31 - 31:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved13(&mut self) -> RESERVED13_W {
        RESERVED13_W { w: self }
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vcg2p5ct(&mut self) -> VCG2P5CT_W {
        VCG2P5CT_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_ct(&mut self) -> VIN_CT_W {
        VIN_CT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvnvct](index.html) module"]
pub struct FVNVCT_SPEC;
impl crate::RegisterSpec for FVNVCT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fvnvct::R](R) reader structure"]
impl crate::Readable for FVNVCT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fvnvct::W](W) writer structure"]
impl crate::Writable for FVNVCT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FVNVCT to value 0x0800"]
impl crate::Resettable for FVNVCT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
