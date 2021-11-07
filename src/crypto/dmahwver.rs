#[doc = "Register `DMAHWVER` reader"]
pub struct R(crate::R<DMAHWVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAHWVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAHWVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAHWVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAHWVER` writer"]
pub struct W(crate::W<DMAHWVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAHWVER_SPEC>;
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
impl From<crate::W<DMAHWVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAHWVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED28` reader - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
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
#[doc = "Field `HW_MAJOR_VER` reader - 27:24\\]
Major version number"]
pub struct HW_MAJOR_VER_R(crate::FieldReader<u8, u8>);
impl HW_MAJOR_VER_R {
    pub(crate) fn new(bits: u8) -> Self {
        HW_MAJOR_VER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_MAJOR_VER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HW_MAJOR_VER` writer - 27:24\\]
Major version number"]
pub struct HW_MAJOR_VER_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_MAJOR_VER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `HW_MINOR_VER` reader - 23:20\\]
Minor version number"]
pub struct HW_MINOR_VER_R(crate::FieldReader<u8, u8>);
impl HW_MINOR_VER_R {
    pub(crate) fn new(bits: u8) -> Self {
        HW_MINOR_VER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_MINOR_VER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HW_MINOR_VER` writer - 23:20\\]
Minor version number"]
pub struct HW_MINOR_VER_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_MINOR_VER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `HW_PATCH_LVL` reader - 19:16\\]
Patch level."]
pub struct HW_PATCH_LVL_R(crate::FieldReader<u8, u8>);
impl HW_PATCH_LVL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HW_PATCH_LVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HW_PATCH_LVL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HW_PATCH_LVL` writer - 19:16\\]
Patch level."]
pub struct HW_PATCH_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_PATCH_LVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `VER_NUM_COMPL` reader - 15:8\\]
Bit-by-bit complement of the VER_NUM field bits."]
pub struct VER_NUM_COMPL_R(crate::FieldReader<u8, u8>);
impl VER_NUM_COMPL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VER_NUM_COMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VER_NUM_COMPL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VER_NUM_COMPL` writer - 15:8\\]
Bit-by-bit complement of the VER_NUM field bits."]
pub struct VER_NUM_COMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> VER_NUM_COMPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `VER_NUM` reader - 7:0\\]
Version number of the DMA Controller (209)"]
pub struct VER_NUM_R(crate::FieldReader<u8, u8>);
impl VER_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        VER_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VER_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VER_NUM` writer - 7:0\\]
Version number of the DMA Controller (209)"]
pub struct VER_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> VER_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Major version number"]
    #[inline(always)]
    pub fn hw_major_ver(&self) -> HW_MAJOR_VER_R {
        HW_MAJOR_VER_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minor version number"]
    #[inline(always)]
    pub fn hw_minor_ver(&self) -> HW_MINOR_VER_R {
        HW_MINOR_VER_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Patch level."]
    #[inline(always)]
    pub fn hw_patch_lvl(&self) -> HW_PATCH_LVL_R {
        HW_PATCH_LVL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit complement of the VER_NUM field bits."]
    #[inline(always)]
    pub fn ver_num_compl(&self) -> VER_NUM_COMPL_R {
        VER_NUM_COMPL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Version number of the DMA Controller (209)"]
    #[inline(always)]
    pub fn ver_num(&self) -> VER_NUM_R {
        VER_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&mut self) -> RESERVED28_W {
        RESERVED28_W { w: self }
    }
    #[doc = "Bits 24:27 - 27:24\\]
Major version number"]
    #[inline(always)]
    pub fn hw_major_ver(&mut self) -> HW_MAJOR_VER_W {
        HW_MAJOR_VER_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Minor version number"]
    #[inline(always)]
    pub fn hw_minor_ver(&mut self) -> HW_MINOR_VER_W {
        HW_MINOR_VER_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Patch level."]
    #[inline(always)]
    pub fn hw_patch_lvl(&mut self) -> HW_PATCH_LVL_W {
        HW_PATCH_LVL_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Bit-by-bit complement of the VER_NUM field bits."]
    #[inline(always)]
    pub fn ver_num_compl(&mut self) -> VER_NUM_COMPL_W {
        VER_NUM_COMPL_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Version number of the DMA Controller (209)"]
    #[inline(always)]
    pub fn ver_num(&mut self) -> VER_NUM_W {
        VER_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Controller Version\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmahwver](index.html) module"]
pub struct DMAHWVER_SPEC;
impl crate::RegisterSpec for DMAHWVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmahwver::R](R) reader structure"]
impl crate::Readable for DMAHWVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmahwver::W](W) writer structure"]
impl crate::Writable for DMAHWVER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAHWVER to value 0x0101_2ed1"]
impl crate::Resettable for DMAHWVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0101_2ed1
    }
}
