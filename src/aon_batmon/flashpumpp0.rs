#[doc = "Register `FLASHPUMPP0` reader"]
pub struct R(crate::R<FLASHPUMPP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASHPUMPP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASHPUMPP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASHPUMPP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASHPUMPP0` writer"]
pub struct W(crate::W<FLASHPUMPP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASHPUMPP0_SPEC>;
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
impl From<crate::W<FLASHPUMPP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASHPUMPP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED9_R(crate::FieldReader<u32, u32>);
impl RESERVED9_R {
    pub(crate) fn new(bits: u32) -> Self {
        RESERVED9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED9_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED9` writer - 31:9\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | ((value as u32 & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "Field `FALLB` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct FALLB_R(crate::FieldReader<bool, bool>);
impl FALLB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FALLB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FALLB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FALLB` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct FALLB_W<'a> {
    w: &'a mut W,
}
impl<'a> FALLB_W<'a> {
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
#[doc = "Field `HIGHLIM` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub struct HIGHLIM_R(crate::FieldReader<u8, u8>);
impl HIGHLIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        HIGHLIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGHLIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIGHLIM` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub struct HIGHLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `LOWLIM` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct LOWLIM_R(crate::FieldReader<bool, bool>);
impl LOWLIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOWLIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOWLIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWLIM` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub struct LOWLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWLIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `OVR` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct OVR_R(crate::FieldReader<bool, bool>);
impl OVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CFG` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct CFG_R(crate::FieldReader<u8, u8>);
impl CFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFG` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:31 - 31:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fallb(&self) -> FALLB_R {
        FALLB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn highlim(&self) -> HIGHLIM_R {
        HIGHLIM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lowlim(&self) -> LOWLIM_R {
        LOWLIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fallb(&mut self) -> FALLB_W {
        FALLB_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn highlim(&mut self) -> HIGHLIM_W {
        HIGHLIM_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lowlim(&mut self) -> LOWLIM_W {
        LOWLIM_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W {
        CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashpumpp0](index.html) module"]
pub struct FLASHPUMPP0_SPEC;
impl crate::RegisterSpec for FLASHPUMPP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flashpumpp0::R](R) reader structure"]
impl crate::Readable for FLASHPUMPP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flashpumpp0::W](W) writer structure"]
impl crate::Writable for FLASHPUMPP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASHPUMPP0 to value 0"]
impl crate::Resettable for FLASHPUMPP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
