#[doc = "Register `FEFUSECTL` reader"]
pub struct R(crate::R<FEFUSECTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEFUSECTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEFUSECTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEFUSECTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEFUSECTL` writer"]
pub struct W(crate::W<FEFUSECTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEFUSECTL_SPEC>;
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
impl From<crate::W<FEFUSECTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEFUSECTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED27` reader - 31:27\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED27_R(crate::FieldReader<u8, u8>);
impl RESERVED27_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED27_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED27` writer - 31:27\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED27_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED27_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
#[doc = "Field `CHAIN_SEL` reader - 26:24\\]
Internal. Only to be used through TI provided API."]
pub struct CHAIN_SEL_R(crate::FieldReader<u8, u8>);
impl CHAIN_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHAIN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAIN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAIN_SEL` writer - 26:24\\]
Internal. Only to be used through TI provided API."]
pub struct CHAIN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAIN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `RESERVED18` reader - 23:18\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED18_R(crate::FieldReader<u8, u8>);
impl RESERVED18_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED18_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED18` writer - 23:18\\]
Internal. Only to be used through TI provided API."]
pub struct RESERVED18_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `WRITE_EN` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct WRITE_EN_R(crate::FieldReader<bool, bool>);
impl WRITE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITE_EN` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub struct WRITE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `BP_SEL` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct BP_SEL_R(crate::FieldReader<bool, bool>);
impl BP_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BP_SEL` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub struct BP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BP_SEL_W<'a> {
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
#[doc = "Field `EF_CLRZ` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct EF_CLRZ_R(crate::FieldReader<bool, bool>);
impl EF_CLRZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CLRZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CLRZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EF_CLRZ` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub struct EF_CLRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CLRZ_W<'a> {
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
#[doc = "Field `EF_TEST` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct EF_TEST_R(crate::FieldReader<bool, bool>);
impl EF_TEST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_TEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_TEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EF_TEST` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub struct EF_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_TEST_W<'a> {
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
#[doc = "Field `EFUSE_EN` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct EFUSE_EN_R(crate::FieldReader<u8, u8>);
impl EFUSE_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        EFUSE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EFUSE_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EFUSE_EN` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub struct EFUSE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:31 - 31:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved27(&self) -> RESERVED27_R {
        RESERVED27_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn chain_sel(&self) -> CHAIN_SEL_R {
        CHAIN_SEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn write_en(&self) -> WRITE_EN_R {
        WRITE_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bp_sel(&self) -> BP_SEL_R {
        BP_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
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
    pub fn ef_clrz(&self) -> EF_CLRZ_R {
        EF_CLRZ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ef_test(&self) -> EF_TEST_R {
        EF_TEST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuse_en(&self) -> EFUSE_EN_R {
        EFUSE_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:31 - 31:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved27(&mut self) -> RESERVED27_W {
        RESERVED27_W { w: self }
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn chain_sel(&mut self) -> CHAIN_SEL_W {
        CHAIN_SEL_W { w: self }
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&mut self) -> RESERVED18_W {
        RESERVED18_W { w: self }
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn write_en(&mut self) -> WRITE_EN_W {
        WRITE_EN_W { w: self }
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bp_sel(&mut self) -> BP_SEL_W {
        BP_SEL_W { w: self }
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
    pub fn ef_clrz(&mut self) -> EF_CLRZ_W {
        EF_CLRZ_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ef_test(&mut self) -> EF_TEST_W {
        EF_TEST_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuse_en(&mut self) -> EFUSE_EN_W {
        EFUSE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fefusectl](index.html) module"]
pub struct FEFUSECTL_SPEC;
impl crate::RegisterSpec for FEFUSECTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fefusectl::R](R) reader structure"]
impl crate::Readable for FEFUSECTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fefusectl::W](W) writer structure"]
impl crate::Writable for FEFUSECTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FEFUSECTL to value 0x0701_010a"]
impl crate::Resettable for FEFUSECTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0701_010a
    }
}
